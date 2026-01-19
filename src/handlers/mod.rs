// Third-party imports
use axum::{
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::{json, Value};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Project dependencies
mod product_handler;
mod user_handler;

use crate::models::{product::*, user::*};

#[derive(OpenApi)]
#[openapi(
    paths(
        // User endpoints
        user_handler::get_all_users,
        user_handler::get_user,
        user_handler::create_user,
        user_handler::update_user,
        user_handler::delete_user,
        // Product endpoints
        product_handler::get_all_products,
        product_handler::get_product,
        product_handler::create_product,
        product_handler::update_product,
        product_handler::delete_product,
    ),
    components(
        schemas(User, CreateUser, UpdateUser, Product, CreateProduct, UpdateProduct)
    ),
    tags(
        (name = "Users", description = "User management endpoints"),
        (name = "Products", description = "Product management endpoints")
    ),
    info(
        title = "Rust Axum Microservice API",
        version = "1.0.0",
        description = "A RESTful API built with Axum framework and PostgreSQL",
        contact(
            name = "Henrique Azank",
        )
    )
)]
struct ApiDoc;

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
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
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
