pub mod entity;
pub mod repository;

pub struct DatabaseManager;

use kernel::config::database_config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

// 使用 Mutex 存储数据库连接池（支持优雅关闭）
static DB_POOL: Mutex<Option<Arc<DatabaseConnection>>> = Mutex::new(None);

impl DatabaseManager {
    /// 初始化全局数据库连接（应用启动时调用）
    pub async fn init() -> Result<(), sea_orm::DbErr> {
        let config = database_config();

        // 未配置数据库时跳过初始化
        if !config.enabled {
            tracing::info!("未配置 DATABASE_URL，跳过数据库初始化");
            return Ok(());
        }

        let mut opt = ConnectOptions::new(config.database_url.to_owned());
        opt.max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .connect_timeout(Duration::from_secs(config.connect_timeout_seconds as u64))
            .idle_timeout(Duration::from_secs(config.connect_timeout_seconds as u64))
            .sqlx_logging(false);

        let connection = Database::connect(opt).await?;
        *DB_POOL.lock().unwrap() = Some(Arc::new(connection));
        Ok(())
    }

    /// 获取数据库连接（可在任何地方调用）
    pub fn get() -> Option<Arc<DatabaseConnection>> {
        DB_POOL.lock().unwrap().clone()
    }

    /// 获取数据库连接，如果未初始化则panic
    pub fn get_unwrap() -> Arc<DatabaseConnection> {
        DB_POOL.lock().unwrap().clone().expect("Database not initialized")
    }

    /// 关闭数据库连接池（优雅关闭时调用）
    pub fn close() {
        let mut guard = DB_POOL.lock().unwrap();
        if guard.is_some() {
            guard.take();
            tracing::info!("数据库连接池已关闭");
        }
    }
}

// 为了方便使用，提供全局函数
pub fn get_db() -> Option<Arc<DatabaseConnection>> {
    DatabaseManager::get()
}

pub fn get_db_unwrap() -> Arc<DatabaseConnection> {
    DatabaseManager::get_unwrap()
}
