use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http,
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::app::ServerContext;
use crate::models::user as model;
use crate::services::user as service;

pub(crate) fn router(state: Arc<ServerContext>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(users))
        .routes(routes!(user, delete))
        .routes(routes!(update))
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
    State(ctx): State<Arc<ServerContext>>,
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
    State(ctx): State<Arc<ServerContext>>,
    Path(id): Path<i64>,
) -> Result<Json<model::User>, crate::Error> {
    let response = service::user(&ctx.db, id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    responses(
        (status = 204, description = "Delete User"),
    ),
)]
pub async fn delete(
    State(ctx): State<Arc<ServerContext>>,
    Path(id): Path<i64>,
) -> Result<http::StatusCode, crate::Error> {
    service::delete(&ctx.db, id).await?;
    Ok(http::StatusCode::NO_CONTENT)
}

#[axum::debug_handler]
#[utoipa::path(
    patch,
    path = "/users/{id}",
    request_body = model::UpdateUser,
    responses(
        (status = 200, description = "Update User", body = model::User),
    ),
)]
pub async fn update(
    State(ctx): State<Arc<ServerContext>>,
    Path(id): Path<i64>,
    Json(body): Json<model::UpdateUser>,
) -> Result<Json<model::User>, crate::Error> {
    let user = service::update(&ctx.db, id, body).await?;
    Ok(Json(user))
}
