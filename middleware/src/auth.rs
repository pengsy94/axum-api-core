use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use common::auth::verify_jwt;
use kernel::config::server_config;

/// 认证中间件
///
/// 从 `Authorization: Bearer <token>` 中提取 JWT 并校验。
/// 校验成功则将 [`AuthUser`] 注入 request extensions。
/// 校验失败返回 401。
///
/// # 使用
/// ```ignore
/// router.layer(middleware::from_fn(auth_middleware))
/// ```
pub async fn auth_middleware(mut request: Request, next: Next) -> Response {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    match auth_header {
        Some(token) => {
            let config = server_config();
            match verify_jwt(token, &config.jwt_secret) {
                Ok(user) => {
                    request.extensions_mut().insert(user);
                    next.run(request).await
                }
                Err(msg) => {
                    tracing::warn!("JWT 认证失败: {}", msg);
                    (StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({
                        "code": 401,
                        "message": msg,
                    })))
                        .into_response()
                }
            }
        }
        None => {
            tracing::warn!("认证失败: 缺少 Authorization 头");
            (StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({
                "code": 401,
                "message": "缺少 Authorization 头",
            })))
                .into_response()
        }
    }
}

// 在 handler 中提取当前用户：
//   use axum::Extension;
//   use common::auth::AuthUser;
//   async fn handler(Extension(user): Extension<AuthUser>) -> Json<...> { ... }
