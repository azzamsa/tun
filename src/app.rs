use std::sync::Arc;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{config, db::Db, handlers};

pub type HttpClient = reqwest::Client;

pub(crate) struct ServerContext {
    pub config: config::Config,
    pub db: Db,
    pub http_client: HttpClient,
}

pub async fn create(db: Db, config: config::Config) -> Result<Router, crate::Error> {
    let http_client = reqwest::Client::new();
    let server_context = Arc::new(ServerContext {
        config,
        db,
        http_client,
    });

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
