pub mod input;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::user::entities;

#[derive(ToSchema, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub full_name: Option<String>,
}

impl From<entities::User> for User {
    fn from(user: entities::User) -> Self {
        User {
            id: user.id,
            name: user.name,
            full_name: user.full_name,
        }
    }
}
