use crate::error::{MocksError, EXCEPTION_ERROR_MESSAGE};
use crate::server::state::SharedState;
use crate::server::{format_err, response};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

pub async fn get_all(
    Path(resource): Path<String>,
    state: State<SharedState>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.lock() {
        Ok(state) => match state.storage.get_all(&resource) {
            None => response(
                StatusCode::NOT_FOUND,
                format_err(&MocksError::ObjectNotFound.to_string()),
            ),
            Some(v) => response(StatusCode::OK, format!("{{\"{}\":{}}}", resource, v)),
        },
        Err(e) => response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format_err(&e.to_string()),
        ),
    }
}

pub async fn get_one(
    Path((resource, id)): Path<(String, String)>,
    state: State<SharedState>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.lock() {
        Ok(state) => match state.storage.get_one(&resource, &id) {
            None => response(
                StatusCode::NOT_FOUND,
                format_err(&MocksError::ObjectNotFound.to_string()),
            ),
            Some(v) => response(StatusCode::OK, format!("{{\"{}\":{}}}", resource, v)),
        },
        Err(e) => response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format_err(&e.to_string()),
        ),
    }
}

pub async fn post(
    Path(resource): Path<String>,
    state: State<SharedState>,
    Json(input): Json<Value>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.lock() {
        Ok(mut state) => match state.storage.insert(&resource, &input) {
            Ok(v) => response(StatusCode::CREATED, format!("{}", v)),
            Err(e) => match e {
                MocksError::ObjectNotFound => {
                    response(StatusCode::NOT_FOUND, format_err(&e.to_string()))
                }
                MocksError::MethodNotAllowed => {
                    response(StatusCode::METHOD_NOT_ALLOWED, format_err(&e.to_string()))
                }
                _ => response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format_err(EXCEPTION_ERROR_MESSAGE),
                ),
            },
        },
        Err(e) => response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format_err(&e.to_string()),
        ),
    }
}

pub async fn put(
    Path((resource, id)): Path<(String, String)>,
    state: State<SharedState>,
    Json(input): Json<Value>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.lock() {
        Ok(mut state) => match state.storage.replace(&resource, &id, &input) {
            Ok(v) => response(StatusCode::OK, format!("{}", v)),
            Err(e) => match e {
                MocksError::ObjectNotFound => {
                    response(StatusCode::NOT_FOUND, format_err(&e.to_string()))
                }
                _ => response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format_err(EXCEPTION_ERROR_MESSAGE),
                ),
            },
        },
        Err(e) => response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format_err(&e.to_string()),
        ),
    }
}

pub async fn put_one(
    Path(resource): Path<String>,
    state: State<SharedState>,
    Json(input): Json<Value>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.lock() {
        Ok(mut state) => match state.storage.replace_one(&resource, &input) {
            Ok(v) => response(StatusCode::OK, format!("{}", v)),
            Err(e) => match e {
                MocksError::ObjectNotFound => {
                    response(StatusCode::NOT_FOUND, format_err(&e.to_string()))
                }
                _ => response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format_err(EXCEPTION_ERROR_MESSAGE),
                ),
            },
        },
        Err(e) => response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format_err(&e.to_string()),
        ),
    }
}

pub async fn patch(
    Path((resource, id)): Path<(String, String)>,
    state: State<SharedState>,
    Json(input): Json<Value>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.lock() {
        Ok(mut state) => match state.storage.update(&resource, &id, &input) {
            Ok(v) => response(StatusCode::OK, format!("{}", v)),
            Err(e) => match e {
                MocksError::ObjectNotFound => {
                    response(StatusCode::NOT_FOUND, format_err(&e.to_string()))
                }
                _ => response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format_err(EXCEPTION_ERROR_MESSAGE),
                ),
            },
        },
        Err(e) => response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format_err(&e.to_string()),
        ),
    }
}

pub async fn patch_one(
    Path(resource): Path<String>,
    state: State<SharedState>,
    Json(input): Json<Value>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.lock() {
        Ok(mut state) => match state.storage.update_one(&resource, &input) {
            Ok(v) => response(StatusCode::OK, format!("{}", v)),
            Err(e) => match e {
                MocksError::ObjectNotFound => {
                    response(StatusCode::NOT_FOUND, format_err(&e.to_string()))
                }
                _ => response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format_err(EXCEPTION_ERROR_MESSAGE),
                ),
            },
        },
        Err(e) => response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format_err(&e.to_string()),
        ),
    }
}

pub async fn delete(
    Path((resource, id)): Path<(String, String)>,
    state: State<SharedState>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.lock() {
        Ok(mut state) => match state.storage.delete(&resource, &id) {
            Ok(v) => response(StatusCode::OK, format!("{}", v)),
            Err(e) => match e {
                MocksError::ObjectNotFound => {
                    response(StatusCode::NOT_FOUND, format_err(&e.to_string()))
                }
                _ => response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format_err(EXCEPTION_ERROR_MESSAGE),
                ),
            },
        },
        Err(e) => response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format_err(&e.to_string()),
        ),
    }
}
