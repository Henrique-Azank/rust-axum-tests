// Standard library dependencies
use std::net::SocketAddr;

// Third party dependencies
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Crate dependencies
mod database;
mod handlers;
mod models;

// Main function entrypoint
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_axum_tests=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get configuration from environment
    let app_name =
        std::env::var("APP_NAME").unwrap_or_else(|_| "Rust Axum Microservice".to_string());
    let app_version = std::env::var("APP_VERSION").unwrap_or_else(|_| "1.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    tracing::info!("Starting {} v{}", app_name, app_version);

    // Initialize database connection pool
    let db_pool = database::create_pool().await?;

    // Run migrations
    database::run_migrations(&db_pool).await?;

    // Build application routes
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        // User routes
        .route("/api/v1/users", get(handlers::user_handler::get_all_users))
        .route("/api/v1/users", post(handlers::user_handler::create_user))
        .route("/api/v1/users/:id", get(handlers::user_handler::get_user))
        .route(
            "/api/v1/users/:id",
            put(handlers::user_handler::update_user),
        )
        .route(
            "/api/v1/users/:id",
            delete(handlers::user_handler::delete_user),
        )
        // Product routes
        .route(
            "/api/v1/products",
            get(handlers::product_handler::get_all_products),
        )
        .route(
            "/api/v1/products",
            post(handlers::product_handler::create_product),
        )
        .route(
            "/api/v1/products/:id",
            get(handlers::product_handler::get_product),
        )
        .route(
            "/api/v1/products/:id",
            put(handlers::product_handler::update_product),
        )
        .route(
            "/api/v1/products/:id",
            delete(handlers::product_handler::delete_product),
        )
        // Add database pool to all routes
        .with_state(db_pool)
        // Add tracing middleware
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap_or(3000)));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
