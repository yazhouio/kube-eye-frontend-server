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
use tokio_util::io::ReaderStream;
use tower_http::services::ServeDir;
use tracing::info;

use crate::api;
use crate::config::ServerConfig;
use crate::error::{BindSnafu, FileIoSnafu, Result, ServeSnafu};
use crate::typst_lib::generate_pdf_new;

pub struct Server {
    pub config: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ReportRequest {
    pub name: String,
    pub report_id: String,
}

pub async fn report(
    // 获取请求头
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    // 获取请求体
    Json(request): Json<ReportRequest>,
) -> Result<impl IntoResponse> {
    let mut resp_header = HeaderMap::new();
    resp_header.insert(CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
    resp_header.insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_static("attachment; filename=\"Report.pdf\""),
    );
    let content = api::query_report();
    let pdf: Vec<u8> = generate_pdf_new(content)?;
    // pdf to file
    // let mut file = File::options()
    //     .create(true)
    //     .write(true)
    //     .open("./Report.pdf")
    //     .await
    //     .context(FileIoSnafu)?;
    // file.write_all(&pdf).await.context(FileIoSnafu)?;
    let body = Body::from(pdf);
    Ok((resp_header, body).into_response())
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }

    pub fn public_dir_dist(&self, mut router: Router) -> Router {
        dbg!(&self.config.public_dir_dist);
        for (dir, path) in self.config.public_dir_dist.iter() {
            router = router.nest_service(path, ServeDir::new(dir));
        }
        router
    }

    pub async fn run(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = TcpListener::bind(&addr).await.context(BindSnafu)?;
        info!("Server is running on {}", &addr);
        let router = Router::new();
        let router = self
            .public_dir_dist(router)
            .nest("/api", Router::new().route("/report", post(report)));
        let app = router.into_make_service();
        axum::serve(listener, app).await.context(ServeSnafu)?;
        Ok(())
    }
}
