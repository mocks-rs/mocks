use crate::error::MocksError;
use crate::storage::operation::build_search_resource_key;
use crate::storage::StorageData;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum MatchType {
    Exact,
    StartsWith,
    EndsWith,
    Contains,
}

impl MatchType {
    fn from_str(s: &str) -> Result<Self, MocksError> {
        match s {
            "exact" => Ok(MatchType::Exact),
            "startswith" => Ok(MatchType::StartsWith),
            "endswith" => Ok(MatchType::EndsWith),
            "contains" => Ok(MatchType::Contains),
            _ => Err(MocksError::InvalidMatchType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterCriteria {
    pub field_name: String,
    pub match_type: MatchType,
    pub value: String,
}

fn parse_query_params(params: &HashMap<String, String>) -> Result<Vec<FilterCriteria>, MocksError> {
    let mut criteria = Vec::new();

    for (key, value) in params {
        let filter_criteria = if key.contains('.') {
            // Format: field_name.match_type
            let parts: Vec<&str> = key.rsplitn(2, '.').collect();
            if parts.len() != 2 {
                return Err(MocksError::InvalidQueryParam);
            }
            let match_type_str = parts[0];
            let field_name = parts[1];

            FilterCriteria {
                field_name: field_name.to_string(),
                match_type: MatchType::from_str(match_type_str)?,
                value: value.clone(),
            }
        } else {
            // Return error if only field name provided (match type is required)
            return Err(MocksError::MatchTypeRequired);
        };

        criteria.push(filter_criteria);
    }

    Ok(criteria)
}

pub fn select_with_filter(
    data: &StorageData,
    resource_key: &str,
    filters: &HashMap<String, String>,
) -> Result<Value, MocksError> {
    let search_resource_key = build_search_resource_key(data, resource_key);

    let resource_value = data
        .get(&search_resource_key)
        .ok_or(MocksError::ResourceNotFound)?;

    // Parse query parameters
    let criteria = parse_query_params(filters)?;

    match resource_value {
        Value::Array(array) => {
            let mut filtered_items = Vec::new();
            for item in array {
                match matches_filters(item, &criteria) {
                    Ok(true) => filtered_items.push(item.clone()),
                    Ok(false) => {}          // Do nothing if no match
                    Err(e) => return Err(e), // Return error immediately
                }
            }
            Ok(Value::Array(filtered_items))
        }
        Value::Object(_) => {
            // Return error if query parameters are specified for object-type resources
            if filters.is_empty() {
                Ok(resource_value.clone())
            } else {
                Err(MocksError::QueryParamsNotAllowed)
            }
        }
        _ => Err(MocksError::ResourceNotFound),
    }
}

fn matches_filters(item: &Value, criteria: &[FilterCriteria]) -> Result<bool, MocksError> {
    if let Value::Object(obj) = item {
        for criterion in criteria {
            if let Some(field_value) = obj.get(&criterion.field_name) {
                let field_str = value_to_search_string(field_value)?;
                let field_lower = field_str.to_lowercase();
                let search_lower = criterion.value.to_lowercase();

                let matches = match criterion.match_type {
                    MatchType::Exact => field_lower == search_lower,
                    MatchType::StartsWith => field_lower.starts_with(&search_lower),
                    MatchType::EndsWith => field_lower.ends_with(&search_lower),
                    MatchType::Contains => field_lower.contains(&search_lower),
                };

                if !matches {
                    return Ok(false);
                }
            } else {
                // No match if field doesn't exist
                return Ok(false);
            }
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

fn value_to_search_string(value: &Value) -> Result<String, MocksError> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Number(n) => Ok(n.to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Null => Ok("null".to_string()),
        Value::Array(_) | Value::Object(_) => Err(MocksError::InvalidSearchValue),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_select_with_filter_match_type_required_error() {
        let data = json!({
            "posts": [
                {"id": "1", "title": "First Post", "author": "John"},
                {"id": "2", "title": "Second Post", "author": "Jane"},
                {"id": "3", "title": "Third Post", "author": "John"}
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("author".to_string(), "john".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Err(MocksError::MatchTypeRequired) => {
                // Expected error when match type is not specified
            }
            _ => panic!("Expected MatchTypeRequired error"),
        }
    }

    #[test]
    fn test_select_with_filter_exact_match() {
        let data = json!({
            "posts": [
                {"id": "1", "title": "Post", "author": "John"},
                {"id": "2", "title": "Post Title", "author": "Jane"},
                {"id": "3", "title": "Another Post", "author": "John"}
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("title.exact".to_string(), "post".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Ok(Value::Array(values)) => {
                assert_eq!(values.len(), 1);
                assert_eq!(values[0]["id"], "1");
            }
            _ => panic!("Expected array result"),
        }
    }

    #[test]
    fn test_select_with_filter_startswith_match() {
        let data = json!({
            "posts": [
                {"id": "1", "title": "Hello World", "author": "John"},
                {"id": "2", "title": "Hi there", "author": "Jane"},
                {"id": "3", "title": "Hello everyone", "author": "Bob"}
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("title.startswith".to_string(), "hello".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Ok(Value::Array(values)) => {
                assert_eq!(values.len(), 2);
                assert_eq!(values[0]["id"], "1");
                assert_eq!(values[1]["id"], "3");
            }
            _ => panic!("Expected array result"),
        }
    }

    #[test]
    fn test_select_with_filter_endswith_match() {
        let data = json!({
            "posts": [
                {"id": "1", "title": "Learning Rust", "author": "John"},
                {"id": "2", "title": "Programming in Rust", "author": "Jane"},
                {"id": "3", "title": "Java Tutorial", "author": "Bob"}
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("title.endswith".to_string(), "rust".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Ok(Value::Array(values)) => {
                assert_eq!(values.len(), 2);
                assert_eq!(values[0]["id"], "1");
                assert_eq!(values[1]["id"], "2");
            }
            _ => panic!("Expected array result"),
        }
    }

    #[test]
    fn test_select_with_filter_contains_explicit() {
        let data = json!({
            "posts": [
                {"id": "1", "title": "First Post", "author": "John Smith"},
                {"id": "2", "title": "Second Post", "author": "Jane Doe"},
                {"id": "3", "title": "Third Post", "author": "John Johnson"}
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("author.contains".to_string(), "john".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Ok(Value::Array(values)) => {
                assert_eq!(values.len(), 2);
                assert_eq!(values[0]["id"], "1");
                assert_eq!(values[1]["id"], "3");
            }
            _ => panic!("Expected array result"),
        }
    }

    #[test]
    fn test_select_with_filter_array_partial_match() {
        let data = json!({
            "posts": [
                {"id": "1", "title": "First Post", "views": 100},
                {"id": "2", "title": "Second Post", "views": 200},
                {"id": "3", "title": "Another Story", "views": 50}
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("title.contains".to_string(), "post".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Ok(Value::Array(values)) => {
                assert_eq!(values.len(), 2);
                assert_eq!(values[0]["id"], "1");
                assert_eq!(values[1]["id"], "2");
            }
            _ => panic!("Expected array result"),
        }
    }

    #[test]
    fn test_select_with_filter_array_no_match() {
        let data = json!({
            "posts": [
                {"id": "1", "title": "First Post", "author": "John"},
                {"id": "2", "title": "Second Post", "author": "Jane"}
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("author.contains".to_string(), "nonexistent".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Ok(Value::Array(values)) => {
                assert_eq!(values.len(), 0);
            }
            _ => panic!("Expected empty array result"),
        }
    }

    #[test]
    fn test_select_with_filter_object_returns_error_with_query_params() {
        let data = json!({
            "profile": {"id": "1", "name": "John Smith", "age": 25}
        });

        let mut filters = HashMap::new();
        filters.insert("name.contains".to_string(), "jane".to_string());

        match select_with_filter(&data, "profile", &filters) {
            Err(MocksError::QueryParamsNotAllowed) => {
                // Expected error for object resource with query parameters
            }
            _ => panic!("Expected QueryParamsNotAllowed error"),
        }
    }

    #[test]
    fn test_select_with_filter_object_works_without_query_params() {
        let data = json!({
            "profile": {"id": "1", "name": "John Smith", "age": 25}
        });

        let filters = HashMap::new();

        match select_with_filter(&data, "profile", &filters) {
            Ok(value) => {
                // Normal operation for object type with no query parameters
                assert_eq!(value, json!({"id": "1", "name": "John Smith", "age": 25}));
            }
            _ => panic!("Expected object result"),
        }
    }

    #[test]
    fn test_select_with_filter_numeric_field() {
        let data = json!({
            "posts": [
                {"id": "1", "title": "First", "views": 100},
                {"id": "2", "title": "Second", "views": 200}
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("views.exact".to_string(), "100".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Ok(Value::Array(values)) => {
                assert_eq!(values.len(), 1);
                assert_eq!(values[0]["id"], "1");
            }
            _ => panic!("Expected array result"),
        }
    }

    #[test]
    fn test_select_with_filter_multiple_filters_mixed_types() {
        let data = json!({
            "posts": [
                {"id": "1", "title": "Hello World", "author": "John", "category": "tech"},
                {"id": "2", "title": "Hello there", "author": "John", "category": "life"},
                {"id": "3", "title": "Hi World", "author": "Jane", "category": "tech"}
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("title.startswith".to_string(), "hello".to_string());
        filters.insert("author.exact".to_string(), "john".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Ok(Value::Array(values)) => {
                assert_eq!(values.len(), 2);
                assert_eq!(values[0]["id"], "1");
                assert_eq!(values[1]["id"], "2");
            }
            _ => panic!("Expected array result"),
        }
    }

    #[test]
    fn test_select_with_filter_field_with_dots() {
        let data = json!({
            "posts": [
                {"id": "1", "user.name": "John Doe", "title": "First Post"},
                {"id": "2", "user.name": "Jane Smith", "title": "Second Post"}
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("user.name.exact".to_string(), "john doe".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Ok(Value::Array(values)) => {
                assert_eq!(values.len(), 1);
                assert_eq!(values[0]["id"], "1");
            }
            _ => panic!("Expected array result"),
        }
    }

    #[test]
    fn test_select_with_filter_invalid_match_type() {
        let data = json!({
            "posts": [
                {"id": "1", "title": "First Post", "author": "John"}
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("title.invalid".to_string(), "post".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Err(MocksError::InvalidMatchType) => {
                // Expected error
            }
            _ => panic!("Expected InvalidMatchType error"),
        }
    }

    #[test]
    fn test_select_with_filter_resource_not_found() {
        let data = json!({
            "posts": [{"id": "1", "title": "First Post"}]
        });

        let filters = HashMap::new();

        match select_with_filter(&data, "nonexistent", &filters) {
            Err(MocksError::ResourceNotFound) => {
                // Expected error
            }
            _ => panic!("Expected ResourceNotFound error"),
        }
    }

    #[test]
    fn test_select_with_filter_object_value_search_error() {
        let data = json!({
            "posts": [
                {
                    "id": "1",
                    "title": "First Post",
                    "metadata": {"tags": ["tech", "rust"], "category": "programming"}
                },
                {
                    "id": "2",
                    "title": "Second Post",
                    "metadata": {"tags": ["life"], "category": "personal"}
                }
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("metadata.contains".to_string(), "tech".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Err(MocksError::InvalidSearchValue) => {
                // Expected error when trying to search on object values
            }
            _ => panic!("Expected InvalidSearchValue error"),
        }
    }

    #[test]
    fn test_select_with_filter_array_value_search_error() {
        let data = json!({
            "posts": [
                {
                    "id": "1",
                    "title": "First Post",
                    "tags": ["tech", "rust"]
                },
                {
                    "id": "2",
                    "title": "Second Post",
                    "tags": ["life"]
                }
            ]
        });

        let mut filters = HashMap::new();
        filters.insert("tags.contains".to_string(), "tech".to_string());

        match select_with_filter(&data, "posts", &filters) {
            Err(MocksError::InvalidSearchValue) => {
                // Expected error when trying to search on array values
            }
            _ => panic!("Expected InvalidSearchValue error"),
        }
    }
}
