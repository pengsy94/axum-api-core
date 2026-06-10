use kernel::config::redis_config;
use redis::aio::ConnectionManager as RedisConnectionManager;
use redis::AsyncCommands;
use std::sync::{Arc, Mutex};

static REDIS_POOL: Mutex<Option<Arc<RedisConnectionManager>>> = Mutex::new(None);

pub struct RedisManager;

impl RedisManager {
    pub async fn init() -> Result<(), String> {
        let config = redis_config();
        if !config.enabled {
            println!("✺ the REDIS_URL is not configured; Redis initialization is skipped.");
            return Ok(());
        }
        let client =
            redis::Client::open(config.url.as_str()).map_err(|e| format!("Redis 连接字符串无效: {}", e))?;
        let conn = RedisConnectionManager::new(client)
            .await
            .map_err(|e| format!("Redis 连接失败: {}", e))?;
        *REDIS_POOL.lock().unwrap() = Some(Arc::new(conn));
        println!("✅ Redis connection pool initialized successfully!");
        Ok(())
    }

    pub fn get() -> Option<Arc<RedisConnectionManager>> {
        REDIS_POOL.lock().unwrap().clone()
    }

    /// 检查 Redis 连接是否存活
    pub async fn ping() -> bool {
        match Self::get() {
            Some(conn) => {
                let mut conn = conn.as_ref().clone();
                redis::cmd("PING")
                    .query_async::<String>(&mut conn)
                    .await
                    .is_ok()
            }
            None => false,
        }
    }

    pub fn close() {
        let mut guard = REDIS_POOL.lock().unwrap();
        if guard.is_some() {
            guard.take();
            println!("📌 the Redis connection pool has been closed.");
        }
    }
}

pub fn get_redis() -> Option<Arc<RedisConnectionManager>> {
    RedisManager::get()
}

// ========================================
// 便捷缓存操作 Cache
// ========================================

/// 缓存操作（对 Redis 的便捷封装）
///
/// # 用法
/// ```ignore
/// use database::Cache;
///
/// // 字符串
/// Cache::set("key", "value").await?;
/// let val: Option<String> = Cache::get("key").await?;
/// Cache::setex("key", 3600, "value").await?;
/// Cache::del("key").await?;
///
/// // 哈希
/// Cache::hset("user:1", "name", "张三").await?;
/// let name: Option<String> = Cache::hget("user:1", "name").await?;
///
/// // 判断
/// let ok = Cache::exists("key").await?;
/// ```
pub struct Cache;

impl Cache {
    fn conn() -> Result<RedisConnectionManager, String> {
        RedisManager::get()
            .ok_or_else(|| "Redis 未连接".to_string())
            .map(|arc| arc.as_ref().clone())
    }

    pub async fn get(key: &str) -> Result<Option<String>, String> {
        let mut conn = Self::conn()?;
        conn.get(key).await.map_err(|e| format!("Redis GET 失败: {}", e))
    }

    pub async fn set(key: &str, value: &str) -> Result<(), String> {
        let mut conn = Self::conn()?;
        conn.set(key, value).await.map_err(|e| format!("Redis SET 失败: {}", e))
    }

    pub async fn setex(key: &str, ttl: u64, value: &str) -> Result<(), String> {
        let mut conn = Self::conn()?;
        conn.set_ex(key, value, ttl)
            .await
            .map_err(|e| format!("Redis SETEX 失败: {}", e))
    }

    pub async fn del(key: &str) -> Result<(), String> {
        let mut conn = Self::conn()?;
        conn.del(key).await.map_err(|e| format!("Redis DEL 失败: {}", e))
    }

    pub async fn exists(key: &str) -> Result<bool, String> {
        let mut conn = Self::conn()?;
        conn.exists(key).await.map_err(|e| format!("Redis EXISTS 失败: {}", e))
    }

    pub async fn expire(key: &str, ttl: u64) -> Result<(), String> {
        let mut conn = Self::conn()?;
        conn.expire(key, ttl as i64)
            .await
            .map_err(|e| format!("Redis EXPIRE 失败: {}", e))
    }

    pub async fn hget(key: &str, field: &str) -> Result<Option<String>, String> {
        let mut conn = Self::conn()?;
        conn.hget(key, field)
            .await
            .map_err(|e| format!("Redis HGET 失败: {}", e))
    }

    pub async fn hset(key: &str, field: &str, value: &str) -> Result<(), String> {
        let mut conn = Self::conn()?;
        conn.hset(key, field, value)
            .await
            .map_err(|e| format!("Redis HSET 失败: {}", e))
    }

    pub async fn hdel(key: &str, field: &str) -> Result<(), String> {
        let mut conn = Self::conn()?;
        conn.hdel(key, field)
            .await
            .map_err(|e| format!("Redis HDEL 失败: {}", e))
    }
}
