use std::sync::Arc;

use crate::{Error, config};

use sea_orm as orm;

pub async fn connect(config: Arc<config::Config>) -> Result<orm::DatabaseConnection, Error> {
    Ok(orm::Database::connect(&config.database_url).await.unwrap())
}
