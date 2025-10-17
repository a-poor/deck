# Deck Implementation TODO

This document tracks the implementation status of the deck declarative web server.

## Legend
- ✅ Implemented and tested
- 🚧 In progress
- ⏸️ Planned but not started
- ❓ Design decision needed

---

## Core Infrastructure

### Type System
- ✅ Configuration types (DeckConfig, Route, Response, etc.)
- ✅ Database schema types
- ✅ Template configuration types
- ✅ Middleware types
- ✅ All operator type definitions
- ✅ Pipeline step types

### Runtime
- ✅ Context - Variable storage with path resolution
- ✅ ExecutionError - Comprehensive error types
- ✅ Executor - Pipeline evaluation engine
- ✅ Dependency injection traits (DatabaseProvider, TimeProvider, RequestContext)
- ✅ Mock implementations for testing

---

## Operators

### Data Access
- ✅ `$get` - Simple dot notation path (e.g., `user.email`)
- ✅ `$jsonPath` - Full JSONPath support (wildcards, filters, recursion)

### Conditionals & Branching
- ✅ `$if` - Conditional branching with truthiness
- ⏸️ `$switch` - Multi-way branching (type defined, not implemented)

### Collection Operations
- ⏸️ `$map` - Transform each item in collection
- ⏸️ `$filter` - Filter items by condition
- ⏸️ `$reduce` - Aggregate/fold operation

### Database Operations
- ⏸️ `$dbQuery` - Query documents (type defined, needs implementation)
- ⏸️ `$dbInsert` - Insert document (type defined, needs implementation)
- ⏸️ `$dbUpdate` - Update documents (type defined, needs implementation)
- ⏸️ `$dbDelete` - Delete documents (type defined, needs implementation)

### Utility Operators
- ✅ `$merge` - Combine multiple objects
- ✅ `$exists` - Check if value is non-null
- ✅ `$now` - Get current timestamp
- ⏸️ `$renderString` - Template string rendering (e.g., `"Hello {{name}}"`)
- ⏸️ `$return` - Early return from pipeline
- ⏸️ `$validate` - JSON Schema validation

### Comparison Operators
- ✅ `$eq` - Equality
- ✅ `$ne` - Not equal
- ✅ `$gt` - Greater than
- ✅ `$gte` - Greater than or equal
- ✅ `$lt` - Less than
- ✅ `$lte` - Less than or equal

### Logical Operators
- ✅ `$and` - Logical AND
- ✅ `$or` - Logical OR
- ✅ `$not` - Logical NOT

### Math Operators
- ⏸️ `$add` - Addition
- ⏸️ `$subtract` - Subtraction
- ⏸️ `$multiply` - Multiplication
- ⏸️ `$divide` - Division

### Future Operators (From Design Doc)
- ❓ `$decodeJWT` - Decode JWT tokens
- ❓ `$renderTemplate` - Render HTML templates
- ❓ `$join` - SQL-like joins across collections
- ❓ `$groupBy` - Aggregation
- ❓ `$try` - Error handling within pipeline
- ❓ `$parallel` - Execute multiple queries concurrently
- ❓ `$cache` - Cache results

---

## Database Integration

- ⏸️ Actual database backend implementation
- ⏸️ Schema validation on insert/update
- ⏸️ Index support
- ⏸️ Query optimization
- ⏸️ Transaction support
- ❓ Database backend choice (in-memory, SQLite, MongoDB, etc.)

---

## HTTP Server

- ⏸️ Axum server setup
- ⏸️ Route registration from config
- ⏸️ Request parsing (params, query, headers, body)
- ⏸️ Response formatting
- ⏸️ Middleware execution
- ⏸️ Error handling and error boundaries

---

## CLI

- ⏸️ Config file loading
- ⏸️ Config validation
- ⏸️ Server start command
- ⏸️ Development mode with hot reload
- ⏸️ Config introspection/debugging tools

---

## Template System

- ❓ Template engine selection (Jinja, Handlebars, etc.)
- ⏸️ Template loading and caching
- ⏸️ Template rendering in pipeline
- ⏸️ Partial/include support
- ⏸️ Layout inheritance

---

## Validation

- ✅ JSON Schema types defined
- ⏸️ JSON Schema validation implementation
- ⏸️ Database schema validation
- ⏸️ Config validation at startup

---

## Testing

### Unit Tests
- ✅ Context path resolution (10 tests)
- ✅ ExecutionError types (5 tests)
- ✅ Executor basics (50 tests - includes comparison and logical operators)
- ⏸️ All operator implementations
- ⏸️ Database operations
- ⏸️ Middleware execution

### Integration Tests
- ⏸️ Full pipeline execution
- ⏸️ HTTP request/response cycle
- ⏸️ Database operations end-to-end
- ⏸️ Middleware chain execution

### Example Configs
- ✅ Simple config (basic $get and $if)
- ✅ Complete config (with database, middleware)
- ⏸️ Real-world examples (CRUD API, etc.)

---

## Documentation

- ✅ CLAUDE.md - Overview for AI assistance
- ✅ README.md - Project introduction
- ✅ design-docs/01.initial.md - Complete design specification
- ✅ docs/types-overview.md - Type system documentation
- ⏸️ User guide
- ⏸️ Operator reference documentation
- ⏸️ API reference (cargo doc)
- ⏸️ Tutorial/getting started guide

---

## Build & Tooling

- ✅ Cargo.toml with dependencies
- ✅ Examples (parse_config)
- ⏸️ CI/CD setup
- ⏸️ Benchmarks
- ⏸️ Release builds
- ⏸️ Docker support

---

## Next Immediate Steps (Recommended Order)

1. ✅ ~~**Comparison Operators** (`$eq`, `$gt`, etc.) - Needed for filters~~
2. ✅ ~~**Logical Operators** (`$and`, `$or`, `$not`) - Needed for complex conditions~~
3. **Collection Operators** (`$map`, `$filter`) - Core functionality
4. **Math Operators** (`$add`, `$subtract`, `$multiply`, `$divide`) - Basic arithmetic
5. **String Rendering** (`$renderString`) - Useful utility
6. **Switch Statement** (`$switch`) - Complete conditionals
7. **Database Operations** - Core feature
8. **Basic HTTP Server** - Get something running end-to-end
9. **Middleware Execution** - Complete request handling
10. **Remaining Operators** - Fill in the gaps

---

## Design Decisions Needed

1. **Template Engine**: Which library to use? Custom implementation?
2. **Database Backend**: In-memory for MVP? Which persistent backend(s)?
3. **Error Boundaries**: How should errors propagate through pipelines?
4. **Middleware `$next`**: Explicit or implicit continuation?
5. **Response vs Early Return**: Should all returns use the `response` field?
6. **Configuration Format**: Stick with JSON or support JSON5/YAML?
7. **Auto-CRUD Generation**: Should database schemas auto-generate routes?

---

## Test Coverage Goals

- [ ] 100% coverage for Context
- [ ] 100% coverage for all implemented operators
- [ ] 80%+ coverage for executor
- [ ] Integration tests for common patterns
- [ ] Property-based tests for operator composition

---

## Performance Considerations (Future)

- ⏸️ Pipeline compilation/optimization
- ⏸️ Context cloning optimization
- ⏸️ Database query batching
- ⏸️ Operator result caching
- ⏸️ Lazy evaluation where possible
