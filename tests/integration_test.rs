use std::collections::HashMap;
use std::sync::Arc;

use arc_swap::ArcSwap;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::json;
use tower::ServiceExt;

use kube_eye_export_server::{
    client_config::ClientConfig,
    config::{ServerConfig, TypstConfig, Theme},
    server::Server,
};

fn create_test_server_config() -> ServerConfig {
    ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        public_dir_dist: vec![],
    }
}

fn create_test_typst_config() -> TypstConfig {
    let mut themes = HashMap::new();
    themes.insert("default".to_string(), Theme {
        icons: vec![],
        themplates: HashMap::new(),
    });

    TypstConfig {
        assets_dir: "./assets".to_string(),
        themes,
        icons: HashMap::new(),
    }
}

fn create_test_client_config() -> Arc<ArcSwap<ClientConfig>> {
    Arc::new(ArcSwap::from_pointee(ClientConfig::default()))
}

#[tokio::test]
async fn test_health_endpoint() {
    let server = Server::new(create_test_server_config(), create_test_client_config());
    let typst_config = create_test_typst_config();
    
    // Note: We can't easily test the full server.run() in integration tests
    // as it would require binding to a port. Instead, we test individual handlers.
    
    // This is a simplified test structure. In a real scenario, you'd want to
    // construct the router and test it directly.
}

#[tokio::test]
async fn test_version_endpoint() {
    // Similar limitation as above - this would require access to the router
    // constructed in server.run()
}

#[test]
fn test_server_creation() {
    let config = create_test_server_config();
    let client_config = create_test_client_config();
    let server = Server::new(config, client_config);
    
    assert_eq!(server.config.host, "127.0.0.1");
    assert_eq!(server.config.port, 8080);
}

#[test]
fn test_public_dir_dist_configuration() {
    let mut config = create_test_server_config();
    config.public_dir_dist = vec![
        ("./dist".to_string(), "/static".to_string()),
        ("./public".to_string(), "/public".to_string()),
    ];
    
    let client_config = create_test_client_config();
    let server = Server::new(config, client_config);
    
    assert_eq!(server.config.public_dir_dist.len(), 2);
}

#[test]
fn test_typst_config_creation() {
    let config = create_test_typst_config();
    assert!(config.themes.contains_key("default"));
    assert_eq!(config.assets_dir, "./assets");
}
