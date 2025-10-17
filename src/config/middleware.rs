use serde::{Deserialize, Serialize};

use crate::pipeline::PipelineStep;

/// Middleware definition
///
/// Middleware are reusable pipeline fragments that can:
/// 1. Add variables to the context (e.g., authenticated user)
/// 2. Short-circuit execution with early return ($return operator)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Middleware {
    /// Pipeline steps to execute
    pub pipeline: Vec<PipelineStep>,
}
