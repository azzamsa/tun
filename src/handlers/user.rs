use std::sync::Arc;

use async_graphql::{Context, FieldResult, Object};

use crate::app::ServerContext;
use crate::models::user as model;
use crate::services::user as service;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Get all users
    pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::User>> {
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        Ok(service::all(&ctx.db).await?)
    }

    /// Get a single user by ID
    pub async fn user(&self, ctx: &Context<'_>, id: i64) -> FieldResult<model::User> {
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        Ok(service::one(&ctx.db, id).await?)
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Create a new user
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: model::CreateUser,
    ) -> FieldResult<model::User> {
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        Ok(service::create(&ctx.db, input).await?)
    }

    /// Update an existing user
    pub async fn update_user(
        &self,
        ctx: &async_graphql::Context<'_>,
        input: model::UpdateUser,
    ) -> FieldResult<model::User> {
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        Ok(service::update(&ctx.db, input.id, input).await?)
    }

    /// Delete a user
    pub async fn delete_user(&self, ctx: &Context<'_>, id: i64) -> FieldResult<model::User> {
        let ctx = ctx.data::<Arc<ServerContext>>()?;
        Ok(service::delete(&ctx.db, id).await?)
    }
}
