# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**mocks** is a Rust CLI tool that provides instant mock REST APIs with zero coding required. It dynamically generates RESTful endpoints from JSON files, supporting full CRUD operations.

## Common Development Commands

### Build and Run
```bash
cargo build --release
cargo run -- run storage.json
cargo run -- --help  # View CLI options
cargo run -- init --help  # View init command options
cargo run -- run --help  # View run command options
```

### Testing
```bash
cargo test                    # Run all tests
cargo test storage::tests    # Run specific module tests
cargo fmt -- --check         # Check formatting
cargo clippy -- -D warnings  # Lint with strict warnings
```

### Coverage and Quality
```bash
cargo llvm-cov              # Generate coverage report
cargo install cargo-msrv --locked
cargo msrv find             # Check MSRV compatibility
```

### End-to-End Testing
```bash
cd runn-e2e
runn run runbooks/test.yml --verbose
```

## Architecture Overview

### Core Components
- **CLI Entry** (`src/main.rs`): Argument parsing with clap, server initialization
- **Web Server** (`src/server.rs`): Axum-based HTTP server with dynamic routing
- **Storage Layer** (`src/storage.rs`): File-based JSON storage with atomic operations
- **HTTP Handlers** (`src/server/handler/`): Full CRUD endpoint implementations
- **State Management** (`src/server/state.rs`): Thread-safe shared application state

### Key Patterns
- **Dynamic Routing**: Routes auto-generated from JSON file structure (e.g., `api/v1/users` → `/api/v1/{resource}`)
- **Async Architecture**: Built on Tokio runtime with async/await throughout
- **Shared State**: `Arc<Mutex<AppState>>` for thread-safe data access
- **Resource Abstraction**: Uniform CRUD operations across all detected resources
- **Error Mapping**: Custom error types with proper HTTP status code mapping

### API Design
Each JSON resource automatically gets these endpoints:
- `GET /{resource}` - List all items
- `GET /{resource}/{id}` - Get specific item  
- `POST /{resource}` - Create new item
- `PUT /{resource}/{id}` - Replace entire item
- `PATCH /{resource}/{id}` - Partial update
- `DELETE /{resource}/{id}` - Delete item
- `GET /_hc` - Health check (returns 204)

### Storage Operations
Located in `src/storage/operation/`:
- `select_all.rs`, `select_one.rs` - Read operations
- `insert.rs` - Create operations  
- `update.rs`, `update_one.rs` - Update operations
- `replace.rs`, `replace_one.rs` - Replace operations
- `remove.rs` - Delete operations

## Development Guidelines

### Testing Strategy
- Unit tests in `#[cfg(test)]` modules within each source file
- Integration tests use `tempfile` for isolated test environments
- E2E tests use runn with YAML test specifications in `runn-e2e/`
- Test data files use `*.test.json` pattern

### Code Quality Requirements
- All code must pass `cargo clippy -- -D warnings`
- Format code with `cargo fmt`
- Maintain MSRV compatibility (currently 1.78.0)
- Add comprehensive error handling with proper HTTP status codes

### File Patterns
- Main storage file: `storage.json`
- Test data: `storage.test.json` 
- Debug files: `*.debug.json` (when `MOCKS_DEBUG_OVERWRITTEN_FILE` is set)

### CLI Usage Examples
```bash
mocks init storage.json                      # Initialize storage file
mocks init --empty storage.json             # Initialize empty storage file
mocks run storage.json                      # Basic usage
mocks run -H 127.0.0.1 -p 8080 storage.json  # Custom host/port
mocks run --no-overwrite storage.json       # Prevent file modifications
```

## Important Notes
- Resource names in JSON must be unique (validation enforced)
- Singleton objects vs arrays are handled differently in routing
- File operations are atomic to prevent corruption
- Debug mode available via environment variable
- Health check endpoint is always available at `/_hc`
