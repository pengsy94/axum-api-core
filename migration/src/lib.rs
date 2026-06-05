pub use sea_orm_migration::prelude::*;

mod m20250301_000001_create_sys_user_table;
mod m20250301_000002_create_sys_order_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250301_000001_create_sys_user_table::Migration),
            Box::new(m20250301_000002_create_sys_order_table::Migration),
        ]
    }
}
