use crate::error::MocksError;
use crate::storage::StorageData;
use serde_json::Value;

use super::build_search_resource_key;

pub fn select_one(
    data: &StorageData,
    resource_key: &str,
    search_key: &str,
) -> Result<Value, MocksError> {
    let search_resource_key = build_search_resource_key(data, resource_key);

    data.get(&search_resource_key)
        .and_then(Value::as_array)
        .ok_or(MocksError::ObjectNotFound)?
        .iter()
        .find(|&value| {
            value.is_object()
                && match value.get("id") {
                    Some(Value::Number(key)) => key.to_string() == search_key,
                    Some(Value::String(key)) => key == search_key,
                    _ => false,
                }
        })
        .cloned()
        .ok_or(MocksError::ObjectNotFound)
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
                panic!("panic in test_select_one_with_string_id: {e}");
            }
        }
    }

    #[test]
    fn test_select_one_nested_resource_with_string_id() {
        let data = json!({"api/v1/posts":[{"id":"test1","title":"first post","views":100}]});

        match select_one(&data, "api/v1/posts", "test1") {
            Ok(v) => {
                assert_eq!(v, json!({"id":"test1","title":"first post","views":100}));
            }
            Err(e) => {
                panic!("panic in test_select_one_with_string_id: {e}");
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
                panic!("panic in test_select_one_with_number_id: {e}");
            }
        }
    }

    #[test]
    fn test_select_one_nested_resource_with_number_id() {
        let data = json!({"api/v1/posts":[{"id":1,"title":"first post","views":100}]});

        match select_one(&data, "api/v1/posts", "1") {
            Ok(v) => {
                assert_eq!(v, json!({"id":1,"title":"first post","views":100}));
                assert_eq!(v["id"], Value::Number(1.into()));
            }
            Err(e) => {
                panic!("panic in test_select_one_with_number_id: {e}");
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
                assert_eq!(e, MocksError::ObjectNotFound);
            }
        }
    }
}
