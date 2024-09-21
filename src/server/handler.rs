pub mod delete;
pub mod get;
pub mod hc;
pub mod patch;
pub mod post;
pub mod put;

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::server::context::{Payload, PayloadWithId};
    use crate::server::state::AppState;
    use crate::server::state::SharedState;
    use crate::storage::Storage;
    use axum::extract::{Path, State};
    use serde_json::json;

    fn init_state() -> SharedState {
        let storage = Storage::new("storage.json", false)
            .unwrap_or_else(|e| panic!("Failed to init storage: {}", e));
        AppState::new(storage)
    }

    #[tokio::test]
    async fn test_get_all() {
        let state = init_state();
        let path: Path<String> = Path("posts".to_string());
        assert!(get_all(path, State(state)).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_one() {
        let state = init_state();
        let path: Path<(String, String)> = Path((
            "posts".to_string(),
            "01J7BAKH37HPG116ZRRFKHBDGB".to_string(),
        ));
        assert!(get_one(path, State(state)).await.is_ok());
    }

    #[tokio::test]
    async fn test_post() {
        let state = init_state();
        let path: Path<String> = Path("posts".to_string());
        let payload = json!({"id":"01J8593X0V7Q34X011BYD92CHP","title":"posted post","views":0});
        assert!(post(path, State(state), PayloadWithId(payload))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_put() {
        let state = init_state();
        let path: Path<(String, String)> = Path((
            "posts".to_string(),
            "01J7BAKH37HPG116ZRRFKHBDGB".to_string(),
        ));
        let payload = json!({"id":"01J7BAKH37HPG116ZRRFKHBDGB","title":"putted post","views":200});
        assert!(put(path, State(state), PayloadWithId(payload))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_put_one() {
        let state = init_state();
        let path: Path<String> = Path("profile".to_string());
        let payload = json!({"id":1,"name":"John Smith","age":25});
        assert!(put_one(path, State(state), PayloadWithId(payload))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_patch() {
        let state = init_state();
        let path: Path<(String, String)> = Path((
            "posts".to_string(),
            "01J7BAKH37HPG116ZRRFKHBDGB".to_string(),
        ));
        let payload = json!({"title":"patched post","views":200});
        assert!(patch(path, State(state), Payload(payload)).await.is_ok());
    }

    #[tokio::test]
    async fn test_patch_one() {
        let state = init_state();
        let path: Path<String> = Path("profile".to_string());
        let payload = json!({"name":"Jane Smith","age":30});
        assert!(patch_one(path, State(state), Payload(payload))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_delete() {
        let state = init_state();
        let path: Path<(String, String)> = Path((
            "posts".to_string(),
            "01J7BAKH37HPG116ZRRFKHBDGB".to_string(),
        ));
        assert!(delete(path, State(state)).await.is_ok());
    }
}
