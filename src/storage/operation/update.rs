use crate::error::MocksError;
use crate::storage::operation::select_one::select_one;
use crate::storage::{Input, StorageData};
use serde_json::Value;

pub fn update(
    data: &mut StorageData,
    resource_key: &str,
    search_key: &str,
    input: &Input,
) -> Result<Value, MocksError> {
    match data[resource_key] {
        Value::Array(ref values) => {
            let mut updated_values: Vec<Value> = Vec::new();
            for value in values {
                if let Some(map) = value.to_owned().as_object_mut() {
                    match &map["id"] {
                        Value::Number(key) => {
                            if key.to_string() == *search_key {
                                if let Value::Object(m) = input {
                                    for (k, v) in m {
                                        map.insert(k.clone(), v.clone());
                                    }
                                }
                                updated_values.push(Value::Object(map.to_owned()));
                            } else {
                                updated_values.push(value.to_owned());
                            }
                        }
                        Value::String(key) => {
                            if key == search_key {
                                if let Value::Object(m) = input {
                                    for (k, v) in m {
                                        map.insert(k.clone(), v.clone());
                                    }
                                }
                                updated_values.push(Value::Object(map.to_owned()));
                            } else {
                                updated_values.push(value.to_owned());
                            }
                        }
                        _ => {
                            // Do nothing
                        }
                    }
                }
            }

            data[resource_key] = Value::Array(updated_values);
            select_one(data, resource_key, search_key)
        }
        _ => Err(MocksError::ObjectNotFound()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_update_with_string_id() {
        let mut data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});
        let input = json!({"title":"fixed post","views":200});

        match update(&mut data, "posts", "test1", &input) {
            Ok(v) => {
                assert_eq!(v, json!({"id":"test1","title":"fixed post","views":200}));

                if let Value::Array(values) = &data["posts"] {
                    assert_eq!(values.len(), 1);
                } else {
                    panic!("panic in test_update_with_string_id");
                }
            }
            Err(e) => {
                panic!("panic in test_update_with_string_id: {}", e.to_string());
            }
        }
    }

    #[test]
    fn test_update_with_number_id() {
        let mut data = json!({"posts":[{"id":1,"title":"first post","views":100}]});
        let input = json!({"title":"fixed post","views":200});

        match update(&mut data, "posts", "1", &input) {
            Ok(v) => {
                assert_eq!(v, json!({"id":1,"title":"fixed post","views":200}));

                if let Value::Array(values) = &data["posts"] {
                    assert_eq!(values.len(), 1);
                } else {
                    panic!("panic in test_update_with_number_id");
                }
            }
            Err(e) => {
                panic!("panic in test_update_with_number_id: {}", e.to_string());
            }
        }
    }

    #[test]
    fn test_update_error_resource_not_found() {
        let mut data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});
        let input = json!({"title":"fixed post","views":200});

        match update(&mut data, "errors", "test1", &input) {
            Ok(_v) => {
                panic!("panic in test_update_error_resource_not_found")
            }
            Err(e) => {
                assert_eq!(e.to_string(), MocksError::ObjectNotFound().to_string());
            }
        }
    }

    #[test]
    fn test_update_error_object_not_found() {
        let mut data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});
        let input = json!({"title":"fixed post","views":200});

        match update(&mut data, "posts", "error", &input) {
            Ok(_v) => {
                panic!("panic in test_update_error_object_not_found")
            }
            Err(e) => {
                assert_eq!(e.to_string(), MocksError::ObjectNotFound().to_string());
            }
        }
    }
}
