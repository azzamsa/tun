use axum::{Json, Router, routing};

use crate::models::health as model;
use crate::services::health as service;

pub(crate) fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new().route("/health", routing::get(health))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "health information", body = Health),
    ),
)]
pub async fn health() -> Result<Json<model::Health>, crate::Error> {
    let response = service::health().await?;
    Ok(Json(response))
}
