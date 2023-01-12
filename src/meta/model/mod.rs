use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::meta::entities;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Meta {
    pub build: String,
    pub version: String,
}

impl From<entities::Meta> for Meta {
    fn from(meta: entities::Meta) -> Self {
        Self {
            build: meta.build,
            version: meta.version,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MetaResponse {
    pub data: Meta,
}
