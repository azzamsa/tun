use std::sync::Arc;

use axum::Router;
use sea_orm as orm;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{Error, config, handlers};

pub(crate) struct ServerContext {
    pub db: sea_orm::DatabaseConnection,
    pub config: config::Config,
}

pub async fn create(
    db: orm::DatabaseConnection,
    config: config::Config,
) -> Result<Router, crate::Error> {
    let server_context = Arc::new(ServerContext { config, db });

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "Tun", description = "Rust REST API Boilerplate 🛝")
        )
    )]
    struct ApiDoc;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(handlers::router(server_context))
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()));

    Ok(router)
}

pub async fn db(config: &config::Config) -> Result<orm::DatabaseConnection, Error> {
    Ok(orm::Database::connect(&config.database_url).await.unwrap())
}
