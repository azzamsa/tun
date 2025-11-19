use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Application configuration loaded from command-line flags or environment
/// variables. Environment variables take precedence and are the recommended
/// way to configure the service in production.
/// See `.env.sample` in the repository root for details.
#[derive(clap::Parser)]
pub struct Config {
    /// The deployment environment the application is running in.
    ///
    /// Common values include:
    /// - `"dev"` for local development
    /// - `"staging"` for staging/testing
    /// - `"production"` for production deployments
    #[clap(long, env)]
    pub env: Env,

    /// The base URL used to construct absolute links or public-facing endpoints.
    #[clap(long, env)]
    pub base_url: String,

    /// The port the HTTP server should listen on.
    ///
    /// Examples:
    /// - `3000` for local development
    /// - `80` or `443` behind a load balancer
    #[clap(long, env)]
    pub port: u16,

    /// The connection URL for the database.
    #[clap(long, env)]
    pub database_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ValueEnum)]
pub enum Env {
    Dev,
    Staging,
    Production,
}
