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
