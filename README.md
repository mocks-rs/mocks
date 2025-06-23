# mocks

[![Crates.io](https://img.shields.io/crates/v/mocks.svg)](https://crates.io/crates/mocks)
[![msrv 1.78.0](https://img.shields.io/badge/msrv-1.78.0-dea584.svg?logo=rust)](https://github.com/rust-lang/rust/releases/tag/1.78.0)
[![codecov](https://codecov.io/gh/mocks-rs/mocks/branch/main/graph/badge.svg?token=1WZ0YCZK9J)](https://codecov.io/gh/mocks-rs/mocks)
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
  "friends": [],
}
```

```json
{
  "api/v1/posts": [
    { "id": "01J7BAKH37HPG116ZRRFKHBDGB", "title": "first post", "views": 100 },
    { "id": "01J7BAKH37GE8B688PT4RC7TP4", "title": "second post", "views": 10 }
  ],
  "api/v1/comments": [
    { "id": 1, "text": "a comment", "post_id": "01J7BAKH37HPG116ZRRFKHBDGB" },
    { "id": 2, "text": "another comment", "post_id": "01J7BAKH37HPG116ZRRFKHBDGB" }
  ],
  "api/v1/profile": { "id": "01J7BAQE1GMD78FN3J0FJCNS8T", "name": "mocks" },
  "api/v1/friends": [],
}
```

> [!WARNING]
> You cannot define duplicate resource (e.g., `api/v1/users` and `api/v2/users`) in the storage file. Each resource name must be unique.

Pass it to `mocks` CLI.

```shell
mocks storage.json
```

```shell
mocks -H 127.0.0.1 -p 3000 storage.json
```

If you want to access from the host OS (e.g., when running in a container), specify `0.0.0.0` as the host:

```shell
mocks -H 0.0.0.0 storage.json
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
GET     /posts?title.contains=first    # Query parameter search (contains match)
GET     /posts?views.exact=100         # Query parameter search (exact match)
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

#### Query Parameter Search

For array-based resources (like `posts`, `comments`, `friends`), you can use query parameters to filter results. The query parameter format requires specifying both the field name and match type:

**Format:** `field.matchtype=value`

**Available match types:**
- `exact` - Exact match (case-insensitive)
- `contains` - Contains substring (case-insensitive)
- `startswith` - Starts with substring (case-insensitive)
- `endswith` - Ends with substring (case-insensitive)

**Examples:**

```shell
# Search posts with titles containing "first"
curl "http://localhost:3000/posts?title.contains=first"

# Search posts with exact view count
curl "http://localhost:3000/posts?views.exact=100"

# Search posts starting with "Hello"
curl "http://localhost:3000/posts?title.startswith=hello"

# Search posts ending with "World"
curl "http://localhost:3000/posts?title.endswith=world"

# Multiple query parameters (AND logic)
curl "http://localhost:3000/posts?title.contains=post&views.exact=100"
```

**Restrictions:**
- Query parameters are only supported for array-based resources (not single objects like `profile`)
- Query parameters are not allowed on individual item endpoints (like `/posts/:id`)
- Match type is required - using just `field=value` will return an error
- Complex values (objects or arrays) cannot be searched

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
