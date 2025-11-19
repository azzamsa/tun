use sea_orm as orm;

use crate::models::user as model;
use crate::repos::user as repo;

pub async fn users(db: &orm::DatabaseConnection) -> Result<Vec<model::User>, crate::Error> {
    let users = repo::users(db).await?; // Vec<Model>
    let users = users.into_iter().map(Into::into).collect(); // Vec<User>
    Ok(users)
}
