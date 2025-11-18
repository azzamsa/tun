use std::sync::Arc;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config;
use crate::handlers;
use crate::models;

pub async fn app() -> Result<Router, crate::Error> {
    let config = Arc::new(config::Config::load()?);

    #[derive(OpenApi)]
    #[openapi(
        paths(
            handlers::meta::meta,
            handlers::health::health,
        ),
        components(schemas(
            models::meta::Meta,
            models::health::Health,
        )),
        tags(
            (name = "Tun", description = "Rust REST API Boilerplate 🏗")
        )
    )]
    struct ApiDoc;

    let mut app = Router::new()
        .merge(handlers::health::router())
        .merge(handlers::meta::router());

    if config.env != config::Env::Production {
        app = app.merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()));
    }

    Ok(app)
}
