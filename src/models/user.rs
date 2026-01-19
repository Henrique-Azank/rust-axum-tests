use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUser {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUser {
    #[schema(example = "John Updated")]
    pub name: Option<String>,
    #[schema(example = "john.updated@example.com")]
    pub email: Option<String>,
}
