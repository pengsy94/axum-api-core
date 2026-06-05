use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[derive(Deserialize, Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub token_type: String,
    pub message: String,
}