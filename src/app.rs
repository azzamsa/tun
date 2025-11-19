use std::sync::Arc;

use axum::Router;
use sea_orm as orm;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::config;
use crate::handlers;

#[derive(Clone)]
pub(crate) struct ServerContext {
    config: Arc<config::Config>,
    db: sea_orm::DatabaseConnection,
}

pub async fn create(
    config: Arc<config::Config>,
    db: orm::DatabaseConnection,
) -> Result<Router, crate::Error> {
    let server_context = ServerContext {
        config: Arc::clone(&config),
        db,
    };

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "Tun", description = "Rust REST API Boilerplate 🛝")
        )
    )]
    struct ApiDoc;

    let (mut router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .with_state(server_context)
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
