use super::Service;
use crate::{
    Error,
    domain::user::{entities::User, service::UpdateProfileInput},
};

impl Service {
    pub async fn update_user(&self, input: UpdateProfileInput) -> Result<User, Error> {
        let input = User {
            id: input.id,
            name: input.name,
            full_name: input.full_name,
        };
        let user = self.repo.update_user(input).await?;
        Ok(user)
    }
}
