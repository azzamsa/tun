use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{Error, app::ServerContext, models::user::User};

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Get Users", body = [User]),
    ),
)]
pub async fn find_users(ctx: State<Arc<ServerContext>>) -> Result<Json<Vec<User>>, Error> {
    let response = ctx.service.find_users().await?;
    Ok(Json(response))
}
