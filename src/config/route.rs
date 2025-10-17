use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::operators::OperatorValue;
use crate::pipeline::PipelineStep;

/// HTTP method enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

/// Route definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    /// URL path pattern (e.g., "/api/v1/posts/:id")
    pub path: String,

    /// HTTP method
    pub method: HttpMethod,

    /// Middleware to apply (references middleware names)
    #[serde(default)]
    pub middleware: Vec<String>,

    /// Pipeline steps to execute
    #[serde(default)]
    pub pipeline: Vec<PipelineStep>,

    /// Response definition (can be conditional using operators)
    pub response: Response,
}

/// HTTP response definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    /// Static response with status, headers, and body
    Static {
        status: u16,
        #[serde(default)]
        headers: HashMap<String, OperatorValue>,
        body: OperatorValue,
    },
    /// Conditional response (using an operator like $if)
    Conditional(OperatorValue),
}
