mod handler;
mod hc;
mod state;

use crate::error::MocksError;
use crate::server::handler::{delete, get_all, get_one, patch, patch_one, post, put, put_one};
use crate::server::hc::hc;
use crate::server::state::AppState;
use crate::storage::Storage;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use serde_json::Value;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Mock server module
pub struct Server {}

impl Server {
    pub async fn startup(
        socket_addr: SocketAddr,
        url: &str,
        storage: Storage,
    ) -> Result<(), MocksError> {
        match TcpListener::bind(socket_addr).await {
            Ok(listener) => {
                println!();
                println!("Endpoints:");
                print_endpoints(url, &storage.data);

                let state = AppState::new(storage);

                let hc_router = Router::new().route("/", get(hc));
                let storage_router = Router::new()
                    .route("/", get(get_all).post(post).put(put_one).patch(patch_one))
                    .route("/:id", get(get_one).put(put).patch(patch).delete(delete));

                let app = Router::new()
                    .nest("/_hc", hc_router)
                    .nest("/:resource", storage_router)
                    .with_state(state);

                axum::serve(listener, app)
                    .await
                    .map_err(|e| MocksError::Exception(e.to_string()))
            }
            Err(e) => Err(MocksError::Exception(e.to_string())),
        }
    }
}

fn print_endpoints(url: &str, value: &Value) {
    println!("{}/{}", url, "_hc");

    if let Value::Object(obj) = value {
        for (key, _) in obj {
            println!("{}/{}", url, key);
        }
    }
}

pub fn response(status_code: StatusCode, body: String) -> Result<impl IntoResponse, StatusCode> {
    match Response::builder()
        .status(status_code.as_u16())
        .header("Content-Type", "application/json")
        .body(body)
    {
        Ok(response) => Ok(response),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn format_err(message: &str) -> String {
    format!("{{\"error\": \"{}\"}}", message)
}
