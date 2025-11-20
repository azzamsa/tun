use axum::Json;

use crate::models::meta as model;
use crate::services::meta as service;

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
