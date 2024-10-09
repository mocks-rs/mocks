use crate::error::MocksError;
use crate::server::context::Payload;
use crate::server::state::SharedState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

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

#[cfg(test)]
mod tests {
    use crate::server::context::Payload;
    use crate::server::handler::patch::{patch, patch_one};
    use crate::server::handler::tests::init_state;
    use axum::extract::{Path, State};
    use serde_json::json;

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
}
