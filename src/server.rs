mod context;
mod handler;
mod state;

use crate::error::MocksError;
use crate::server::handler::delete::delete;
use crate::server::handler::get::{get_all, get_one};
use crate::server::handler::hc::hc;
use crate::server::handler::patch::{patch, patch_one};
use crate::server::handler::post::post;
use crate::server::handler::put::{put, put_one};
use crate::server::state::{AppState, SharedState};
use crate::storage::Storage;
use axum::routing::get;
use axum::Router;
use serde_json::Value;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Mock server module
pub struct Server {}

impl Server {
    /// Starts the mock server
    ///
    /// # Arguments
    /// * `socket_addr` - The socket address to bind the server to
    /// * `url` - The base URL of the server
    /// * `storage` - The storage instance to use
    ///
    /// # Returns
    /// * `Result<(), MocksError>` - Ok if the server starts successfully, Err otherwise
    pub async fn startup(
        socket_addr: SocketAddr,
        url: &str,
        storage: Storage,
    ) -> Result<(), MocksError> {
        let listener = TcpListener::bind(socket_addr)
            .await
            .map_err(|e| MocksError::Exception(e.to_string()))?;

        println!("Endpoints:");
        print_endpoints(url, &storage.data);

        let state = AppState::new(storage);
        let router = create_router(state);
        axum::serve(listener, router)
            .await
            .map_err(|e| MocksError::Exception(e.to_string()))
    }
}

fn print_endpoints(url: &str, value: &Value) {
    let mut endpoints = vec![format!("{}/_hc", url)];

    if let Value::Object(obj) = value {
        for (key, _) in obj {
            endpoints.push(format!("{}/{}", url, key));
        }
    }

    for endpoint in endpoints {
        println!("{}", endpoint);
    }
}

fn create_router(state: SharedState) -> Router {
    let hc_router = Router::new().route("/", get(hc));
    let storage_router = Router::new()
        .route("/", get(get_all).post(post).put(put_one).patch(patch_one))
        .route("/:id", get(get_one).put(put).patch(patch).delete(delete));

    Router::new()
        .nest("/_hc", hc_router)
        .nest("/:resource", storage_router)
        .with_state(state)
}
