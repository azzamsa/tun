use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize, Serialize)]
pub struct Register {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(ToSchema, Deserialize, Serialize)]
pub struct UpdateProfile {
    pub name: String,
    pub full_name: Option<String>,
}
