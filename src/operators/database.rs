use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::OperatorValue;

/// $dbQuery operator - Query documents from a collection
///
/// Example:
/// ```json
/// {
///   "$dbQuery": {
///     "collection": "posts",
///     "filter": {"id": {"$get": "params.id"}},
///     "select": ["title", "body", "authorId"],
///     "limit": 10
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbQueryOp {
    /// Collection name
    pub collection: String,
    /// Filter criteria (MongoDB-like query)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HashMap<String, OperatorValue>>,
    /// Fields to select (projection)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<Vec<String>>,
    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Number of results to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u32>,
    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<HashMap<String, SortOrder>>,
}

/// Sort order for database queries
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// $dbInsert operator - Insert a document into a collection
///
/// Example:
/// ```json
/// {
///   "$dbInsert": {
///     "collection": "posts",
///     "document": {
///       "title": {"$get": "body.title"},
///       "authorId": {"$get": "user.id"},
///       "createdAt": {"$now": null}
///     },
///     "validate": true
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbInsertOp {
    /// Collection name
    pub collection: String,
    /// Document to insert
    pub document: HashMap<String, OperatorValue>,
    /// Whether to validate against schema
    #[serde(default)]
    pub validate: bool,
}

/// $dbUpdate operator - Update documents in a collection
///
/// Example:
/// ```json
/// {
///   "$dbUpdate": {
///     "collection": "posts",
///     "filter": {"id": {"$get": "params.id"}},
///     "update": {
///       "title": {"$get": "body.title"},
///       "updatedAt": {"$now": null}
///     }
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbUpdateOp {
    /// Collection name
    pub collection: String,
    /// Filter criteria for documents to update
    pub filter: HashMap<String, OperatorValue>,
    /// Fields to update
    pub update: HashMap<String, OperatorValue>,
    /// Whether to validate against schema
    #[serde(default)]
    pub validate: bool,
}

/// $dbDelete operator - Delete documents from a collection
///
/// Example:
/// ```json
/// {
///   "$dbDelete": {
///     "collection": "posts",
///     "filter": {"id": {"$get": "params.id"}}
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbDeleteOp {
    /// Collection name
    pub collection: String,
    /// Filter criteria for documents to delete
    pub filter: HashMap<String, OperatorValue>,
}
