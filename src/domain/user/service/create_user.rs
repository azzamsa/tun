use super::Service;
use crate::{
    Error,
    domain::user::entities::{NewUser, User},
};

impl Service {
    pub async fn create_user(&self, input: NewUser) -> Result<User, Error> {
        let user = self.repo.create_user(input).await?;
        Ok(user)
    }
}
