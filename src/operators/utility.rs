use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::OperatorValue;

/// $merge operator - Combine multiple objects into one
///
/// Example: `{"$merge": [{"$get": "post"}, {"extra": "field"}]}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MergeOp {
    /// Objects to merge (later objects override earlier ones)
    pub objects: Vec<OperatorValue>,
}

/// $exists operator - Check if a value exists (is non-null)
///
/// Example: `{"$exists": {"$get": "post"}}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExistsOp {
    /// Value to check for existence
    pub value: OperatorValue,
}

/// $renderString operator - Template string rendering
///
/// Example: `{"$renderString": "Hello {{user.name}}, you have {{user.messageCount}} messages"}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenderStringOp {
    /// Template string with {{variable}} placeholders
    pub template: String,
}

/// $return operator - Early return from pipeline
///
/// Example:
/// ```json
/// {
///   "$return": {
///     "status": 404,
///     "headers": {"X-Custom": "value"},
///     "body": {"error": "Not found"}
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnOp {
    /// HTTP status code
    pub status: u16,
    /// Response headers
    #[serde(default)]
    pub headers: HashMap<String, OperatorValue>,
    /// Response body
    pub body: OperatorValue,
}

/// $validate operator - Validate data against a JSON Schema
///
/// Example:
/// ```json
/// {
///   "$validate": {
///     "data": {"$get": "body"},
///     "schema": {
///       "type": "object",
///       "properties": {
///         "title": {"type": "string", "minLength": 1}
///       }
///     },
///     "onFail": {"$return": {"status": 400, "body": {"error": "Invalid"}}}
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateOp {
    /// Data to validate
    pub data: OperatorValue,
    /// JSON Schema or reference to a schema
    pub schema: serde_json::Value,
    /// Action to take if validation fails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_fail: Option<OperatorValue>,
}

/// $now operator - Get current timestamp
///
/// Example: `{"$now": null}`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowOp {
    /// Null value (operator takes no parameters)
    /// This field will deserialize from null in JSON
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<()>,
}

impl Default for NowOp {
    fn default() -> Self {
        Self { value: None }
    }
}
