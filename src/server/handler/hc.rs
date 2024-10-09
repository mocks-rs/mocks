use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn hc() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

#[cfg(test)]
mod tests {
    use crate::server::handler::hc::hc;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn test_hc() {
        let resp = hc().await.into_response();
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }
}
