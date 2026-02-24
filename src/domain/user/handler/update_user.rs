use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    Error,
    app::ServerContext,
    domain::user::{
        model::{User, input::UpdateProfile},
        service,
    },
};

#[utoipa::path(
    patch,
    path = "/users/{id}",
    request_body = UpdateProfile,
    responses(
        (status = 200, description = "Update User", body = User),
    ),
)]
pub async fn update_user(
    ctx: State<Arc<ServerContext>>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateProfile>,
) -> Result<Json<User>, Error> {
    let input = service::UpdateProfileInput {
        id,
        name: input.name,
        full_name: input.full_name,
    };
    let user = ctx.user_service.update_user(input).await?;
    Ok(Json(user.into()))
}
