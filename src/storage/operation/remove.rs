use crate::error::MocksError;
use crate::storage::operation::select_one::select_one;
use crate::storage::StorageData;
use serde_json::Value;

pub fn remove(
    data: &mut StorageData,
    resource_key: &str,
    search_key: &str,
) -> Result<Value, MocksError> {
    let value = data[resource_key].to_owned();

    match value {
        Value::Array(values) => {
            select_one(data, resource_key, search_key).inspect(|_| {
                let mut temp = Vec::new();
                for value in values {
                    if !value.is_object() {
                        break;
                    }

                    match &value["id"] {
                        Value::Number(key) => {
                            if key.to_string() != *search_key {
                                temp.push(value.to_owned());
                            }
                        }
                        Value::String(key) => {
                            if key != search_key {
                                temp.push(value.to_owned());
                            }
                        }
                        _ => {
                            // Do nothing
                        }
                    }
                }

                data[resource_key] = Value::Array(temp);
            })
        }
        _ => Err(MocksError::ObjectNotFound()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_remove_with_string_id() {
        let mut data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});

        match remove(&mut data, "posts", "test1") {
            Ok(v) => {
                assert_eq!(v, json!({"id":"test1","title":"first post","views":100}));

                match &data["posts"].as_array() {
                    None => {
                        panic!("panic in test_remove_with_string_id");
                    }
                    Some(values) => {
                        assert_eq!(values.len(), 0);
                    }
                }
            }
            Err(e) => {
                panic!("panic in test_remove_with_string_id: {}", e.to_string());
            }
        }
    }

    #[test]
    fn test_remove_with_number_id() {
        let mut data = json!({"posts":[{"id":1,"title":"first post","views":100}]});

        match remove(&mut data, "posts", "1") {
            Ok(v) => {
                assert_eq!(v, json!({"id":1,"title":"first post","views":100}));

                match &data["posts"].as_array() {
                    None => {
                        panic!("panic in test_remove_with_number_id");
                    }
                    Some(values) => {
                        assert_eq!(values.len(), 0);
                    }
                }
            }
            Err(e) => {
                panic!("panic in test_remove_with_number_id: {}", e.to_string());
            }
        }
    }

    #[test]
    fn test_remove_error_resource_not_found() {
        let mut data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});

        match remove(&mut data, "errors", "test1") {
            Ok(_v) => {
                panic!("panic in test_remove_error_resource_not_found");
            }
            Err(e) => {
                assert_eq!(e.to_string(), MocksError::ObjectNotFound().to_string());
            }
        }
    }

    #[test]
    fn test_remove_error_object_not_found() {
        let mut data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});

        match remove(&mut data, "posts", "error") {
            Ok(_v) => {
                panic!("panic in test_remove_error_object_not_found");
            }
            Err(e) => {
                assert_eq!(e.to_string(), MocksError::ObjectNotFound().to_string());
            }
        }
    }
}
