use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Product {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "Laptop")]
    pub name: String,
    #[schema(example = "High-performance laptop")]
    pub description: String,
    #[schema(example = 999.99)]
    pub price: f64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProduct {
    #[schema(example = "Laptop")]
    pub name: String,
    #[schema(example = "High-performance laptop")]
    pub description: String,
    #[schema(example = 999.99)]
    pub price: f64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProduct {
    #[schema(example = "Gaming Laptop")]
    pub name: Option<String>,
    #[schema(example = "Ultimate gaming performance")]
    pub description: Option<String>,
    #[schema(example = 1299.99)]
    pub price: Option<f64>,
}
