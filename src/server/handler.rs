use crate::error::{MocksError, EXCEPTION_ERROR_MESSAGE};
use crate::server::state::SharedState;
use crate::server::{format_err, response};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

const DUMMY_ITEM_KEY: &str = "dummy";

pub async fn get_all(
    Path(resource): Path<String>,
    state: State<SharedState>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.read() {
        Ok(state) => match state.storage.get_resource(&resource) {
            None => response(
                StatusCode::NOT_FOUND,
                format_err(&MocksError::ResourceNotFound().to_string()),
            ),
            Some(v) => response(StatusCode::OK, format!("{{\"{}\": {}}}", resource, v)),
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
    match state.read() {
        Ok(storage) => match storage.storage.get_one(&resource, &id) {
            None => response(
                StatusCode::NOT_FOUND,
                format_err(&MocksError::ObjectNotFound().to_string()),
            ),
            Some(v) => response(StatusCode::OK, format!("{{\"{}\": {}}}", resource, v)),
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
    match state.write() {
        Ok(mut writer) => match writer.storage.upsert(&resource, &input) {
            Ok(v) => response(StatusCode::CREATED, format!("{}", v)),
            Err(e) => match e {
                MocksError::ResourceNotFound() => {
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

pub async fn put(
    Path((resource, id)): Path<(String, String)>,
    state: State<SharedState>,
    Json(input): Json<Value>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.write() {
        Ok(mut writer) => match writer.storage.replace(&resource, &id, &input) {
            Ok(v) => response(StatusCode::OK, format!("{}", v)),
            Err(e) => match e {
                MocksError::ResourceNotFound() => {
                    response(StatusCode::NOT_FOUND, format_err(&e.to_string()))
                }
                MocksError::ObjectNotFound() => {
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
    match state.write() {
        Ok(mut writer) => match writer.storage.replace(&resource, DUMMY_ITEM_KEY, &input) {
            Ok(v) => response(StatusCode::OK, format!("{}", v)),
            Err(e) => match e {
                MocksError::ResourceNotFound() => {
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
    match state.write() {
        Ok(mut writer) => match writer.storage.update(&resource, &id, &input) {
            Ok(v) => response(StatusCode::OK, format!("{}", v)),
            Err(e) => match e {
                MocksError::ResourceNotFound() => {
                    response(StatusCode::NOT_FOUND, format_err(&e.to_string()))
                }
                MocksError::ObjectNotFound() => {
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
    match state.write() {
        Ok(mut writer) => match writer.storage.update(&resource, DUMMY_ITEM_KEY, &input) {
            Ok(v) => response(StatusCode::OK, format!("{}", v)),
            Err(e) => match e {
                MocksError::ResourceNotFound() => {
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
    match state.write() {
        Ok(mut writer) => match writer.storage.delete(&resource, &id) {
            Ok(v) => response(StatusCode::OK, format!("{}", v)),
            Err(e) => match e {
                MocksError::ResourceNotFound() => {
                    response(StatusCode::NOT_FOUND, format_err(&e.to_string()))
                }
                MocksError::ObjectNotFound() => {
                    response(StatusCode::NOT_FOUND, format_err(&e.to_string()))
                }
                MocksError::ExceptionError() => response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format_err(&e.to_string()),
                ),
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
