use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Template configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateConfig {
    /// Base directory for template files (relative to config file)
    pub path: String,

    /// Template engine to use (e.g., "jinja", "handlebars", "ejs")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,

    /// Named references to template files
    #[serde(default)]
    pub files: HashMap<String, String>,
}
