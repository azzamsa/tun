use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{
    Error,
    app::ServerContext,
    domain::user::entities::{NewUser, User},
};

#[utoipa::path(
    post,
    path = "/users",
    request_body = NewUser,
    responses(
        (status = 201, description = "Create User", body = User),
    ),
)]
pub async fn create_user(
    ctx: State<Arc<ServerContext>>,
    Json(input): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), Error> {
    let user = ctx.user_service.create_user(input).await?;
    Ok((StatusCode::CREATED, Json(user)))
}
