use std::io::Cursor;

use axum::body::Body;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::response::file_stream::FileStream;
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use bytes::Bytes;
use reqwest::StatusCode;
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use serde::Deserialize;
use snafu::ResultExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

use crate::config::ServerConfig;
use crate::error::{BindSnafu, FileIoSnafu, Result, ServeSnafu};
use crate::typst_lib::generate_pdf_new;

pub struct Server {
    pub config: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ReportRequest {
    pub name: String,
    pub content: String,
    // pub report_id: String,
}

pub async fn report(
    TypedHeader(_authorization): TypedHeader<Authorization<Bearer>>,
    Json(request): Json<ReportRequest>,
) -> Result<impl IntoResponse> {
    let mut resp_header = HeaderMap::new();
    resp_header.insert(CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
    let disposition = format!("attachment; filename=\"{}.pdf\"", request.name);
    resp_header.insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(&disposition).unwrap(),
    );
    let content =  request.content;
    let pdf: Vec<u8> = generate_pdf_new(content)?;
    let body = Body::from(pdf);
    Ok((resp_header, body).into_response())
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }

    pub fn public_dir_dist(&self, mut router: Router) -> Router {
        tracing::info!("public_dir_dist: {:#?}", self.config.public_dir_dist);
        for (dir, path) in self.config.public_dir_dist.iter() {
            router = router.nest_service(path, ServeDir::new(dir));
        }
        router
    }

    pub async fn run(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = TcpListener::bind(&addr).await.context(BindSnafu)?;
        info!("Server is running on http://{}", &addr);
        let router = Router::new();
        let router = self
            .public_dir_dist(router)
            .route("/version", get(|| async { "0.1.0" }))
            .nest("/api", Router::new().route("/report", post(report)));
        let app = router.into_make_service();
        axum::serve(listener, app).await.context(ServeSnafu)?;
        Ok(())
    }
}
