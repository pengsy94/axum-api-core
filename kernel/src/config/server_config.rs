use crate::config::error::ConfigError;
use std::env;
use std::net::IpAddr;

/// 服务器配置
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub debug: bool,
    pub host: IpAddr,
    pub port: u16,
    pub content_gzip: bool,
    /// 是否开启定时任务
    pub cron: bool,
    /// 是否开启ws
    pub ws_open: bool,
    pub ws_path: String,
    /// `log_level` 日志输出等级 TRACE DEBUG INFO  WARN ERROR
    pub log_level: String,
    /// `dir` 日志输出文件夹
    pub log_dir: String,
    /// `file` 日志输出文件名
    pub log_file: String,
    /// 允许操作日志输出
    pub log_enable_oper_log: bool,
    /// 是否启用限流
    pub rate_limit_enabled: bool,
    /// CORS 允许的来源（逗号分隔，"*" 表示允许所有）
    pub cors_allowed_origins: String,
}

impl ServerConfig {
    /// 从环境变量创建服务器配置
    pub fn from_env() -> Result<Self, ConfigError> {
        let debug = env::var("DEBUG")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .map_err(|_| ConfigError::MissingEnvVar("DEBUG".to_string()))?;

        let host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string())
            .parse::<IpAddr>()
            .map_err(|e| ConfigError::InvalidValue("SERVER_HOST".to_string(), e.to_string()))?;

        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|e| ConfigError::InvalidValue("SERVER_PORT".to_string(), e.to_string()))?;

        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "DEBUG".to_string());

        let content_gzip = env::var("SERVER_CONTENT_GZIP")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .map_err(|e| ConfigError::InvalidValue("SERVER_CONTENT_GZIP".to_string(), e.to_string()))?;

        let cron = env::var("SERVER_CRON")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .map_err(|e| ConfigError::InvalidValue("SERVER_CRON".to_string(), e.to_string()))?;

        let ws_open = env::var("SERVER_WS_OPEN")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .map_err(|_| ConfigError::MissingEnvVar("SERVER_WS_OPEN".to_string()))?;

        let ws_path = env::var("SERVER_WS_PATH")
            .unwrap_or_else(|_| "/ws".to_string());

        let log_dir = env::var("LOG_DIR")
            .unwrap_or_else(|_| "logs".to_string());

        let log_file = env::var("LOG_FILE")
            .unwrap_or_else(|_| "axum_log".to_string());

        let log_enable_oper_log = env::var("LOG_ENABLE_OPER_LOG")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .map_err(|_| ConfigError::MissingEnvVar("LOG_ENABLE_OPER_LOG".to_string()))?;

        let rate_limit_enabled = env::var("SERVER_RATE_LIMIT_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .map_err(|e| ConfigError::InvalidValue("SERVER_RATE_LIMIT_ENABLED".to_string(), e.to_string()))?;

        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "*".to_string());

        Ok(Self {
            debug,
            host,
            port,
            content_gzip,
            cron,
            ws_open,
            ws_path,
            log_level,
            log_dir,
            log_file,
            log_enable_oper_log,
            rate_limit_enabled,
            cors_allowed_origins,
        })
    }

    /// 校验配置值合法性
    pub fn validate(&self) -> Result<(), ConfigError> {
        // 校验日志级别
        let valid_levels = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR"];
        if !valid_levels.contains(&self.log_level.to_uppercase().as_str()) {
            return Err(ConfigError::ValidationError(format!(
                "LOG_LEVEL 必须为 {:?} 之一，当前为: {}",
                valid_levels, self.log_level
            )));
        }

        Ok(())
    }
}
