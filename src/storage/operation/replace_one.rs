use crate::error::MocksError;
use crate::storage::{Input, StorageData};
use serde_json::Value;

pub fn replace_one(
    data: &mut StorageData,
    resource_key: &str,
    input: &Input,
) -> Result<Value, MocksError> {
    match data[resource_key] {
        Value::Object(ref _map) => {
            data[resource_key] = input.to_owned();
            Ok(input.to_owned())
        }
        _ => Err(MocksError::ObjectNotFound()),
    }
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
                panic!(
                    "panic in test_replace_one_with_string_id: {}",
                    e.to_string()
                );
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
                assert_eq!(e.to_string(), MocksError::ObjectNotFound().to_string());
            }
        }
    }
}
