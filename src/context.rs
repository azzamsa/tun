use std::sync::Arc;

use crate::meta;

#[derive(Clone)]
pub struct ServerContext {
    pub meta_service: Arc<meta::Service>,
}
