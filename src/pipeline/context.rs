use serde_json::Value;
use std::collections::HashMap;

/// Execution context that stores variables and their values
///
/// The context is immutable - methods that modify it return a new Context.
/// This makes it easier to reason about state and enables time-travel debugging.
#[derive(Debug, Clone, Default)]
pub struct Context {
    /// Variable storage
    variables: HashMap<String, Value>,
}

impl Context {
    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Create a new context with a variable set
    ///
    /// # Example
    /// ```
    /// use deck::pipeline::Context;
    /// use serde_json::json;
    ///
    /// let ctx = Context::new()
    ///     .with_var("user", json!({"id": "123", "email": "user@example.com"}))
    ///     .with_var("count", json!(42));
    /// ```
    pub fn with_var(mut self, name: impl Into<String>, value: Value) -> Self {
        self.variables.insert(name.into(), value);
        self
    }

    /// Set a variable in the context (mutable version)
    pub fn set_var(&mut self, name: impl Into<String>, value: Value) {
        self.variables.insert(name.into(), value);
    }

    /// Get a variable by name (top-level only)
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    /// Get a value using a JSON path (e.g., "user.email" or "params.id")
    ///
    /// Supports:
    /// - Simple paths: "user"
    /// - Nested object paths: "user.email"
    /// - Array indices: "items.0"
    /// - Deep nesting: "response.data.items.0.name"
    ///
    /// # Example
    /// ```
    /// use deck::pipeline::Context;
    /// use serde_json::json;
    ///
    /// let ctx = Context::new()
    ///     .with_var("user", json!({"id": "123", "email": "user@example.com"}));
    ///
    /// assert_eq!(
    ///     ctx.get_path("user.email").unwrap(),
    ///     &json!("user@example.com")
    /// );
    /// ```
    pub fn get_path(&self, path: &str) -> Option<&Value> {
        let parts: Vec<&str> = path.split('.').collect();

        if parts.is_empty() {
            return None;
        }

        // Start with the root variable
        let mut current = self.variables.get(parts[0])?;

        // Traverse the path
        for part in &parts[1..] {
            current = match current {
                Value::Object(map) => map.get(*part)?,
                Value::Array(arr) => {
                    // Try to parse as array index
                    let index: usize = part.parse().ok()?;
                    arr.get(index)?
                }
                _ => return None,
            };
        }

        Some(current)
    }

    /// Get all variables as a reference to the internal HashMap
    pub fn variables(&self) -> &HashMap<String, Value> {
        &self.variables
    }

    /// Check if a variable exists
    pub fn has(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// Check if a path exists
    pub fn has_path(&self, path: &str) -> bool {
        self.get_path(path).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_context() {
        let ctx = Context::new();
        assert!(ctx.variables.is_empty());
    }

    #[test]
    fn test_with_var() {
        let ctx = Context::new()
            .with_var("name", json!("Alice"))
            .with_var("age", json!(30));

        assert_eq!(ctx.get("name"), Some(&json!("Alice")));
        assert_eq!(ctx.get("age"), Some(&json!(30)));
    }

    #[test]
    fn test_set_var() {
        let mut ctx = Context::new();
        ctx.set_var("name", json!("Bob"));

        assert_eq!(ctx.get("name"), Some(&json!("Bob")));
    }

    #[test]
    fn test_get_simple() {
        let ctx = Context::new()
            .with_var("user", json!({"id": "123"}));

        assert_eq!(ctx.get("user"), Some(&json!({"id": "123"})));
        assert_eq!(ctx.get("missing"), None);
    }

    #[test]
    fn test_get_path_nested_object() {
        let ctx = Context::new()
            .with_var("user", json!({
                "id": "123",
                "email": "alice@example.com",
                "profile": {
                    "name": "Alice",
                    "age": 30
                }
            }));

        assert_eq!(ctx.get_path("user.id"), Some(&json!("123")));
        assert_eq!(ctx.get_path("user.email"), Some(&json!("alice@example.com")));
        assert_eq!(ctx.get_path("user.profile.name"), Some(&json!("Alice")));
        assert_eq!(ctx.get_path("user.profile.age"), Some(&json!(30)));
        assert_eq!(ctx.get_path("user.missing"), None);
        assert_eq!(ctx.get_path("missing.path"), None);
    }

    #[test]
    fn test_get_path_array_index() {
        let ctx = Context::new()
            .with_var("items", json!([
                {"name": "Item 1"},
                {"name": "Item 2"},
                {"name": "Item 3"}
            ]));

        assert_eq!(ctx.get_path("items.0.name"), Some(&json!("Item 1")));
        assert_eq!(ctx.get_path("items.1.name"), Some(&json!("Item 2")));
        assert_eq!(ctx.get_path("items.2.name"), Some(&json!("Item 3")));
        assert_eq!(ctx.get_path("items.99"), None);
    }

    #[test]
    fn test_get_path_invalid_on_non_object() {
        let ctx = Context::new()
            .with_var("count", json!(42));

        assert_eq!(ctx.get_path("count"), Some(&json!(42)));
        assert_eq!(ctx.get_path("count.something"), None);
    }

    #[test]
    fn test_has() {
        let ctx = Context::new()
            .with_var("user", json!({"id": "123"}));

        assert!(ctx.has("user"));
        assert!(!ctx.has("missing"));
    }

    #[test]
    fn test_has_path() {
        let ctx = Context::new()
            .with_var("user", json!({
                "profile": {
                    "name": "Alice"
                }
            }));

        assert!(ctx.has_path("user"));
        assert!(ctx.has_path("user.profile"));
        assert!(ctx.has_path("user.profile.name"));
        assert!(!ctx.has_path("user.missing"));
        assert!(!ctx.has_path("missing"));
    }
}
