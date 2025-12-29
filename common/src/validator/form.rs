use crate::response::error::{ErrorResponse, FieldError};
use crate::validator::validation_errors_to_fields;

use axum::{
    Form, Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedForm<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedForm<T>
where
    S: Send + Sync,
    T: Validate + DeserializeOwned,
{
    type Rejection = Response;

    fn from_request(
        req: Request,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        Box::pin(async move {
            let Form(value) = Form::<T>::from_request(req, state).await.map_err(|e| {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        code: 400,
                        message: "Form 参数解析失败".into(),
                        errors: Some(vec![FieldError {
                            field: "Form".into(),
                            message: e.to_string(),
                        }]),
                    }),
                )
                    .into_response();
            })?;

            if let Err(err) = value.validate() {
                return Err((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(ErrorResponse {
                        code: 422,
                        message: "Form 参数校验失败".into(),
                        errors: Some(validation_errors_to_fields(err)),
                    }),
                )
                    .into_response());
            };

            Ok(ValidatedForm(value))
        })
    }
}
