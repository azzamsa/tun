use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{Error, app::ServerContext};

#[utoipa::path(
    delete,
    path = "/users/{id}",
    responses(
        (status = 204, description = "Delete User"),
    ),
)]
pub async fn delete_user(
    ctx: State<Arc<ServerContext>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, Error> {
    ctx.user_service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
