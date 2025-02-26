use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::response::file_stream::FileStream;
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use reqwest::StatusCode;
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use serde::Deserialize;
use snafu::ResultExt;
use tokio::fs::File;
use tokio::net::TcpListener;
use tokio_util::io::ReaderStream;
use tower_http::services::ServeDir;
use tracing::info;

use crate::config::ServerConfig;
use crate::error::{BindSnafu, FileOpenSnafu, Result, ServeSnafu};

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
    // 生成文件流
    let file = File::open("assets/Report.pdf")
        .await
        .context(FileOpenSnafu)?;
    let stream = ReaderStream::new(file);
    let file_stream_resp = FileStream::new(stream).file_name("Report.pdf");
    Ok((resp_header, file_stream_resp).into_response())
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
