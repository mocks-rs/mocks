# mocks

Mock REST APIs from JSON with zero coding within seconds.

[![npm version](https://img.shields.io/npm/v/@mocks-rs/mocks.svg)](https://www.npmjs.com/package/@mocks-rs/mocks)
[![License](https://img.shields.io/github/license/mocks-rs/mocks)](https://github.com/mocks-rs/mocks/blob/main/LICENSE)

**[Complete Documentation](https://mocks-rs.github.io/mocks)** - For detailed usage, advanced features, and examples.

## Install

```shell
npm install -g @mocks-rs/mocks
```

Or use without installing:

```shell
npx @mocks-rs/mocks --help
```

## Quick Start

### 1. Initialize a storage file

Create a JSON file using the `init` command:

```shell
npx @mocks-rs/mocks init storage.json
```

This creates a `storage.json` file with sample data. Use the `--empty` option to create an empty structure:

```shell
npx @mocks-rs/mocks init --empty storage.json
```

### 2. Run a REST API server

Start the mock server using your JSON file:

```shell
npx @mocks-rs/mocks run storage.json
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

### 3. Use your mock API

This automatically creates REST endpoints:

```shell
# List all posts
curl http://localhost:3000/posts

# Get a specific post
curl http://localhost:3000/posts/01J7BAKH37HPG116ZRRFKHBDGB

# Get profile (singleton resource)
curl http://localhost:3000/profile

# Create a new post
curl -X POST http://localhost:3000/posts \
  -H "Content-Type: application/json" \
  -d '{"title": "new post", "views": 0}'

# Health check
curl http://localhost:3000/_hc
```

## Available Endpoints

For each resource in your JSON file, mocks automatically creates:

- `GET /{resource}` - List all items
- `GET /{resource}/{id}` - Get specific item  
- `POST /{resource}` - Create new item
- `PUT /{resource}/{id}` - Replace entire item
- `PATCH /{resource}/{id}` - Partial update
- `DELETE /{resource}/{id}` - Delete item
- `GET /_hc` - Health check (returns 204)

## Documentation

For detailed information about advanced configuration, query parameters, filtering, and more features, visit the **[complete documentation](https://mocks-rs.github.io/mocks)**.

## Contributing

This project is open source. Visit the [GitHub repository](https://github.com/mocks-rs/mocks) to contribute or report issues.

## License

This project is licensed under the [MIT license](https://github.com/mocks-rs/mocks/blob/main/LICENSE).