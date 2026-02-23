use crate::{Error, models::meta::Meta, services::Service};

impl Service {
    pub async fn get_meta(&self) -> Result<Meta, Error> {
        let meta = Meta {
            build_hash: option_env!("BUILD_HASH").unwrap_or("unknown").to_string(),
            build_date: option_env!("BUILD_DATE").unwrap_or("unknown").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };
        Ok(meta)
    }
}
