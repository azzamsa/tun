use async_graphql::{FieldResult, Object};

use crate::models::health::Health;
use crate::services::health as service;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    pub async fn health(&self) -> FieldResult<Health> {
        Ok(service::health().await?)
    }
}
