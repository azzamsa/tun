use sea_orm as orm;

use crate::models::user as model;
use crate::repos::user as repo;

pub async fn all(db: &orm::DatabaseConnection) -> Result<Vec<model::User>, crate::Error> {
    let users = repo::all(db).await?;
    let users = users.into_iter().map(Into::into).collect();
    Ok(users)
}

pub async fn one(db: &orm::DatabaseConnection, id: i64) -> Result<model::User, crate::Error> {
    let user = repo::one(db, id).await?;
    Ok(user.into())
}

pub async fn create(
    db: &orm::DatabaseConnection,
    new_user: model::NewUser,
) -> Result<model::User, crate::Error> {
    let user = repo::create(db, new_user).await?;
    Ok(user.into())
}

pub async fn update(
    db: &orm::DatabaseConnection,
    id: i64,
    new_user: model::UpdateUser,
) -> Result<model::User, crate::Error> {
    let user = repo::update(db, id, new_user).await?;
    Ok(user.into())
}

pub async fn delete(db: &orm::DatabaseConnection, id: i64) -> Result<(), crate::Error> {
    repo::delete(db, id).await?;
    Ok(())
}
