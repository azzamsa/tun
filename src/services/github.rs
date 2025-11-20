use crate::{config::Config, drivers::github as driver};

pub async fn zen(config: &Config) -> Result<String, crate::Error> {
    let resp = driver::zen(config).await?;
    Ok(resp)
}
