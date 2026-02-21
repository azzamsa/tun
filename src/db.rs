use sea_orm as orm;

use crate::config;
use migration::{Migrator, MigratorTrait};

pub type Db = orm::DatabaseConnection;

pub async fn connect(config: &config::Config) -> Result<Db, crate::Error> {
    Ok(orm::Database::connect(&config.database_url).await?)
}

pub async fn migrate(db: &Db) -> Result<(), crate::Error> {
    Migrator::up(db, None).await?;
    tracing::info!("Applied database migrations");
    Ok(())
}
