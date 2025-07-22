# Development Guide

This document contains development-specific information for contributors and maintainers of the mocks project.

## Developer Mode

To help with debugging, you can enable a special feature that saves mock data to a separate file.

To do this, simply set the environment variable called `MOCKS_DEBUG_OVERWRITTEN_FILE`.

```shell
MOCKS_DEBUG_OVERWRITTEN_FILE=storage.debug.json cargo run -- run storage.json
```

We recommend specifying the filename as `*.debug.json`. For more details, please check [.gitignore](.gitignore) file.

## Check MSRV

```shell
cargo install cargo-msrv --locked
cargo msrv find
```

For more details: [cargo-msrv](https://github.com/foresterre/cargo-msrv)

## Check Code Coverage

```shell
cargo llvm-cov
```

## Build for Homebrew

```shell
cargo build --release
cd target/release
tar -czf mocks-X.X.X-x86_64-apple-darwin.tar.gz mocks
shasum -a 256 mocks-X.X.X-x86_64-apple-darwin.tar.gz
```

## Testing

```shell
cargo test                    # Run all tests
cargo test storage::tests    # Run specific module tests
cargo fmt -- --check         # Check formatting
cargo clippy -- -D warnings  # Lint with strict warnings
```

## End-to-End Testing

```shell
cd runn-e2e
runn run runbooks/test.yml --verbose
```