// Third-party imports
use axum::{
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::{json, Value};
use sqlx::PgPool;

// Project dependencies
mod product_handler;
mod user_handler;

/// Health check endpoint
async fn health_check() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "Rust Axum Microservice",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    )
}

/// Configure all application routes
pub fn router() -> Router<PgPool> {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // User routes
        .route("/api/v1/users", get(user_handler::get_all_users))
        .route("/api/v1/users", post(user_handler::create_user))
        .route("/api/v1/users/:id", get(user_handler::get_user))
        .route("/api/v1/users/:id", put(user_handler::update_user))
        .route("/api/v1/users/:id", delete(user_handler::delete_user))
        // Product routes
        .route("/api/v1/products", get(product_handler::get_all_products))
        .route("/api/v1/products", post(product_handler::create_product))
        .route("/api/v1/products/:id", get(product_handler::get_product))
        .route("/api/v1/products/:id", put(product_handler::update_product))
        .route(
            "/api/v1/products/:id",
            delete(product_handler::delete_product),
        )
}
