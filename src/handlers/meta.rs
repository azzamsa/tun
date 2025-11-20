use axum::Json;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::models::meta as model;
use crate::services::meta as service;

pub(crate) fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(meta))
}

#[utoipa::path(
    get,
    path = "/meta",
    responses(
        (status = 200, description = "meta information", body = model::Meta),
    ),
)]
pub async fn meta() -> Result<Json<model::Meta>, crate::Error> {
    let response = service::meta().await?;
    Ok(Json(response))
}
