use crate::error::MocksError;
use crate::storage::operation::select_one::select_one;
use crate::storage::operation::{build_search_resource_key, extract_array_resource};
use crate::storage::StorageData;
use serde_json::Value;

pub fn remove(
    data: &mut StorageData,
    resource_key: &str,
    search_key: &str,
) -> Result<Value, MocksError> {
    let search_resource_key = build_search_resource_key(data, resource_key);
    let values = extract_array_resource(data, &search_resource_key)?;

    // Get the target to be removed
    let remove_one = select_one(data, &search_resource_key, search_key)?;
    let removed_resource = remove_target(values, search_key);
    data[&search_resource_key] = Value::Array(removed_resource);
    Ok(remove_one)
}

fn remove_target(values: Vec<Value>, key: &str) -> Vec<Value> {
    values
        .iter()
        .filter(|&value| {
            value
                .get("id")
                .and_then(|id| match id {
                    Value::Number(n) => Some(n.to_string() != key),
                    Value::String(s) => Some(s != key),
                    _ => None,
                })
                .unwrap_or(true)
        })
        .cloned()
        .collect()
}

// codecov:ignore-start
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
                panic!("panic in test_remove_with_string_id: {e}");
            }
        }
    }

    #[test]
    fn test_remove_nested_resource_with_string_id() {
        let mut data = json!({"api/v1/posts":[{"id":"test1","title":"first post","views":100}]});

        match remove(&mut data, "api/v1/posts", "test1") {
            Ok(v) => {
                assert_eq!(v, json!({"id":"test1","title":"first post","views":100}));

                match &data["api/v1/posts"].as_array() {
                    None => {
                        panic!("panic in test_remove_with_string_id");
                    }
                    Some(values) => {
                        assert_eq!(values.len(), 0);
                    }
                }
            }
            Err(e) => {
                panic!("panic in test_remove_with_string_id: {e}");
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
                panic!("panic in test_remove_with_number_id: {e}");
            }
        }
    }

    #[test]
    fn test_remove_nested_resource_with_number_id() {
        let mut data = json!({"api/v1/posts":[{"id":1,"title":"first post","views":100}]});

        match remove(&mut data, "api/v1/posts", "1") {
            Ok(v) => {
                assert_eq!(v, json!({"id":1,"title":"first post","views":100}));

                match &data["api/v1/posts"].as_array() {
                    None => {
                        panic!("panic in test_remove_with_number_id");
                    }
                    Some(values) => {
                        assert_eq!(values.len(), 0);
                    }
                }
            }
            Err(e) => {
                panic!("panic in test_remove_with_number_id: {e}");
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
                assert_eq!(e, MocksError::ResourceNotFound);
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
                assert_eq!(e, MocksError::ObjectNotFound);
            }
        }
    }
}
// codecov:ignore-end
