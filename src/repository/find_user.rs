use crate::{Error, errors::app::Error::UserNotFound, models::user::User, repository::Repository};

impl Repository {
    pub async fn find_user(&self, id: i32) -> Result<User, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_optional(&self.db)
            .await?
            .ok_or(UserNotFound)?;
        Ok(user)
    }
}
