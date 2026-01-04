use common::request::system::LoginRequest;
use common::response::login::LoginResponse;
use common::utils::response::ApiResponse;
use common::validator::json::ValidatedJson;

pub async fn login(
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> ApiResponse<LoginResponse> {
    println!("{:?}", payload);

    ApiResponse::success(LoginResponse {
        token: String::from("晓风残月"),
    })
}
