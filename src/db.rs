use crate::config;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub type Db = SqlitePool;

pub async fn connect(config: &config::Config) -> Result<Db, crate::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    Ok(pool)
}

pub async fn migrate(db: &Db) -> Result<(), crate::Error> {
    sqlx::migrate!("./migrations").run(db).await.map_err(|e| {
        tracing::error!("Migration failed: {}", e);
        crate::Error::Internal(e.to_string())
    })?;
    tracing::info!("Applied database migrations");
    Ok(())
}
