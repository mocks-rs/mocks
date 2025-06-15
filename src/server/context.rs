use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

const INVALID_JSON_REQUEST: &str = "Invalid JSON format in request body.";

#[derive(Debug, Clone, Default)]
pub struct Payload(pub Value);

impl<S> FromRequest<S> for Payload
where
    S: Send + Sync,
    Json<Value>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request(
        req: axum::http::Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<Value>::from_request(req, state)
            .await
            .map_err(|e| to_rejection(&e.to_string()))?;

        if !value.is_object() {
            return Err(to_rejection(INVALID_JSON_REQUEST));
        }

        Ok(Payload(value))
    }
}

#[derive(Debug, Clone, Default)]
pub struct PayloadWithId(pub Value);

impl<S> FromRequest<S> for PayloadWithId
where
    S: Send + Sync,
    Json<Value>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request(
        req: axum::http::Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<Value>::from_request(req, state)
            .await
            .map_err(|e| to_rejection(&e.to_string()))?;

        if !value.is_object() {
            return Err(to_rejection(INVALID_JSON_REQUEST));
        }

        // ID is required for updates
        if value.get("id").is_none() {
            return Err(to_rejection("ID is required for creation or update."));
        }

        Ok(PayloadWithId(value))
    }
}

fn to_rejection(message: &str) -> (StatusCode, Json<Value>) {
    let json = Json::from(json!({"error": message}));
    (StatusCode::BAD_REQUEST, json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::json;

    // Helper function for creating test requests
    async fn create_request(body: serde_json::Value) -> Request<Body> {
        Request::builder()
            .uri("/")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    #[tokio::test]
    async fn test_payload_valid_json() {
        let body = json!({"key": "value"});
        let request = create_request(body).await;

        let result = Payload::from_request(request, &()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_payload_invalid_json() {
        let body = "invalid json";
        let request = Request::builder()
            .uri("/")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let result = Payload::from_request(request, &()).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_payload_non_object_json() {
        let body = json!(["array", "not", "object"]);
        let request = create_request(body).await;

        let result = Payload::from_request(request, &()).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_payload_with_id_valid() {
        let body = json!({"id": 1, "name": "test"});
        let request = create_request(body).await;

        let result = PayloadWithId::from_request(request, &()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_payload_with_id_missing_id() {
        let body = json!({"name": "test"});
        let request = create_request(body).await;

        let result = PayloadWithId::from_request(request, &()).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_payload_with_id_invalid_json() {
        let body = "invalid json";
        let request = Request::builder()
            .uri("/")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let result = PayloadWithId::from_request(request, &()).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
}
