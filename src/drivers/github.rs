use reqwest::Method;
use serde_json as json;

use crate::config::Config;

pub async fn zen(config: &Config) -> Result<String, crate::Error> {
    // don't add leading `/`
    let url = "zen";
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
        .request(method, &url)
        .header("Accept", "application/json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        // GitHub API requires a User-Agent header
        .header("User-Agent", "Tun")
        .json(&body)
        .send()
        .await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => {
            return Err(crate::Error::Internal(format!("Request failed: {}", e)));
        }
    };

    let status = resp.status();
    let is_success = status.is_success();
    let body = resp.text().await;
    let status_code = status.as_str();

    if is_success {
        match body {
            Ok(body) => Ok(body),
            Err(e) => {
                // Handle error reading the body
                Err(crate::Error::InvalidArgument(format!(
                    "Failed to read response body: {}",
                    e
                )))
            }
        }
    } else {
        tracing::info!("URL: `{}`", &url);
        tracing::info!("Status: `{}`", status_code);
        tracing::info!("Body: `{:?}`", body);
        Err(crate::Error::InvalidArgument("foo".to_string()))
    }
}
