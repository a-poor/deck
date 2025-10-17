/// deck - A declarative web server framework
///
/// This library provides types and runtime for building web servers
/// through declarative JSON configuration files instead of imperative code.

pub mod config;
pub mod executor;
pub mod operators;
pub mod pipeline;

// Re-export commonly used types
pub use config::{DeckConfig, Route};
pub use operators::{Operator, OperatorValue};
pub use pipeline::{Context, ExecutionError, PipelineStep};
