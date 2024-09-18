use crate::storage::Storage;
use std::sync::{Arc, Mutex};

pub type SharedState = Arc<Mutex<AppState>>;

pub struct AppState {
    pub storage: Storage,
}

impl AppState {
    pub fn new(storage: Storage) -> SharedState {
        Arc::new(Mutex::new(AppState { storage }))
    }
}
