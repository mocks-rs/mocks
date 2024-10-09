use crate::error::MocksError;
use crate::server::context::PayloadWithId;
use crate::server::state::SharedState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

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

#[cfg(test)]
mod tests {
    use crate::server::context::PayloadWithId;
    use crate::server::handler::put::{put, put_one};
    use crate::server::handler::tests::init_state;
    use axum::extract::{Path, State};
    use serde_json::json;

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
}
