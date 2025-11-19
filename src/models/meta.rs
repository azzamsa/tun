use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Meta {
    pub version: String,
    pub build_hash: String,
    pub build_date: String,
}
