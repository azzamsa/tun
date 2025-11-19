use std::sync::Arc;

use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use clap::Parser;
use tower::util::ServiceExt;

use tun::{config::Config, router};

#[tokio::test]
async fn health() -> Result<()> {
    dotenvy::dotenv().ok();
    let config = Arc::new(Config::parse());
    let app = router::router(config).await?;

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}
