# Rust Axum Microservice Tests

A microservice testing repository built with [Axum](https://github.com/tokio-rs/axum) framework, PostgreSQL database with SQLx, and Docker Compose for easy deployment.

This repository is meant to be used as a starting point for testing out common web development tasks using Rust and Axum. The main branch is used as a starting ground for other tests like:

- Setup API documentation (e.g., with utoipa/swagger)
- Setup an Admin page for managing the service data
- Creating and applying migrations with SQLx and trying out different database patterns
- Setting up a local API cache (e.g., with Redis)
- Implementing JWT validation middleware
- And whatever you like...

## Features

- **Axum Framework** - Fast, ergonomic web framework built on Tokio
- **PostgreSQL Database** - Robust relational database with SQLx (compile-time checked queries)
- **Docker Compose** - Easy containerization and orchestration
- **RESTful API** - Complete CRUD operations for Users and Products
- **Health Check Endpoint** - Service monitoring
- **Database Migrations** - Schema managed by SQLx migrations
- **Structured Logging** - Built-in tracing support

## Project Structure

```
.
├── src/
│   ├── main.rs              # Application entry point and routing
│   ├── database/
│   │   └── mod.rs          # Database connection and pool
│   ├── models/
│   │   ├── mod.rs          # Model exports
│   │   ├── user.rs         # User model definition
│   │   └── product.rs      # Product model definition
│   └── handlers/
│       ├── mod.rs          # Handler exports and health check
│       ├── user_handler.rs # User CRUD handlers
│       └── product_handler.rs # Product CRUD handlers
├── migrations/              # Database migration files
│   └── 20260117000001_initial_setup.sql
├── Cargo.toml               # Rust dependencies
├── .env                     # Environment variables (not in git)
├── .env.example             # Environment variables template
├── Dockerfile               # Multi-stage Docker build
├── .dockerignore            # Docker build exclusions
├── docker-compose.yml       # Docker Compose configuration
├── Makefile                 # Build and development commands
├── LICENSE                  # MIT License
└── README.md                # This file
```

## Prerequisites

- Docker and Docker Compose
- Rust 1.75+ (for local development)
- PostgreSQL 16 (for local development without Docker)

## Quick Start

### Using Docker Compose (Recommended)

1. **Clone and navigate to the repository**

```bash
cd rust-axum-tests
```

2. **Set up environment variables**

```bash
cp .env.example .env
# Edit .env to customize your configuration
```

3. **Start the services**

```bash
docker-compose up --build
```

4. **Access the API**

- API: http://localhost:3000
- Health Check: http://localhost:3000/health

5. **Stop the services**

```bash
docker-compose down
```

To remove volumes as well:

```bash
docker-compose down -v
# or use make
make docker-down-volumes
```

### Local Development

1. **Set up environment variables**

```bash
cp .env.example .env
# Edit .env and set DB_HOST=localhost
```

2. **Install SQLx CLI (for migrations)**

```bash
cargo install sqlx-cli --no-default-features --features postgres
# or use make
make install-tools
```

3. **Start PostgreSQL (using Docker)**

```bash
docker run --name postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=axumdb \
  -p 5432:5432 -d postgres:16-alpine
```

4. **Run migrations**

```bash
sqlx migrate run
# or use make
make migrate-run
```

5. **Run the application**

```bash
cargo run
# or use make
make run
```

### Using Make Commands

The project includes a Makefile for common tasks:

```bash
make help          # Show all available commands
make build         # Build the binary
make run           # Run locally
make dev           # Run with hot reload (requires cargo-watch)
make test          # Run tests
make test-verbose  # Run tests with verbose output
make clean         # Clean up build artifacts
make fmt           # Format code
make clippy        # Run clippy linter
make check         # Run all checks (format, clippy, test)
make docker-build  # Build Docker image
make docker-up     # Start containers in background
make docker-down   # Stop containers
make docker-logs   # View application logs
make docker-rebuild # Rebuild and restart
make db-shell      # Access PostgreSQL shell
make migrate-create NAME=migration_name # Create new migration
make migrate-run   # Run pending migrations
make migrate-revert # Revert last migration
```

## Database Models

The base application hosts data from two very simple models.

### User

```rust
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}
```

### Product

```rust
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
}
```

## API Endpoints (Base Example)

The main branch has a very simple CRUD endpoints for a Users/Products sample application.

### Health Check

- `GET /health` - Check service status

### Users

- `GET /api/v1/users` - Get all users
- `GET /api/v1/users/:id` - Get user by ID
- `POST /api/v1/users` - Create new user
- `PUT /api/v1/users/:id` - Update user
- `DELETE /api/v1/users/:id` - Delete user

### Products

- `GET /api/v1/products` - Get all products
- `GET /api/v1/products/:id` - Get product by ID
- `POST /api/v1/products` - Create new product
- `PUT /api/v1/products/:id` - Update product
- `DELETE /api/v1/products/:id` - Delete product

## Example Requests (Base Example)

### Create a User

```bash
curl -X POST http://localhost:3000/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com"}'
```

### Get All Users

```bash
curl http://localhost:3000/api/v1/users
```

### Create a Product

```bash
curl -X POST http://localhost:3000/api/v1/products \
  -H "Content-Type: application/json" \
  -d '{"name":"Laptop","description":"High-performance laptop","price":999.99}'
```

### Get All Products

```bash
curl http://localhost:3000/api/v1/products
```

## Environment Variables

The application uses a `.env` file for configuration. Copy `.env.example` to `.env` and customize as needed:

| Variable | Default | Description |
|----------|---------|-------------|
| DB_HOST | postgres | PostgreSQL host (use localhost for local dev) |
| DB_USER | postgres | Database user |
| DB_PASSWORD | postgres | Database password |
| DB_NAME | axumdb | Database name |
| DB_PORT | 5432 | Database port |
| PORT | 3000 | Application port |
| APP_NAME | Rust Axum Microservice | Application name |
| APP_VERSION | 1.0.0 | Application version |
| RUST_LOG | rust_axum_tests=debug | Logging configuration |

**Note**: The `.env` file is not tracked in git. Always use `.env.example` as a template.

## Docker Services

### Application Service

- Built using multi-stage Docker build
- Runs on port 3000
- Automatically connects to PostgreSQL
- Runs migrations on startup

### PostgreSQL Service

- PostgreSQL 16 Alpine
- Persistent volume for data
- Health checks enabled
- Runs on port 5432

## Development Tips

1. **View logs**

```bash
docker-compose logs -f app
# or use make
make docker-logs
```

2. **Rebuild after code changes**

```bash
docker-compose up --build
# or use make
make docker-rebuild
```

3. **Access PostgreSQL**

```bash
docker-compose exec postgres psql -U postgres -d axumdb
# or use make
make db-shell
```

4. **Reset database**

```bash
docker-compose down -v
docker-compose up --build
```

5. **Run with hot reload (local development)**

```bash
cargo install cargo-watch
cargo watch -x run
# or use make
make dev
```

## Architecture

The project follows a clean, modular architecture:

- `src/main.rs` - Application entry point, initializes database and sets up routes
- `src/database/` - Database connection pool management and migrations
- `src/models/` - Data models (User, Product) with serde serialization
- `src/handlers/` - HTTP request handlers organized by resource

## Running Tests

```bash
# Run all tests
cargo test
# or use make
make test

# Run tests with output
cargo test -- --nocapture
# or use make
make test-verbose

# Run specific test
cargo test test_name
```

## Code Quality

```bash
# Format code
cargo fmt
# or use make
make fmt

# Run clippy linter
cargo clippy
# or use make
make clippy

# Run all checks
make check
```

## License

MIT License - see [LICENSE](LICENSE) file for details
