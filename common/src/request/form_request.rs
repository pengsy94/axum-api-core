//! FormRequest 模式 — 类似 Laravel 的 FormRequest
//!
//! 将请求验证逻辑封装到独立的 Request 结构体中，
//! 支持 `rules()` 和 `authorize()` 方法。
//!
//! # 快速使用
//!
//! ```ignore
//! use common::request::form_request::FormRequest;
//! use validator::Validate;
//!
//! #[derive(Debug, Deserialize, Validate)]
//! pub struct StoreUserRequest {
//!     #[validate(length(min = 1, message = "名称不能为空"))]
//!     pub name: String,
//!     #[validate(email(message = "无效的邮箱"))]
//!     pub email: String,
//! }
//!
//! impl FormRequest for StoreUserRequest {
//!     fn authorize(&self) -> Result<(), AppError> {
//!         // 权限检查，默认允许
//!         Ok(())
//!     }
//! }
//!
//! // 在 handler 中使用
//! // async fn store(ValidatedJson(req): ValidatedJson<StoreUserRequest>) -> ...
//! ```

use crate::error::AppError;

/// FormRequest trait — 类似 Laravel 的 FormRequest
///
/// 配合 `#[derive(Validate)]` 使用，在 validator crate 提供的验证规则之外
/// 增加了 `authorize()` 权限检查。
///
/// # 完整的 FormRequest 示例
///
/// ```ignore
/// use common::request::form_request::FormRequest;
/// use serde::Deserialize;
/// use validator::Validate;
///
/// #[derive(Debug, Deserialize, Validate)]
/// pub struct UpdateUserRequest {
///     #[validate(length(min = 1, max = 50))]
///     pub name: Option<String>,
///
///     #[validate(email)]
///     pub email: Option<String>,
///
///     #[validate(range(min = 0, max = 150))]
///     pub age: Option<i32>,
/// }
///
/// impl FormRequest for UpdateUserRequest {
///     fn authorize(&self) -> Result<(), AppError> {
///         // 检查当前用户是否有权限
///         // let user: &AuthUser = ...;
///         // if user.role != "admin" {
///         //     return Err(AppError::forbidden("无权限"));
///         // }
///         Ok(())
///     }
///
///     fn messages() -> Option<Vec<(&'static str, &'static str)>> {
///         Some(vec![
///             ("name", "请输入有效的用户名"),
///             ("email", "请输入有效的邮箱地址"),
///         ])
///     }
/// }
/// ```
pub trait FormRequest {
    /// 权限检查（类似 Laravel 的 `authorize()`）
    ///
    /// 返回 `Ok(())` 表示通过，返回 `Err(AppError)` 表示拒绝。
    /// 默认允许所有请求。
    fn authorize(&self) -> Result<(), AppError> {
        Ok(())
    }

    /// 自定义验证消息（可选，类似 Laravel 的 `messages()`）
    fn messages() -> Option<Vec<(&'static str, &'static str)>> {
        None
    }

    /// 验证前的数据预处理（可选，类似 Laravel 的 `prepareForValidation()`）
    fn prepare_for_validation(&mut self) {}
}

/// 便捷宏：快速定义一个 FormRequest
///
/// # Example
/// ```ignore
/// form_request!(StoreUserRequest, {
///     name: String => [length(min = 1)],
///     email: String => [email],
/// });
/// ```
#[macro_export]
macro_rules! form_request {
    // 简单形式
    ($name:ident, { $($field:ident: $ty:ty => [$($rule:meta),* $(,)?]),* $(,)? }) => {
        #[derive(Debug, serde::Deserialize, validator::Validate)]
        pub struct $name {
            $(
                $(#[$rule])*
                pub $field: $ty,
            )*
        }

        impl $crate::request::form_request::FormRequest for $name {}
    };
}

// Re-export for convenience
pub use super::super::validator::json::ValidatedJson;
