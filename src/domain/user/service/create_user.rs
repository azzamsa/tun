use super::Service;
use crate::{
    Error,
    domain::user::{
        entities::{User, UserInput},
        service::RegisterInput,
    },
};

impl Service {
    pub async fn create_user(&self, input: RegisterInput) -> Result<User, Error> {
        let input = UserInput {
            name: input.name,
            full_name: input.full_name,
        };
        let user = self.repo.create_user(input).await?;
        Ok(user)
    }
}
