use std::sync::Arc;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config, db::Db, drivers::github::Github, handlers, repository::Repository, services::Service,
};

pub type HttpClient = reqwest::Client;

pub(crate) struct ServerContext {
    pub service: Service,
}

pub async fn create(db: Db, config: config::Config) -> Result<Router, crate::Error> {
    let repo = Repository::new(db.clone());
    let github = Github::new(config, reqwest::Client::new());
    let service = Service::new(repo, github);
    let server_context = Arc::new(ServerContext { service });

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
