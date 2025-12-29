use axum::response::Json;

use common::request::system::LoginRequest;
use common::response::login::LoginResponse;
use common::validator::json::ValidatedJson;

pub async fn login(ValidatedJson(payload): ValidatedJson<LoginRequest>) -> Json<LoginResponse> {

    println!("{:?}", payload);

    Json(LoginResponse {
        token: String::from("晓风残月"),
    })
}
