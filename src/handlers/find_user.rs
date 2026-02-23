use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};

use crate::{Error, app::ServerContext, models::user::User};

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "Get User", body = User),
    ),
)]
pub async fn find_user(
    ctx: State<Arc<ServerContext>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, Error> {
    let response = ctx.service.find_user(id).await?;
    Ok(Json(response))
}
