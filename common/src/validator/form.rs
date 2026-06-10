use crate::utils::response::{ApiResponse, FieldError};
use crate::validator::validation_errors_to_fields;

use axum::{
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
    Form,
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
                ApiResponse::<()>::error_with_errors(
                    400,
                    "Form 参数解析失败",
                    vec![FieldError {
                        field: "form".into(),
                        message: e.to_string(),
                    }],
                )
                .into_response()
            })?;

            if let Err(err) = value.validate() {
                return Err(ApiResponse::<()>::error_with_errors(
                    400,
                    "Form 参数校验失败",
                    validation_errors_to_fields(err),
                )
                .into_response());
            };

            Ok(ValidatedForm(value))
        })
    }
}
