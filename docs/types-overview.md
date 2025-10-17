# Deck Type System Overview

This document describes the Rust type system for the deck declarative web server DSL.

## Module Structure

```
src/
├── lib.rs              # Library root, re-exports public API
├── config/             # Configuration types
│   ├── mod.rs          # Module exports
│   ├── root.rs         # DeckConfig (top-level)
│   ├── route.rs        # Route, Response, HttpMethod
│   ├── middleware.rs   # Middleware definitions
│   ├── database.rs     # Database schema types
│   └── template.rs     # Template configuration
├── operators/          # Operator types
│   ├── mod.rs          # Operator enum, OperatorValue
│   ├── data.rs         # GetOp ($get)
│   ├── conditional.rs  # IfOp, SwitchOp ($if, $switch)
│   ├── collection.rs   # MapOp, FilterOp, ReduceOp ($map, $filter, $reduce)
│   ├── database.rs     # DbQueryOp, DbInsertOp, etc. ($dbQuery, $dbInsert, ...)
│   └── utility.rs      # MergeOp, ExistsOp, etc. ($merge, $exists, ...)
└── pipeline/           # Pipeline execution types
    ├── mod.rs          # Module exports
    └── step.rs         # PipelineStep
```

## Key Types

### `DeckConfig`
Top-level configuration structure:
- `database: Option<DatabaseConfig>` - Database schemas
- `templates: Option<TemplateConfig>` - Template configuration
- `routes: Vec<Route>` - Route definitions
- `middleware: HashMap<String, Middleware>` - Reusable middleware
- `schemas: HashMap<String, Value>` - Reusable validation schemas
- `error_handlers: Option<Value>` - Error handlers (TBD)

### `Route`
HTTP route definition:
- `path: String` - URL path pattern (e.g., "/api/posts/:id")
- `method: HttpMethod` - GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
- `middleware: Vec<String>` - Middleware to apply
- `pipeline: Vec<PipelineStep>` - Pipeline steps to execute
- `response: Response` - Response definition

### `PipelineStep`
A single step in a pipeline:
- `name: Option<String>` - Variable name to store result
- `value: OperatorValue` - Operator expression to execute

### `OperatorValue`
Can be either:
- `Operator(Box<Operator>)` - An operator expression
- `Literal(serde_json::Value)` - A literal value (string, number, bool, null, object, array)

This enables recursive nesting of operators.

### `Operator` Enum
All operator types with `$` prefix:

**Data Access:**
- `Get(GetOp)` - `$get` - Extract value from context

**Conditionals:**
- `If(IfOp)` - `$if` - Conditional branching
- `Switch(SwitchOp)` - `$switch` - Multi-way branching

**Collections:**
- `Map(MapOp)` - `$map` - Transform each item
- `Filter(FilterOp)` - `$filter` - Filter items
- `Reduce(ReduceOp)` - `$reduce` - Aggregate/fold

**Database:**
- `DbQuery(DbQueryOp)` - `$dbQuery` - Query documents
- `DbInsert(DbInsertOp)` - `$dbInsert` - Insert document
- `DbUpdate(DbUpdateOp)` - `$dbUpdate` - Update documents
- `DbDelete(DbDeleteOp)` - `$dbDelete` - Delete documents

**Utility:**
- `Merge(MergeOp)` - `$merge` - Combine objects
- `Exists(ExistsOp)` - `$exists` - Check if value exists
- `RenderString(RenderStringOp)` - `$renderString` - Template string
- `Return(ReturnOp)` - `$return` - Early return
- `Validate(ValidateOp)` - `$validate` - JSON Schema validation
- `Now(NowOp)` - `$now` - Current timestamp

**Comparison:** `$eq`, `$ne`, `$gt`, `$gte`, `$lt`, `$lte`

**Logical:** `$and`, `$or`, `$not`

**Math:** `$add`, `$subtract`, `$multiply`, `$divide`

## Database Schema Types

### `DatabaseSchema`
- `fields: HashMap<String, FieldDefinition>` - Field definitions
- `indexes: Vec<IndexDefinition>` - Index definitions

### `FieldDefinition`
- `field_type: FieldType` - string, number, boolean, datetime, array, object, json
- `required: bool` - Whether field is required
- `primary: bool` - Whether this is the primary key
- `unique: bool` - Whether values must be unique
- `default: Option<Value>` - Default value
- `enum: Option<Vec<Value>>` - Allowed values
- `items: Option<Box<FieldDefinition>>` - For arrays, element type

## Serialization/Deserialization

All types use serde for JSON serialization/deserialization:
- `#[serde(rename_all = "camelCase")]` for config types
- `#[serde(rename = "$operatorName")]` for operators
- `#[serde(untagged)]` for `OperatorValue` to support both operators and literals
- External tagging (default) for `Operator` enum

## Examples

See the `examples/` directory:
- `simple_config.json` - Basic route with $dbQuery and $if operators
- `complete_config.json` - Full example with database schemas, middleware, and validation
- `parse_config.rs` - Example program to parse and display configurations

Run: `cargo run --example parse_config [config_file]`

## Next Steps

Future additions to the type system:
1. **Pipeline execution** - Runtime executor for operators
2. **Context management** - Variable storage and JSON path resolution
3. **Database integration** - Actual database query execution
4. **Template rendering** - HTML template system
5. **Error handling** - Error boundaries and propagation
6. **CLI** - Command-line interface for running servers
