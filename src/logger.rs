use tracing_subscriber::{
    filter::{self},
    fmt::layer,
    prelude::*,
    registry,
};

use crate::config::{Config, Env};

pub fn init(config: &Config) -> Result<(), crate::Error> {
    let log_level = if config.env == Env::Production {
        filter::LevelFilter::INFO
    } else {
        filter::LevelFilter::DEBUG
    };

    let env_filter = filter::EnvFilter::builder()
        .with_default_directive(log_level.into())
        .from_env()?
        .add_directive("sqlx::query=error".parse()?)
        .add_directive("hyper=warn".parse()?)
        .add_directive("reqwest=warn".parse()?);

    let fmt_layer = layer().with_target(true).with_filter(env_filter);
    registry().with(fmt_layer).init();

    Ok(())
}
