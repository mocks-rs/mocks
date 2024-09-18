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
    /// Create a new Storage instance
    ///
    /// # Arguments
    /// - `path` - The file path for storage
    /// - `overwrite` - Whether to overwrite the file on changes
    pub fn new(path: &str, overwrite: bool) -> Result<Storage, MocksError> {
        let data = Reader::new(path).read()?;
        Ok(Storage {
            file: path.to_string(),
            data,
            overwrite,
        })
    }

    /// **GET**
    /// Retrieve all items for a given resource
    pub fn get_all(&self, resource_key: &str) -> Option<Value> {
        select_all(&self.data, resource_key).ok()
    }

    /// **GET**
    /// Retrieve a specific item from a resource
    pub fn get_one(&self, resource_key: &str, item_key: &str) -> Option<Value> {
        select_one(&self.data, resource_key, item_key).ok()
    }

    /// **POST**
    /// Insert a new item into a resource
    pub fn insert(&mut self, resource_key: &str, input: &Value) -> Result<Value, MocksError> {
        self.operate(|data| insert(data, resource_key, input))
    }

    /// **PUT**
    /// Replace an entire item in a resource
    pub fn replace(
        &mut self,
        resource_key: &str,
        item_key: &str,
        input: &Value,
    ) -> Result<Value, MocksError> {
        self.operate(|data| replace(data, resource_key, item_key, input))
    }

    /// **PUT**
    /// Replace the first item in a resource
    pub fn replace_one(&mut self, resource_key: &str, input: &Value) -> Result<Value, MocksError> {
        self.operate(|data| replace_one(data, resource_key, input))
    }

    /// **PATCH**
    /// Update parts of an item in a resource
    pub fn update(
        &mut self,
        resource_key: &str,
        item_key: &str,
        input: &Value,
    ) -> Result<Value, MocksError> {
        self.operate(|data| update(data, resource_key, item_key, input))
    }

    /// **PATCH**
    /// Update parts of the first item in a resource
    pub fn update_one(&mut self, resource_key: &str, input: &Value) -> Result<Value, MocksError> {
        self.operate(|data| update_one(data, resource_key, input))
    }

    /// **DELETE**
    /// Delete an item from a resource
    pub fn delete(&mut self, resource_key: &str, item_key: &str) -> Result<Value, MocksError> {
        self.operate(|data| remove(data, resource_key, item_key))
    }

    /// Perform an operation on the storage data and write changes if successful
    ///
    /// This method abstracts the common pattern of performing an operation,
    /// writing the changes, and returning the result.
    fn operate<F>(&mut self, operation: F) -> Result<Value, MocksError>
    where
        F: FnOnce(&mut StorageData) -> Result<Value, MocksError>,
    {
        let result = operation(&mut self.data)?;
        self.write()?;
        Ok(result)
    }

    /// Write changes to the storage file if overwrite is enabled
    fn write(&mut self) -> Result<(), MocksError> {
        if self.overwrite {
            let writer = Writer::new(&self.file);
            writer.write(&self.data)?;
        }
        Ok(())
    }
}
