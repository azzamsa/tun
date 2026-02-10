use async_graphql::{FieldResult, Object};

use crate::models::meta as model;
use crate::services::meta as service;

#[derive(Default)]
pub struct MetaQuery;

#[Object]
impl MetaQuery {
    pub async fn meta(&self) -> FieldResult<model::Meta> {
        Ok(service::meta().await?)
    }
}
