use axum::{Json, Router, routing};

use crate::models::meta as model;
use crate::services::meta as service;

pub(crate) fn router() -> Router {
    Router::new().route("/meta", routing::get(meta))
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
