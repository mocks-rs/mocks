use crate::error::MocksError;
use crate::server::state::SharedState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

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
