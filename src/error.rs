use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use color_eyre::eyre::Report;
use serde::Serialize;
use snafu::Snafu;
use tracing::error;

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
        "{}: Failed to parse figment config: {}\n {:#?}\n",
        loc,
        source,
        source,
    ))]
    FigmentParse {
        #[snafu(source)]
        source: figment::Error,
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
    #[snafu(display("{}: Failed to file io: {:#?}\n {:#?}\n", loc, source, backtrace))]
    FileIo {
        #[snafu(source)]
        source: std::io::Error,
        #[snafu(backtrace)]
        backtrace: snafu::Backtrace,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
    #[snafu(display("Failed to generate pdf:{}", message))]
    TypstPdf { message: String },

    #[snafu(display("Invalid input: {reason}"))]
    InvalidInput { reason: String },

    #[snafu(display("Bad request: {message}"))]
    BadRequest { message: String },

    #[snafu(display("Missing Authorization"))]
    MissingAuth,

    #[snafu(display("Invalid Token"))]
    InvalidToken,

    #[snafu(display("Invalid Json Body. {}", source))]
    InvalidJsonBody {
        source: axum::extract::rejection::JsonRejection,
        #[snafu(implicit)]
        loc: snafu::Location,
    },

    #[snafu(display("Internal Error: {}", source))]
    Internal { source: Report },

    WatchFile {
        source: notify::Error,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Serialize)]
struct ErrorResponse {
    code: u32,
    message: String,
}

impl From<JsonRejection> for Error {
    fn from(rejection: JsonRejection) -> Self {
        tracing::debug!("JsonRejection: {:?}", rejection);
        Error::BadRequest {
            message: format!("JSON parse error: {}", rejection.body_text()),
        }
    }
}

impl From<color_eyre::Report> for Error {
    fn from(source: Report) -> Self {
        Error::Internal { source }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("❌ API Error: {:#?}", self);
        let (status, code, message) = match self {
            Error::MissingAuth => (StatusCode::UNAUTHORIZED, 1001, self.to_string()),
            Error::InvalidToken => (StatusCode::UNAUTHORIZED, 1003, self.to_string()),
            Error::InvalidJsonBody { .. } => {
                (StatusCode::UNPROCESSABLE_ENTITY, 1005, self.to_string())
            }
            // Error::TypstPdf { message } => {
            //     (StatusCode::INTERNAL_SERVER_ERROR, 5000, self.to_string())
            // }
            // Error::Internal { .. } => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     5000,
            //     "内部错误".to_string(),
            // ),
            // Error::FileIo { .. } => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     5000,
            //     "内部错误".to_string(),
            // ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, 5000, self.to_string()),
        };
        let body = axum::Json(ErrorResponse { code, message });
        (status, body).into_response()
    }
}
