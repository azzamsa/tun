use std::sync::Arc;

use axum::extract::State;

use crate::{Error, app::ServerContext};

#[utoipa::path(
    get,
    path = "/zen",
    responses(
        (status = 200, description = "Zen of Github", body = String),
    ),
)]
pub async fn get_zen(ctx: State<Arc<ServerContext>>) -> Result<String, Error> {
    let response = ctx.github_service.get_zen().await?;
    Ok(response)
}
