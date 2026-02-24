use std::sync::Arc;

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    app::ServerContext,
    domain::{
        github::handler::get_zen,
        health::handler::get_health,
        meta::handler::get_meta,
        user::handler::{create_user, delete_user, find_user, find_users, update_user},
    },
};

pub fn create(server_context: Arc<ServerContext>) -> OpenApiRouter {
    OpenApiRouter::new()
        // health
        .routes(routes!(get_health::get_health))
        // meta
        .routes(routes!(get_meta::get_meta))
        // user
        .routes(routes!(find_users::find_users))
        .routes(routes!(find_user::find_user, delete_user::delete_user))
        .routes(routes!(create_user::create_user, update_user::update_user))
        // github
        .routes(routes!(get_zen::get_zen))
        .with_state(server_context)
}
