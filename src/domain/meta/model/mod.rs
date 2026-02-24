use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::meta::entities;

// Decoupled from the internal model to prevent leaking
// sensitive data and to adapt field names/types for the API.
#[derive(ToSchema, Deserialize, Serialize)]
pub struct Meta {
    pub version: String,
    pub build_hash: String,
    pub build_date: String,
}

impl From<entities::Meta> for Meta {
    fn from(meta: entities::Meta) -> Self {
        Meta {
            version: meta.version,
            build_hash: meta.build_hash,
            build_date: meta.build_date,
        }
    }
}
