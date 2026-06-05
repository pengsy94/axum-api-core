use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::utils::response::ApiResponse;

/// 统一应用错误类型
///
/// 所有 handler 返回 `Result<X, AppError>` 即可自动转换为标准 JSON 错误响应。
/// 内部错误通过 `?` 从 `anyhow::Error` 自动转换。
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// 401 未授权
    #[error("未授权: {0}")]
    Unauthorized(String),

    /// 403 禁止访问
    #[error("禁止访问: {0}")]
    Forbidden(String),

    /// 404 资源不存在
    #[error("资源不存在: {0}")]
    NotFound(String),

    /// 400 请求参数错误
    #[error("请求参数错误: {0}")]
    BadRequest(String),

    /// 409 冲突
    #[error("冲突: {0}")]
    Conflict(String),

    /// 500 内部服务器错误
    #[error("内部服务器错误: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, 401),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, 403),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, 404),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, 400),
            AppError::Conflict(_) => (StatusCode::CONFLICT, 409),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, 500),
        };

        let message = self.to_string();
        tracing::error!(%code, %message, "请求处理失败");

        // 先构建 ApiResponse JSON body，再覆写 HTTP 状态码
        let mut resp = ApiResponse::<()>::error(code, &message).into_response();
        *resp.status_mut() = status;
        resp
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

/// 便捷构造方法
impl AppError {
    /// 创建 401 未授权错误
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        AppError::Unauthorized(msg.into())
    }

    /// 创建 403 禁止访问错误
    pub fn forbidden(msg: impl Into<String>) -> Self {
        AppError::Forbidden(msg.into())
    }

    /// 创建 404 资源不存在错误
    pub fn not_found(msg: impl Into<String>) -> Self {
        AppError::NotFound(msg.into())
    }

    /// 创建 400 参数错误
    pub fn bad_request(msg: impl Into<String>) -> Self {
        AppError::BadRequest(msg.into())
    }

    /// 创建 500 内部错误
    pub fn internal(msg: impl Into<String>) -> Self {
        AppError::Internal(msg.into())
    }
}
