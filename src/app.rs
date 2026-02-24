use std::sync::Arc;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::domain::{github, meta, user};
use crate::router;
use crate::{config, db::Db};

pub type HttpClient = reqwest::Client;

pub struct ServerContext {
    pub meta_service: meta::Service,
    pub user_service: user::Service,
    pub github_service: github::Service,
}

pub async fn create(db: Db, config: config::Config) -> Result<Router, crate::Error> {
    let config = Arc::new(config);
    // meta
    let meta_service = meta::Service::new();
    // meta
    let github_service = github::Service::new(config);
    // user
    let user_service = user::Service::new(db);

    let server_context = Arc::new(ServerContext {
        meta_service,
        user_service,
        github_service,
    });

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "Tun", description = "Rust REST API Boilerplate 🛝")
        )
    )]
    struct ApiDoc;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(router::create(server_context))
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()));

    Ok(router)
}
