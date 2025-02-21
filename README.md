# Blogrs

A lightweight blog API built with Rust, using:
- Axum for the web framework
- SQLite for the database

## Features

- REST API endpoints for blogs and authors
- Simple SQLite database with proper relationships
- Async request handling with Axum
- Clean architecture with separation of concerns
- Type-safe database queries with SQLx

## Project Structure

```
blog-api/
├── Cargo.toml              # Project dependencies and metadata
├── migrations/             # Database migration files
│   └── 0001_initial.up.sql
├── data/                   # Database storage
│   └── blog.db
└── src/                    # Source code
    ├── main.rs            # Application entry point
    ├── models/            # Data structures
    │   ├── mod.rs
    │   ├── author.rs
    │   └── blog.rs
    ├── controllers/       # Request handlers
    │   ├── mod.rs
    │   ├── author.rs
    │   └── blog.rs
    ├── routes/           # API route definitions
    │   ├── mod.rs
    │   ├── author.rs
    │   └── blog.rs
    └── db/               # Database connection management
        └── mod.rs
```

## API Endpoints

### Authors
- `POST /authors` - Create a new author
- `GET /authors` - List all authors

### Blogs
- `POST /blogs` - Create a new blog post
- `GET /blogs` - List all blog posts
- `GET /blogs/{id}` - Get a specific blog post
- `PUT /blogs/{id}` - Update a blog post
- `GET /blogs/{id}/author` - Get the author of a specific blog post

## Example Usage

```bash
# Create an author
curl -X POST http://localhost:3000/authors \
  -H "Content-Type: application/json" \
  -d '{"id": 1, "name": "John Doe", "email": "john@example.com"}'

# Create a blog post
curl -X POST http://localhost:3000/blogs \
  -H "Content-Type: application/json" \
  -d '{"title": "My First Post", "content": "Hello World!", "author_id": 1}'

# Get all blog posts
curl http://localhost:3000/blogs

# Get a blog's author
curl http://localhost:3000/blogs/1/author
```

## Setup Instructions

Please see [SETUP.md](SETUP.md) for detailed installation and setup instructions for both NixOS and non-NixOS systems.

## License

MIT License