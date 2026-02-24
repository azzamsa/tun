use super::Repository;
use crate::errors::app::Error::UserNotFound;
use crate::{Error, domain::user::entities::User};

impl Repository {
    pub async fn update_user(&self, input: User) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET name = ?, full_name = ? WHERE id = ? RETURNING *",
            input.name,
            input.full_name,
            input.id,
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or(UserNotFound)?;
        Ok(user)
    }
}
