use super::Repository;
use crate::{
    Error,
    domain::user::entities::{NewUser, User},
};

impl Repository {
    pub async fn create_user(&self, input: NewUser) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (name, full_name) VALUES (?, ?) RETURNING *",
            input.name,
            input.full_name,
        )
        .fetch_one(&self.db)
        .await?;
        Ok(user)
    }
}
