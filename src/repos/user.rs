use entity::user;
use sea_orm as orm;
use sea_orm::EntityTrait;

use crate::errors::app::Error;

pub async fn users(db: &orm::DatabaseConnection) -> Result<Vec<user::Model>, crate::Error> {
    let users: Vec<user::Model> = user::Entity::find().all(db).await?;
    Ok(users)
}

pub async fn user(db: &orm::DatabaseConnection, id: i64) -> Result<user::Model, crate::Error> {
    let user = user::Entity::find_by_id(id).one(db).await?;
    match user {
        Some(user) => Ok(user),
        None => Err(Error::UserNotFound.into()),
    }
}
