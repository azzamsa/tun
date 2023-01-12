use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MetaResponse {
    pub data: Meta,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub build: String,
}
