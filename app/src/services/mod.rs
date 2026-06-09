//! Service 层 — 类似 Laravel 的 Service 类
//!
//! 封装业务逻辑，处于 Controller 和 Model/Repository 之间。
//! 每个 Service 负责一个领域（User、Order 等）的业务逻辑。
//!
//! # 目录约定
//!
//! ```text
//! app/src/services/
//!   mod.rs              ← 本文件（模块声明 + Service trait）
//!   user_service.rs     ← 用户服务
//!   order_service.rs    ← 订单服务
//! ```

pub mod user_service;

/// Service trait — 所有 Service 的基础 trait
///
/// 提供统一的错误类型和数据库访问。
#[allow(async_fn_in_trait)]
pub trait Service {
    /// Service 错误类型
    type Error: std::fmt::Display;

    /// 获取 Service 名称（用于日志）
    fn name() -> &'static str;
}

