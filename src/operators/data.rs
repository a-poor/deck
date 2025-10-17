use serde::{Deserialize, Serialize};

/// $get operator - Extract value from context using simple dot notation
///
/// Supports:
/// - Simple paths: "user"
/// - Nested objects: "user.email"
/// - Array indices: "items.0"
/// - Deep nesting: "user.profile.roles.0"
///
/// Example: `{"$get": "params.id"}` or `{"$get": "user.email"}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GetOp {
    /// Dot-separated path to the value in the context
    pub path: String,
}

/// $jsonPath operator - Extract values from context using JSONPath expressions
///
/// Supports full JSONPath specification (RFC 9535):
/// - Wildcards: `$.store.book[*].author`
/// - Filters: `$..book[?@.price < 10]`
/// - Recursive descent: `$..author`
/// - Array slicing: `$..book[0:2]`
/// - Regex matching: `$..book[?@.author ~= '(?i)tolkien']`
///
/// Returns an array of matched values (even if only one match).
///
/// Example:
/// ```json
/// {"$jsonPath": "$.store.book[?@.price < 10].title"}
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonPathOp {
    /// JSONPath expression (should start with $)
    pub path: String,
}
