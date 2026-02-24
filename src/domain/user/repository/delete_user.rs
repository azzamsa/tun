use super::Repository;
use crate::Error;

impl Repository {
    pub async fn delete_user(&self, user_id: i32) -> Result<(), Error> {
        sqlx::query!("DELETE FROM users WHERE id = ?", user_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}
