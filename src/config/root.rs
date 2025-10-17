use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{DatabaseConfig, Middleware, Route, TemplateConfig};

/// Top-level configuration for a deck application
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckConfig {
    /// Database configuration including schemas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<DatabaseConfig>,

    /// Template configuration for HTML rendering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<TemplateConfig>,

    /// Route definitions
    #[serde(default)]
    pub routes: Vec<Route>,

    /// Reusable middleware definitions
    #[serde(default)]
    pub middleware: HashMap<String, Middleware>,

    /// Reusable validation schemas (JSON Schema format)
    #[serde(default)]
    pub schemas: HashMap<String, serde_json::Value>,

    /// Error handlers (TBD in design)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handlers: Option<serde_json::Value>,
}
