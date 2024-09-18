use crate::error::MocksError;
use crate::storage::operation::select_one::select_one;
use crate::storage::StorageData;
use serde_json::Value;

pub fn remove(
    data: &mut StorageData,
    resource_key: &str,
    search_key: &str,
) -> Result<Value, MocksError> {
    let values = data
        .get(resource_key)
        .and_then(Value::as_array)
        .ok_or(MocksError::ResourceNotFound)?;

    // Get the target to be removed
    let removed = select_one(data, resource_key, search_key)?;

    let filtered: Vec<Value> = values
        .iter()
        .filter(|&value| {
            value
                .get("id")
                .and_then(|id| match id {
                    Value::Number(n) => Some(n.to_string() != search_key),
                    Value::String(s) => Some(s != search_key),
                    _ => None,
                })
                .unwrap_or(true)
        })
        .cloned()
        .collect();

    data[resource_key] = Value::Array(filtered);
    Ok(removed)
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
                assert_eq!(e.to_string(), MocksError::ResourceNotFound.to_string());
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
                assert_eq!(e.to_string(), MocksError::ObjectNotFound.to_string());
            }
        }
    }
}
