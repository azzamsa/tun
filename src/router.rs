use std::sync::Arc;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::config;
use crate::handlers;

pub async fn router() -> Result<Router, crate::Error> {
    let config = Arc::new(config::Config::load()?);

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "Tun", description = "Rust REST API Boilerplate 🛝")
        )
    )]
    struct ApiDoc;

    let (mut router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(handlers::health::router())
        .merge(handlers::meta::router())
        .split_for_parts();

    if config.env != config::Env::Production {
        router = router
            .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", api.clone()))
            .merge(Redoc::with_url("/redoc", api.clone()));
    }

    Ok(router)
}
