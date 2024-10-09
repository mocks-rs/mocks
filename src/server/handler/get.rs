use crate::error::MocksError;
use crate::server::state::SharedState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub async fn get_all(
    Path(resource): Path<String>,
    state: State<SharedState>,
) -> Result<impl IntoResponse, MocksError> {
    let state = state
        .lock()
        .map_err(|e| MocksError::Exception(e.to_string()))?;

    let value = state.storage.get_all(&resource)?;
    let response = json!({
        resource: value
    });

    Ok((StatusCode::OK, Json(response)))
}

pub async fn get_one(
    Path((resource, id)): Path<(String, String)>,
    state: State<SharedState>,
) -> Result<impl IntoResponse, MocksError> {
    let state = state
        .lock()
        .map_err(|e| MocksError::Exception(e.to_string()))?;

    let value = state.storage.get_one(&resource, &id)?;
    Ok((StatusCode::OK, Json(value)))
}

#[cfg(test)]
mod tests {
    use crate::server::handler::get::{get_all, get_one};
    use crate::server::handler::tests::init_state;
    use axum::extract::{Path, State};

    #[tokio::test]
    async fn test_get_all() {
        let state = init_state();
        let path: Path<String> = Path("posts".to_string());
        assert!(get_all(path, State(state)).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_one() {
        let state = init_state();
        let path: Path<(String, String)> = Path((
            "posts".to_string(),
            "01J7BAKH37HPG116ZRRFKHBDGB".to_string(),
        ));
        assert!(get_one(path, State(state)).await.is_ok());
    }
}
