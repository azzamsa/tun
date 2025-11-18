use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::util::ServiceExt;

use tun::{models::health as model, routes::app};

#[tokio::test]
async fn health() -> Result<()> {
    let app = app().await?;

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let body: model::Health = serde_json::from_slice(&body)?;
    assert_eq!(body.status, "running");
    Ok(())
}
