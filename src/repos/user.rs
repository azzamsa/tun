use diesel::prelude::*;

use crate::db::DbPool;
use crate::errors::app::Error::UserNotFound;
use crate::models::user as model;
use crate::schema::users::dsl;
use crate::schema::users::dsl::users;

pub fn all(db: &DbPool) -> Result<Vec<model::User>, crate::Error> {
    let result = users.load::<model::User>(&mut db.get()?)?;
    Ok(result)
}

pub fn one(db: &DbPool, id: i32) -> Result<model::User, crate::Error> {
    let user = users
        .filter(dsl::id.eq(id))
        .first::<model::User>(&mut db.get()?)
        .optional()?
        .ok_or(UserNotFound)?;

    Ok(user)
}

pub fn create(db: &DbPool, new_user: model::NewUser) -> Result<model::User, crate::Error> {
    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<model::User>(&mut db.get()?)?;

    Ok(user)
}

pub fn update(
    db: &DbPool,
    user_id: i32,
    new_user: model::UpdateUser,
) -> Result<model::User, crate::Error> {
    let user = diesel::update(users.filter(dsl::id.eq(user_id)))
        .set(&new_user)
        .get_result::<model::User>(&mut db.get()?)
        .optional()?
        .ok_or(UserNotFound)?;

    Ok(user)
}

pub fn delete(db: &DbPool, user_id: i32) -> Result<(), crate::Error> {
    diesel::delete(users.filter(dsl::id.eq(user_id))).execute(&mut db.get()?)?;
    Ok(())
}
