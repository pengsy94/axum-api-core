use crate::config::error::ConfigError;
use std::env;

/// 数据库配置
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub enabled: bool,
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_seconds: u32,
}

impl DatabaseConfig {
    /// 从环境变量创建数据库配置
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| String::new());

        let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .map_err(|e| {
                ConfigError::InvalidValue("DATABASE_MAX_CONNECTIONS".to_string(), e.to_string())
            })?;

        let min_connections = env::var("DATABASE_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "2".to_string())
            .parse::<u32>()
            .map_err(|e| {
                ConfigError::InvalidValue("DATABASE_MIN_CONNECTIONS".to_string(), e.to_string())
            })?;

        let connect_timeout_seconds = env::var("DATABASE_CONNECT_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u32>()
            .map_err(|e| {
                ConfigError::InvalidValue("DATABASE_CONNECT_TIMEOUT".to_string(), e.to_string())
            })?;

        let enabled = !database_url.trim().is_empty();

        Ok(Self {
            enabled,
            database_url,
            max_connections,
            min_connections,
            connect_timeout_seconds,
        })
    }

    /// 校验配置值合法性
    pub fn validate(&self) -> Result<(), ConfigError> {
        // 未配置数据库时跳过校验
        if !self.enabled {
            return Ok(());
        }

        if self.max_connections < self.min_connections {
            return Err(ConfigError::ValidationError(format!(
                "DATABASE_MAX_CONNECTIONS ({}) 不能小于 DATABASE_MIN_CONNECTIONS ({})",
                self.max_connections, self.min_connections
            )));
        }

        if self.connect_timeout_seconds == 0 {
            return Err(ConfigError::ValidationError(
                "DATABASE_CONNECT_TIMEOUT 必须大于 0".to_string(),
            ));
        }

        Ok(())
    }
}
