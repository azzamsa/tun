use sea_orm as orm;

use crate::models::user as model;
use crate::repos::user as repo;

pub async fn users(db: &orm::DatabaseConnection) -> Result<Vec<model::User>, crate::Error> {
    let users = repo::users(db).await?;
    let users = users.into_iter().map(Into::into).collect();
    Ok(users)
}

pub async fn user(db: &orm::DatabaseConnection, id: i64) -> Result<model::User, crate::Error> {
    let user = repo::user(db, id).await?;
    Ok(user.into())
}
