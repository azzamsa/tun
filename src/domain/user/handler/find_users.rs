use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{Error, app::ServerContext, domain::user::model::User};

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Get Users", body = [User]),
    ),
)]
pub async fn find_users(ctx: State<Arc<ServerContext>>) -> Result<Json<Vec<User>>, Error> {
    let users = ctx.user_service.find_users().await?;
    let users = users.into_iter().map(|user| user.into()).collect();
    Ok(Json(users))
}
