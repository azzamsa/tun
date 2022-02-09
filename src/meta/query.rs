use axum::{Extension, Json};
use std::sync::Arc;

use crate::context::ServerContext;
use crate::meta::model;

pub async fn get_meta(
    ctx: Extension<Arc<ServerContext>>,
) -> Result<Json<model::Meta>, crate::Error> {
    let meta = ctx.meta_service.get_meta().await?;
    Ok(Json(meta.into()))
}
