use crate::db::Db;
use crate::models::user as model;
use crate::repos::user as repo;

pub async fn all(db: &Db) -> Result<Vec<model::User>, crate::Error> {
    let users = repo::all(db)?;
    let users = users.into_iter().collect();
    Ok(users)
}

pub async fn one(db: &Db, id: i32) -> Result<model::User, crate::Error> {
    let user = repo::one(db, id)?;
    Ok(user)
}

pub async fn create(db: &Db, new_user: model::NewUser) -> Result<model::User, crate::Error> {
    let user = repo::create(db, new_user)?;
    Ok(user)
}

pub async fn update(
    db: &Db,
    id: i32,
    new_user: model::UpdateUser,
) -> Result<model::User, crate::Error> {
    let user = repo::update(db, id, new_user)?;
    Ok(user)
}

pub async fn delete(db: &Db, id: i32) -> Result<(), crate::Error> {
    repo::delete(db, id)?;
    Ok(())
}
