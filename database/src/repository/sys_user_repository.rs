use anyhow::{Result, anyhow};
use sea_orm::{IntoActiveModel, QueryFilter};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, TransactionTrait};

use crate::entity::sys_user;
use crate::entity::sys_user::ActiveModel;
use crate::get_db_unwrap;

pub async fn get_by_id(user_id: i32) -> Result<sys_user::Model> {
    let db = get_db_unwrap();
    let user = sys_user::Entity::find()
        .filter(sys_user::Column::Id.eq(user_id))
        .one(&*db)
        .await?;

    match user {
        Some(sys_user) => Ok(sys_user),
        None => Err(anyhow!("用户不存在")),
    }
}

pub async fn insert() -> Result<ActiveModel, anyhow::Error> {
    let db = get_db_unwrap();
    let txn = db.begin().await?;

    let user = ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(Some("李寻欢".to_owned())),
    }
    .save(&txn)
    .await?;

    txn.commit().await?;

    Ok(user)
}

pub async fn delete_by_id(id: i32) -> Result<u64, anyhow::Error> {
    let db = get_db_unwrap();
    let res = sys_user::Entity::delete_by_id(id).exec(&*db).await?;

    tracing::info!(rows_affected = res.rows_affected, "delete by id");
    Ok(res.rows_affected)
}

pub async fn edit_by_id(user_id: i32) -> Result<(), anyhow::Error> {
    let db = get_db_unwrap();

    let sys_user = get_by_id(user_id).await?;

    let mut active_model = sys_user.into_active_model();
    active_model.name = ActiveValue::Set(Some("修改后的用户名".to_owned()));
    active_model.update(&*db).await?;

    tracing::info!(user_id, "edit success");
    Ok(())
}
