pub mod delete;
pub mod get;
pub mod hc;
pub mod patch;
pub mod post;
pub mod put;

#[cfg(test)]
mod tests {
    use crate::server::state::AppState;
    use crate::server::state::SharedState;
    use crate::storage::Storage;

    pub(crate) fn init_state() -> SharedState {
        let storage = Storage::new("storage.json", false)
            .unwrap_or_else(|e| panic!("Failed to init storage: {e}"));
        AppState::new(storage)
    }
}
