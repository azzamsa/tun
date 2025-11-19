use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::util::ServiceExt;

use tun::router;

#[tokio::test]
async fn health() -> Result<()> {
    let app = router::router().await?;

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}
