use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn hc() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}
