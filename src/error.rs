use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Failed to parse config file: {}", source))]
    ConfigParse { source: serde_json::Error },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;