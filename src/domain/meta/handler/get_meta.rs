use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{Error, app::ServerContext, domain::meta::model::Meta};

#[utoipa::path(
    get,
    path = "/meta",
    responses(
        (status = 200, description = "meta information", body = Meta),
    ),
)]
pub async fn get_meta(ctx: State<Arc<ServerContext>>) -> Result<Json<Meta>, Error> {
    let response = ctx.meta_service.get_meta().await?;
    Ok(Json(response.into()))
}
