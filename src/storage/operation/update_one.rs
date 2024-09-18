use crate::error::MocksError;
use crate::storage::{Input, StorageData};
use serde_json::Value;

pub fn update_one(
    data: &mut StorageData,
    resource_key: &str,
    input: &Input,
) -> Result<Value, MocksError> {
    match data.to_owned()[resource_key] {
        Value::Object(ref mut map) => {
            if let Value::Object(m) = input {
                for (k, v) in m {
                    map.insert(k.clone(), v.clone());
                }
            }

            data[resource_key] = Value::Object(map.to_owned());
            Ok(Value::Object(map.to_owned()))
        }
        _ => Err(MocksError::ObjectNotFound()),
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
                assert_eq!(e.to_string(), MocksError::ObjectNotFound().to_string());
            }
        }
    }
}
