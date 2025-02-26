use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub typst: TypstConfig,
    pub ks_api: KsApiConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KsApiConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub public_dir_dist: Vec<(String, String)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TypstConfig {
    pub assets_dir: String,
}
