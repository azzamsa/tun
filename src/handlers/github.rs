use std::sync::Arc;

use async_graphql::{Context, FieldResult, Object};

use crate::app::ServerContext;
use crate::services::github as service;

#[derive(Default)]
pub struct GithubQuery;

#[Object]
impl GithubQuery {
    pub async fn zen(&self, ctx: &Context<'_>) -> FieldResult<String> {
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        Ok(service::zen(&ctx.config).await?)
    }
}
