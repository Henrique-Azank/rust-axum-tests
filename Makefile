.PHONY: help build run test clean fmt clippy check docker-build docker-up docker-down docker-logs docker-rebuild

# Variables
BINARY_NAME=rust-axum-tests
DOCKER_IMAGE=rust-axum-tests
DOCKER_TAG=latest

# Base target to show help

help: ## Show this help message
	@echo "Usage: make [target]"
	@echo ""
	@echo "Available targets:"
	@awk 'BEGIN {FS = ":.*##"; printf "\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  %-20s %s\n", $$1, $$2 } /^##@/ { printf "\n%s\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

# Locally run commands (host)

build: ## Build the application binary
	@echo "Building $(BINARY_NAME)..."
	cargo build --release

run: ## Run the application locally
	@echo "Running $(BINARY_NAME)..."
	cargo run

dev: ## Run the application in development mode with hot reload
	@echo "Running in development mode..."
	cargo watch -x run

test: ## Run tests
	@echo "Running tests..."
	cargo test

test-verbose: ## Run tests with verbose output
	@echo "Running tests (verbose)..."
	cargo test -- --nocapture

clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -f $(BINARY_NAME)

# Code Formatting and Linting

fmt: ## Format code
	@echo "Formatting code..."
	cargo fmt

clippy: ## Run clippy linter
	@echo "Running clippy..."
	cargo clippy -- -D warnings

check: fmt clippy test ## Run all checks (format, clippy, test)
	@echo "All checks passed!"

# Docker-related commands

docker-build: ## Build Docker image
	@echo "Building Docker image..."
	docker-compose build

docker-up: ## Start containers in background
	@echo "Starting containers..."
	docker-compose up -d

docker-down: ## Stop containers
	@echo "Stopping containers..."
	docker-compose down

docker-down-volumes: ## Stop containers and remove volumes
	@echo "Stopping containers and removing volumes..."
	docker-compose down -v

docker-logs: ## View application logs
	@echo "Viewing logs..."
	docker-compose logs -f app

docker-logs-all: ## View all container logs
	@echo "Viewing all logs..."
	docker-compose logs -f

docker-rebuild: docker-down docker-build docker-up ## Rebuild and restart containers
	@echo "Containers rebuilt and restarted!"

docker-exec: ## Execute shell in app container
	docker-compose exec app /bin/sh

db-shell: ## Execute psql shell in database container
	docker-compose exec postgres psql -U postgres -d axumdb

# Database migration commands

install-tools: ## Install development tools
	@echo "Installing development tools..."
	cargo install cargo-watch sqlx-cli

migrate-create: ## Create a new migration (usage: make migrate-create NAME=migration_name)
	@echo "Creating migration: $(NAME)..."
	sqlx migrate add $(NAME)

migrate-run: ## Run pending migrations
	@echo "Running migrations..."
	sqlx migrate run

migrate-revert: ## Revert last migration
	@echo "Reverting last migration..."
	sqlx migrate revert
