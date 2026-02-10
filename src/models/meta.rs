use async_graphql::SimpleObject;

#[derive(Debug, SimpleObject)]
pub struct Meta {
    pub version: String,
    pub build_hash: String,
    pub build_date: String,
}
