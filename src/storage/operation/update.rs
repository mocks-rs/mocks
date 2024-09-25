use crate::error::MocksError;
use crate::storage::{Input, StorageData};
use serde_json::Value;

pub fn update(
    data: &mut StorageData,
    resource_key: &str,
    search_key: &str,
    input: &Input,
) -> Result<Value, MocksError> {
    let values = data
        .get_mut(resource_key)
        .and_then(Value::as_array_mut)
        .ok_or(MocksError::ResourceNotFound)?;
    match update_resource_with_input(values, search_key, input) {
        Some(value) => Ok(value),
        None => Err(MocksError::ObjectNotFound),
    }
}

fn update_resource_with_input(
    values: &mut [Value],
    search_key: &str,
    input: &Input,
) -> Option<Value> {
    values.iter_mut().find_map(|value| {
        let obj = value.as_object_mut()?;
        let id = obj.get("id")?;

        let matches = match id {
            Value::Number(key) => key.to_string() == search_key,
            Value::String(key) => key == search_key,
            _ => false,
        };

        if matches {
            if let Value::Object(input_map) = input {
                obj.extend(input_map.iter().map(|(k, v)| (k.clone(), v.clone())));
            }
            Some(value.clone())
        } else {
            None
        }
    })
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
                assert_eq!(e, MocksError::ResourceNotFound);
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
                assert_eq!(e, MocksError::ObjectNotFound);
            }
        }
    }
}
