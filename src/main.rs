use kube_eye_export_server::{error::ColorEyreInstallSnafu, run};
use snafu::ResultExt;
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
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
