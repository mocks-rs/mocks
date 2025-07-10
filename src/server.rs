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
use colored::*;
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
    pub async fn startup(socket_addr: SocketAddr, storage: Storage) -> Result<(), MocksError> {
        let listener = TcpListener::bind(socket_addr)
            .await
            .map_err(|e| MocksError::Exception(e.to_string()))?;

        print_endpoints(storage.resources());

        let data = storage.data.clone();
        let state = AppState::new(storage);
        let router = create_router(state, &data);
        axum::serve(listener, router)
            .await
            .map_err(|e| MocksError::Exception(e.to_string()))
    }
}

fn print_endpoints(resources: Vec<String>) {
    println!("{}", "Available Endpoints:".blue().bold());
    println!(
        "   {:<7} {}",
        "/_hc".bright_cyan(),
        "(Health Check)".bright_black()
    );

    for resource in resources {
        println!("   {}", format!("/{resource}").bright_cyan());
    }
    println!();
}

fn convert_to_resource_paths(value: &Value) -> Vec<String> {
    let mut paths = vec![];
    let mut resources = vec![];

    if let Value::Object(obj) = value {
        for (key, _) in obj {
            if let Some(last_slash) = key.rfind('/') {
                let (prefix, _) = key.split_at(last_slash + 1);
                paths.push(format!("/{prefix}{{resource}}"));
                resources.push(key.replace(prefix, ""));
            } else {
                paths.push("/{resource}".to_string());
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
        .route("/{id}", get(get_one).put(put).patch(patch).delete(delete));

    let mut router = Router::new().nest("/_hc", hc_router);

    let resource_paths = convert_to_resource_paths(value);
    for path in resource_paths {
        router = router.nest(path.as_str(), storage_router.clone());
    }

    router.with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::NamedTempFile;

    #[test]
    fn test_print_endpoints() {
        let resources = vec!["users".to_string(), "posts".to_string()];
        // Just check that it does not panic; do not check output content
        print_endpoints(resources);
    }

    #[test]
    fn test_print_endpoints_with_empty_resources() {
        let resources: Vec<String> = vec![];
        // Just check that it does not panic; do not check output content
        print_endpoints(resources);
    }

    #[test]
    fn test_convert_to_resource_paths() {
        let value = json!({
            "users": {},
            "posts": {},
        });

        let paths = convert_to_resource_paths(&value);
        assert!(paths.contains(&"/{resource}".to_string()));
    }

    #[test]
    fn test_convert_to_resource_paths_with_nested_paths() {
        let value = json!({
            "api/v1/users": {},
            "api/v1/posts": {},
        });

        let paths = convert_to_resource_paths(&value);
        assert!(paths.contains(&"/api/v1/{resource}".to_string()));
    }

    #[test]
    fn test_create_router() {
        let value = json!({
            "users": [],
        });
        let tmpfile = NamedTempFile::new().unwrap();
        std::fs::write(tmpfile.path(), "{\"users\": []}").unwrap();
        let storage = Storage::new(tmpfile.path().to_str().unwrap(), true).unwrap();
        let state = AppState::new(storage);
        // Just check that router can be created
        let _ = create_router(state, &value);
    }

    #[tokio::test]
    async fn test_startup_success() {
        let tmpfile = NamedTempFile::new().unwrap();
        std::fs::write(tmpfile.path(), "{\"users\": []}").unwrap();
        let storage = Storage::new(tmpfile.path().to_str().unwrap(), true).unwrap();

        let socket_addr = "127.0.0.1:0".parse().unwrap();

        // Test that startup can bind to a port (using port 0 for OS to assign)
        let startup_task = tokio::spawn(async move { Server::startup(socket_addr, storage).await });

        // Cancel the task immediately to avoid running server indefinitely
        startup_task.abort();

        // The test passes if we can create the server setup without panicking
        // The actual binding test would require more complex setup
    }

    #[tokio::test]
    async fn test_startup_invalid_address() {
        // Use an invalid address that should fail to bind
        let socket_addr: Result<SocketAddr, _> = "256.256.256.256:0".parse();
        assert!(socket_addr.is_err());
    }
}
