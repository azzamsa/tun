use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::util::ServiceExt;

use tun::{models::health as model, routes::app};

#[tokio::test]
async fn health() -> Result<()> {
    let app = app().await?;

    let request = Request::builder().uri("/health").body(Body::empty())?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body: model::Health = serde_json::from_slice(&body)?;
    assert_eq!(body.status, "running");
    Ok(())
}
