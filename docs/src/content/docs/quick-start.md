---
title: Quick Start
description: Create your first mock API in 5 minutes
---

## 1. Create JSON File

First, create a JSON file that defines your data. Save it as `storage.json`:

```json
{
  "posts": [
    { "id": "1", "title": "First Post", "views": 100 },
    { "id": "2", "title": "Second Post", "views": 10 }
  ],
  "comments": [
    { "id": 1, "text": "Great post!", "post_id": "1" },
    { "id": 2, "text": "Thanks for sharing", "post_id": "1" }
  ],
  "profile": { "id": "1", "name": "mocks" },
  "friends": []
}
```

## 2. Start the Server

Launch the mock server using your JSON file:

```bash
mocks storage.json
```

By default, the server starts at `http://localhost:3000`.

### Custom Options

You can specify custom host and port:

```bash
mocks -H 127.0.0.1 -p 8080 storage.json
```

For Docker containers or external access:

```bash
mocks -H 0.0.0.0 storage.json
```

## 3. Use the API

Once the server is running, the following endpoints are available:

### Get Posts

```bash
# Get all posts
curl http://localhost:3000/posts

# Get specific post
curl http://localhost:3000/posts/1
```

### Create Post

```bash
curl -X POST http://localhost:3000/posts \
  -H "Content-Type: application/json" \
  -d '{"id": "3", "title": "New Post", "views": 0}'
```

### Update Post

```bash
# Complete update (PUT)
curl -X PUT http://localhost:3000/posts/1 \
  -H "Content-Type: application/json" \
  -d '{"id": "1", "title": "Updated Post", "views": 200}'

# Partial update (PATCH)
curl -X PATCH http://localhost:3000/posts/1 \
  -H "Content-Type: application/json" \
  -d '{"views": 150}'
```

### Delete Post

```bash
curl -X DELETE http://localhost:3000/posts/1
```

## 4. Search Functionality

For array-type resources, you can use query parameters to search:

```bash
# Search posts containing "Post" in title
curl "http://localhost:3000/posts?title.contains=Post"

# Search posts with exactly 100 views
curl "http://localhost:3000/posts?views.exact=100"

# Multiple search conditions
curl "http://localhost:3000/posts?title.contains=Post&views.exact=100"
```

## 5. Health Check

Check server status using the health check endpoint:

```bash
curl http://localhost:3000/_hc
```

## Important Concepts

- **Auto-generation**: Endpoints are automatically generated from JSON file structure
- **Persistence**: Changes made via API are saved to the original JSON file
- **Arrays vs Objects**: Array resources use `GET /posts`, single objects use `GET /profile`
- **ID Requirements**: Each item in array resources needs a unique ID

## Next Steps

Now that you understand the basics, explore the [API Reference](/api-reference/) for detailed functionality or check out [Examples](/examples/) for more complex usage scenarios.