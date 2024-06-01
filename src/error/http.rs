use super::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// The type sent as an error response to the client.
#[derive(Debug, Serialize)]
struct ErrorDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: Vec<String>,
}

impl ErrorDto {
    /// Create an error DTO with a single error message.
    pub fn one(message: &str) -> Self {
        Self {
            error: Some(message.to_string()),
            errors: Default::default(),
        }
    }

    /// Create an error DTO with multiple error messages.
    pub fn many(messages: &[String]) -> Self {
        Self {
            error: None,
            errors: messages.to_owned(),
        }
    }
}

/// Map error into a http response
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = http_status_code(&self);
        let error = http_error_dto(&self);
        if status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!("internal error: {}", self);
        }
        (status, Json(error)).into_response()
    }
}

/// Map error types for handlers that only return status codes.
impl From<Error> for StatusCode {
    fn from(err: Error) -> Self {
        let status = http_status_code(&err);
        if status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!("internal error: {}", err);
        }
        status
    }
}

/// Get the http status code for an error.
fn http_status_code(err: &Error) -> StatusCode {
    match err {
        Error::NotFound { .. } => StatusCode::NOT_FOUND,
        Error::InvalidArgs { .. } => StatusCode::BAD_REQUEST,
        Error::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Get response type for an error.
fn http_error_dto(err: &Error) -> ErrorDto {
    match err {
        Error::InvalidArgs { messages } => ErrorDto::many(messages),
        Error::NotFound { message } => ErrorDto::one(message),
        Error::Internal { message } => ErrorDto::one(message),
    }
}
