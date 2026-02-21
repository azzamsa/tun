use anyhow::Result;
use clap::Parser;
use tun::{app, config::Config, db};

pub async fn setup() -> Result<axum::Router> {
    // config
    dotenvy::dotenv().ok();
    let config = Config::parse();

    // db
    let db = db::connect(&config).await?;
    db::migrate(&db).await?;

    let app = app::create(db, config).await?;
    Ok(app)
}
