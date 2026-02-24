use super::Service;
use crate::Error;

impl Service {
    pub async fn delete_user(&self, id: i32) -> Result<(), Error> {
        self.repo.delete_user(id).await?;
        Ok(())
    }
}
