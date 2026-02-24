use super::Repository;
use crate::errors::app::Error::UserNotFound;
use crate::{
    Error,
    domain::user::entities::{UpdateUser, User},
};

impl Repository {
    pub async fn update_user(&self, user_id: i32, new_user: UpdateUser) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET name = ?, full_name = ? WHERE id = ? RETURNING *",
            new_user.name,
            new_user.full_name,
            user_id,
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or(UserNotFound)?;
        Ok(user)
    }
}
