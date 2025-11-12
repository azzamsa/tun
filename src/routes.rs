use std::sync::Arc;

use axum::{Extension, Router, routing::get};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{config, context::ServerContext, health, meta};

pub async fn app() -> Result<Router, crate::Error> {
    let config = Arc::new(config::Config::load()?);

    let health_service = Arc::new(health::Service::new());
    let meta_service = Arc::new(meta::Service::new());
    let server_context = Arc::new(ServerContext {
        health_service,
        meta_service,
    });

    #[derive(OpenApi)]
    #[openapi(
        paths(
            meta::query::meta,
            health::query::health,
        ),
        components(schemas(
            meta::model::Meta, meta::model::MetaResponse,
            health::model::Health, health::model::HealthResponse
        )),
        tags(
            (name = "Tun", description = "Rust REST API Boilerplate 🏗")
        )
    )]
    struct ApiDoc;

    let mut app = Router::new()
        .route("/meta", get(meta::query::meta))
        .route("/health", get(health::query::health));
    if config.env != config::Env::Production {
        app = app.merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()));
    }
    let app = app.layer(Extension(server_context));

    Ok(app)
}
