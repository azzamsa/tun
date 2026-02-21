use std::net::{IpAddr, SocketAddr};

use clap::Parser;

use tun::{app, config::Config, db, logger};

#[tokio::main]
async fn main() -> Result<(), tun::Error> {
    // config
    dotenvy::dotenv().ok();
    let config = Config::parse();

    // db
    let db = db::connect(&config).await?;
    db::migrate(&db).await?;

    // address
    let address = &SocketAddr::new(config.base_url.parse::<IpAddr>()?, config.port);
    let listener = tokio::net::TcpListener::bind(address).await?;

    logger::init(&config)?;
    let app = app::create(db, config).await?;

    tracing::info!("App started at `{}`", address);
    axum::serve(listener, app).await?;

    Ok(())
}
