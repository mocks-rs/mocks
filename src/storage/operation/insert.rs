use crate::error::MocksError;
use crate::storage::{Input, StorageData};
use serde_json::Value;

pub fn insert(
    data: &mut StorageData,
    resource_key: &str,
    input: &Input,
) -> Result<Value, MocksError> {
    match data[resource_key] {
        Value::Array(ref mut values) => {
            values.push(input.to_owned());

            data[resource_key] = Value::Array(values.to_owned());
            Ok(input.to_owned())
        }
        Value::Object(_) => Err(MocksError::MethodNotAllowed()),
        _ => Err(MocksError::ObjectNotFound()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_insert_with_string_id() {
        let mut data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});
        let input = json!({"id":"test2","title":"second post","views":0});

        match insert(&mut data, "posts", &input) {
            Ok(v) => {
                assert_eq!(v, json!({"id":"test2","title":"second post","views":0}));

                match &data["posts"].as_array() {
                    None => {
                        panic!("panic in test_insert_with_string_id");
                    }
                    Some(values) => {
                        assert_eq!(values.len(), 2);
                    }
                }
            }
            Err(e) => {
                panic!("panic in test_insert_with_string_id: {}", e.to_string());
            }
        }
    }

    #[test]
    fn test_insert_with_number_id() {
        let mut data = json!({"posts":[{"id":1,"title":"first post","views":100}]});
        let input = json!({"id":2,"title":"second post","views":0});

        match insert(&mut data, "posts", &input) {
            Ok(v) => {
                assert_eq!(v, json!({"id":2,"title":"second post","views":0}));

                match &data["posts"].as_array() {
                    None => {
                        panic!("panic in test_insert_with_number_id");
                    }
                    Some(values) => {
                        assert_eq!(values.len(), 2);
                    }
                }
            }
            Err(e) => {
                panic!("panic in test_insert_with_string_id: {}", e.to_string());
            }
        }
    }

    #[test]
    fn test_insert_error_method_not_allowed() {
        let mut data = json!({"profile":{"id":"user1","name":"John Smith","age":25}});
        let input = json!({"id":"user2","name":"Jane Smith","age":25});

        match insert(&mut data, "profile", &input) {
            Ok(_v) => {
                panic!("panic in test_insert_error_method_not_allowed");
            }
            Err(e) => {
                assert_eq!(e.to_string(), MocksError::MethodNotAllowed().to_string());
            }
        }
    }

    #[test]
    fn test_insert_error_resource_not_found() {
        let mut data = json!({"posts":[{"id":1,"title":"first post","views":100}]});
        let input = json!({"id":2,"title":"second post","views":0});

        match insert(&mut data, "errors", &input) {
            Ok(_v) => {
                panic!("panic in test_insert_error_resource_not_found");
            }
            Err(e) => {
                assert_eq!(e.to_string(), MocksError::ObjectNotFound().to_string());
            }
        }
    }
}
