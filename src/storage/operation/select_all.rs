use crate::error::MocksError;
use crate::storage::StorageData;
use serde_json::Value;

pub fn select_all(data: &StorageData, resource_key: &str) -> Result<Value, MocksError> {
    data.get(resource_key)
        .filter(|&value| value.is_array() || value.is_object())
        .cloned()
        .ok_or(MocksError::ResourceNotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_select_all_list() {
        let data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});

        match select_all(&data, "posts") {
            Ok(v) => {
                if let Value::Array(values) = v {
                    assert_eq!(values.len(), 1);

                    let v = &values[0];
                    assert_eq!(*v, json!({"id":"test1","title":"first post","views":100}));
                } else {
                    panic!("panic in test_insert_with_string_id");
                }
            }
            Err(_) => {
                panic!("panic in test_insert_with_string_id");
            }
        }
    }

    #[test]
    fn test_select_all_object() {
        let mut data = json!({"profile":{"id":1,"name":"John Smith","age":25}});

        match select_all(&mut data, "profile") {
            Ok(v) => {
                assert_eq!(v, json!({"id":1,"name":"John Smith","age":25}));
            }
            Err(_) => {
                panic!("panic in test_insert_with_string_id");
            }
        }
    }

    #[test]
    fn test_select_all_error_list() {
        let data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});

        match select_all(&data, "errors") {
            Ok(_v) => {
                panic!("panic in test_select_all_error_list");
            }
            Err(e) => {
                assert_eq!(e.to_string(), MocksError::ResourceNotFound.to_string());
            }
        }
    }

    #[test]
    fn test_select_all_error_object() {
        let mut data = json!({"profile":{"id":1,"name":"John Smith","age":25}});

        match select_all(&mut data, "error") {
            Ok(_v) => {
                panic!("panic in test_select_all_error_object");
            }
            Err(e) => {
                assert_eq!(e.to_string(), MocksError::ResourceNotFound.to_string());
            }
        }
    }
}
