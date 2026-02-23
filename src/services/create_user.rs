use crate::models::user::{self as model, User};
use crate::{Error, services::Service};

impl Service {
    pub async fn create_user(&self, input: model::NewUser) -> Result<User, Error> {
        let user = self.repo.create_user(input).await?;
        Ok(user)
    }
}
