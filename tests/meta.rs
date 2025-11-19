use std::sync::Arc;

use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use clap::Parser;
use http_body_util::BodyExt;
use tower::util::ServiceExt;

use tun::{config::Config, models::meta as model, router};

#[tokio::test]
async fn health() -> Result<()> {
    dotenvy::dotenv().ok();
    let config = Arc::new(Config::parse());
    let app = router::router(config).await?;

    let response = app
        .oneshot(Request::builder().uri("/meta").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let body: model::Meta = serde_json::from_slice(&body)?;
    assert_eq!(body.build, "unknown");
    Ok(())
}
