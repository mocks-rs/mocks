use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::fmt;

pub const EXCEPTION_ERROR_MESSAGE: &str = "An unexpected error occurred.";

#[derive(Debug, Eq, PartialEq)]
pub enum MocksError {
    FailedReadFile(String),
    FailedWriteFile(String),
    InvalidArgs(String),
    Exception(String),
    ResourceNotFound,
    ObjectNotFound,
    MethodNotAllowed,
    InvalidRequest,
    DuplicateId,
}

impl std::error::Error for MocksError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl fmt::Display for MocksError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FailedReadFile(err) => write!(fmt, "{err}"),
            Self::FailedWriteFile(err) => write!(fmt, "{err}"),
            Self::InvalidArgs(err) => write!(fmt, "{err}"),
            Self::Exception(err) => write!(fmt, "{err}"),
            Self::ResourceNotFound => write!(fmt, "Resource not found."),
            Self::ObjectNotFound => write!(fmt, "Object not found."),
            Self::MethodNotAllowed => write!(fmt, "Method not allowed."),
            Self::InvalidRequest => write!(fmt, "Invalid request."),
            Self::DuplicateId => write!(fmt, "Duplicate ID."),
        }
    }
}
impl IntoResponse for MocksError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            MocksError::FailedReadFile(err)
            | MocksError::FailedWriteFile(err)
            | MocksError::InvalidArgs(err)
            | MocksError::Exception(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            MocksError::ResourceNotFound | MocksError::ObjectNotFound => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
            MocksError::MethodNotAllowed => (StatusCode::METHOD_NOT_ALLOWED, self.to_string()),
            MocksError::InvalidRequest => (StatusCode::BAD_REQUEST, self.to_string()),
            MocksError::DuplicateId => (StatusCode::CONFLICT, self.to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
