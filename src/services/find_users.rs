use crate::{Error, models::user::User, services::Service};

impl Service {
    pub async fn find_users(&self) -> Result<Vec<User>, Error> {
        let users = self.repo.find_users().await?;
        let users = users.into_iter().collect();
        Ok(users)
    }
}
