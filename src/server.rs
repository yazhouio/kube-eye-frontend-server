use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::State,
    http::{HeaderMap, HeaderValue},
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use serde::Deserialize;
use snafu::ResultExt;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

use crate::{
    auth,
    config::{ServerConfig, TypstConfig},
    error::{BindSnafu, Result, ServeSnafu},
    extractor::ValidatedJson,
    typst_lib::generate_pdf_new,
};

pub struct Server {
    pub config: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ReportRequest {
    pub name: String,
    pub content: String,
    // pub report_id: String,
}

#[tracing::instrument(name = "report", skip(payload))]
pub async fn report(
    State(state): State<Arc<TypstConfig>>,
    ValidatedJson(payload): ValidatedJson<ReportRequest>,
) -> Result<impl IntoResponse> {
    let mut resp_header = HeaderMap::new();
    resp_header.insert(CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
    let disposition = format!("attachment; filename=\"{}.pdf\"", payload.name);
    resp_header.insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(&disposition).unwrap(),
    );
    let assets_dir = state.assets_dir.as_str();
    let content = payload.content;
    let pdf: Vec<u8> = generate_pdf_new(content, assets_dir)?;
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

    pub async fn run(&self, typst_config: TypstConfig) -> Result<()> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let typst_config = Arc::new(typst_config);
        let listener = TcpListener::bind(&addr).await.context(BindSnafu)?;
        info!("Server is running on http://{}", &addr);
        let router = Router::new();
        let router = self
            .public_dir_dist(router)
            .route("/version", get(|| async { "0.1.0" }))
            .nest(
                "/api",
                Router::new()
                    .route("/report", post(report))
                    .with_state(typst_config),
            )
            .layer(middleware::from_fn(auth::simple_token_auth));
        let app = router.into_make_service();
        axum::serve(listener, app).await.context(ServeSnafu)?;
        Ok(())
    }
}
