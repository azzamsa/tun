use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub full_name: Option<String>,
}

impl From<entity::user::Model> for User {
    fn from(model: entity::user::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            full_name: model.full_name,
        }
    }
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
