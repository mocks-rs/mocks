# @mocks-rs/mocks

Get a mock REST APIs with zero coding within seconds.

This is the npm distribution of the `mocks` CLI tool, originally written in Rust.

## Installation

### Global Installation
```bash
npm install -g @mocks-rs/mocks
```

### Using npx
```bash
npx @mocks-rs/mocks storage.json
```

## Usage

Create a `storage.json` file:

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
  "profile": { "id": "01J7BAQE1GMD78FN3J0FJCNS8T", "name": "mocks" }
}
```

Run the mock server:

```bash
mocks storage.json
```

Or with custom host and port:

```bash
mocks -H 127.0.0.1 -p 3000 storage.json
```

## Supported Platforms

- Linux x64
- macOS (Intel and Apple Silicon)
- Windows x64

## Features

- Zero configuration REST API server
- Full CRUD operations
- Query parameter filtering
- JSON file-based storage
- Health check endpoint at `/_hc`

## Routes

Based on your storage.json, you'll automatically get:

```
GET     /posts
GET     /posts?title.contains=first
GET     /posts/:id
POST    /posts
PUT     /posts/:id
PATCH   /posts/:id
DELETE  /posts/:id
```

## Original Project

This package wraps the original Rust CLI tool. For more information, visit:
https://github.com/mocks-rs/mocks

## License

MIT
