use axum::Json;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::models::health as model;
use crate::services::health as service;

pub(crate) fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(health))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "health information", body = model::Health),
    ),
)]
pub async fn health() -> Result<Json<model::Health>, crate::Error> {
    let response = service::health().await?;
    Ok(Json(response))
}
