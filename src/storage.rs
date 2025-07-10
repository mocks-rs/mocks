use crate::error::MocksError;
use crate::storage::operation::insert::insert;
use crate::storage::operation::remove::remove;
use crate::storage::operation::replace::replace;
use crate::storage::operation::replace_one::replace_one;
use crate::storage::operation::select_all::select_all;
use crate::storage::operation::select_one::select_one;
use crate::storage::operation::select_with_filter::select_with_filter;
use crate::storage::operation::update::update;
use crate::storage::operation::update_one::update_one;
use crate::storage::reader::Reader;
use crate::storage::writer::Writer;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

mod operation;
mod reader;
mod writer;

pub type StorageData = Value;
pub type Input = Value;

/// Storage module
#[derive(Clone)]
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

    /// Initialize a new storage file
    ///
    /// # Arguments
    /// - `file_path` - The path where the storage file will be created
    /// - `empty` - Whether to create an empty structure or include sample data
    pub fn init_file(file_path: &str, empty: bool) -> Result<(), MocksError> {
        Self::init_file_with_overwrite(file_path, empty, false)
    }

    /// Initialize a new storage file with overwrite control
    ///
    /// # Arguments
    /// - `file_path` - The path where the storage file will be created
    /// - `empty` - Whether to create an empty structure or include sample data
    /// - `force_overwrite` - Whether to overwrite existing files without prompting
    pub fn init_file_with_overwrite(
        file_path: &str,
        empty: bool,
        force_overwrite: bool,
    ) -> Result<(), MocksError> {
        let path = Path::new(file_path);

        if path.exists() && !force_overwrite {
            print!("File {file_path} already exists. Overwrite? (y/N): ");
            io::stdout()
                .flush()
                .map_err(|e| MocksError::Exception(format!("Failed to flush stdout: {e}")))?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| MocksError::InvalidArgs(format!("Failed to read input: {e}")))?;

            if !input.trim().to_lowercase().starts_with('y') {
                return Err(MocksError::Aborted);
            }
        }

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| {
                    MocksError::InvalidArgs(format!("Failed to create directory: {e}"))
                })?;
            }
        }

        let data = if empty {
            serde_json::json!({
                "posts": [],
                "profile": {}
            })
        } else {
            serde_json::json!({
                "posts": [
                    {
                        "id": 1,
                        "title": "Hello World",
                        "content": "This is a sample post"
                    }
                ],
                "profile": {
                    "id": 1,
                    "name": "Sample User"
                }
            })
        };

        let writer = Writer::new(file_path);
        writer.write(&data)?;

        Ok(())
    }

    /// Resources for API endpoints
    pub fn resources(&self) -> Vec<String> {
        let mut resources = vec![];
        if let Value::Object(obj) = &self.data {
            for (key, val) in obj {
                if !key.is_empty() && (val.is_object() || val.is_array()) {
                    resources.push(key.to_string());
                }
            }
        }

        resources
    }

    /// **GET**
    /// Retrieve all items for a given resource
    pub fn get_all(&self, resource_key: &str) -> Result<Value, MocksError> {
        self.fetch(|data| select_all(data, resource_key))
    }

    /// **GET**
    /// Retrieve filtered items for a given resource
    pub fn get_all_with_filter(
        &self,
        resource_key: &str,
        filters: &HashMap<String, String>,
    ) -> Result<Value, MocksError> {
        self.fetch(|data| select_with_filter(data, resource_key, filters))
    }

    /// **GET**
    /// Retrieve a specific item from a resource
    pub fn get_one(&self, resource_key: &str, item_key: &str) -> Result<Value, MocksError> {
        self.fetch(|data| select_one(data, resource_key, item_key))
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

    /// Fetches data from the storage using the provided operation
    ///
    /// This method abstracts the common pattern of performing a fetch operation,
    /// and returning the result.
    fn fetch<F>(&self, operation: F) -> Result<Value, MocksError>
    where
        F: FnOnce(&StorageData) -> Result<Value, MocksError>,
    {
        let result = operation(&self.data)?;
        Ok(result)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_init_file_creates_default_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let file_path_str = file_path.to_str().unwrap();

        let result = Storage::init_file(file_path_str, false);
        assert!(result.is_ok());

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("\"posts\""));
        assert!(content.contains("\"Hello World\""));
        assert!(content.contains("\"profile\""));
        assert!(content.contains("\"Sample User\""));
    }

    #[test]
    fn test_init_file_creates_empty_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let file_path_str = file_path.to_str().unwrap();

        let result = Storage::init_file(file_path_str, true);
        assert!(result.is_ok());

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("\"posts\": []"));
        assert!(content.contains("\"profile\": {}"));
        assert!(!content.contains("Hello World"));
        assert!(!content.contains("Sample User"));
    }

    #[test]
    fn test_init_file_creates_directories() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("sub").join("dir").join("test.json");
        let file_path_str = file_path.to_str().unwrap();

        let result = Storage::init_file(file_path_str, false);
        assert!(result.is_ok());

        assert!(file_path.exists());
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("\"posts\""));
    }

    #[test]
    fn test_init_file_with_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("existing.json");
        let file_path_str = file_path.to_str().unwrap();

        fs::write(&file_path, "existing content").unwrap();
        assert!(file_path.exists());

        // Test with force_overwrite = true to avoid interactive prompt
        let result = Storage::init_file_with_overwrite(file_path_str, false, true);
        assert!(result.is_ok());

        // Verify the file was overwritten with new content
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("\"posts\""));
        assert!(content.contains("\"profile\""));
        assert!(!content.contains("existing content"));
    }

    #[test]
    fn test_init_file_with_force_overwrite_false() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("existing.json");
        let _file_path_str = file_path.to_str().unwrap();

        fs::write(&file_path, "existing content").unwrap();
        assert!(file_path.exists());

        // Test with force_overwrite = false
        // Note: This test would hang in a real scenario requiring user input,
        // but we can't easily test the interactive behavior in unit tests
        // The function should work correctly when force_overwrite is false
        // and user input is provided through stdin
        // When user aborts, it should return MocksError::Aborted
    }

    #[test]
    fn test_init_file_with_empty_and_force_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("existing.json");
        let file_path_str = file_path.to_str().unwrap();

        fs::write(&file_path, "existing content").unwrap();
        assert!(file_path.exists());

        // Test with empty = true and force_overwrite = true
        let result = Storage::init_file_with_overwrite(file_path_str, true, true);
        assert!(result.is_ok());

        // Verify the file was overwritten with empty content
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("\"posts\": []"));
        assert!(content.contains("\"profile\": {}"));
        assert!(!content.contains("existing content"));
        assert!(!content.contains("Hello World"));
    }

    #[test]
    fn test_resources_with_objects_and_arrays() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let file_path_str = file_path.to_str().unwrap();

        let test_data = serde_json::json!({
            "posts": [
                {"id": 1, "title": "Test"}
            ],
            "users": [],
            "profile": {
                "name": "Test User"
            },
            "settings": {}
        });

        fs::write(&file_path, test_data.to_string()).unwrap();
        let storage = Storage::new(file_path_str, false).unwrap();

        let resources = storage.resources();
        assert_eq!(resources.len(), 4);
        assert!(resources.contains(&"posts".to_string()));
        assert!(resources.contains(&"users".to_string()));
        assert!(resources.contains(&"profile".to_string()));
        assert!(resources.contains(&"settings".to_string()));
    }

    #[test]
    fn test_resources_excludes_empty_keys() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let file_path_str = file_path.to_str().unwrap();

        let test_data = serde_json::json!({
            "posts": [{"id": 1}],
            "": {"invalid": "key"},
            "users": {}
        });

        fs::write(&file_path, test_data.to_string()).unwrap();
        let storage = Storage::new(file_path_str, false).unwrap();

        let resources = storage.resources();
        assert_eq!(resources.len(), 2);
        assert!(resources.contains(&"posts".to_string()));
        assert!(resources.contains(&"users".to_string()));
        assert!(!resources.contains(&"".to_string()));
    }

    #[test]
    fn test_resources_excludes_primitive_values() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let file_path_str = file_path.to_str().unwrap();

        let test_data = serde_json::json!({
            "posts": [{"id": 1}],
            "title": "String value",
            "count": 42,
            "active": true,
            "nullable": null,
            "users": {}
        });

        fs::write(&file_path, test_data.to_string()).unwrap();
        let storage = Storage::new(file_path_str, false).unwrap();

        let resources = storage.resources();
        assert_eq!(resources.len(), 2);
        assert!(resources.contains(&"posts".to_string()));
        assert!(resources.contains(&"users".to_string()));
        assert!(!resources.contains(&"title".to_string()));
        assert!(!resources.contains(&"count".to_string()));
        assert!(!resources.contains(&"active".to_string()));
        assert!(!resources.contains(&"nullable".to_string()));
    }

    #[test]
    fn test_resources_with_empty_resource_list() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let file_path_str = file_path.to_str().unwrap();

        // Create data with at least one object/array to pass Reader validation
        // but also include some primitive values to ensure they're excluded
        let test_data = serde_json::json!({
            "posts": [],
            "title": "String value",
            "count": 42,
            "active": true
        });

        fs::write(&file_path, test_data.to_string()).unwrap();
        let storage = Storage::new(file_path_str, false).unwrap();

        let resources = storage.resources();
        assert_eq!(resources.len(), 1);
        assert!(resources.contains(&"posts".to_string()));
        assert!(!resources.contains(&"title".to_string()));
        assert!(!resources.contains(&"count".to_string()));
        assert!(!resources.contains(&"active".to_string()));
    }

    #[test]
    fn test_resources_with_non_object_root_data() {
        // Test resources() method behavior when self.data is not an object
        // This is an edge case scenario where we directly test the method logic
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let file_path_str = file_path.to_str().unwrap();

        // Start with valid data to create Storage instance
        let test_data = serde_json::json!({
            "posts": [{"id": 1}]
        });

        fs::write(&file_path, test_data.to_string()).unwrap();
        let mut storage = Storage::new(file_path_str, false).unwrap();

        // Manually modify the data to be non-object for testing the method
        storage.data = serde_json::json!([1, 2, 3]);

        let resources = storage.resources();
        assert_eq!(resources.len(), 0);

        // Test with primitive value
        storage.data = serde_json::json!("string");
        let resources = storage.resources();
        assert_eq!(resources.len(), 0);
    }

    #[test]
    fn test_storage_new_with_invalid_root_data() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let file_path_str = file_path.to_str().unwrap();

        // Test with array at root level - this should fail in Reader validation
        let test_data = serde_json::json!([
            {"id": 1, "name": "Item 1"},
            {"id": 2, "name": "Item 2"}
        ]);

        fs::write(&file_path, test_data.to_string()).unwrap();
        let storage_result = Storage::new(file_path_str, false);
        assert!(storage_result.is_err());

        // Test with primitive value at root level - this should fail in Reader validation
        let test_data = serde_json::json!("string value");
        fs::write(&file_path, test_data.to_string()).unwrap();
        let storage_result = Storage::new(file_path_str, false);
        assert!(storage_result.is_err());

        // Test with number at root level - this should fail in Reader validation
        let test_data = serde_json::json!(42);
        fs::write(&file_path, test_data.to_string()).unwrap();
        let storage_result = Storage::new(file_path_str, false);
        assert!(storage_result.is_err());
    }
}
