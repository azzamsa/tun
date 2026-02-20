use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::config;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub async fn connect(config: &config::Config) -> Result<DbPool, crate::Error> {
    let manager = ConnectionManager::<SqliteConnection>::new(&config.database_url);
    let pool = r2d2::Pool::builder().build(manager)?;

    Ok(pool)
}

pub fn migrate(db: &DbPool) -> Result<(), crate::Error> {
    let mut conn = db.get()?;
    conn.run_pending_migrations(MIGRATIONS).map_err(|e| {
        tracing::error!("Migration failed: {}", e);
        crate::Error::Internal(e.to_string())
    })?;
    tracing::info!("Applied database migrations");
    Ok(())
}
