use serde::{Deserialize, Serialize};

use super::OperatorValue;

/// $if operator - Conditional branching
///
/// Example:
/// ```json
/// {
///   "$if": {
///     "condition": {"$exists": {"$get": "post"}},
///     "then": {"$get": "post"},
///     "else": {"$return": {"status": 404}}
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfOp {
    /// Condition to evaluate
    pub condition: OperatorValue,
    /// Value to return if condition is true
    pub then: OperatorValue,
    /// Value to return if condition is false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#else: Option<OperatorValue>,
}

/// $switch operator - Multi-way branching (SQL CASE-like)
///
/// Example:
/// ```json
/// {
///   "$switch": {
///     "on": {"$get": "user.role"},
///     "cases": [
///       {"when": "admin", "then": {"$get": "fullData"}},
///       {"when": "user", "then": {"$get": "limitedData"}}
///     ],
///     "default": {"$return": {"status": 403}}
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchOp {
    /// Value to switch on
    pub on: OperatorValue,
    /// Case branches
    pub cases: Vec<SwitchCase>,
    /// Default value if no cases match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<OperatorValue>,
}

/// A single case in a switch statement
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchCase {
    /// Value to match against
    pub when: serde_json::Value,
    /// Value to return if matched
    pub then: OperatorValue,
}
