use crate::config::error::ConfigError;
use std::env;

/// Redis 配置
#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub enabled: bool,
    pub url: String,
    pub pool_size: u32,
}

impl RedisConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let url = env::var("REDIS_URL").unwrap_or_default();
        let enabled = !url.trim().is_empty();

        let pool_size = env::var("REDIS_POOL_SIZE")
            .unwrap_or_else(|_| "4".to_string())
            .parse::<u32>()
            .map_err(|e| ConfigError::InvalidValue("REDIS_POOL_SIZE".to_string(), e.to_string()))?;

        Ok(Self { enabled, url, pool_size })
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}
