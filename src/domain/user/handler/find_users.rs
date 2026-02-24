use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{Error, app::ServerContext, domain::user::entities::User};

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Get Users", body = [User]),
    ),
)]
pub async fn find_users(ctx: State<Arc<ServerContext>>) -> Result<Json<Vec<User>>, Error> {
    let response = ctx.user_service.find_users().await?;
    Ok(Json(response))
}
