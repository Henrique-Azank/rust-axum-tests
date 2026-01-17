use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub mod user_handler;
pub mod product_handler;

/// Health check endpoint
pub async fn health_check() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "Rust Axum Microservice",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    )
}
