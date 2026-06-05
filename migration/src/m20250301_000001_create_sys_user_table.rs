use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysUser::Table)
                    .if_not_exists()
                    .col(pk_auto(SysUser::Id))
                    .col(string_null(SysUser::Name))
                    .col(string_null(SysUser::Email))
                    .col(string_null(SysUser::PasswordHash))
                    .col(string_null(SysUser::Role))
                    .col(
                        ColumnDef::new(SysUser::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SysUser::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysUser {
    Table,
    Id,
    Name,
    Email,
    PasswordHash,
    Role,
    CreatedAt,
    UpdatedAt,
}
