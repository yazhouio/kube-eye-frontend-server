use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Legacy structured client config; kept for compatibility or reference.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LegacyClientConfig {
    pub report_title: HashMap<String, String>,
}

/// Dynamic client-side configuration; shape is defined by YAML at runtime.
pub type ClientConfig = Value;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_legacy_client_config_serialization() {
        let mut report_title = HashMap::new();
        report_title.insert("en".to_string(), "Report".to_string());
        report_title.insert("zh".to_string(), "报告".to_string());

        let config = LegacyClientConfig { report_title };
        let json = serde_json::to_string(&config).unwrap();
        
        assert!(json.contains("Report"));
        assert!(json.contains("报告"));
    }

    #[test]
    fn test_legacy_client_config_deserialization() {
        let json = r#"{
            "report_title": {
                "en": "Annual Report",
                "fr": "Rapport Annuel"
            }
        }"#;

        let config: LegacyClientConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.report_title.len(), 2);
        assert_eq!(config.report_title.get("en"), Some(&"Annual Report".to_string()));
        assert_eq!(config.report_title.get("fr"), Some(&"Rapport Annuel".to_string()));
    }

    #[test]
    fn test_legacy_client_config_clone() {
        let mut report_title = HashMap::new();
        report_title.insert("en".to_string(), "Test".to_string());

        let config = LegacyClientConfig { report_title };
        let cloned = config.clone();

        assert_eq!(config.report_title, cloned.report_title);
    }

    #[test]
    fn test_client_config_as_object() {
        let config: ClientConfig = json!({
            "theme": "dark",
            "language": "en",
            "features": {
                "pdf_export": true,
                "auto_save": false
            }
        });

        assert!(config.is_object());
        assert_eq!(config["theme"], "dark");
        assert_eq!(config["language"], "en");
        assert_eq!(config["features"]["pdf_export"], true);
    }

    #[test]
    fn test_client_config_as_array() {
        let config: ClientConfig = json!([
            {"id": 1, "name": "Config 1"},
            {"id": 2, "name": "Config 2"}
        ]);

        assert!(config.is_array());
        assert_eq!(config[0]["id"], 1);
        assert_eq!(config[1]["name"], "Config 2");
    }

    #[test]
    fn test_client_config_as_string() {
        let config: ClientConfig = json!("simple_config");
        assert!(config.is_string());
        assert_eq!(config.as_str(), Some("simple_config"));
    }

    #[test]
    fn test_client_config_as_number() {
        let config: ClientConfig = json!(42);
        assert!(config.is_number());
        assert_eq!(config.as_i64(), Some(42));
    }

    #[test]
    fn test_client_config_nested_structure() {
        let config: ClientConfig = json!({
            "server": {
                "host": "localhost",
                "port": 8080,
                "ssl": {
                    "enabled": true,
                    "cert_path": "/path/to/cert"
                }
            },
            "logging": {
                "level": "info",
                "format": "json"
            }
        });

        assert_eq!(config["server"]["host"], "localhost");
        assert_eq!(config["server"]["port"], 8080);
        assert_eq!(config["server"]["ssl"]["enabled"], true);
        assert_eq!(config["logging"]["level"], "info");
    }

    #[test]
    fn test_legacy_client_config_empty() {
        let config = LegacyClientConfig {
            report_title: HashMap::new(),
        };

        assert!(config.report_title.is_empty());
    }
}
