use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::util::ServiceExt;

use tun::{models::meta as model, routes::app};

#[tokio::test]
async fn meta() -> Result<()> {
    let app = app().await?;

    let request = Request::builder().uri("/meta").body(Body::empty())?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body: model::Meta = serde_json::from_slice(&body)?;
    assert_eq!(body.build, "unknown");
    Ok(())
}
