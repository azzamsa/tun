use reqwest::Method;
use serde_json as json;

use crate::config::Config;

pub async fn zen(config: &Config) -> Result<String, crate::Error> {
    let url = "/zen";
    let resp = request(config, Method::GET, url, None).await?;
    Ok(resp)
}

async fn request(
    config: &Config,
    method: Method,
    url: &str,
    body: Option<json::Value>,
) -> Result<String, crate::Error> {
    let url = format!("{}/{}", config.github_url, url);

    let client = reqwest::Client::new();
    let resp = client
        .request(method.clone(), url)
        .json(&body)
        .send()
        .await?;

    if resp.status().is_success() {
        let body = resp.text().await?;

        Ok(body)
    } else {
        Err(crate::Error::Internal("foo".to_string()))
    }
}
