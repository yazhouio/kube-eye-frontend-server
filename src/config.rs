use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub typst: TypstConfig,
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
    pub themes: HashMap<String, Theme>,
    pub icons: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    #[serde(default)]
    pub icons: Vec<String>,
    #[serde(default)]
    pub themplates: HashMap<String, String>,
}
