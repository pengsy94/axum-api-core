use common::request::system::LoginRequest;
use common::response::login::LoginResponse;
use common::utils::response::ApiResponse;
use common::validator::json::ValidatedJson;

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
pub async fn login(
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> ApiResponse<LoginResponse> {
    tracing::info!("{:?}", payload);

    let response = LoginResponse {
        token: String::from("冢中枯骨，吾早晚必擒之！"),
        token_type: String::from("追比圣贤，本是读书人的愿望！"),
        message: String::from("为天地立心，为生民立命，为往圣继绝学，为万世开太平!"),
    };
    ApiResponse::success(response)
}
