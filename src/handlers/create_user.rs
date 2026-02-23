use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{app::ServerContext, models::user as model};

#[utoipa::path(
    post,
    path = "/users",
    request_body = model::NewUser,
    responses(
        (status = 201, description = "Create User", body = model::User),
    ),
)]
pub async fn create_user(
    ctx: State<Arc<ServerContext>>,
    Json(input): Json<model::NewUser>,
) -> Result<(StatusCode, Json<model::User>), crate::Error> {
    let user = ctx.service.create_user(input).await?;
    Ok((StatusCode::CREATED, Json(user)))
}
