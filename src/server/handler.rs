use crate::error::MocksError;
use crate::server::context::{Payload, PayloadWithId};
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

    let value = state
        .storage
        .get_all(&resource)
        .ok_or(MocksError::ResourceNotFound)?;
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

    let value = state
        .storage
        .get_one(&resource, &id)
        .ok_or(MocksError::ObjectNotFound)?;
    let response = json!({
        resource: value
    });
    Ok((StatusCode::OK, Json(response)))
}

pub async fn post(
    Path(resource): Path<String>,
    state: State<SharedState>,
    PayloadWithId(input): PayloadWithId,
) -> Result<impl IntoResponse, MocksError> {
    let mut state = state
        .lock()
        .map_err(|e| MocksError::Exception(e.to_string()))?;

    let value = state.storage.insert(&resource, &input)?;
    Ok((StatusCode::CREATED, Json(value)))
}

pub async fn put(
    Path((resource, id)): Path<(String, String)>,
    state: State<SharedState>,
    PayloadWithId(input): PayloadWithId,
) -> Result<impl IntoResponse, MocksError> {
    let mut state = state
        .lock()
        .map_err(|e| MocksError::Exception(e.to_string()))?;

    let value = state.storage.replace(&resource, &id, &input)?;
    Ok((StatusCode::OK, Json(value)))
}

pub async fn put_one(
    Path(resource): Path<String>,
    state: State<SharedState>,
    PayloadWithId(input): PayloadWithId,
) -> Result<impl IntoResponse, MocksError> {
    let mut state = state
        .lock()
        .map_err(|e| MocksError::Exception(e.to_string()))?;

    let value = state.storage.replace_one(&resource, &input)?;
    Ok((StatusCode::OK, Json(value)))
}

pub async fn patch(
    Path((resource, id)): Path<(String, String)>,
    state: State<SharedState>,
    Payload(input): Payload,
) -> Result<impl IntoResponse, MocksError> {
    let mut state = state
        .lock()
        .map_err(|e| MocksError::Exception(e.to_string()))?;

    let value = state.storage.update(&resource, &id, &input)?;
    Ok((StatusCode::OK, Json(value)))
}

pub async fn patch_one(
    Path(resource): Path<String>,
    state: State<SharedState>,
    Payload(input): Payload,
) -> Result<impl IntoResponse, MocksError> {
    let mut state = state
        .lock()
        .map_err(|e| MocksError::Exception(e.to_string()))?;

    let value = state.storage.update_one(&resource, &input)?;
    Ok((StatusCode::OK, Json(value)))
}

pub async fn delete(
    Path((resource, id)): Path<(String, String)>,
    state: State<SharedState>,
) -> Result<impl IntoResponse, MocksError> {
    let mut state = state
        .lock()
        .map_err(|e| MocksError::Exception(e.to_string()))?;

    let value = state.storage.delete(&resource, &id)?;
    Ok((StatusCode::OK, Json(value)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::state::AppState;
    use crate::storage::Storage;

    fn init_state() -> SharedState {
        let storage = Storage::new("storage.json", false)
            .unwrap_or_else(|e| panic!("Failed to init storage: {}", e));
        AppState::new(storage)
    }

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

    #[tokio::test]
    async fn test_post() {
        let state = init_state();
        let path: Path<String> = Path("posts".to_string());
        let payload = json!({"id":"01J8593X0V7Q34X011BYD92CHP","title":"posted post","views":0});
        assert!(post(path, State(state), PayloadWithId(payload))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_put() {
        let state = init_state();
        let path: Path<(String, String)> = Path((
            "posts".to_string(),
            "01J7BAKH37HPG116ZRRFKHBDGB".to_string(),
        ));
        let payload = json!({"id":"01J7BAKH37HPG116ZRRFKHBDGB","title":"putted post","views":200});
        assert!(put(path, State(state), PayloadWithId(payload))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_put_one() {
        let state = init_state();
        let path: Path<String> = Path("profile".to_string());
        let payload = json!({"id":1,"name":"John Smith","age":25});
        assert!(put_one(path, State(state), PayloadWithId(payload))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_patch() {
        let state = init_state();
        let path: Path<(String, String)> = Path((
            "posts".to_string(),
            "01J7BAKH37HPG116ZRRFKHBDGB".to_string(),
        ));
        let payload = json!({"title":"patched post","views":200});
        assert!(patch(path, State(state), Payload(payload)).await.is_ok());
    }

    #[tokio::test]
    async fn test_patch_one() {
        let state = init_state();
        let path: Path<String> = Path("profile".to_string());
        let payload = json!({"name":"Jane Smith","age":30});
        assert!(patch_one(path, State(state), Payload(payload))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_delete() {
        let state = init_state();
        let path: Path<(String, String)> = Path((
            "posts".to_string(),
            "01J7BAKH37HPG116ZRRFKHBDGB".to_string(),
        ));
        assert!(delete(path, State(state)).await.is_ok());
    }
}
