use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{config, context::ServerContext, meta};

pub async fn app() -> Result<Router, crate::Error> {
    let config = Arc::new(config::Config::load()?);

    let meta_service = Arc::new(meta::Service::new());

    let server_context = Arc::new(ServerContext { meta_service });

    #[derive(OpenApi)]
    #[openapi(
        paths(
            meta::query::meta,
        ),
        components(schemas(meta::model::Meta, meta::model::MetaResponse)),
        tags(
            (name = "Tun", description = "Rust REST API Boilerplate 🏗")
        )
    )]
    struct ApiDoc;

    let mut app = Router::new().route("/meta", get(meta::query::meta));
    if config.env != config::Env::Production {
        app = app.merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()));
    }
    let app = app.layer(Extension(server_context));

    Ok(app)
}
