use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use common::auth::AuthUser;

/// 认证中间件
///
/// 从 `Authorization: Bearer <token>` 中提取 token 并校验。
/// 校验成功则将 [`AuthUser`] 注入 request extensions。
/// 校验失败返回 401。
///
/// # 使用
/// ```ignore
/// use ::middleware::auth::auth_middleware;
/// router.layer(middleware::from_fn(auth_middleware))
/// ```
///
/// # 扩展
/// 当前为骨架实现（仅校验 `token == "test-token"`），
/// 实际使用时替换 `validate_token` 函数为 JWT / Session 校验逻辑即可。
pub async fn auth_middleware(mut request: Request, next: Next) -> Response {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    match auth_header {
        Some(token) => match validate_token(token) {
            Some(user) => {
                request.extensions_mut().insert(user);
                next.run(request).await
            }
            None => {
                tracing::warn!("认证失败: 无效 token");
                (StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({
                    "code": 401,
                    "message": "无效的访问令牌"
                })))
                    .into_response()
            }
        },
        None => {
            tracing::warn!("认证失败: 缺少 Authorization 头");
            (StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({
                "code": 401,
                "message": "缺少 Authorization 头"
            })))
                .into_response()
        }
    }
}

/// Token 校验函数（骨架，替换为真实 JWT / Session 逻辑）
fn validate_token(token: &str) -> Option<AuthUser> {
    // 🔧 TODO: 替换为真实 JWT 解码逻辑，例如：
    //   jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)?
    if token == "test-token" {
        Some(AuthUser {
            id: 1,
            username: "admin".to_string(),
            role: "admin".to_string(),
        })
    } else {
        None
    }
}

// 从 request extensions 中获取当前用户（需在 `auth_middleware` 之后使用）：
//   async fn profile(Extension(user): Extension<AuthUser>) -> Json<AuthUser> { Json(user) }
// 或在 handler 内手动提取：
//   let Extension(user) = request.extensions().get::<AuthUser>().unwrap();