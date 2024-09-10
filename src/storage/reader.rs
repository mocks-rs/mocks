use crate::error::MocksError;
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Storage file reader
pub struct Reader {
    path: String,
}

impl Reader {
    pub fn new(path: &str) -> Reader {
        Self {
            path: path.to_string(),
        }
    }

    pub fn read(self) -> Result<Value, MocksError> {
        let path = Path::new(&self.path);

        match fs::read_to_string(path) {
            Ok(text) => match serde_json::from_str(&text) {
                Ok(v) => Ok(v),
                Err(e) => Err(MocksError::ReadError(e.to_string())),
            },
            Err(e) => Err(MocksError::ReadError(e.to_string())),
        }
    }
}
