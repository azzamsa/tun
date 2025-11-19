use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use clap::Parser;
use tun::{config::Config, logger, router};

#[tokio::main]
async fn main() -> Result<(), tun::Error> {
    dotenvy::dotenv().ok();
    let config = Arc::new(Config::parse());

    let address = &SocketAddr::new(config.base_url.parse::<IpAddr>()?, config.port);
    let listener = tokio::net::TcpListener::bind(address).await?;

    logger::init(&config)?;
    let app = router::router(config).await?;

    tracing::info!("App started at `{}`", address);
    axum::serve(listener, app).await?;

    Ok(())
}
