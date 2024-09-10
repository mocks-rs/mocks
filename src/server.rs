mod get;
mod hc;
mod state;

use crate::server::get::{get_all, get_one};
use crate::server::hc::hc;
use crate::server::state::AppState;
use crate::storage::Storage;
use axum::routing::get;
use axum::Router;
use serde_json::Value;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Mock server module
pub struct Server {}

impl Server {
    pub async fn startup(socket_addr: SocketAddr, url: &str, storage: Storage) {
        let listener = TcpListener::bind(socket_addr)
            .await
            .unwrap_or_else(|_| panic!("TcpListener cannot bind."));

        let value = storage
            .read()
            .unwrap_or_else(|_| panic!("Storage file cannot be read."));

        println!();
        println!("Endpoints:");
        print_endpoints(url, &value);

        let state = AppState::new(value);

        let hc_router = Router::new().route("/", get(hc));
        let storage_router = Router::new()
            .route("/", get(get_all))
            .route("/:id", get(get_one));

        let app = Router::new()
            .nest("/hc", hc_router)
            .nest("/:resource", storage_router)
            .with_state(state);

        axum::serve(listener, app)
            .await
            .unwrap_or_else(|_| panic!("Server cannot launch."));
    }
}

fn print_endpoints(url: &str, value: &Value) {
    if let Value::Object(obj) = value {
        for (key, _) in obj {
            println!("{}/{}", url, key);
        }
    }
}
