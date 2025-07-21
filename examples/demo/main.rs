use color_eyre::eyre::Result;
use figment::{Figment, providers::Format};
use kube_eye_export_server::{
    config::Config,
    error::{self, ColorEyreInstallSnafu, FigmentParseSnafu},
    server,
};
use snafu::ResultExt;
use tracing::{Level, error as error_log};
use tracing_subscriber::{
    FmtSubscriber, fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().context(ColorEyreInstallSnafu)?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");
    if let Err(e) = run().await {
        error_log!("{e}");
        std::process::exit(1);
    }

    Ok(())
}

pub async fn run() -> error::Result<()> {
    let config: Config = Figment::new()
        // .merge(figment::providers::Toml::file(
        //     "/etc/kube-eye-export-server/Config.toml",
        // ))
        .merge(figment::providers::Toml::file(
            "./examples/demo/Config.toml",
        ))
        .merge(figment::providers::Env::prefixed("APP_"))
        .extract()
        .context(FigmentParseSnafu)?;
    let server = server::Server::new(config.server);
    server.run(config.typst).await
}
