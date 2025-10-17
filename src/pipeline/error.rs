use std::fmt;

/// Errors that can occur during pipeline execution
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionError {
    /// A variable or path was not found in the context
    PathNotFound {
        path: String,
    },

    /// Type mismatch or invalid type operation
    TypeError {
        message: String,
        expected: Option<String>,
        actual: Option<String>,
    },

    /// Database operation failed
    DatabaseError {
        message: String,
    },

    /// Validation failed
    ValidationError {
        message: String,
        errors: Vec<String>,
    },

    /// Template rendering failed
    TemplateError {
        message: String,
    },

    /// Invalid operator usage or configuration
    InvalidOperator {
        operator: String,
        message: String,
    },

    /// Division by zero
    DivisionByZero,

    /// Array index out of bounds
    IndexOutOfBounds {
        index: usize,
        length: usize,
    },

    /// Pipeline was terminated early with $return
    /// This is not an error but a control flow mechanism
    EarlyReturn {
        status: u16,
        headers: std::collections::HashMap<String, serde_json::Value>,
        body: serde_json::Value,
    },

    /// Generic error for custom error messages
    Custom {
        message: String,
    },
}

impl ExecutionError {
    /// Create a PathNotFound error
    pub fn path_not_found(path: impl Into<String>) -> Self {
        Self::PathNotFound { path: path.into() }
    }

    /// Create a TypeError error
    pub fn type_error(message: impl Into<String>) -> Self {
        Self::TypeError {
            message: message.into(),
            expected: None,
            actual: None,
        }
    }

    /// Create a TypeError error with expected and actual types
    pub fn type_error_with_types(
        message: impl Into<String>,
        expected: impl Into<String>,
        actual: impl Into<String>,
    ) -> Self {
        Self::TypeError {
            message: message.into(),
            expected: Some(expected.into()),
            actual: Some(actual.into()),
        }
    }

    /// Create a DatabaseError
    pub fn database_error(message: impl Into<String>) -> Self {
        Self::DatabaseError {
            message: message.into(),
        }
    }

    /// Create a ValidationError
    pub fn validation_error(message: impl Into<String>, errors: Vec<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
            errors,
        }
    }

    /// Create a Custom error
    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom {
            message: message.into(),
        }
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::PathNotFound { path } => {
                write!(f, "Path not found: {}", path)
            }
            ExecutionError::TypeError {
                message,
                expected,
                actual,
            } => {
                write!(f, "Type error: {}", message)?;
                if let (Some(exp), Some(act)) = (expected, actual) {
                    write!(f, " (expected {}, got {})", exp, act)?;
                }
                Ok(())
            }
            ExecutionError::DatabaseError { message } => {
                write!(f, "Database error: {}", message)
            }
            ExecutionError::ValidationError { message, errors } => {
                write!(f, "Validation error: {}", message)?;
                if !errors.is_empty() {
                    write!(f, "\n  Errors:")?;
                    for error in errors {
                        write!(f, "\n    - {}", error)?;
                    }
                }
                Ok(())
            }
            ExecutionError::TemplateError { message } => {
                write!(f, "Template error: {}", message)
            }
            ExecutionError::InvalidOperator { operator, message } => {
                write!(f, "Invalid operator '{}': {}", operator, message)
            }
            ExecutionError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            ExecutionError::IndexOutOfBounds { index, length } => {
                write!(f, "Index out of bounds: {} (length: {})", index, length)
            }
            ExecutionError::EarlyReturn { status, .. } => {
                write!(f, "Early return with status {}", status)
            }
            ExecutionError::Custom { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl std::error::Error for ExecutionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_not_found() {
        let err = ExecutionError::path_not_found("user.email");
        assert_eq!(err.to_string(), "Path not found: user.email");
    }

    #[test]
    fn test_type_error() {
        let err = ExecutionError::type_error("Cannot add string to number");
        assert_eq!(err.to_string(), "Type error: Cannot add string to number");
    }

    #[test]
    fn test_type_error_with_types() {
        let err = ExecutionError::type_error_with_types(
            "Invalid operation",
            "number",
            "string",
        );
        assert_eq!(
            err.to_string(),
            "Type error: Invalid operation (expected number, got string)"
        );
    }

    #[test]
    fn test_validation_error() {
        let err = ExecutionError::validation_error(
            "Invalid input",
            vec!["Field 'name' is required".to_string()],
        );
        let display = err.to_string();
        assert!(display.contains("Validation error: Invalid input"));
        assert!(display.contains("Field 'name' is required"));
    }

    #[test]
    fn test_division_by_zero() {
        let err = ExecutionError::DivisionByZero;
        assert_eq!(err.to_string(), "Division by zero");
    }
}
