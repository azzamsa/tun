pub mod github;
pub mod health;
pub mod meta;
pub mod user;

use std::sync::Arc;

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::app::ServerContext;

pub fn router(server_context: Arc<ServerContext>) -> OpenApiRouter {
    OpenApiRouter::new()
        // health
        .routes(routes!(health::health))
        // meta
        .routes(routes!(meta::meta))
        // user
        .routes(routes!(user::all))
        .routes(routes!(user::one, user::delete))
        .routes(routes!(user::create, user::update))
        // github
        .routes(routes!(github::zen))
        .with_state(server_context)
}
