use color_eyre::eyre::Result;
use config::Config;
use error::{ColorEyreInstallSnafu, FigmentParseSnafu};
use figment::{Figment, providers::Format};
use snafu::ResultExt;
use tokio::main;
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
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
                    "{}=debug,tower_http=debug,axum=trace",
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

pub async fn run() -> error::Result<()> {
    let config: Config = Figment::new()
        .merge(figment::providers::Toml::file("/etc/kube-eye-export-server/Config.toml"))
        .merge(figment::providers::Toml::file("./Config.toml"))
        .merge(figment::providers::Env::prefixed("APP_"))
        .extract()
        .context(FigmentParseSnafu)?;
    let server = server::Server::new(config.server);
    server.run(config.typst).await
}
