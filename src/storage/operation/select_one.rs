use crate::error::MocksError;
use crate::storage::StorageData;
use serde_json::Value;

pub fn select_one(
    data: &StorageData,
    resource_key: &str,
    search_key: &str,
) -> Result<Value, MocksError> {
    let value = data[resource_key].to_owned();

    match value.as_array() {
        Some(values) => {
            for value in values {
                if !value.is_object() {
                    break;
                }

                match &value["id"] {
                    Value::Number(key) => {
                        if key.to_string() == *search_key {
                            return Ok(value.to_owned());
                        }
                    }
                    Value::String(key) => {
                        if key == search_key {
                            return Ok(value.to_owned());
                        }
                    }
                    _ => {
                        // Do nothing
                    }
                }
            }
            Err(MocksError::ObjectNotFound())
        }
        None => Err(MocksError::ObjectNotFound()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_select_one_with_string_id() {
        let data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});

        match select_one(&data, "posts", "test1") {
            Ok(v) => {
                assert_eq!(v, json!({"id":"test1","title":"first post","views":100}));
            }
            Err(e) => {
                panic!("panic in test_select_one_with_string_id: {}", e.to_string());
            }
        }
    }

    #[test]
    fn test_select_one_with_number_id() {
        let data = json!({"posts":[{"id":1,"title":"first post","views":100}]});

        match select_one(&data, "posts", "1") {
            Ok(v) => {
                assert_eq!(v, json!({"id":1,"title":"first post","views":100}));
                assert_eq!(v["id"], Value::Number(1.into()));
            }
            Err(e) => {
                panic!("panic in test_select_one_with_number_id: {}", e.to_string());
            }
        }
    }

    #[test]
    fn test_select_one_error() {
        let data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});

        match select_one(&data, "posts", "error") {
            Ok(_v) => {
                panic!("panic in test_select_one_error")
            }
            Err(e) => {
                assert_eq!(e.to_string(), MocksError::ObjectNotFound().to_string());
            }
        }
    }
}
