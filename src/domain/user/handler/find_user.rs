use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};

use crate::{Error, app::ServerContext, domain::user::model::User};

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
    let user = ctx.user_service.find_user(id).await?;
    Ok(Json(user.into()))
}
