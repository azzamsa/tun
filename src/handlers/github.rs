use std::sync::Arc;

use axum::extract::State;

use crate::app::ServerContext;
use crate::services::github as service;

#[utoipa::path(
    get,
    path = "/zen",
    responses(
        (status = 200, description = "Zen of Github", body = String),
    ),
)]
pub async fn zen(State(ctx): State<Arc<ServerContext>>) -> Result<String, crate::Error> {
    let response = service::zen(&ctx.config).await?;
    Ok(response)
}
