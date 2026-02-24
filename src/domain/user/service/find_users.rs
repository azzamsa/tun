use super::Service;
use crate::{Error, domain::user::entities::User};

impl Service {
    pub async fn find_users(&self) -> Result<Vec<User>, Error> {
        let users = self.repo.find_users().await?;
        let users = users.into_iter().collect();
        Ok(users)
    }
}
