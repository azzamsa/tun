use crate::{
    Error,
    models::user::{UpdateUser, User},
    services::Service,
};

impl Service {
    pub async fn update_user(&self, id: i32, input: UpdateUser) -> Result<User, Error> {
        let user = self.repo.update_user(id, input).await?;
        Ok(user)
    }
}
