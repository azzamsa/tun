use std::sync::Arc;

use crate::{health, meta};

#[derive(Clone)]
pub struct ServerContext {
    pub health_service: Arc<health::Service>,
    pub meta_service: Arc<meta::Service>,
}
