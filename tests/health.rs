use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::util::ServiceExt;

use tun::{models::health as model, router};

#[tokio::test]
async fn health() -> Result<()> {
    let app = router::router().await?;

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let body: model::Health = serde_json::from_slice(&body)?;
    assert_eq!(body.status, "running");
    Ok(())
}
