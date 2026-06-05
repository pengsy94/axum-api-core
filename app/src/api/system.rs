use common::auth::{AuthUser, hash_password, sign_jwt, verify_password};
use common::request::system::LoginRequest;
use common::response::login::LoginResponse;
use common::utils::response::ApiResponse;
use common::validator::json::ValidatedJson;
use kernel::config::server_config;

/// 演示用户（TODO: 替换为数据库查询）
const DEMO_EMAIL: &str = "admin@example.com";
const DEMO_PASSWORD: &str = "admin123";

/// 用户登录
///
/// 使用邮箱和密码登录，返回 JWT token。
#[cfg_attr(
    feature = "openapi",
    utoipa::path(
        post,
        path = "/api/login",
        request_body = LoginRequest,
        responses(
            (status = 200, description = "登录成功", body = ApiResponse<LoginResponse>),
            (status = 400, description = "参数校验失败", body = common::utils::response::ErrorResponse),
        ),
    )
)]
/// 密码验证（使用懒初始化缓存哈希）
fn check_password(input: &str) -> bool {
    use std::sync::OnceLock;
    static HASH: OnceLock<String> = OnceLock::new();
    let hash = HASH.get_or_init(|| {
        hash_password(DEMO_PASSWORD).unwrap_or_else(|e| {
            tracing::error!("密码哈希失败: {}", e);
            String::new()
        })
    });
    verify_password(input, hash).unwrap_or(false)
}

pub async fn login(
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> ApiResponse<LoginResponse> {
    // 验证密码
    if !check_password(&payload.password) || payload.email != DEMO_EMAIL {
        return ApiResponse::error(401, "邮箱或密码错误");
    }

    // 签发 JWT
    let user = AuthUser {
        id: 1,
        username: "admin".to_string(),
        role: "admin".to_string(),
    };

    let config = server_config();
    let token = sign_jwt(&user, &config.jwt_secret)
        .unwrap_or_else(|_| "token_error".to_string());

    tracing::info!("用户 {} 登录成功", payload.email);

    let response = LoginResponse {
        token,
        token_type: "Bearer".to_string(),
        message: "登录成功".to_string(),
    };
    ApiResponse::success(response)
}
