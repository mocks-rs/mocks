---
title: Examples
description: Real-world usage examples of mocks in various development scenarios
---

## Blog API Example

This example demonstrates creating a mock API for a blog application.

### Data File (storage.json)

```json
{
  "posts": [
    {
      "id": "1",
      "title": "Getting Started with mocks",
      "content": "Using mocks can streamline your frontend development process.",
      "author": "developer",
      "published": true,
      "created_at": "2024-01-01T00:00:00Z",
      "tags": ["API", "development", "mocks"]
    },
    {
      "id": "2",
      "title": "REST API Design Patterns",
      "content": "Let's explore good REST API design practices.",
      "author": "developer",
      "published": false,
      "created_at": "2024-01-02T00:00:00Z",
      "tags": ["API", "design"]
    }
  ],
  "comments": [
    {
      "id": 1,
      "post_id": "1",
      "author": "user1",
      "content": "Very helpful, thanks!",
      "created_at": "2024-01-01T10:00:00Z"
    },
    {
      "id": 2,
      "post_id": "1",
      "author": "user2",
      "content": "I'll try implementing this.",
      "created_at": "2024-01-01T11:00:00Z"
    }
  ],
  "categories": [
    { "id": "1", "name": "Technology", "description": "Tech-related articles" },
    { "id": "2", "name": "Tutorials", "description": "Tutorial articles" }
  ],
  "user_profile": {
    "name": "John Doe",
    "bio": "Frontend Developer",
    "avatar": "https://example.com/avatar.jpg",
    "social_links": {
      "twitter": "@johndoe",
      "github": "johndoe"
    }
  }
}
```

### Usage Examples

```bash
# Get all posts
curl http://localhost:3000/posts

# Search for published posts only
curl "http://localhost:3000/posts?published.exact=true"

# Search for posts containing specific tags
curl "http://localhost:3000/posts?tags.contains=API"

# Get comments for a specific post
curl "http://localhost:3000/comments?post_id.exact=1"

# Create a new post
curl -X POST http://localhost:3000/posts \
  -H "Content-Type: application/json" \
  -d '{
    "id": "3",
    "title": "New Post",
    "content": "New content here",
    "author": "new_author",
    "published": true,
    "created_at": "2024-01-03T00:00:00Z",
    "tags": ["new"]
  }'

# Update user profile
curl -X PATCH http://localhost:3000/user_profile \
  -H "Content-Type: application/json" \
  -d '{"bio": "Senior Frontend Developer"}'
```

## E-commerce Product API

This example shows creating a mock API for an online store.

### Data File (products.json)

```json
{
  "products": [
    {
      "id": "p001",
      "name": "Wireless Headphones",
      "price": 99.99,
      "currency": "USD",
      "category": "electronics",
      "in_stock": true,
      "stock_quantity": 50,
      "description": "High-quality wireless headphones",
      "images": ["image1.jpg", "image2.jpg"],
      "rating": 4.5,
      "reviews_count": 120
    },
    {
      "id": "p002",
      "name": "Phone Case",
      "price": 24.99,
      "currency": "USD",
      "category": "accessories",
      "in_stock": true,
      "stock_quantity": 200,
      "description": "Durable phone case",
      "images": ["case1.jpg"],
      "rating": 4.2,
      "reviews_count": 85
    }
  ],
  "categories": [
    { "id": "electronics", "name": "Electronics", "parent": null },
    { "id": "accessories", "name": "Accessories", "parent": null }
  ],
  "cart": {
    "items": [
      { "product_id": "p001", "quantity": 1, "price": 99.99 }
    ],
    "total": 99.99,
    "currency": "USD"
  },
  "orders": []
}
```

### Usage Examples

```bash
# Product search
curl "http://localhost:3000/products?category.exact=electronics"
curl "http://localhost:3000/products?in_stock.exact=true"
curl "http://localhost:3000/products?name.contains=headphones"

# Add item to cart
curl -X PATCH http://localhost:3000/cart \
  -H "Content-Type: application/json" \
  -d '{
    "items": [
      { "product_id": "p001", "quantity": 1, "price": 99.99 },
      { "product_id": "p002", "quantity": 2, "price": 24.99 }
    ],
    "total": 149.97
  }'

# Create an order
curl -X POST http://localhost:3000/orders \
  -H "Content-Type: application/json" \
  -d '{
    "id": "order_001",
    "items": [
      { "product_id": "p001", "quantity": 1, "price": 99.99 }
    ],
    "total": 99.99,
    "status": "pending",
    "created_at": "2024-01-01T12:00:00Z"
  }'
```

## Team Management System

This example demonstrates creating a mock API for a project management tool.

### Data File (team.json)

```json
{
  "users": [
    {
      "id": "u001",
      "name": "John Smith",
      "email": "john@example.com",
      "role": "manager",
      "department": "development",
      "active": true
    },
    {
      "id": "u002",
      "name": "Jane Doe",
      "email": "jane@example.com",
      "role": "developer",
      "department": "development",
      "active": true
    }
  ],
  "projects": [
    {
      "id": "proj001",
      "name": "New Feature Development",
      "description": "Development of new application features",
      "status": "active",
      "start_date": "2024-01-01",
      "end_date": "2024-03-31",
      "manager_id": "u001"
    }
  ],
  "tasks": [
    {
      "id": "task001",
      "title": "API Design",
      "description": "Design API for the new feature",
      "project_id": "proj001",
      "assignee_id": "u002",
      "status": "in_progress",
      "priority": "high",
      "due_date": "2024-01-15"
    }
  ],
  "team_settings": {
    "notification_enabled": true,
    "default_project_duration": 90,
    "working_hours": {
      "start": "09:00",
      "end": "18:00"
    }
  }
}
```

### Usage Examples

```bash
# Get active users only
curl "http://localhost:3000/users?active.exact=true"

# Get users from specific department
curl "http://localhost:3000/users?department.exact=development"

# Get tasks for a project
curl "http://localhost:3000/tasks?project_id.exact=proj001"

# Get high priority tasks
curl "http://localhost:3000/tasks?priority.exact=high"

# Create a new task
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "id": "task002",
    "title": "Test Implementation",
    "description": "Implement unit tests",
    "project_id": "proj001",
    "assignee_id": "u002",
    "status": "todo",
    "priority": "medium",
    "due_date": "2024-01-20"
  }'
```

## Using mocks with Docker

This example shows how to use mocks in a Docker environment.

### Dockerfile

```dockerfile
FROM rust:1.78-slim as builder
WORKDIR /app
RUN cargo install mocks

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/mocks /usr/local/bin/mocks
WORKDIR /app
COPY storage.json .
EXPOSE 3000
CMD ["mocks", "-H", "0.0.0.0", "storage.json"]
```

### docker-compose.yml

```yaml
version: '3.8'
services:
  mocks:
    build: .
    ports:
      - "3000:3000"
    volumes:
      - ./storage.json:/app/storage.json
    environment:
      - MOCKS_DEBUG_OVERWRITTEN_FILE=storage.debug.json
```

### Usage

```bash
# Build and start the container
docker-compose up --build

# Test the API
curl http://localhost:3000/posts
```

## Frontend Integration

### React Integration

```javascript
// API client configuration
const API_BASE_URL = 'http://localhost:3000';

// Fetch posts
const fetchPosts = async () => {
  const response = await fetch(`${API_BASE_URL}/posts`);
  return await response.json();
};

// Create a post
const createPost = async (postData) => {
  const response = await fetch(`${API_BASE_URL}/posts`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(postData),
  });
  return await response.json();
};
```

### Vue.js Integration

```javascript
// Using with Vuex store
const store = new Vuex.Store({
  state: {
    posts: [],
    loading: false,
  },
  mutations: {
    SET_POSTS(state, posts) {
      state.posts = posts;
    },
    SET_LOADING(state, loading) {
      state.loading = loading;
    },
  },
  actions: {
    async fetchPosts({ commit }) {
      commit('SET_LOADING', true);
      try {
        const response = await fetch('http://localhost:3000/posts');
        const posts = await response.json();
        commit('SET_POSTS', posts);
      } finally {
        commit('SET_LOADING', false);
      }
    },
  },
});
```

## Advanced Usage Patterns

### Nested Resource Relationships

```json
{
  "api/v1/users": [
    {
      "id": "1",
      "name": "John Doe",
      "email": "john@example.com",
      "profile": {
        "bio": "Software Developer",
        "avatar": "avatar1.jpg"
      }
    }
  ],
  "api/v1/posts": [
    {
      "id": "1",
      "title": "Hello World",
      "author_id": "1",
      "content": "This is my first post",
      "metadata": {
        "views": 100,
        "likes": 5,
        "tags": ["intro", "hello"]
      }
    }
  ]
}
```

### Environment-specific Configurations

```bash
# Development environment
MOCKS_DEBUG_OVERWRITTEN_FILE=storage.dev.json mocks storage.json

# Testing environment
mocks --no-overwrite storage.test.json

# Production-like environment
mocks -H 0.0.0.0 -p 80 storage.prod.json
```

Use these examples as a starting point for your own mock API implementations. Modify the data structures and endpoints to match your specific project needs.