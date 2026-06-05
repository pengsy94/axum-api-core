use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// ========================================
// JWT
// ========================================

/// JWT 载荷
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// 用户 ID
    pub sub: i32,
    /// 用户名
    pub username: String,
    /// 角色
    pub role: String,
    /// 签发时间（Unix 时间戳）
    pub iat: u64,
    /// 过期时间（Unix 时间戳）
    pub exp: u64,
}

/// 签发 JWT token
pub fn sign_jwt(user: &AuthUser, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        role: user.role.clone(),
        iat: now,
        exp: now + 86400, // 24 小时过期
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

/// 验证 JWT token，返回用户信息
pub fn verify_jwt(token: &str, secret: &str) -> Result<AuthUser, String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| format!("Token 无效: {}", e))?;

    Ok(AuthUser {
        id: token_data.claims.sub,
        username: token_data.claims.username,
        role: token_data.claims.role,
    })
}

// ========================================
// 密码哈希
// ========================================

/// 对密码进行 argon2 哈希
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

/// 验证密码是否匹配 argon2 哈希
pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

// ========================================
// 认证用户
// ========================================

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
