use crate::{Error, services::Service};

impl Service {
    pub async fn delete_user(&self, id: i32) -> Result<(), Error> {
        self.repo.delete_user(id).await?;
        Ok(())
    }
}
