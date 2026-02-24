use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub full_name: Option<String>,
}

pub struct UserInput {
    pub name: String,
    pub full_name: Option<String>,
}
