use crate::error::MocksError;
use serde_json::Value;
use std::fs;
use std::path::Path;

const INVALID_JSON_FORMAT_ERROR: &str = "Storage file contains invalid JSON format.";

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
        let text =
            fs::read_to_string(path).map_err(|e| MocksError::FailedReadFile(e.to_string()))?;

        let value: Value =
            serde_json::from_str(&text).map_err(|e| MocksError::FailedReadFile(e.to_string()))?;

        let obj = value
            .as_object()
            .ok_or_else(|| MocksError::FailedReadFile(INVALID_JSON_FORMAT_ERROR.to_string()))?;

        if obj.values().any(|v| !v.is_object() && !v.is_array()) {
            return Err(MocksError::FailedReadFile(
                INVALID_JSON_FORMAT_ERROR.to_string(),
            ));
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let reader = Reader::new("storage.json");

        match reader.read() {
            Ok(v) => {
                if v.is_object() {
                    assert!(true);
                } else {
                    panic!("panic in test_read");
                }
            }
            Err(e) => {
                panic!("panic in test_read: {}", e.to_string());
            }
        }
    }
}
