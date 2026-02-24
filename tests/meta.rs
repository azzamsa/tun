use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use tower::util::ServiceExt;

mod common;

#[tokio::test]
async fn meta() -> Result<()> {
    let app = common::setup().await?;

    let response = app
        .oneshot(Request::builder().uri("/meta").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let body: Meta = serde_json::from_slice(&body)?;
    assert_eq!(body.build_hash, "unknown");
    Ok(())
}

// Integration test shouldn't see implementation details
#[derive(Deserialize, Serialize)]
pub struct Meta {
    pub build_hash: String,
}
