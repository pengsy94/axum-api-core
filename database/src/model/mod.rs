//! Eloquent 风格的 Model 门面层
//!
//! 将 SeaORM 的 Entity/ActiveModel 封装为类似 Laravel Eloquent 的 API。

use std::marker::PhantomData;
use common::error::AppError;
use common::utils::pagination::{PageParams, Paginated};
use sea_orm::*;
use serde::Serialize;

/// Eloquent 风格的 Model 门面
pub struct Model<E>(PhantomData<E>);


// ========================================
// 查询方法
// ========================================

impl<E: EntityTrait> Model<E>
where
    E::Model: FromQueryResult + Send + Sync,
{
    pub async fn all() -> Result<Vec<E::Model>, AppError> {
        let db = get_conn()?;
        E::find().all(&*db).await.map_err(|e| AppError::internal(e.to_string()))
    }

    pub async fn first() -> Result<Option<E::Model>, AppError> {
        let db = get_conn()?;
        E::find().one(&*db).await.map_err(|e| AppError::internal(e.to_string()))
    }

    pub async fn count() -> Result<u64, AppError> {
        let db = get_conn()?;
        E::find().count(&*db).await.map_err(|e| AppError::internal(e.to_string()))
    }

    pub fn query() -> Query<E> {
        Query::new()
    }
}


impl<E: EntityTrait> Model<E>
where
    E::Model: FromQueryResult + Send + Sync + Serialize,
{
    pub async fn paginate(params: &PageParams) -> Result<Paginated<E::Model>, AppError> {
        Self::query().paginate(params).await
    }
}


impl<E: EntityTrait> Model<E>
where
    E::Model: FromQueryResult + Send + Sync,
{
    pub async fn find<T>(id: T) -> Result<Option<E::Model>, AppError>
    where
        T: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
    {
        let db = get_conn()?;
        E::find_by_id(id).one(&*db).await.map_err(|e| AppError::internal(e.to_string()))
    }
}


impl<E: EntityTrait> Model<E>
where
    E::Model: IntoActiveModel<E::ActiveModel> + FromQueryResult + Send + Sync,
    E::ActiveModel: ActiveModelBehavior + Send,
{
    pub async fn create<F>(build: F) -> Result<E::Model, AppError>
    where
        F: FnOnce(&mut E::ActiveModel),
    {
        let db = get_conn()?;
        let mut model = E::ActiveModel::new();
        build(&mut model);
        model.insert(&*db).await.map_err(|e| AppError::internal(e.to_string()))
    }

    pub async fn update_by_id<T, F>(id: T, build: F) -> Result<E::Model, AppError>
    where
        T: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
        F: FnOnce(&mut E::ActiveModel),
        E::ActiveModel: From<E::Model>,
    {
        let db = get_conn()?;
        let model = E::find_by_id(id).one(&*db).await
            .map_err(|e| AppError::internal(e.to_string()))?
            .ok_or_else(|| AppError::not_found("记录不存在"))?;
        let mut active: E::ActiveModel = model.into();
        build(&mut active);
        active.update(&*db).await.map_err(|e| AppError::internal(e.to_string()))
    }

    pub async fn delete_by_id<T>(id: T) -> Result<DeleteResult, AppError>
    where
        T: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
    {
        let db = get_conn()?;
        E::delete_by_id(id).exec(&*db).await.map_err(|e| AppError::internal(e.to_string()))
    }
}




// ========================================
// 流式查询构建器
// ========================================

pub struct Query<E: EntityTrait> {
    condition: Condition,
    limit_val: Option<u64>,
    offset_val: Option<u64>,
    _phantom: PhantomData<E>,
}

impl<E: EntityTrait> Query<E>
where
    E::Model: FromQueryResult + Send + Sync,
{
    pub fn new() -> Self {
        Self { condition: Condition::all(), limit_val: None, offset_val: None, _phantom: PhantomData }
    }

    pub fn filter_eq(mut self, col: impl ColumnTrait, val: impl Into<Value>) -> Self {
        self.condition = self.condition.add(col.eq(val)); self
    }
    pub fn filter_ne(mut self, col: impl ColumnTrait, val: impl Into<Value>) -> Self {
        self.condition = self.condition.add(col.ne(val)); self
    }
    pub fn filter_gt(mut self, col: impl ColumnTrait, val: impl Into<Value>) -> Self {
        self.condition = self.condition.add(col.gt(val)); self
    }
    pub fn filter_gte(mut self, col: impl ColumnTrait, val: impl Into<Value>) -> Self {
        self.condition = self.condition.add(col.gte(val)); self
    }
    pub fn filter_lt(mut self, col: impl ColumnTrait, val: impl Into<Value>) -> Self {
        self.condition = self.condition.add(col.lt(val)); self
    }
    pub fn filter_lte(mut self, col: impl ColumnTrait, val: impl Into<Value>) -> Self {
        self.condition = self.condition.add(col.lte(val)); self
    }
    pub fn filter_like(mut self, col: impl ColumnTrait, val: &str) -> Self {
        self.condition = self.condition.add(col.like(val)); self
    }
    pub fn limit(mut self, n: u64) -> Self { self.limit_val = Some(n); self }
    pub fn offset(mut self, n: u64) -> Self { self.offset_val = Some(n); self }

    fn build_select(&self) -> Select<E> {
        let mut select = E::find().filter(self.condition.clone());
        if let Some(n) = self.limit_val { select = select.limit(n); }
        if let Some(n) = self.offset_val { select = select.offset(n); }
        select
    }

    pub async fn first(self) -> Result<Option<E::Model>, AppError> {
        let db = get_conn()?;
        self.build_select().one(&*db).await.map_err(|e| AppError::internal(e.to_string()))
    }

    pub async fn all(self) -> Result<Vec<E::Model>, AppError> {
        let db = get_conn()?;
        self.build_select().all(&*db).await.map_err(|e| AppError::internal(e.to_string()))
    }
}


impl<E: EntityTrait> Query<E>
where
    E::Model: FromQueryResult + Send + Sync + Serialize,
{
    pub async fn count(self) -> Result<u64, AppError> {
        let db = get_conn()?;
        E::find().filter(self.condition).count(&*db).await
            .map_err(|e| AppError::internal(e.to_string()))
    }

    pub async fn paginate(self, params: &PageParams) -> Result<Paginated<E::Model>, AppError> {
        let db = get_conn()?;
        let total = E::find().filter(self.condition.clone()).count(&*db).await
            .map_err(|e| AppError::internal(e.to_string()))?;
        let items = self.build_select()
            .limit(params.limit()).offset(params.offset())
            .all(&*db).await
            .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(Paginated::new(items, total, params))
    }

    pub async fn exists(self) -> Result<bool, AppError> {
        self.count().await.map(|c| c > 0)
    }
}


impl<E: EntityTrait> Default for Query<E>
where
    E::Model: FromQueryResult + Send + Sync,
{
    fn default() -> Self { Self::new() }
}


// ========================================
// 内部辅助
// ========================================

fn get_conn() -> Result<std::sync::Arc<DatabaseConnection>, AppError> {
    crate::get_db().ok_or_else(|| AppError::internal("数据库未初始化"))
}
