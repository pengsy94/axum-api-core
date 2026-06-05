use serde::{Deserialize, Serialize};

/// 认证用户信息
///
/// 通过认证中间件校验 token 后注入到 request extensions，
/// handler 中用 `Extension<AuthUser>` 提取当前用户。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i32,
    pub username: String,
    pub role: String,
}
