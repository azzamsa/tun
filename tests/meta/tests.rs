use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::util::ServiceExt;
use tun::routes::app;

use super::schema::MetaResponse;

#[tokio::test]
async fn meta() -> Result<()> {
    let app = app().await?;

    let request = Request::builder().uri("/meta").body(Body::empty())?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body: MetaResponse = serde_json::from_slice(&body)?;
    assert_eq!(body.data.build, "unknown");
    Ok(())
}
