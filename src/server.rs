use std::sync::Arc;

use arc_swap::ArcSwap;
use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{
        HeaderMap, HeaderValue,
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    },
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Deserialize;
use snafu::ResultExt;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

use crate::{
    auth,
    client_config::ClientConfig,
    config::{ServerConfig, TypstConfig},
    error::{BindSnafu, Result, ServeSnafu},
    extractor::ValidatedJson,
    typst_lib::generate_pdf,
};

pub struct Server {
    pub config: ServerConfig,
    pub client_config: Arc<ArcSwap<ClientConfig>>,
}

#[derive(Clone)]
pub struct ServerState {
    pub client_config: Arc<ArcSwap<ClientConfig>>,
    pub typst_config: Arc<TypstConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ReportRequest {
    pub name: String,
    pub content: String,
    pub theme: Option<String>,
}

#[tracing::instrument(name = "report", skip(payload))]
pub async fn report(
    State(ServerState { typst_config, .. }): State<ServerState>,
    ValidatedJson(payload): ValidatedJson<ReportRequest>,
) -> Result<impl IntoResponse> {
    let state = typst_config.clone();
    let mut resp_header = HeaderMap::new();
    resp_header.insert(CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
    let encoded_filename = utf8_percent_encode(&payload.name, NON_ALPHANUMERIC).to_string();
    let fallback_filename = "export";
    let disposition = format!(
        "attachment; filename=\"{fallback}{ext}\"; filename*=UTF-8''{encoded}{ext}",
        fallback = fallback_filename,
        ext = ".pdf",
        encoded = encoded_filename
    );
    resp_header.insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(&disposition).unwrap(),
    );
    let content = payload.content;
    let pdf: Vec<u8> = generate_pdf(
        content,
        state.as_ref(),
        payload.theme.unwrap_or("default".to_string()).as_str(),
    )?;
    let body = Body::from(pdf);
    Ok((resp_header, body).into_response())
}

pub async fn client_config_handler(
    State(ServerState { client_config, .. }): State<ServerState>,
) -> impl IntoResponse {
    let arc_cfg: Arc<ClientConfig> = client_config.load().clone();
    Json(arc_cfg)
}

impl Server {
    pub fn new(config: ServerConfig, client_config: Arc<ArcSwap<ClientConfig>>) -> Self {
        Self {
            config,
            client_config,
        }
    }

    pub fn public_dir_dist(&self, mut router: Router<ServerState>) -> Router<ServerState> {
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
        let state = ServerState {
            client_config: Arc::clone(&self.client_config),
            typst_config: typst_config.clone(),
        };
        // let router: Router<ServerState> = Router::new();
        let router = self
            .public_dir_dist(Router::new())
            .layer(TraceLayer::new_for_http())
            .route("/", get(|| async { "ok" }))
            .route("/version", get(|| async { "0.1.0" }))
            .route("/health", get(|| async { "ok" }))
            .nest(
                "/api",
                Router::new()
                    .with_state(state.clone())
                    .route("/report", post(report))
                    .route("/client_config", get(client_config_handler))
                    .layer(middleware::from_fn(auth::simple_token_auth))
                    .layer(TraceLayer::new_for_http()) as Router<ServerState>,
            )
            .with_state(state);
        let app = router.into_make_service();
        axum::serve(listener, app).await.context(ServeSnafu)?;
        Ok(())
    }
}
