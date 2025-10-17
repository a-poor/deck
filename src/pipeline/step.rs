use serde::{Deserialize, Serialize};

use crate::operators::OperatorValue;

/// A single step in a pipeline
///
/// Each step can optionally store its result in the context with a name,
/// and contains an operator expression to execute.
///
/// Example:
/// ```json
/// {
///   "name": "post",
///   "value": {
///     "$dbQuery": {
///       "collection": "posts",
///       "filter": {"id": {"$get": "params.id"}}
///     }
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineStep {
    /// Optional name to store the result in the context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The operator expression to execute
    pub value: OperatorValue,
}
