# mocks

[![Crates.io](https://img.shields.io/crates/v/mocks.svg)](https://crates.io/crates/mocks)
[![msrv 1.80.1](https://img.shields.io/badge/msrv-1.80.1-dea584.svg?logo=rust)](https://github.com/rust-lang/rust/releases/tag/1.80.1)
[![codecov](https://codecov.io/gh/mocks-rs/mocks/branch/main/graph/badge.svg?token=1WZ0YCZK9J)](https://codecov.io/gh/mocks-rs/mocks)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/mocks-rs/mocks)
[![License](https://img.shields.io/github/license/mocks-rs/mocks)](LICENSE)

Mock REST APIs from JSON with zero coding within seconds.

**[Complete Documentation](https://mocks-rs.github.io/mocks)** - For detailed usage, advanced features, and examples.

## Install

### Homebrew

If you're a macOS Homebrew user, then you can install `mocks` from [homebrew-tap](https://github.com/mocks-rs/homebrew-tap).

```shell
brew install mocks-rs/tap/mocks
```

### Cargo

If you're a Rust programmer, `mocks` can be installed with `cargo`.

```shell
cargo install mocks
```

### npm

If you're a Node.js user, you can install `mocks` globally with `npm`.

```shell
npm install -g mocks
```

Alternatively, you can run `mocks` without installing using `npx`.

```shell
npx @mocks-rs/mocks init storage.json
npx @mocks-rs/mocks run storage.json
```

## Usage

### Initialize a storage file

Create a JSON file using the `init` command:

```shell
mocks init storage.json
```

This creates a `storage.json` file with sample data. Use the `--empty` option to create an empty structure:

```shell
mocks init --empty storage.json
```

### Run a REST API server

Start the mock server using your JSON file:

```shell
mocks run storage.json
```

Example JSON structure:

```json
{
  "posts": [
    { "id": "01J7BAKH37HPG116ZRRFKHBDGB", "title": "first post", "views": 100 },
    { "id": "01J7BAKH37GE8B688PT4RC7TP4", "title": "second post", "views": 10 }
  ],
  "profile": { "id": "01J7BAQE1GMD78FN3J0FJCNS8T", "name": "mocks" }
}
```

This automatically creates REST endpoints:

```shell
curl http://localhost:3000/posts
curl http://localhost:3000/posts/01J7BAKH37HPG116ZRRFKHBDGB
curl http://localhost:3000/profile
```

## Documentation

For detailed information about available routes, query parameters, advanced configuration, and more features, visit the **[complete documentation](https://mocks-rs.github.io/mocks)**.

For development information, see [DEVELOPMENT.md](DEVELOPMENT.md).

## LICENSE

This project is licensed under the [MIT license](LICENSE).
