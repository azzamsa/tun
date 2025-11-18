use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use tun::{config::Config, logger, routes::app};

#[tokio::main]
async fn main() -> Result<(), tun::Error> {
    let config = Arc::new(Config::load()?);
    logger::init(&config)?;

    let app = app().await?;

    let host: IpAddr = config.base_url.parse()?;
    let port = config.http.port;
    let address = &SocketAddr::new(host, port);
    let listener = tokio::net::TcpListener::bind(address).await?;

    tracing::info!("App started at `{}`", address);
    axum::serve(listener, app).await?;

    Ok(())
}
