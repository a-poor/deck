use serde::{Deserialize, Serialize};

use super::OperatorValue;

/// $map operator - Transform each item in a collection
///
/// Example:
/// ```json
/// {
///   "$map": {
///     "over": {"$get": "posts"},
///     "do": {
///       "$merge": [
///         {"$get": "item"},
///         {"url": {"$renderString": "/posts/{{item.id}}"}}
///       ]
///     }
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapOp {
    /// Collection to map over
    pub over: OperatorValue,
    /// Operation to perform on each item
    /// Within this operation, the current item is available as "item"
    pub r#do: OperatorValue,
}

/// $filter operator - Filter items in a collection
///
/// Example:
/// ```json
/// {
///   "$filter": {
///     "over": {"$get": "posts"},
///     "where": {"$eq": [{"$get": "item.published"}, true]}
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterOp {
    /// Collection to filter
    pub over: OperatorValue,
    /// Condition that must be true for items to be included
    /// Within this condition, the current item is available as "item"
    pub r#where: OperatorValue,
}

/// $reduce operator - Aggregate/fold a collection
///
/// Example:
/// ```json
/// {
///   "$reduce": {
///     "over": {"$get": "numbers"},
///     "with": {"$add": [{"$get": "accumulator"}, {"$get": "item"}]},
///     "initial": 0
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReduceOp {
    /// Collection to reduce
    pub over: OperatorValue,
    /// Operation to perform on accumulator and each item
    /// "accumulator" and "item" are available in the context
    pub with: OperatorValue,
    /// Initial value for the accumulator
    pub initial: serde_json::Value,
}
