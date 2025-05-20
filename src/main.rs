use crate::error::Result;
use config::Config;
use error::{ColorEyreInstallSnafu, FigmentParseSnafu};
use figment::{Figment, providers::Format};
use snafu::ResultExt;
use tokio::main;
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod server;
mod typst_lib;

#[main]
async fn main() -> Result<()> {
    color_eyre::install().context(ColorEyreInstallSnafu)?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug,axum=trace", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if let Err(e) = run().await {
        error!("{e}");
        std::process::exit(1);
    }

    Ok(())
}

pub async fn run() -> Result<()> {
    let config: Config = Figment::new()
        .merge(figment::providers::Toml::file("Config.toml"))
        .merge(figment::providers::Env::prefixed("APP_"))
        .extract()
        .context(FigmentParseSnafu {})?;
    let server = server::Server::new(config.server);
    server.run().await
}
