use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tun::{app, config::Config, db};

pub async fn setup() -> Result<axum::Router> {
    // config
    dotenvy::dotenv().ok();
    let config = Arc::new(Config::parse());

    // db
    let db = db::connect(Arc::clone(&config)).await?;

    let app = app::create(config, db).await?;
    Ok(app)
}
