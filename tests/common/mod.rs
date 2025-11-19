use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tun::{config::Config, router};

pub async fn setup() -> Result<axum::Router> {
    dotenvy::dotenv().ok();
    let config = Arc::new(Config::parse());
    let app = router::router(config).await?;
    Ok(app)
}
