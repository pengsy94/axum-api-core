use axum::response::Json;

use common::request::system::LoginRequest;
use common::response::login::LoginResponse;
use common::utils::response::ApiResponse;
use common::validator::json::ValidatedJson;

pub async fn login(
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    println!("{:?}", payload);

    ApiResponse::success(LoginResponse {
        token: String::from("晓风残月"),
    })
    // Json(LoginResponse {
    //     token: String::from("晓风残月"),
    // })
}
