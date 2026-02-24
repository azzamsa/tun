use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{
    Error,
    app::ServerContext,
    domain::user::{
        model::{User, input::Register},
        service,
    },
};

#[utoipa::path(
    post,
    path = "/users",
    request_body = Register,
    responses(
        (status = 201, description = "Create User", body = User),
    ),
)]
pub async fn create_user(
    ctx: State<Arc<ServerContext>>,
    Json(input): Json<Register>,
) -> Result<(StatusCode, Json<User>), Error> {
    let input = service::RegisterInput {
        name: input.name,
        full_name: input.full_name,
    };
    let user = ctx.user_service.create_user(input).await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}
