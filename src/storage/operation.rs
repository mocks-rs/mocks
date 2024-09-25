use crate::error::MocksError;
use crate::storage::{Input, StorageData};
use serde_json::Value;

pub mod insert;
pub mod remove;
pub mod replace;
pub mod replace_one;
pub mod select_all;
pub mod select_one;
pub mod update;
pub mod update_one;

pub fn extract_id_in_input(input: &Input) -> Result<String, MocksError> {
    input
        .get("id")
        .and_then(|v| match v {
            Value::Number(id) => Some(id.to_string()),
            Value::String(id) => Some(id.to_string()),
            _ => None,
        })
        .ok_or(MocksError::InvalidRequest)
}

pub fn check_duplicate_id(
    data: &StorageData,
    resource_key: &str,
    id: &str,
) -> Result<(), MocksError> {
    if select_one::select_one(data, resource_key, id).is_ok() {
        Err(MocksError::DuplicateId)
    } else {
        Ok(())
    }
}

pub fn extract_array_resource(
    data: &StorageData,
    resource_key: &str,
) -> Result<Vec<Value>, MocksError> {
    data.get(resource_key)
        .and_then(Value::as_array)
        .ok_or(MocksError::ResourceNotFound)
        .cloned()
}
