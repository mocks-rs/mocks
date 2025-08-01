use crate::error::MocksError;
use crate::server::state::SharedState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::collections::HashMap;

pub async fn get_all(
    Path(resource): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    state: State<SharedState>,
) -> Result<impl IntoResponse, MocksError> {
    let state = state
        .lock()
        .map_err(|e| MocksError::Exception(e.to_string()))?;

    let value = if params.is_empty() {
        state.storage.get_all(&resource)?
    } else {
        state.storage.get_all_with_filter(&resource, &params)?
    };
    let response = json!({
        resource: value
    });

    Ok((StatusCode::OK, Json(response)))
}

pub async fn get_one(
    Path((resource, id)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    state: State<SharedState>,
) -> Result<impl IntoResponse, MocksError> {
    // Prohibit query parameters for ID endpoints
    if !params.is_empty() {
        return Err(MocksError::QueryParamsNotAllowed);
    }
    let state = state
        .lock()
        .map_err(|e| MocksError::Exception(e.to_string()))?;

    let value = state.storage.get_one(&resource, &id)?;
    Ok((StatusCode::OK, Json(value)))
}

#[cfg(test)]
mod tests {
    use crate::error::MocksError;
    use crate::server::handler::get::{get_all, get_one};
    use crate::server::handler::tests::init_state;
    use axum::extract::{Path, Query, State};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_all() {
        let state = init_state();
        let path: Path<String> = Path("posts".to_string());
        let query: Query<HashMap<String, String>> = Query(HashMap::new());
        assert!(get_all(path, query, State(state)).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_all_with_filter() {
        let state = init_state();
        let path: Path<String> = Path("posts".to_string());
        let mut params = HashMap::new();
        params.insert("title.contains".to_string(), "post".to_string());
        let query: Query<HashMap<String, String>> = Query(params);
        assert!(get_all(path, query, State(state)).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_one() {
        let state = init_state();
        let path: Path<(String, String)> = Path((
            "posts".to_string(),
            "01J7BAKH37HPG116ZRRFKHBDGB".to_string(),
        ));
        let query: Query<HashMap<String, String>> = Query(HashMap::new());
        assert!(get_one(path, query, State(state)).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_one_with_query_params_error() {
        let state = init_state();
        let path: Path<(String, String)> = Path((
            "posts".to_string(),
            "01J7BAKH37HPG116ZRRFKHBDGB".to_string(),
        ));
        let mut params = HashMap::new();
        params.insert("name".to_string(), "test".to_string());
        let query: Query<HashMap<String, String>> = Query(params);

        match get_one(path, query, State(state)).await {
            Err(MocksError::QueryParamsNotAllowed) => {
                // Expected error
            }
            _ => panic!("Expected QueryParamsNotAllowed error"),
        }
    }
}
