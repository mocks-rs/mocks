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
            Ok(text) => match serde_json::from_str::<Value>(&text) {
                Ok(v) => match v.as_object() {
                    None => Err(MocksError::FailedReadFile(
                        "Storage file contains invalid JSON format.".to_string(),
                    )),
                    Some(m) => {
                        for (_, v) in m {
                            if !v.is_object() && !v.is_array() {
                                return Err(MocksError::FailedReadFile(
                                    "Storage file contains invalid JSON format.".to_string(),
                                ));
                            }
                        }

                        Ok(v)
                    }
                },
                Err(e) => Err(MocksError::FailedReadFile(e.to_string())),
            },
            Err(e) => Err(MocksError::FailedReadFile(e.to_string())),
        }
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
