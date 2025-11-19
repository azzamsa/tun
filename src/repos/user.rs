use entity::{prelude::*, *};

use sea_orm as orm;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::errors::app::Error;
use crate::models::user as model;

pub async fn all(db: &orm::DatabaseConnection) -> Result<Vec<user::Model>, crate::Error> {
    let users = User::find().all(db).await?;
    Ok(users)
}

pub async fn one(db: &orm::DatabaseConnection, id: i64) -> Result<user::Model, crate::Error> {
    let user = User::find_by_id(id).one(db).await?;
    match user {
        Some(user) => Ok(user),
        None => Err(Error::UserNotFound.into()),
    }
}

pub async fn create(
    db: &orm::DatabaseConnection,
    new_user: model::NewUser,
) -> Result<user::Model, crate::Error> {
    let user = user::ActiveModel {
        name: Set(new_user.name),
        full_name: Set(new_user.full_name),
        ..Default::default()
    };
    let result = user::Entity::insert(user).exec(db).await?;
    let user = one(db, result.last_insert_id).await?;
    Ok(user)
}

pub async fn update(
    db: &orm::DatabaseConnection,
    id: i64,
    new_user: model::UpdateUser,
) -> Result<user::Model, crate::Error> {
    let current = User::find_by_id(id)
        .one(db)
        .await?
        .ok_or(Error::UserNotFound)?;

    let mut active: user::ActiveModel = current.into();
    active.name = Set(new_user.name.clone());
    active.full_name = Set(new_user.full_name.clone());

    let updated = active.update(db).await?;
    Ok(updated)
}

pub async fn delete(db: &orm::DatabaseConnection, id: i64) -> Result<(), crate::Error> {
    let result = User::delete_by_id(id).exec(db).await?;
    match result.rows_affected {
        1 => Ok(()),
        _ => Err(Error::UserNotFound.into()),
    }
}
