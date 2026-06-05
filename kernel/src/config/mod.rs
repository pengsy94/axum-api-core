mod database_config;
pub mod error;
mod redis_config;
mod server_config;

use std::env;
use std::sync::OnceLock;

use dotenvy::{dotenv, from_filename};
use tracing::info;

use crate::config::{database_config::DatabaseConfig, redis_config::RedisConfig, server_config::ServerConfig};
use error::ConfigError;

/// 应用配置
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub server: ServerConfig,
}

// 全局配置单例
static CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    /// 初始化配置（应用启动时调用一次）
    pub fn init() -> Result<(), ConfigError> {
        // 如果已经初始化，返回错误
        if CONFIG.get().is_some() {
            return Err(ConfigError::AlreadyInitialized);
        }

        // ===== 按环境加载 .env 文件 =====
        // 优先级：已有环境变量 > .env.{APP_ENV} > .env
        let app_env = env::var("APP_ENV").unwrap_or_default();

        if !app_env.is_empty() {
            let env_file = format!(".env.{}", app_env);
            if let Err(e) = from_filename(&env_file) {
                info!("未找到 {}，仅加载 .env（{})", env_file, e);
            } else {
                info!("已加载环境配置: {}", env_file);
            }
        }

        dotenv().ok(); // .env 作为 fallback，不报错

        // 从环境变量创建配置
        let config = Self::from_env()?;

        // 设置全局单例
        CONFIG
            .set(config)
            .map_err(|_| ConfigError::AlreadyInitialized)
    }

    /// 从环境变量创建配置并校验
    fn from_env() -> Result<Self, ConfigError> {
        let server = ServerConfig::from_env()?;
        let database = DatabaseConfig::from_env()?;
        let redis = RedisConfig::from_env()?;

        // 校验配置值
        server.validate()?;
        database.validate()?;
        redis.validate()?;

        Ok(Self { server, database, redis })
    }

    /// 获取全局配置（初始化后使用）
    pub fn global() -> &'static AppConfig {
        CONFIG
            .get()
            .expect("Configuration not initialized. Call AppConfig::init() first")
    }

    /// 安全获取配置（不会 panic）
    pub fn try_global() -> Option<&'static AppConfig> {
        CONFIG.get()
    }
}

/// 便捷函数：获取服务器配置
pub fn server_config() -> &'static ServerConfig {
    &AppConfig::global().server
}

/// 便捷函数：获取数据库配置
pub fn database_config() -> &'static DatabaseConfig {
    &AppConfig::global().database
}

/// 便捷函数：获取 Redis 配置
pub fn redis_config() -> &'static RedisConfig {
    &AppConfig::global().redis
}
