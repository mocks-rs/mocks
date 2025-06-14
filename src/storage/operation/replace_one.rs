use crate::error::MocksError;
use crate::storage::operation::{build_search_resource_key, extract_id_in_input};
use crate::storage::{Input, StorageData};
use serde_json::{Map, Value};

pub fn replace_one(
    data: &mut StorageData,
    resource_key: &str,
    input: &Input,
) -> Result<Value, MocksError> {
    extract_id_in_input(input)?;
    let valid_input = input.as_object().ok_or(MocksError::InvalidRequest)?;
    let search_resource_key = build_search_resource_key(data, resource_key);
    replace_target_with_map_input(data, &search_resource_key, valid_input.clone())
}

fn replace_target_with_map_input(
    data: &mut StorageData,
    resource_key: &str,
    map_input: Map<String, Value>,
) -> Result<Value, MocksError> {
    data.get_mut(resource_key)
        .and_then(Value::as_object_mut)
        .map(|v| {
            *v = map_input.clone();
            Value::Object(map_input)
        })
        .ok_or(MocksError::ObjectNotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_replace_one_with_string_id() {
        let mut data = json!({"profile":{"id":"user1","name":"John Smith","age":25}});
        let input = json!({"id":"user1","name":"Jane Smith","age":30});

        match replace_one(&mut data, "profile", &input) {
            Ok(v) => {
                assert_eq!(v, json!({"id":"user1","name":"Jane Smith","age":30}));

                let updated_value = &data["profile"];
                assert_eq!(
                    *updated_value,
                    json!({"id":"user1","name":"Jane Smith","age":30})
                );
            }
            Err(e) => {
                panic!("panic in test_replace_one_with_string_id: {}", e);
            }
        }
    }

    #[test]
    fn test_replace_one_nested_resource_with_string_id() {
        let mut data = json!({"api/v1/profile":{"id":"user1","name":"John Smith","age":25}});
        let input = json!({"id":"user1","name":"Jane Smith","age":30});

        match replace_one(&mut data, "api/v1/profile", &input) {
            Ok(v) => {
                assert_eq!(v, json!({"id":"user1","name":"Jane Smith","age":30}));

                let updated_value = &data["api/v1/profile"];
                assert_eq!(
                    *updated_value,
                    json!({"id":"user1","name":"Jane Smith","age":30})
                );
            }
            Err(e) => {
                panic!("panic in test_replace_one_with_string_id: {}", e);
            }
        }
    }

    #[test]
    fn test_replace_one_error_not_found() {
        let mut data = json!({"profile":{"id":"user1","name":"John Smith","age":25}});
        let input = json!({"id":"user1","name":"Jane Smith","age":30});

        match replace_one(&mut data, "error", &input) {
            Ok(_v) => {
                panic!("panic in test_replace_one_error_not_found")
            }
            Err(e) => {
                assert_eq!(e, MocksError::ObjectNotFound);
            }
        }
    }
}
