use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseConfig {
    /// Collection/table schemas
    #[serde(default)]
    pub schemas: HashMap<String, DatabaseSchema>,
}

/// Database schema for a collection/table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseSchema {
    /// Field definitions
    pub fields: HashMap<String, FieldDefinition>,

    /// Index definitions
    #[serde(default)]
    pub indexes: Vec<IndexDefinition>,
}

/// Field definition in a database schema
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition {
    /// Field type
    #[serde(rename = "type")]
    pub field_type: FieldType,

    /// Whether the field is required
    #[serde(default)]
    pub required: bool,

    /// Whether this is the primary key
    #[serde(default)]
    pub primary: bool,

    /// Whether values must be unique
    #[serde(default)]
    pub unique: bool,

    /// Default value (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// Enum values (restricted set of allowed values)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<serde_json::Value>>,

    /// For array types, defines the element type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<FieldDefinition>>,
}

/// Field type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Datetime,
    Array,
    Object,
    Json,
}

/// Index definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexDefinition {
    /// Fields included in the index
    pub fields: Vec<String>,

    /// Whether this is a unique index
    #[serde(default)]
    pub unique: bool,
}
