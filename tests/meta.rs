use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::util::ServiceExt;

use tun::models::meta as model;

mod common;

#[tokio::test]
async fn meta() -> Result<()> {
    let app = common::setup().await?;

    let response = app
        .oneshot(Request::builder().uri("/meta").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await?.to_bytes();
    let body: model::Meta = serde_json::from_slice(&body)?;
    assert_eq!(body.build, "unknown");
    Ok(())
}
