// Third-party imports
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

// Project dependencies
use crate::models::product::{CreateProduct, Product, UpdateProduct};

/// Get all products
pub async fn get_all_products(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    let products = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, price FROM products ORDER BY id",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch products: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok(Json(products))
}

/// Get a single product by ID
pub async fn get_product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let product = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, price FROM products WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch product {}: {}", id, e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Product {} not found", id);
        (
            StatusCode::NOT_FOUND,
            format!("Product with id {} not found", id),
        )
    })?;

    Ok(Json(product))
}

/// Create a new product
pub async fn create_product(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProduct>,
) -> Result<(StatusCode, Json<Product>), (StatusCode, String)> {
    let product = sqlx::query_as::<_, Product>(
        "INSERT INTO products (name, description, price) VALUES ($1, $2, $3) RETURNING id, name, description, price",
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.price)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create product: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    tracing::info!("Created product with id {}", product.id);
    Ok((StatusCode::CREATED, Json(product)))
}

/// Update an existing product
pub async fn update_product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, (StatusCode, String)> {
    // First, check if product exists
    let existing_product = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, price FROM products WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch product {}: {}", id, e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Product {} not found", id);
        (
            StatusCode::NOT_FOUND,
            format!("Product with id {} not found", id),
        )
    })?;

    // Use existing values if not provided in payload
    let name = payload.name.unwrap_or(existing_product.name);
    let description = payload.description.unwrap_or(existing_product.description);
    let price = payload.price.unwrap_or(existing_product.price);

    let product = sqlx::query_as::<_, Product>(
        "UPDATE products SET name = $1, description = $2, price = $3 WHERE id = $4 RETURNING id, name, description, price",
    )
    .bind(&name)
    .bind(&description)
    .bind(price)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update product {}: {}", id, e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    tracing::info!("Updated product {}", id);
    Ok(Json(product))
}

/// Delete a product
pub async fn delete_product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete product {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    if result.rows_affected() == 0 {
        tracing::warn!("Product {} not found for deletion", id);
        return Err((
            StatusCode::NOT_FOUND,
            format!("Product with id {} not found", id),
        ));
    }

    tracing::info!("Deleted product {}", id);
    Ok(StatusCode::NO_CONTENT)
}
