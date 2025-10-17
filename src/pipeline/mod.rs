/// Pipeline execution types
///
/// This module contains types for representing and executing
/// pipelines of operations.

mod context;
mod error;
mod step;

pub use context::Context;
pub use error::ExecutionError;
pub use step::PipelineStep;
