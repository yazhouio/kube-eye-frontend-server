use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display(
        "{}: Failed to parse config file: {:#?}\n {:#?}\n",
        loc,
        source,
        backtrace
    ))]
    ConfigParse {
        source: serde_json::Error,
        #[snafu(backtrace)]
        backtrace: snafu::Backtrace,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
    #[snafu(display(
        "{}: Failed to bind to address: {:#?}\n {:#?}\n",
        loc,
        source,
        backtrace
    ))]
    Bind {
        #[snafu(source)]
        source: std::io::Error,
        #[snafu(backtrace)]
        backtrace: snafu::Backtrace,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
    #[snafu(display(
        "{}: Failed to parse figment config: {:#?}\n {:#?}\n",
        loc,
        source,
        backtrace
    ))]
    FigmentParse {
        #[snafu(source)]
        source: figment::Error,
        #[snafu(backtrace)]
        backtrace: snafu::Backtrace,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
    #[snafu(display(
        "{}: Failed to install color_eyre: {:#?}\n {:#?}\n",
        loc,
        source,
        backtrace
    ))]
    ColorEyreInstall {
        #[snafu(source)]
        source: color_eyre::Report,
        #[snafu(backtrace)]
        backtrace: snafu::Backtrace,
        #[snafu(implicit)]
        loc: snafu::Location,
    },

    #[snafu(display("{}: Failed to serve: {:#?}\n {:#?}\n", loc, source, backtrace))]
    Serve {
        #[snafu(source)]
        source: std::io::Error,
        #[snafu(backtrace)]
        backtrace: snafu::Backtrace,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
    #[snafu(display("{}: Failed to open file: {:#?}\n {:#?}\n", loc, source, backtrace))]
    FileOpen {
        #[snafu(source)]
        source: std::io::Error,
        #[snafu(backtrace)]
        backtrace: snafu::Backtrace,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = match self {
            Error::FileOpen { .. } => "Failed to open file",
            _ => "Server Error",
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
