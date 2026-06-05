use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250301_000001_create_sys_user_table::SysUser;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysOrder::Table)
                    .if_not_exists()
                    .col(pk_auto(SysOrder::Id))
                    .col(string_null(SysOrder::Title))
                    .col(integer_null(SysOrder::UserId))
                    .col(
                        ColumnDef::new(SysOrder::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_user_id")
                            .from(SysOrder::Table, SysOrder::UserId)
                            .to(SysUser::Table, SysUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysOrder::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysOrder {
    Table,
    Id,
    Title,
    UserId,
    CreatedAt,
}
