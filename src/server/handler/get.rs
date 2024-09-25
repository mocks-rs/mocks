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
