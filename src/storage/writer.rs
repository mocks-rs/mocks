use crate::error::MocksError;
use serde_json::Value;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

const ENV_KEY: &str = "MOCKS_DEBUG_OVERWRITTEN_FILE";

pub struct Writer {
    path: String,
}

impl Writer {
    pub fn new(path: &str) -> Writer {
        Self {
            path: path.to_string(),
        }
    }

    pub fn write(&self, value: &Value) -> Result<(), MocksError> {
        // Check debug mode
        let file_path = env::var(ENV_KEY).unwrap_or_else(|_| self.path.clone());
        let path = Path::new(&file_path);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| MocksError::FailedWriteFile(e.to_string()))?;

        let json_string = serde_json::to_string_pretty(value)
            .map_err(|e| MocksError::FailedWriteFile(e.to_string()))?;

        file.write_all(json_string.as_bytes())
            .map_err(|e| MocksError::FailedWriteFile(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_read() {
        let writer = Writer::new("storage.test.json");
        let value = json!({"posts":[{"id":"test1","title":"first post","views":100}]});
        assert_eq!(writer.write(&value).is_ok(), true);
    }
}
