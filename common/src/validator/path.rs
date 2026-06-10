use crate::utils::response::{ApiResponse, FieldError};
use crate::validator::validation_errors_to_fields;

use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedPath<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidatedPath<T>
where
    S: Send + Sync,
    T: Validate + DeserializeOwned + Send,
{
    type Rejection = Response;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        Box::pin(async move {
            let Path(value) = Path::<T>::from_request_parts(parts, state)
                .await
                .map_err(|e| {
                    ApiResponse::<()>::error_with_errors(
                        400,
                        "Path 参数解析失败",
                        vec![FieldError {
                            field: "path".into(),
                            message: e.to_string(),
                        }],
                    )
                    .into_response()
                })?;

            if let Err(err) = value.validate() {
                return Err(ApiResponse::<()>::error_with_errors(
                    400,
                    "Path 参数校验失败",
                    validation_errors_to_fields(err),
                )
                .into_response());
            };

            Ok(ValidatedPath(value))
        })
    }
}
