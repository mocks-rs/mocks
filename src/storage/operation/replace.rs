use crate::error::MocksError;
use crate::storage::operation::select_one::select_one;
use crate::storage::{Input, StorageData};
use serde_json::Value;

pub fn replace(
    data: &mut StorageData,
    resource_key: &str,
    search_key: &str,
    input: &Input,
) -> Result<Value, MocksError> {
    let values = data
        .get(resource_key)
        .and_then(Value::as_array)
        .ok_or(MocksError::ResourceNotFound)?;

    // Validation to confirm the existence
    select_one(data, resource_key, search_key)?;

    let replaced: Vec<Value> = values
        .iter()
        .map(|value| {
            let id = value.get("id");
            if matches!(id, Some(Value::Number(n)) if n.to_string() == search_key)
                || matches!(id, Some(Value::String(s)) if s == search_key)
            {
                input.clone()
            } else {
                value.clone()
            }
        })
        .collect();

    data[resource_key] = Value::Array(replaced);
    Ok(input.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_replace_with_string_id() {
        let mut data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});
        let input = json!({"id":"test1","title":"replace test","views":200});

        match replace(&mut data, "posts", "test1", &input) {
            Ok(v) => {
                assert_eq!(v, json!({"id":"test1","title":"replace test","views":200}));

                let updated_post = &data["posts"][0];
                assert_eq!(
                    *updated_post,
                    json!({"id":"test1","title":"replace test","views":200})
                );
            }
            Err(e) => {
                panic!("panic in test_replace_with_string_id: {}", e.to_string());
            }
        }
    }

    #[test]
    fn test_replace_with_number_id() {
        let mut data = json!({"posts":[{"id":1,"title":"first post","views":100}]});
        let input = json!({"id":1,"title":"replace test","views":200});

        match replace(&mut data, "posts", "1", &input) {
            Ok(v) => {
                assert_eq!(v, json!({"id":1,"title":"replace test","views":200}));

                let updated_post = &data["posts"][0];
                assert_eq!(
                    *updated_post,
                    json!({"id":1,"title":"replace test","views":200})
                );
            }
            Err(e) => {
                panic!("panic in test_replace_with_string_id: {}", e.to_string());
            }
        }
    }

    #[test]
    fn test_replace_error_resource_not_found() {
        let mut data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});
        let input = json!({"id":"error","title":"replace error","views":200});

        match replace(&mut data, "errors", "test1", &input) {
            Ok(_v) => {
                panic!("panic in test_replace_error_resource_not_found")
            }
            Err(e) => {
                assert_eq!(e, MocksError::ResourceNotFound);
            }
        }
    }

    #[test]
    fn test_replace_error_object_not_found() {
        let mut data = json!({"posts":[{"id":"test1","title":"first post","views":100}]});
        let input = json!({"id":"error","title":"replace error","views":200});

        match replace(&mut data, "posts", "error", &input) {
            Ok(_v) => {
                panic!("panic in test_replace_error_object_not_found")
            }
            Err(e) => {
                assert_eq!(e, MocksError::ObjectNotFound);
            }
        }
    }
}
