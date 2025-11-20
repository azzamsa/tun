use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::app::ServerContext;
use crate::models::user as model;
use crate::services::user as service;

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Get Users", body = [model::User]),
    ),
)]
pub async fn all(ctx: State<Arc<ServerContext>>) -> Result<Json<Vec<model::User>>, crate::Error> {
    let response = service::all(&ctx.db).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "Get User", body = model::User),
    ),
)]
pub async fn one(
    ctx: State<Arc<ServerContext>>,
    Path(id): Path<i64>,
) -> Result<Json<model::User>, crate::Error> {
    let response = service::one(&ctx.db, id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = model::NewUser,
    responses(
        (status = 201, description = "Create User", body = model::User),
    ),
)]
pub async fn create(
    ctx: State<Arc<ServerContext>>,
    Json(body): Json<model::NewUser>,
) -> Result<(StatusCode, Json<model::User>), crate::Error> {
    let user = service::create(&ctx.db, body).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    patch,
    path = "/users/{id}",
    request_body = model::UpdateUser,
    responses(
        (status = 200, description = "Update User", body = model::User),
    ),
)]
pub async fn update(
    ctx: State<Arc<ServerContext>>,
    Path(id): Path<i64>,
    Json(body): Json<model::UpdateUser>,
) -> Result<Json<model::User>, crate::Error> {
    let user = service::update(&ctx.db, id, body).await?;
    Ok(Json(user))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    responses(
        (status = 204, description = "Delete User"),
    ),
)]
pub async fn delete(
    ctx: State<Arc<ServerContext>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, crate::Error> {
    service::delete(&ctx.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
