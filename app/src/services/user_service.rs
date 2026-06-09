//! 用户服务
//!
//! 封装用户相关的业务逻辑。
//!
//! # 使用
//! ```ignore
//! let service = UserService::new();
//! let users = service.list(&params).await?;
//! let user = service.show(1).await?;
//! ```

use common::error::AppError;
use common::utils::pagination::{PageParams, Paginated};
use database::entity::sys_user;
use database::model::Model;

use super::Service;

/// 用户服务
pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self
    }
}

impl Service for UserService {
    type Error = AppError;

    fn name() -> &'static str {
        "UserService"
    }
}

// ========================================
// 业务方法
// ========================================

type UserModel = Model<sys_user::Entity>;

impl UserService {
    /// 获取用户列表（分页）
    pub async fn list(&self, params: &PageParams) -> Result<Paginated<sys_user::Model>, AppError> {
        UserModel::paginate(params).await
    }

    /// 获取用户详情
    pub async fn show(&self, id: i32) -> Result<Option<sys_user::Model>, AppError> {
        UserModel::find(id).await
    }

    /// 创建用户
    pub async fn store(
        &self,
        name: impl Into<String>,
    ) -> Result<sys_user::Model, AppError> {
        UserModel::create(|m: &mut sys_user::ActiveModel| {
            m.name = sea_orm::Set(Some(name.into()));
        })
        .await
    }

    /// 更新用户
    pub async fn update(
        &self,
        id: i32,
        name: impl Into<String>,
    ) -> Result<sys_user::Model, AppError> {
        UserModel::update_by_id(id, |m: &mut sys_user::ActiveModel| {
            m.name = sea_orm::Set(Some(name.into()));
        })
        .await
    }

    /// 删除用户
    pub async fn destroy(&self, id: i32) -> Result<(), AppError> {
        UserModel::delete_by_id(id).await?;
        Ok(())
    }

    /// 根据名称搜索用户
    pub async fn search_by_name(
        &self,
        name: &str,
    ) -> Result<Vec<sys_user::Model>, AppError> {
        UserModel::query()
            .filter_like(sys_user::Column::Name, &format!("%{}%", name))
            .all()
            .await
    }
}
