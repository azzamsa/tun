use super::Service;
use crate::{Error, domain::user::entities::User};

impl Service {
    pub async fn find_user(&self, id: i32) -> Result<User, Error> {
        let user = self.repo.find_user(id).await?;
        Ok(user)
    }
}
