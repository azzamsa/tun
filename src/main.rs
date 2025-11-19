use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use clap::Parser;

use migration::{Migrator, MigratorTrait};
use tun::{app, config::Config, logger};

#[tokio::main]
async fn main() -> Result<(), tun::Error> {
    // config
    dotenvy::dotenv().ok();
    let config = Arc::new(Config::parse());

    // db
    let db = app::db(Arc::clone(&config)).await?;
    Migrator::up(&db, None).await?;

    // address
    let address = &SocketAddr::new(config.base_url.parse::<IpAddr>()?, config.port);
    let listener = tokio::net::TcpListener::bind(address).await?;

    logger::init(&config)?;
    let app = app::create(config, db).await?;

    tracing::info!("App started at `{}`", address);
    axum::serve(listener, app).await?;

    Ok(())
}
