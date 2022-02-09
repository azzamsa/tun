use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use crate::{config::Config, context::ServerContext, meta};

pub async fn app() -> Result<Router, crate::Error> {
    let _config = Arc::new(Config::load()?);

    let meta_service = Arc::new(meta::Service::new());

    let server_context = Arc::new(ServerContext { meta_service });

    let app = Router::new()
        .route("/meta", get(meta::query::get_meta))
        .layer(Extension(server_context));

    Ok(app)
}
