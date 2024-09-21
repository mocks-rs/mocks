use crate::error::MocksError;
use crate::storage::operation::select_one::select_one;
use crate::storage::{Input, StorageData};
use serde_json::Value;

pub fn insert(
    data: &mut StorageData,
    resource_key: &str,
    input: &Input,
) -> Result<Value, MocksError> {
    // Validation to check duplicate IDs
    let id = input
        .get("id")
        .and_then(|v| match v {
            Value::Number(id) => Some(id.to_string()),
            Value::String(id) => Some(id.to_string()),
            _ => None,
        })
        .ok_or(MocksError::InvalidRequest)?;
    if select_one(data, resource_key, &id).is_ok() {
        return Err(MocksError::DuplicateId);
    }

    data.get_mut(resource_key)
        .and_then(Value::as_array_mut)
        .map(|values| {
            values.push(input.clone());
            input.clone()
        })
        .ok_or_else(|| {
            if data.get(resource_key).map_or(false, Value::is_object) {
                MocksError::MethodNotAllowed
            } else {
                MocksError::ObjectNotFound
            }
        })
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
                panic!("panic in test_insert_with_string_id: {}", e);
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
                panic!("panic in test_insert_with_string_id: {}", e);
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
                assert_eq!(e, MocksError::MethodNotAllowed);
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
                assert_eq!(e, MocksError::ObjectNotFound);
            }
        }
    }

    #[test]
    fn test_insert_error_with_duplicated_string_id() {
        let mut data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});
        let input = json!({"id":"test1","title":"duplicated id","views":0});

        match insert(&mut data, "posts", &input) {
            Ok(_v) => {
                panic!("panic in test_insert_error_with_duplicated_string_id");
            }
            Err(e) => {
                assert_eq!(e, MocksError::DuplicateId);
            }
        }
    }

    #[test]
    fn test_insert_error_with_duplicated_number_id() {
        let mut data = json!({"posts":[{"id":1,"title":"first post","views":100}]});
        let input = json!({"id":1,"title":"duplicated id","views":0});

        match insert(&mut data, "posts", &input) {
            Ok(_v) => {
                panic!("panic in test_insert_error_with_duplicated_number_id");
            }
            Err(e) => {
                assert_eq!(e, MocksError::DuplicateId);
            }
        }
    }
}
