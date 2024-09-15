use crate::error::MocksError;
use crate::error::MocksError::WriteError;
use serde_json::Value;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

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
        let file_path = match env::var("MOCKS_DEBUG_OVERWRITTEN_FILE") {
            Ok(debug_file) => debug_file,
            Err(_) => self.path.clone(),
        };

        let path = Path::new(&file_path);

        match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
        {
            Ok(mut file) => match serde_json::to_string_pretty(value) {
                Ok(json_string) => match file.write_all(json_string.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(WriteError(e.to_string())),
                },
                Err(e) => Err(WriteError(e.to_string())),
            },
            Err(e) => Err(WriteError(e.to_string())),
        }
    }
}
