pub mod get_zen;

use std::sync::Arc;

use crate::{config::Config, drivers::github::Gh};

pub struct Service {
    gh: Gh,
}

impl Service {
    pub fn new(config: Arc<Config>) -> Self {
        let gh = Gh::new(config, reqwest::Client::new());
        Self { gh }
    }
}
