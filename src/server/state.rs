use crate::storage::Storage;
use std::sync::{Arc, RwLock};

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
pub struct AppState {
    pub storage: Storage,
}

impl AppState {
    pub fn new(storage: Storage) -> SharedState {
        Arc::new(RwLock::new(AppState { storage }))
    }
}
