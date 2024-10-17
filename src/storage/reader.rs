use crate::error::MocksError;
use serde_json::Value;
use std::fs;
use std::path::Path;

const INVALID_JSON_FORMAT_ERROR: &str = "Storage file is invalid JSON format.";
const UNABLE_TO_GEN_API: &str = "Unable to generate API endpoints.";

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

        // Allow only Object or Array
        if obj
            .iter()
            .filter(|(k, _)| !k.is_empty())
            .any(|(_, v)| v.is_object() || v.is_array())
        {
            Ok(value)
        } else {
            Err(MocksError::FailedReadFile(UNABLE_TO_GEN_API.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_read() {
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
