use std::{path::PathBuf, sync::Arc};

use arc_swap::ArcSwap;
use config::Config;
use error::{ColorEyreInstallSnafu, FigmentParseSnafu};
use figment::{Figment, providers::Format};
use notify::{Watcher, recommended_watcher};
use snafu::ResultExt;
use tokio::{main, sync::mpsc};
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{client_config::ClientConfig, error::WatchFileSnafu};

mod auth;
mod client_config;
mod config;
mod error;
mod extractor;
mod server;
mod typst_lib;

#[main]
async fn main() -> color_eyre::eyre::Result<()> {
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

async fn load_server_config() -> error::Result<Config> {
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

pub async fn load_client_config() -> error::Result<ClientConfig> {
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
    let client_config = Arc::new(ArcSwap::from_pointee(client_config));
    let path = vec![
        PathBuf::from("client_config.yaml"),
        PathBuf::from("local_client_config.yaml"),
    ];
    spawn_config_watcher(path, Arc::clone(&client_config)).await?;
    let server = server::Server::new(config.server, client_config);
    server.run(config.typst).await
}

pub async fn spawn_config_watcher(
    path: Vec<PathBuf>,
    client_config: Arc<ArcSwap<ClientConfig>>,
) -> error::Result<()> {
    tokio::spawn(async move {
        let (tx, mut rx) = mpsc::channel(1);
        let mut watcher = recommended_watcher(move |res| {
            let _ = tx.blocking_send(res).map_err(|e| {
                tracing::error!("watch error: {}", e);
                e
            });
        })
        .context(WatchFileSnafu)?;

        for path in &path {
            watcher
                .watch(path.as_path(), notify::RecursiveMode::Recursive)
                .context(WatchFileSnafu)?;
        }

        while let Some(event) = rx.recv().await {
            tracing::info!("event: {:#?}", event);
            Figment::new()
                .merge(figment::providers::Yaml::file("client_config.yaml"))
                .merge(figment::providers::Yaml::file("local_client_config.yaml"))
                .extract()
                .context(FigmentParseSnafu)
                .map_or_else(
                    |e| {
                        tracing::error!("get new config error: {}", e);
                    },
                    |config| {
                        tracing::info!("get new config: {:#?}", &config);
                        client_config.store(Arc::new(config));
                    },
                );
        }
        Ok::<(), error::Error>(())
    });
    Ok(())
}
