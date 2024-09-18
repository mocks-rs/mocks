use std::fmt;

pub const EXCEPTION_ERROR_MESSAGE: &str = "An unexpected error occurred.";

#[derive(Debug)]
pub enum MocksError {
    FailedReadFile(String),
    FailedWriteFile(String),
    InvalidArgs(String),
    Exception(String),
    ObjectNotFound(),
    MethodNotAllowed(),
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
            Self::ObjectNotFound() => write!(fmt, "Object not found."),
            Self::MethodNotAllowed() => write!(fmt, "Method not allowed."),
        }
    }
}
