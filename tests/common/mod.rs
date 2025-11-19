use anyhow::Result;
use clap::Parser;
use migration::{Migrator, MigratorTrait};
use tun::{app, config::Config};

pub async fn setup() -> Result<axum::Router> {
    // config
    dotenvy::dotenv().ok();
    let config = Config::parse();

    // db
    let db = app::db(&config).await?;
    Migrator::up(&db, None).await?;

    let app = app::create(db).await?;
    Ok(app)
}
