use serde_json::Value;
use std::sync::{Arc, RwLock};

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
pub struct AppState {
    pub db: Value,
}

impl AppState {
    pub fn new(db: Value) -> SharedState {
        Arc::new(RwLock::new(AppState { db }))
    }
}
