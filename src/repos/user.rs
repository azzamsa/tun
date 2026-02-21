use crate::db::Db;
use crate::errors::app::Error::UserNotFound;
use crate::models::user as model;

pub async fn all(db: &Db) -> Result<Vec<model::User>, crate::Error> {
    let users = sqlx::query_as!(model::User, "SELECT * FROM users")
        .fetch_all(db)
        .await?;
    Ok(users)
}

pub async fn one(db: &Db, id: i32) -> Result<model::User, crate::Error> {
    let user = sqlx::query_as!(model::User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_optional(db)
        .await?
        .ok_or(UserNotFound)?;
    Ok(user)
}

pub async fn create(db: &Db, new_user: model::NewUser) -> Result<model::User, crate::Error> {
    let user = sqlx::query_as!(
        model::User,
        "INSERT INTO users (name, full_name) VALUES (?, ?) RETURNING *",
        new_user.name,
        new_user.full_name,
    )
    .fetch_one(db)
    .await?;
    Ok(user)
}

pub async fn update(
    db: &Db,
    user_id: i32,
    new_user: model::UpdateUser,
) -> Result<model::User, crate::Error> {
    let user = sqlx::query_as!(
        model::User,
        "UPDATE users SET name = ?, full_name = ? WHERE id = ? RETURNING *",
        new_user.name,
        new_user.full_name,
        user_id,
    )
    .fetch_optional(db)
    .await?
    .ok_or(UserNotFound)?;
    Ok(user)
}

pub async fn delete(db: &Db, user_id: i32) -> Result<(), crate::Error> {
    sqlx::query!("DELETE FROM users WHERE id = ?", user_id)
        .execute(db)
        .await?;
    Ok(())
}
