use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    Error,
    app::ServerContext,
    models::user::{UpdateUser, User},
};

#[utoipa::path(
    patch,
    path = "/users/{id}",
    request_body = UpdateUser,
    responses(
        (status = 200, description = "Update User", body = User),
    ),
)]
pub async fn update_user(
    ctx: State<Arc<ServerContext>>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateUser>,
) -> Result<Json<User>, Error> {
    let user = ctx.service.update_user(id, input).await?;
    Ok(Json(user))
}
