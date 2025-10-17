# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**deck** is a declarative web server framework written in Rust. Instead of writing imperative code, developers define routes, handlers, and data transformations in a JSON configuration file. The server executes these configurations through a pipeline-based execution model with composable operators.

## Core Concepts

### Declarative Configuration Model
The project is built around JSON configuration files that define:
- **Routes**: HTTP endpoints with path, method, middleware, and response definitions
- **Pipeline**: Sequential execution stages where data flows through operators
- **Operators**: Small, composable `$` prefixed operations (e.g., `$get`, `$if`, `$dbQuery`, `$map`)
- **Context**: Named variables that accumulate throughout pipeline execution, accessed via JSON paths

### Operator System
All operators use a `$` prefix. Key operator categories:
- **Data Access**: `$get` - extract values from context
- **Conditionals**: `$if`, `$switch` - branching logic
- **Collections**: `$map`, `$filter`, `$reduce` - transform/query collections
- **Database**: `$dbQuery`, `$dbInsert`, `$dbUpdate`, `$dbDelete` - data persistence
- **Utilities**: `$merge`, `$exists`, `$renderString`, `$return` - helper operations

Operators nest freely for composition, enabling complex logic without imperative code.

### Context & Variable Model
The pipeline maintains a context object accessed via JSON paths:
- `params.id` - Path parameters
- `query.page` - Query parameters
- `headers.authorization` - Request headers
- `body` - Request body
- Custom variables added by middleware or pipeline steps

## Development Commands

### Building
```bash
cargo build
```

### Running
```bash
cargo run
```

### Testing
```bash
cargo test
```

### Running with release optimizations
```bash
cargo run --release
```

## Architecture

### Technology Stack
- **Language**: Rust (edition 2024)
- **Web Framework**: Axum 0.8.6
- **CLI**: clap 4.5+ with derive features
- **Serialization**: serde + serde_json
- **Async Runtime**: Tokio with full features

### Project Structure
Currently in early development stage:
- `src/main.rs` - Entry point (placeholder)
- `design-docs/01.initial.md` - Complete design specification
- Configuration will be JSON-based with top-level sections:
  - `database` - schemas for collections/tables
  - `templates` - HTML template configuration (future)
  - `routes` - route definitions with pipelines
  - `middleware` - reusable pipeline fragments
  - `schemas` - reusable validation schemas

### Database Model
Treats storage as a document/key-value store where records are JSON objects. Database schemas are optional but enable:
- Validation before inserts/updates
- Documentation and type information
- Future: auto-generated CRUD endpoints

### Middleware System
Middleware are reusable pipeline fragments that can:
1. Add variables to context (e.g., authenticated user)
2. Short-circuit execution with `$return`

Middleware execute before route pipelines and implicitly continue unless `$return` is called.

## Design Principles

1. **Declarative over imperative**: Express *what* should happen, not *how*
2. **Composition over complexity**: Small operators that nest, not monolithic constructs
3. **Pipeline-based execution**: Data flows through sequential stages
4. **Context-aware**: Named variables accessible throughout via JSON paths
5. **Fail fast**: Validation and error handling should be explicit and early

## Current Implementation Status

The project is in the initial implementation phase. The core pipeline executor and basic operators (`$get`, `$if`, `$dbQuery`, `$dbInsert`) are the immediate priorities. See design-docs/01.initial.md for the complete specification and outstanding design decisions.

## Outstanding Design Questions

Key decisions still being evaluated (see design-docs/01.initial.md):
- Error handling strategy (error boundaries concept)
- HTML template rendering system
- Middleware execution model (`$next` explicit vs implicit)
- Advanced operator additions (`$join`, `$groupBy`, `$parallel`, etc.)
- Configuration format (JSON vs JSON5/YAML/TOML)
- Do your best to follow TDD where possible.
- Do not alter failing tests in order to get them to pass. If the test implementation is correct you **must** correct the code not cheat by updating the test.
- Keep track of overall progress in the @TODO.md file.