use crate::{app::HttpClient, config::Config, drivers::github as driver};

pub async fn zen(config: &Config, client: &HttpClient) -> Result<String, crate::Error> {
    let resp = driver::zen(client, config).await?;
    Ok(resp)
}
