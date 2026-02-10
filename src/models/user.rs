use async_graphql::{InputObject, SimpleObject};

#[derive(Debug, SimpleObject)]
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

#[derive(Debug, InputObject)]
pub struct CreateUser {
    pub name: String,
    pub full_name: Option<String>,
}

#[derive(Debug, InputObject)]
pub struct UpdateUser {
    pub id: i64,
    pub name: String,
    pub full_name: Option<String>,
}
