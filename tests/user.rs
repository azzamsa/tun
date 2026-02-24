use anyhow::Result;
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use serde_json as json;
use tower::util::ServiceExt;

mod common;

#[tokio::test]
async fn create_user() -> Result<()> {
    let app = common::setup().await?;

    let body = json::json!({
        "name": "Frodo",
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/users")
                .header("Content-Type", "application/json")
                .body(Body::from(json::to_string(&body)?))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await?.to_bytes();
    let body: User = serde_json::from_slice(&body)?;
    assert_eq!(body.name, "Frodo");

    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub full_name: Option<String>,
}
