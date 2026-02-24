use super::Service;
use crate::Error;

impl Service {
    pub async fn get_zen(&self) -> Result<String, Error> {
        let resp = self.gh.zen().await?;
        Ok(resp)
    }
}
