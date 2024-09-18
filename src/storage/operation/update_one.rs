use crate::error::MocksError;
use crate::storage::{Input, StorageData};
use serde_json::Value;

pub fn update_one(
    data: &mut StorageData,
    resource_key: &str,
    input: &Input,
) -> Result<Value, MocksError> {
    let resource = data
        .get_mut(resource_key)
        .ok_or(MocksError::ObjectNotFound)?;

    match resource {
        Value::Object(map) => {
            if let Value::Object(input_map) = input {
                map.extend(input_map.iter().map(|(k, v)| (k.clone(), v.clone())));
            } else {
                // フォーマットエラー
                return Err(MocksError::Exception("".to_string()));
            }

            Ok(resource.clone())
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
                panic!("panic in test_update_one: {}", e.to_string());
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
                assert_eq!(e.to_string(), MocksError::ObjectNotFound.to_string());
            }
        }
    }
}
