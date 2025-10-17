use serde_json::Value;
use std::collections::HashMap;

use crate::operators::SortOrder;
use crate::pipeline::ExecutionError;

/// Trait for database operations
///
/// This trait allows the executor to be decoupled from any specific
/// database implementation. Implementations can be mocked for testing
/// or swapped for different database backends.
pub trait DatabaseProvider: Send + Sync {
    /// Query documents from a collection
    fn query(
        &self,
        collection: &str,
        filter: Option<&HashMap<String, Value>>,
        select: Option<&[String]>,
        limit: Option<u32>,
        skip: Option<u32>,
        sort: Option<&HashMap<String, SortOrder>>,
    ) -> Result<Vec<Value>, ExecutionError>;

    /// Insert a document into a collection
    fn insert(
        &self,
        collection: &str,
        document: &HashMap<String, Value>,
    ) -> Result<Value, ExecutionError>;

    /// Update documents in a collection
    fn update(
        &self,
        collection: &str,
        filter: &HashMap<String, Value>,
        update: &HashMap<String, Value>,
    ) -> Result<Vec<Value>, ExecutionError>;

    /// Delete documents from a collection
    fn delete(
        &self,
        collection: &str,
        filter: &HashMap<String, Value>,
    ) -> Result<Vec<Value>, ExecutionError>;
}

/// Trait for getting the current time
///
/// This allows time-dependent operations like $now to be deterministic
/// in tests by providing a fixed time.
pub trait TimeProvider: Send + Sync {
    /// Get the current timestamp as an ISO 8601 string
    fn now(&self) -> String;

    /// Get the current Unix timestamp in seconds
    fn unix_timestamp(&self) -> i64;
}

/// Trait for accessing HTTP request data
///
/// This provides access to request parameters, query strings, headers,
/// and body without coupling the executor to a specific HTTP framework.
pub trait RequestContext: Send + Sync {
    /// Get path parameters (e.g., :id in /posts/:id)
    fn params(&self) -> &HashMap<String, String>;

    /// Get query string parameters
    fn query(&self) -> &HashMap<String, String>;

    /// Get request headers
    fn headers(&self) -> &HashMap<String, String>;

    /// Get the request body as JSON
    fn body(&self) -> Option<&Value>;

    /// Get the HTTP method
    fn method(&self) -> &str;

    /// Get the request path
    fn path(&self) -> &str;
}

// Mock implementations for testing

/// Mock database provider for testing
#[derive(Debug, Clone, Default)]
pub struct MockDatabase {
    /// Predefined responses for queries
    pub query_responses: HashMap<String, Vec<Value>>,
    /// Predefined responses for inserts
    pub insert_responses: HashMap<String, Value>,
}

impl MockDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_query_response(mut self, collection: &str, response: Vec<Value>) -> Self {
        self.query_responses.insert(collection.to_string(), response);
        self
    }

    pub fn with_insert_response(mut self, collection: &str, response: Value) -> Self {
        self.insert_responses.insert(collection.to_string(), response);
        self
    }
}

impl DatabaseProvider for MockDatabase {
    fn query(
        &self,
        collection: &str,
        _filter: Option<&HashMap<String, Value>>,
        _select: Option<&[String]>,
        _limit: Option<u32>,
        _skip: Option<u32>,
        _sort: Option<&HashMap<String, SortOrder>>,
    ) -> Result<Vec<Value>, ExecutionError> {
        self.query_responses
            .get(collection)
            .cloned()
            .ok_or_else(|| ExecutionError::database_error(format!("Collection not found: {}", collection)))
    }

    fn insert(
        &self,
        collection: &str,
        _document: &HashMap<String, Value>,
    ) -> Result<Value, ExecutionError> {
        self.insert_responses
            .get(collection)
            .cloned()
            .ok_or_else(|| ExecutionError::database_error(format!("Collection not found: {}", collection)))
    }

    fn update(
        &self,
        _collection: &str,
        _filter: &HashMap<String, Value>,
        _update: &HashMap<String, Value>,
    ) -> Result<Vec<Value>, ExecutionError> {
        Ok(vec![])
    }

    fn delete(
        &self,
        _collection: &str,
        _filter: &HashMap<String, Value>,
    ) -> Result<Vec<Value>, ExecutionError> {
        Ok(vec![])
    }
}

/// Fixed time provider for testing
#[derive(Debug, Clone)]
pub struct FixedTimeProvider {
    timestamp: String,
    unix_timestamp: i64,
}

impl FixedTimeProvider {
    pub fn new(timestamp: &str, unix_timestamp: i64) -> Self {
        Self {
            timestamp: timestamp.to_string(),
            unix_timestamp,
        }
    }
}

impl TimeProvider for FixedTimeProvider {
    fn now(&self) -> String {
        self.timestamp.clone()
    }

    fn unix_timestamp(&self) -> i64 {
        self.unix_timestamp
    }
}

/// Mock request context for testing
#[derive(Debug, Clone, Default)]
pub struct MockRequestContext {
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Option<Value>,
    pub method: String,
    pub path: String,
}

impl MockRequestContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_param(mut self, key: &str, value: &str) -> Self {
        self.params.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }
}

impl RequestContext for MockRequestContext {
    fn params(&self) -> &HashMap<String, String> {
        &self.params
    }

    fn query(&self) -> &HashMap<String, String> {
        &self.query
    }

    fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    fn body(&self) -> Option<&Value> {
        self.body.as_ref()
    }

    fn method(&self) -> &str {
        &self.method
    }

    fn path(&self) -> &str {
        &self.path
    }
}
