/// Configuration types for deck
///
/// This module contains types for parsing and representing
/// the declarative JSON configuration format.

mod database;
mod middleware;
mod route;
mod root;
mod template;

pub use database::{DatabaseConfig, DatabaseSchema, FieldDefinition, FieldType, IndexDefinition};
pub use middleware::Middleware;
pub use route::{HttpMethod, Response, Route};
pub use root::DeckConfig;
pub use template::TemplateConfig;
