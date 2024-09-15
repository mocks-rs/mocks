use std::fmt;

pub const EXCEPTION_ERROR_MESSAGE: &str = "An unexpected error occurred.";

#[derive(Debug)]
pub enum MocksError {
    ReadError(String),
    WriteError(String),
    ArgsError(String),
    ResourceNotFound(),
    ObjectNotFound(),
    ExceptionError(),
}

impl fmt::Display for MocksError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReadError(err) => write!(fmt, "{err}"),
            Self::WriteError(err) => write!(fmt, "{err}"),
            Self::ArgsError(err) => write!(fmt, "{err}"),
            Self::ResourceNotFound() => write!(fmt, "Resource not found."),
            Self::ObjectNotFound() => write!(fmt, "Object not found."),
            Self::ExceptionError() => write!(fmt, "An unexpected error occurred."),
        }
    }
}
