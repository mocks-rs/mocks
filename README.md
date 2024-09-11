## mocks

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
    { "id": "01J7BAKH38CKY69XE3CAP5TE1P", "text": "a comment", "post_id": "01J7BAKH37HPG116ZRRFKHBDGB" },
    { "id": "01J7BAKH38E0S2GWTDF94EWCGR", "text": "another comment", "post_id": "01J7BAKH37HPG116ZRRFKHBDGB" }
  ],
  "profile": { "id": "01J7BAQE1GMD78FN3J0FJCNS8T", "name": "mocks" }
}
```

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
GET    /posts
GET    /posts/:id
```

```
GET    /comments
GET    /comments/:id
```

```
GET    /profile
```

### Options

Run `mocks --help` for a list of options.

## LICENSE

This project is licensed under the [MIT license](LICENSE).
