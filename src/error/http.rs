use super::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

/// The type sent as an error response to the client.
#[derive(Debug, Serialize, ToSchema)]
pub struct Errors {
    errors: Vec<String>,
}

/// Map error into a http response
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = http_status_code(&self);
        let errors = http_errors(&self);
        (status, Json(errors)).into_response()
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
fn http_errors(err: &Error) -> Errors {
    let errors = match err {
        Error::InvalidArgs { messages } => messages.to_owned(),
        Error::NotFound { message } => vec![message.to_owned()],
        Error::Internal { message } => {
            tracing::error!("internal error: {}", message);
            vec![message.to_owned()]
        }
    };
    Errors { errors }
}
