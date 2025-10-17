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

use std::sync::{Arc, Mutex};

/// Mock database provider for testing
///
/// This is a simple in-memory database that supports:
/// - Collections of JSON documents
/// - Simple equality filtering
/// - Sorting, pagination (limit/skip)
/// - Field projection
/// - Update with merge semantics
/// - Delete with audit trail
#[derive(Clone)]
pub struct MockDatabase {
    /// Collections stored in memory
    /// Outer HashMap: collection name -> documents
    /// Inner Vec: list of documents in the collection
    collections: Arc<Mutex<HashMap<String, Vec<Value>>>>,
    /// ID generator function (defaults to incrementing counter)
    id_generator: Arc<dyn Fn() -> String + Send + Sync>,
}

impl std::fmt::Debug for MockDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockDatabase")
            .field("collections", &self.collections)
            .field("id_generator", &"<function>")
            .finish()
    }
}

impl Default for MockDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl MockDatabase {
    pub fn new() -> Self {
        // Default ID generator: simple incrementing counter
        let counter = Arc::new(Mutex::new(0));
        let id_gen = move || {
            let mut c = counter.lock().unwrap();
            *c += 1;
            format!("id_{}", c)
        };

        Self {
            collections: Arc::new(Mutex::new(HashMap::new())),
            id_generator: Arc::new(id_gen),
        }
    }

    /// Add a collection with initial documents
    pub fn with_collection(self, name: &str, documents: Vec<Value>) -> Self {
        let mut collections = self.collections.lock().unwrap();
        collections.insert(name.to_string(), documents);
        drop(collections);
        self
    }

    /// Set a custom ID generator
    pub fn with_id_generator<F>(mut self, generator: F) -> Self
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        self.id_generator = Arc::new(generator);
        self
    }

    /// Helper: Check if a document matches a simple equality filter
    fn matches_filter(doc: &Value, filter: &HashMap<String, Value>) -> bool {
        let obj = match doc.as_object() {
            Some(o) => o,
            None => return false,
        };

        // All filter fields must match (implicit AND)
        for (key, filter_value) in filter {
            let doc_value = obj.get(key);
            match (doc_value, filter_value) {
                (Some(dv), fv) if dv == fv => continue,
                (None, Value::Null) => continue, // null matches missing field
                _ => return false,
            }
        }
        true
    }

    /// Helper: Apply field projection (select)
    fn project_fields(doc: &Value, select: &[String]) -> Value {
        let obj = match doc.as_object() {
            Some(o) => o,
            None => return doc.clone(),
        };

        let mut result = serde_json::Map::new();
        for field in select {
            if let Some(value) = obj.get(field) {
                result.insert(field.clone(), value.clone());
            }
        }
        Value::Object(result)
    }

    /// Helper: Sort documents
    fn sort_documents(docs: &mut [Value], sort: &HashMap<String, SortOrder>) {
        // For simplicity, we'll only sort by the first sort field
        // (supporting multiple sort fields would require more complex logic)
        if let Some((field, order)) = sort.iter().next() {
            docs.sort_by(|a, b| {
                let a_val = a.get(field);
                let b_val = b.get(field);

                let cmp = match (a_val, b_val) {
                    (Some(Value::Number(a)), Some(Value::Number(b))) => {
                        // Compare numbers
                        if let (Some(a_f), Some(b_f)) = (a.as_f64(), b.as_f64()) {
                            a_f.partial_cmp(&b_f).unwrap_or(std::cmp::Ordering::Equal)
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    }
                    (Some(Value::String(a)), Some(Value::String(b))) => a.cmp(b),
                    (Some(_), None) => std::cmp::Ordering::Greater,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    _ => std::cmp::Ordering::Equal,
                };

                match order {
                    SortOrder::Ascending => cmp,
                    SortOrder::Descending => cmp.reverse(),
                }
            });
        }
    }

    /// Helper: Merge update fields into document (partial update)
    fn merge_update(doc: &mut Value, update: &HashMap<String, Value>) {
        if let Some(obj) = doc.as_object_mut() {
            for (key, value) in update {
                obj.insert(key.clone(), value.clone());
            }
        }
    }
}

impl DatabaseProvider for MockDatabase {
    fn query(
        &self,
        collection: &str,
        filter: Option<&HashMap<String, Value>>,
        select: Option<&[String]>,
        limit: Option<u32>,
        skip: Option<u32>,
        sort: Option<&HashMap<String, SortOrder>>,
    ) -> Result<Vec<Value>, ExecutionError> {
        let collections = self.collections.lock().unwrap();

        // Get the collection (return empty array if not found)
        let docs = match collections.get(collection) {
            Some(d) => d.clone(),
            None => return Ok(vec![]),
        };
        drop(collections);

        // Apply filter
        let mut filtered: Vec<Value> = if let Some(f) = filter {
            docs.into_iter()
                .filter(|doc| Self::matches_filter(doc, f))
                .collect()
        } else {
            docs
        };

        // Apply sorting
        if let Some(s) = sort {
            Self::sort_documents(&mut filtered, s);
        }

        // Apply skip
        let skip_count = skip.unwrap_or(0) as usize;
        if skip_count > 0 {
            filtered = filtered.into_iter().skip(skip_count).collect();
        }

        // Apply limit
        if let Some(l) = limit {
            filtered.truncate(l as usize);
        }

        // Apply field projection
        if let Some(fields) = select {
            filtered = filtered
                .into_iter()
                .map(|doc| Self::project_fields(&doc, fields))
                .collect();
        }

        Ok(filtered)
    }

    fn insert(
        &self,
        collection: &str,
        document: &HashMap<String, Value>,
    ) -> Result<Value, ExecutionError> {
        let mut collections = self.collections.lock().unwrap();

        // Convert HashMap to Value::Object
        let mut doc_obj = serde_json::Map::new();
        for (k, v) in document {
            doc_obj.insert(k.clone(), v.clone());
        }

        // Generate ID if not present
        if !doc_obj.contains_key("_id") {
            let id = (self.id_generator)();
            doc_obj.insert("_id".to_string(), Value::String(id));
        }

        let doc_value = Value::Object(doc_obj);

        // Add to collection (create if doesn't exist)
        collections
            .entry(collection.to_string())
            .or_insert_with(Vec::new)
            .push(doc_value.clone());

        Ok(doc_value)
    }

    fn update(
        &self,
        collection: &str,
        filter: &HashMap<String, Value>,
        update: &HashMap<String, Value>,
    ) -> Result<Vec<Value>, ExecutionError> {
        let mut collections = self.collections.lock().unwrap();

        // Get the collection (return empty array if not found)
        let docs = match collections.get_mut(collection) {
            Some(d) => d,
            None => return Ok(vec![]),
        };

        let mut updated_docs = vec![];

        // Find and update matching documents
        for doc in docs.iter_mut() {
            if Self::matches_filter(doc, filter) {
                Self::merge_update(doc, update);
                updated_docs.push(doc.clone());
            }
        }

        Ok(updated_docs)
    }

    fn delete(
        &self,
        collection: &str,
        filter: &HashMap<String, Value>,
    ) -> Result<Vec<Value>, ExecutionError> {
        let mut collections = self.collections.lock().unwrap();

        // Get the collection (return empty array if not found)
        let docs = match collections.get_mut(collection) {
            Some(d) => d,
            None => return Ok(vec![]),
        };

        let mut deleted_docs = vec![];
        let mut i = 0;

        // Remove matching documents and collect them
        while i < docs.len() {
            if Self::matches_filter(&docs[i], filter) {
                let deleted = docs.remove(i);
                deleted_docs.push(deleted);
                // Don't increment i, as we removed an element
            } else {
                i += 1;
            }
        }

        Ok(deleted_docs)
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
