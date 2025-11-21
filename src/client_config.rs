use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ClientConfig {
    pub report_title: HashMap<String, String>,
}
