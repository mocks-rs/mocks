# mocks

[![Crates.io](https://img.shields.io/crates/v/mocks.svg)](https://crates.io/crates/mocks)
[![msrv 1.65.0](https://img.shields.io/badge/msrv-1.74.1-dea584.svg?logo=rust)](https://github.com/rust-lang/rust/releases/tag/1.74.1)
[![License](https://img.shields.io/github/license/mocks-rs/mocks)](LICENSE)

Get a mock REST APIs with zero coding within seconds.

## Install

If you're a macOS Homebrew user, then you can install `mocks` from [homebrew-tap](https://github.com/mocks-rs/homebrew-tap).

```shell
brew install mocks-rs/tap/mocks
```

If you're a Rust programmer, `mocks` can be installed with `cargo`.

```shell
cargo install mocks
```

## Usage

### Run a REST API server

Create a `storage.json`.

```json
{
  "posts": [
    { "id": "01J7BAKH37HPG116ZRRFKHBDGB", "title": "first post", "views": 100 },
    { "id": "01J7BAKH37GE8B688PT4RC7TP4", "title": "second post", "views": 10 }
  ],
  "comments": [
    { "id": 1, "text": "a comment", "post_id": "01J7BAKH37HPG116ZRRFKHBDGB" },
    { "id": 2, "text": "another comment", "post_id": "01J7BAKH37HPG116ZRRFKHBDGB" }
  ],
  "profile": { "id": "01J7BAQE1GMD78FN3J0FJCNS8T", "name": "mocks" },
  "friends": []
}
```

> [!WARNING]
> You cannot define duplicate resource (e.g., `api/v1/users` and `api/v2/users`) in the storage file. Each resource name must be unique.

Pass it to `mocks` CLI.

```shell
mocks storage.json
```

```shell
mocks -H 127.0.0.1 -p 8080 storage.json
```

Get a REST API with `curl`.

```shell
% curl http://localhost:3000/posts/01J7BAKH37HPG116ZRRFKHBDGB
{"id":"01J7BAKH37HPG116ZRRFKHBDGB","title":"first post","views":100}
```

### Routes

Based on the example [storage.json](storage.json), you'll get the following routes:

```
GET     /posts
GET     /posts/:id
POST    /posts
PUT     /posts/:id
PATCH   /posts/:id
DELETE  /posts/:id

# Same for comments and friends
```

```
GET     /profile
PUT     /profile
PATCH   /profile
```

```
GET     /_hc

# Health check endpoint returns a 204 response.
```

### Options

Run `mocks --help` for a list of options.

### Developer mode

To help with debugging, you can enable a special feature that saves mock data to a separate file. 

To do this, simply set the environment variable called `MOCKS_DEBUG_OVERWRITTEN_FILE`.

```shell
MOCKS_DEBUG_OVERWRITTEN_FILE=storage.debug.json cargo run -- storage.json
```

We recommend specifying the filename as `*.debug.json`. For more details, please check [.gitignore](.gitignore) file.

## LICENSE

This project is licensed under the [MIT license](LICENSE).
