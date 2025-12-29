use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
    pub errors: Option<Vec<FieldError>>,
}

#[derive(Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}
