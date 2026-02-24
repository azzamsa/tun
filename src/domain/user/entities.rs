use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(ToSchema, Deserialize, Serialize)]
pub struct NewUser {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(ToSchema, Deserialize, Serialize)]
pub struct UpdateUser {
    pub name: String,
    pub full_name: Option<String>,
}
