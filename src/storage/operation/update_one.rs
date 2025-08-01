use crate::error::{MocksError, EXCEPTION_ERROR_MESSAGE};
use crate::storage::operation::build_search_resource_key;
use crate::storage::{Input, StorageData};
use serde_json::Value;

pub fn update_one(
    data: &mut StorageData,
    resource_key: &str,
    input: &Input,
) -> Result<Value, MocksError> {
    let search_resource_key = build_search_resource_key(data, resource_key);
    let resource = data
        .get_mut(&search_resource_key)
        .ok_or(MocksError::ObjectNotFound)?;
    update_target_with_input(resource, input)
}

fn update_target_with_input(value: &mut Value, input: &Input) -> Result<Value, MocksError> {
    match value {
        Value::Object(map) => {
            if let Value::Object(input_map) = input {
                map.extend(input_map.iter().map(|(k, v)| (k.clone(), v.clone())));
            } else {
                // Format error
                return Err(MocksError::Exception(EXCEPTION_ERROR_MESSAGE.to_string()));
            }

            Ok(value.clone())
        }
        _ => Err(MocksError::ObjectNotFound),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_update_one() {
        let mut data = json!({"profile":{"id":"user1","name":"John Smith","age":25}});
        let input = json!({"id":"user1","name":"Jane Smith","age":30});

        match update_one(&mut data, "profile", &input) {
            Ok(v) => {
                assert_eq!(v, json!({"id":"user1","name":"Jane Smith","age":30}));

                let updated_profile = &data["profile"];
                assert_eq!(
                    *updated_profile,
                    json!({"id":"user1","name":"Jane Smith","age":30})
                );
            }
            Err(e) => {
                panic!("panic in test_update_one: {e}");
            }
        }
    }

    #[test]
    fn test_update_one_nested_resource() {
        let mut data = json!({"api/v1/profile":{"id":"user1","name":"John Smith","age":25}});
        let input = json!({"id":"user1","name":"Jane Smith","age":30});

        match update_one(&mut data, "api/v1/profile", &input) {
            Ok(v) => {
                assert_eq!(v, json!({"id":"user1","name":"Jane Smith","age":30}));

                let updated_profile = &data["api/v1/profile"];
                assert_eq!(
                    *updated_profile,
                    json!({"id":"user1","name":"Jane Smith","age":30})
                );
            }
            Err(e) => {
                panic!("panic in test_update_one: {e}");
            }
        }
    }

    #[test]
    fn test_update_one_error() {
        let mut data = json!({"profile":{"id":"user1","name":"John Smith","age":25}});
        let input = json!({"id":"user1","name":"Jane Smith","age":30});

        match update_one(&mut data, "error", &input) {
            Ok(_v) => {
                panic!("panic in test_update_one_error")
            }
            Err(e) => {
                assert_eq!(e, MocksError::ObjectNotFound);
            }
        }
    }
}
