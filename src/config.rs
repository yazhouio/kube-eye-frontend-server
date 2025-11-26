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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let config = Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                public_dir_dist: vec![
                    ("./dist".to_string(), "/static".to_string()),
                ],
            },
            typst: TypstConfig {
                assets_dir: "./assets".to_string(),
                themes: HashMap::new(),
                icons: HashMap::new(),
            },
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("127.0.0.1"));
        assert!(json.contains("8080"));
    }

    #[test]
    fn test_config_deserialization() {
        let json = r#"{
            "server": {
                "host": "0.0.0.0",
                "port": 3000,
                "public_dir_dist": [["./public", "/assets"]]
            },
            "typst": {
                "assets_dir": "./typst_assets",
                "themes": {},
                "icons": {}
            }
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.typst.assets_dir, "./typst_assets");
    }

    #[test]
    fn test_theme_with_defaults() {
        let json = r#"{}"#;
        let theme: Theme = serde_json::from_str(json).unwrap();
        assert!(theme.icons.is_empty());
        assert!(theme.themplates.is_empty());
    }

    #[test]
    fn test_theme_with_values() {
        let json = r#"{
            "icons": ["icon1", "icon2"],
            "themplates": {
                "template1": "path/to/template1.typ",
                "template2": "path/to/template2.typ"
            }
        }"#;
        
        let theme: Theme = serde_json::from_str(json).unwrap();
        assert_eq!(theme.icons.len(), 2);
        assert_eq!(theme.themplates.len(), 2);
        assert_eq!(theme.themplates.get("template1"), Some(&"path/to/template1.typ".to_string()));
    }

    #[test]
    fn test_server_config_with_multiple_public_dirs() {
        let config = ServerConfig {
            host: "localhost".to_string(),
            port: 4000,
            public_dir_dist: vec![
                ("./dist".to_string(), "/static".to_string()),
                ("./public".to_string(), "/public".to_string()),
                ("./assets".to_string(), "/assets".to_string()),
            ],
        };

        assert_eq!(config.public_dir_dist.len(), 3);
        assert_eq!(config.public_dir_dist[0].0, "./dist");
        assert_eq!(config.public_dir_dist[0].1, "/static");
    }

    #[test]
    fn test_typst_config_with_themes_and_icons() {
        let mut themes = HashMap::new();
        themes.insert("default".to_string(), Theme {
            icons: vec!["icon1".to_string()],
            themplates: HashMap::new(),
        });

        let mut icons = HashMap::new();
        icons.insert("icon1".to_string(), "path/to/icon1.ttf".to_string());

        let config = TypstConfig {
            assets_dir: "./assets".to_string(),
            themes,
            icons,
        };

        assert_eq!(config.themes.len(), 1);
        assert_eq!(config.icons.len(), 1);
        assert!(config.themes.contains_key("default"));
        assert!(config.icons.contains_key("icon1"));
    }
}
