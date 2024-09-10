use crate::error::MocksError;
use crate::storage::reader::Reader;
use serde_json::Value;

mod reader;

/// Storage module
pub struct Storage {
    pub file: String,
}

impl Storage {
    pub fn new(path: &str) -> Storage {
        Storage {
            file: path.to_string(),
        }
    }

    pub fn read(&self) -> Result<Value, MocksError> {
        let v = Reader::new(&self.file).read()?;
        Ok(v)
    }
}
