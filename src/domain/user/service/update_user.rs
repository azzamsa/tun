use super::Service;
use crate::{
    Error,
    domain::user::entities::{UpdateUser, User},
};

impl Service {
    pub async fn update_user(&self, id: i32, input: UpdateUser) -> Result<User, Error> {
        let user = self.repo.update_user(id, input).await?;
        Ok(user)
    }
}
