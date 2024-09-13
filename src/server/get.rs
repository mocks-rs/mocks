use crate::server::state::SharedState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::Value;

pub async fn get_all(
    Path(resource): Path<String>,
    state: State<SharedState>,
) -> Result<impl IntoResponse, StatusCode> {
    let data = state.read().unwrap().db.clone();
    match data.get(&resource) {
        None => Err(StatusCode::NOT_FOUND),
        Some(value) => {
            match Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(format!("{{\"{}\": {}}}", resource, value))
            {
                Ok(response) => Ok(response),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}

pub async fn get_one(
    Path((resource, id)): Path<(String, String)>,
    state: State<SharedState>,
) -> Result<impl IntoResponse, StatusCode> {
    let data = state.read().unwrap().db.clone();
    match data.get(&resource) {
        None => Err(StatusCode::NOT_FOUND),
        Some(value) => {
            let one = if let Value::Array(values) = value {
                let mut result = None;
                for v in values {
                    if let Value::Object(target) = v {
                        for (key, value) in target {
                            if key == "id" && value == &id {
                                result = Some(v);
                            }
                        }
                    }
                }
                result
            } else {
                None
            };

            match one {
                None => Err(StatusCode::NOT_FOUND),
                Some(value) => {
                    match Response::builder()
                        .status(200)
                        .header("Content-Type", "application/json")
                        .body(format!("{}", value))
                    {
                        Ok(response) => Ok(response),
                        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                }
            }
        }
    }
}
