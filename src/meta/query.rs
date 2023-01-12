use axum::{Extension, Json};
use std::sync::Arc;

use crate::context::ServerContext;
use crate::meta::model;

// utoipa can't find `model::<StructName>`
#[allow(unused_imports)]
use crate::meta::model::MetaResponse;

#[utoipa::path(
    get,
    path = "/meta",
    responses(
        (status = 200, description = "meta information", body = MetaResponse),
    ),
)]
pub async fn meta(
    ctx: Extension<Arc<ServerContext>>,
) -> Result<Json<model::MetaResponse>, crate::Error> {
    let meta = ctx.meta_service.get_meta().await?;

    let response = model::MetaResponse { data: meta.into() };
    Ok(Json(response))
}
