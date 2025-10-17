use serde::{Deserialize, Serialize};

/// $get operator - Extract value from context using JSON path
///
/// Example: `{"$get": "params.id"}` or `{"$get": "user.email"}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GetOp {
    /// JSON path to the value in the context
    pub path: String,
}
