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
