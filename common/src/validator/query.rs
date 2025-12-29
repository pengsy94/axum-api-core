use crate::response::error::{ErrorResponse, FieldError};
use crate::validator::validation_errors_to_fields;

use axum::{
    Json,
    extract::{FromRequestParts, Query},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedQuery<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidatedQuery<T>
where
    S: Send + Sync,
    T: Validate + DeserializeOwned,
{
    type Rejection = Response;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        Box::pin(async move {
            let Query(value) = Query::<T>::from_request_parts(parts, state)
                .await
                .map_err(|e| {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(ErrorResponse {
                            code: 400,
                            message: "Query 参数解析失败".into(),
                            errors: Some(vec![FieldError {
                                field: "query".into(),
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
                        message: "Query 参数校验失败".into(),
                        errors: Some(validation_errors_to_fields(err)),
                    }),
                )
                    .into_response());
            };

            Ok(ValidatedQuery(value))
        })
    }
}
