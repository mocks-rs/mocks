use crate::error::MocksError;
use crate::storage::operation::insert::insert;
use crate::storage::operation::remove::remove;
use crate::storage::operation::replace::replace;
use crate::storage::operation::replace_one::replace_one;
use crate::storage::operation::select_all::select_all;
use crate::storage::operation::select_one::select_one;
use crate::storage::operation::update::update;
use crate::storage::operation::update_one::update_one;
use crate::storage::reader::Reader;
use crate::storage::writer::Writer;
use serde_json::Value;

mod operation;
mod reader;
mod writer;

pub type StorageData = Value;
pub type Input = Value;

/// Storage module
pub struct Storage {
    pub file: String,
    pub data: StorageData,
    pub overwrite: bool,
}

impl Storage {
    pub fn new(path: &str, overwrite: bool) -> Result<Storage, MocksError> {
        let data = Reader::new(path).read()?;
        Ok(Storage {
            file: path.to_string(),
            data,
            overwrite,
        })
    }

    /// GET method for list
    pub fn get_all(&self, resource_key: &str) -> Option<Value> {
        select_all(&self.data, resource_key).ok()
    }

    /// GET method for item
    pub fn get_one(&self, resource_key: &str, item_key: &str) -> Option<Value> {
        select_one(&self.data, resource_key, item_key).ok()
    }

    /// POST method
    pub fn upsert(&mut self, resource_key: &str, input: &Value) -> Result<Value, MocksError> {
        match insert(&mut self.data, resource_key, input) {
            Ok(value) => {
                self.write()?;
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    /// PUT method
    pub fn replace(
        &mut self,
        resource_key: &str,
        item_key: &str,
        input: &Value,
    ) -> Result<Value, MocksError> {
        match replace(&mut self.data, resource_key, item_key, input) {
            Ok(value) => {
                self.write()?;
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    pub fn replace_one(&mut self, resource_key: &str, input: &Value) -> Result<Value, MocksError> {
        match replace_one(&mut self.data, resource_key, input) {
            Ok(value) => {
                self.write()?;
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    /// PATCH method
    pub fn patch(
        &mut self,
        resource_key: &str,
        item_key: &str,
        input: &Value,
    ) -> Result<Value, MocksError> {
        match update(&mut self.data, resource_key, item_key, input) {
            Ok(value) => {
                self.write()?;
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    pub fn patch_one(&mut self, resource_key: &str, input: &Value) -> Result<Value, MocksError> {
        match update_one(&mut self.data, resource_key, input) {
            Ok(value) => {
                self.write()?;
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    pub fn delete(&mut self, resource_key: &str, item_key: &str) -> Result<Value, MocksError> {
        match remove(&mut self.data, resource_key, item_key) {
            Ok(value) => {
                self.write()?;
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    fn write(&mut self) -> Result<(), MocksError> {
        if self.overwrite {
            let writer = Writer::new(&self.file);
            writer.write(&self.data)?;
        }
        Ok(())
    }
}
