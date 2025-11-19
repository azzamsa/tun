use std::sync::Arc;

use axum::Json;
use axum::extract;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::app::ServerContext;
use crate::models::user as model;
use crate::services::user as service;

pub(crate) fn router(state: Arc<ServerContext>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(users))
        .routes(routes!(user))
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
    ctx: extract::State<Arc<ServerContext>>,
) -> Result<Json<Vec<model::User>>, crate::Error> {
    let response = service::users(&ctx.db).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "Get User", body = model::User),
    ),
)]
pub async fn user(
    ctx: extract::State<Arc<ServerContext>>,
    extract::Path(id): extract::Path<i64>,
) -> Result<Json<model::User>, crate::Error> {
    let response = service::user(&ctx.db, id).await?;
    Ok(Json(response))
}
