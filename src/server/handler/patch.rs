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
