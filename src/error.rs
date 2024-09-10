use std::fmt;

#[derive(Debug)]
pub enum MocksError {
    ReadError(String),
    ArgsError(String),
}

impl fmt::Display for MocksError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReadError(err) => write!(fmt, "{err}"),
            Self::ArgsError(err) => write!(fmt, "{err}"),
        }
    }
}
