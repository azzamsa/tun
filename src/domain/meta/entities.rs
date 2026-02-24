use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize, Serialize)]
pub struct Meta {
    pub version: String,
    pub build_hash: String,
    pub build_date: String,
}
