use crate::{Error, services::Service};

impl Service {
    pub async fn get_zen(&self) -> Result<String, Error> {
        let resp = self.github.zen().await?;
        Ok(resp)
    }
}
