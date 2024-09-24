use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::{async_trait, Json};
use serde_json::{json, Value};

const INVALID_JSON_REQUEST: &str = "Invalid JSON format in request body.";

#[derive(Debug, Clone, Default)]
pub struct Payload(pub Value);

#[async_trait]
impl<S> FromRequest<S> for Payload
where
    S: Send + Sync,
    Json<Value>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
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

#[async_trait]
impl<S> FromRequest<S> for PayloadWithId
where
    S: Send + Sync,
    Json<Value>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
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
