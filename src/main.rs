use std::sync::Arc;

use arc_swap::ArcSwap;
use color_eyre::eyre::{Ok, Result};
use config::Config;
use error::{ColorEyreInstallSnafu, FigmentParseSnafu};
use figment::{Figment, providers::Format};
use snafu::ResultExt;
use tokio::main;
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::client_config::ClientConfig;

mod auth;
mod client_config;
mod config;
mod error;
mod extractor;
mod server;
mod typst_lib;

#[main]
async fn main() -> Result<()> {
    color_eyre::install().context(ColorEyreInstallSnafu)?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
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

async fn load_server_config() -> Result<Config> {
    let config: Config = Figment::new()
        .merge(figment::providers::Toml::file(
            "/etc/kube-eye-export-server/Config.toml",
        ))
        .merge(figment::providers::Toml::file("Config.toml"))
        .merge(figment::providers::Env::prefixed("APP_"))
        .extract()
        .context(FigmentParseSnafu)?;
    Ok(config)
}

pub async fn load_client_config() -> Result<ClientConfig> {
    let config: ClientConfig = Figment::new()
        .merge(figment::providers::Yaml::file("client_config.yaml"))
        .merge(figment::providers::Yaml::file("local_client_config.yaml"))
        .extract()
        .context(FigmentParseSnafu)?;
    Ok(config)
}

pub async fn run() -> error::Result<()> {
    let config: Config = load_server_config().await?;
    let client_config: ClientConfig = load_client_config().await?;
    tracing::info!("get client config: {:#?}", &client_config);

    let server = server::Server::new(config.server, Arc::new(ArcSwap::from_pointee(client_config)));
    server.run(config.typst).await
}
