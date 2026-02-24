use super::Repository;
use crate::errors::app::Error::UserNotFound;
use crate::{Error, domain::user::entities::User};

impl Repository {
    pub async fn find_user(&self, id: i32) -> Result<User, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_optional(&self.db)
            .await?
            .ok_or(UserNotFound)?;
        Ok(user)
    }
}
