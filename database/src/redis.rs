use kernel::config::redis_config;
use redis::aio::ConnectionManager as RedisConnectionManager;
use std::sync::{Arc, Mutex};

static REDIS_POOL: Mutex<Option<Arc<RedisConnectionManager>>> = Mutex::new(None);

pub struct RedisManager;

impl RedisManager {
    /// 初始化 Redis 连接
    pub async fn init() -> Result<(), String> {
        let config = redis_config();

        if !config.enabled {
            tracing::info!("未配置 REDIS_URL，跳过 Redis 初始化");
            return Ok(());
        }

        let client = redis::Client::open(config.url.as_str())
            .map_err(|e| format!("Redis 连接字符串无效: {}", e))?;

        let conn = RedisConnectionManager::new(client)
            .await
            .map_err(|e| format!("Redis 连接失败: {}", e))?;

        *REDIS_POOL.lock().unwrap() = Some(Arc::new(conn));
        tracing::info!("Redis 连接已建立");
        Ok(())
    }

    /// 获取 Redis 连接
    pub fn get() -> Option<Arc<RedisConnectionManager>> {
        REDIS_POOL.lock().unwrap().clone()
    }

    /// 关闭 Redis 连接
    pub fn close() {
        let mut guard = REDIS_POOL.lock().unwrap();
        if guard.is_some() {
            guard.take();
            println!("❌ Redis 连接已关闭");
        }
    }
}

/// 全局便捷函数
pub fn get_redis() -> Option<Arc<RedisConnectionManager>> {
    RedisManager::get()
}
