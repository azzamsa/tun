use super::Repository;
use crate::{Error, domain::user::entities::User};

impl Repository {
    pub async fn find_users(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&self.db)
            .await?;
        Ok(users)
    }
}
