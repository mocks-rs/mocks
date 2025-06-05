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
        print_endpoints(url, storage.resources());
        println!();

        let data = storage.data.clone();
        let state = AppState::new(storage);
        let router = create_router(state, &data);
        axum::serve(listener, router)
            .await
            .map_err(|e| MocksError::Exception(e.to_string()))
    }
}

fn print_endpoints(url: &str, resources: Vec<String>) {
    let mut endpoints = vec![format!("{}/_hc", url)];

    for r in resources {
        endpoints.push(format!("{}/{}", url, r));
    }

    for endpoint in endpoints {
        println!("{}", endpoint);
    }
}

fn convert_to_resource_paths(value: &Value) -> Vec<String> {
    let mut paths = vec![];
    let mut resources = vec![];

    if let Value::Object(obj) = value {
        for (key, _) in obj {
            if let Some(last_slash) = key.rfind('/') {
                let (prefix, _) = key.split_at(last_slash + 1);
                paths.push(format!("/{}:resource", prefix));
                resources.push(key.replace(prefix, ""));
            } else {
                paths.push("/:resource".to_string());
                resources.push(key.to_string());
            }
        }
    }

    paths.dedup();
    paths.sort_by(|a, b| {
        let a_count = a.matches('/').count();
        let b_count = b.matches('/').count();
        b_count.cmp(&a_count)
    });

    paths
}

fn create_router(state: SharedState, value: &Value) -> Router {
    let hc_router = Router::new().route("/", get(hc));
    let storage_router = Router::new()
        .route("/", get(get_all).post(post).put(put_one).patch(patch_one))
        .route("/:id", get(get_one).put(put).patch(patch).delete(delete));

    let mut router = Router::new().nest("/_hc", hc_router);

    let resource_paths = convert_to_resource_paths(value);
    for path in resource_paths {
        router = router.nest(path.as_str(), storage_router.clone());
    }

    router.with_state(state)
}
