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
    QueryParamsNotAllowed,
    InvalidSearchValue,
    InvalidMatchType,
    InvalidQueryParam,
    MatchTypeRequired,
    Aborted,
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
            Self::QueryParamsNotAllowed => write!(fmt, "Query parameters not allowed."),
            Self::InvalidSearchValue => {
                write!(fmt, "Cannot search on complex values (objects or arrays).")
            }
            Self::InvalidMatchType => write!(
                fmt,
                "Invalid match type. Use: exact, startswith, endswith, contains."
            ),
            Self::InvalidQueryParam => write!(fmt, "Invalid query parameter format."),
            Self::MatchTypeRequired => write!(fmt, "Match type is required. Use: field.exact, field.startswith, field.endswith, or field.contains."),
            Self::Aborted => write!(fmt, "Operation aborted by user."),
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
            MocksError::QueryParamsNotAllowed => (StatusCode::BAD_REQUEST, self.to_string()),
            MocksError::InvalidSearchValue => (StatusCode::BAD_REQUEST, self.to_string()),
            MocksError::InvalidMatchType => (StatusCode::BAD_REQUEST, self.to_string()),
            MocksError::InvalidQueryParam => (StatusCode::BAD_REQUEST, self.to_string()),
            MocksError::MatchTypeRequired => (StatusCode::BAD_REQUEST, self.to_string()),
            MocksError::Aborted => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use std::error::Error;

    #[test]
    fn test_display_implementation() {
        let read_error = MocksError::FailedReadFile("Failed to read file".to_string());
        assert_eq!(read_error.to_string(), "Failed to read file");

        let write_error =
            MocksError::FailedWriteFile("Failed to write file".to_string());
        assert_eq!(write_error.to_string(), "Failed to write file");

        let invalid_args = MocksError::InvalidArgs("Invalid arguments".to_string());
        assert_eq!(invalid_args.to_string(), "Invalid arguments");

        let exception = MocksError::Exception("Unexpected error occurred".to_string());
        assert_eq!(exception.to_string(), "Unexpected error occurred");

        assert_eq!(
            MocksError::ResourceNotFound.to_string(),
            "Resource not found."
        );
        assert_eq!(MocksError::ObjectNotFound.to_string(), "Object not found.");
        assert_eq!(
            MocksError::MethodNotAllowed.to_string(),
            "Method not allowed."
        );
        assert_eq!(MocksError::InvalidRequest.to_string(), "Invalid request.");
        assert_eq!(MocksError::DuplicateId.to_string(), "Duplicate ID.");
    }

    #[test]
    fn test_into_response_implementation() {
        // Internal error tests
        let read_error = MocksError::FailedReadFile("Failed to read file".to_string());
        let response = read_error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        // 404 error test
        let not_found = MocksError::ResourceNotFound;
        let response = not_found.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // 405 error test
        let method_not_allowed = MocksError::MethodNotAllowed;
        let response = method_not_allowed.into_response();
        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);

        // 400 error test
        let invalid_request = MocksError::InvalidRequest;
        let response = invalid_request.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // 409 error test
        let duplicate_id = MocksError::DuplicateId;
        let response = duplicate_id.into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[test]
    fn test_error_equality() {
        let resource_not_found_error = MocksError::ResourceNotFound;
        let other_resource_not_found_error = MocksError::ResourceNotFound;
        let object_not_found_error = MocksError::ObjectNotFound;

        assert_eq!(resource_not_found_error, other_resource_not_found_error);
        assert_ne!(resource_not_found_error, object_not_found_error);
    }

    #[test]
    fn test_error_source() {
        let error = MocksError::ResourceNotFound;
        assert!(error.source().is_none());
    }

    #[test]
    fn test_error_response_body() {
        let error = MocksError::ResourceNotFound;
        let response = error.into_response();

        // Verify response body content
        let (parts, _) = response.into_parts();
        assert_eq!(parts.status, StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_all_error_variants() {
        // FailedReadFile
        let error = MocksError::FailedReadFile("test error".to_string());
        assert_eq!(error.to_string(), "test error");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        // FailedWriteFile
        let error = MocksError::FailedWriteFile("test error".to_string());
        assert_eq!(error.to_string(), "test error");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        // InvalidArgs
        let error = MocksError::InvalidArgs("test error".to_string());
        assert_eq!(error.to_string(), "test error");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        // Exception
        let error = MocksError::Exception("test error".to_string());
        assert_eq!(error.to_string(), "test error");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        // ResourceNotFound
        let error = MocksError::ResourceNotFound;
        assert_eq!(error.to_string(), "Resource not found.");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // ObjectNotFound
        let error = MocksError::ObjectNotFound;
        assert_eq!(error.to_string(), "Object not found.");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // MethodNotAllowed
        let error = MocksError::MethodNotAllowed;
        assert_eq!(error.to_string(), "Method not allowed.");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);

        // InvalidRequest
        let error = MocksError::InvalidRequest;
        assert_eq!(error.to_string(), "Invalid request.");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // DuplicateId
        let error = MocksError::DuplicateId;
        assert_eq!(error.to_string(), "Duplicate ID.");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);

        // QueryParamsNotAllowed
        let error = MocksError::QueryParamsNotAllowed;
        assert_eq!(error.to_string(), "Query parameters not allowed.");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // InvalidSearchValue
        let error = MocksError::InvalidSearchValue;
        assert_eq!(
            error.to_string(),
            "Cannot search on complex values (objects or arrays)."
        );
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // InvalidMatchType
        let error = MocksError::InvalidMatchType;
        assert_eq!(
            error.to_string(),
            "Invalid match type. Use: exact, startswith, endswith, contains."
        );
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // InvalidQueryParam
        let error = MocksError::InvalidQueryParam;
        assert_eq!(error.to_string(), "Invalid query parameter format.");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // MatchTypeRequired
        let error = MocksError::MatchTypeRequired;
        assert_eq!(error.to_string(), "Match type is required. Use: field.exact, field.startswith, field.endswith, or field.contains.");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // Aborted
        let error = MocksError::Aborted;
        assert_eq!(error.to_string(), "Operation aborted by user.");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
