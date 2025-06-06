use crate::error::MocksError;
use crate::server::context::PayloadWithId;
use crate::server::state::SharedState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

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

#[cfg(test)]
mod tests {
    use crate::server::context::PayloadWithId;
    use crate::server::handler::post::post;
    use crate::server::handler::tests::init_state;
    use axum::extract::{Path, State};
    use serde_json::json;

    #[tokio::test]
    async fn test_post() {
        let state = init_state();
        let path: Path<String> = Path("posts".to_string());
        let payload = json!({"id":"01J8593X0V7Q34X011BYD92CHP","title":"posted post","views":0});
        assert!(post(path, State(state), PayloadWithId(payload))
            .await
            .is_ok());
    }
}
