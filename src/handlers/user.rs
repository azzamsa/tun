use axum::Json;
use axum::extract;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::app::ServerContext;
use crate::models::user as model;
use crate::services::user as service;

pub(crate) fn router(state: ServerContext) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(users))
        .with_state(state)
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Get Users", body = [model::User]),
    ),
)]
pub async fn users(
    ctx: extract::State<ServerContext>,
) -> Result<Json<Vec<model::User>>, crate::Error> {
    let response = service::users(&ctx.db).await?;
    Ok(Json(response))
}
