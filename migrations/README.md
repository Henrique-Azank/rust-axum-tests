# Database Migrations

This directory contains SQL migration files managed by [SQLx](https://github.com/launchbadge/sqlx).

## Overview

Migrations are automatically run when the application starts (see `src/main.rs`). SQLx tracks applied migrations in the `_sqlx_migrations` table and uses checksums to detect modifications.

## Migration File Format

Migration files follow the naming convention:
```
<timestamp>_<description>.sql
```

Example: `20260117000001_initial_setup.sql`

## Creating a New Migration

### Using the Makefile (Recommended)

```bash
make migrate-create NAME=add_orders_table
```

### Using SQLx CLI Directly

```bash
sqlx migrate add add_orders_table
```

This creates a new timestamped file in the `migrations/` directory.

## Writing Migrations

### Best Practices

1. **Be Explicit**: Use `IF NOT EXISTS` for tables/indexes
2. **Add Constraints**: Foreign keys, checks, NOT NULL
3. **Create Indexes**: For columns used in WHERE/JOIN clauses
4. **Use Transactions**: Migrations run in a transaction by default
5. **Document Changes**: Add comments explaining complex logic

### Example Migration

```sql
-- Create orders table
CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    total_amount DECIMAL(10, 2) NOT NULL CHECK (total_amount >= 0),
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);

-- Add helpful comment
COMMENT ON TABLE orders IS 'Stores customer orders with references to users';
```

### Common Patterns

#### Adding a Column
```sql
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS phone VARCHAR(20);
```

#### Creating a Junction Table
```sql
CREATE TABLE IF NOT EXISTS user_roles (
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, role_id)
);
```

#### Adding an Enum Type
```sql
CREATE TYPE order_status AS ENUM ('pending', 'processing', 'completed', 'cancelled');

ALTER TABLE orders 
ADD COLUMN status order_status NOT NULL DEFAULT 'pending';
```

## Validating Migrations

**Important**: SQLx requires a live database connection to validate migrations. You cannot validate migrations offline.

### Option 1: Test with Local Database (Recommended)

```bash
# Start a fresh database
make docker-down-volumes
make docker-up

# Run migrations (validates SQL syntax and compatibility)
make migrate-run

# Check the logs for errors
make docker-logs
```

### Option 2: Validate During App Startup

```bash
# The app automatically runs migrations on startup
make run

# Watch for migration errors in console output
```

### Option 3: Database Shell Inspection

```bash
# After running migrations, inspect the schema
make db-shell

# List all tables
\dt

# Describe a specific table
\d orders

# View migration history
SELECT * FROM _sqlx_migrations ORDER BY version;

# Exit
\q
```

## Running Migrations Manually

### Run All Pending Migrations

```bash
# Using Makefile
make migrate-run

# Using SQLx CLI directly
sqlx migrate run
```

### Run Migrations in Docker

```bash
# Start the database
docker-compose up -d postgres

# Run migrations
docker-compose exec app ./rust-axum-tests

# Or rebuild everything
make docker-rebuild
```

### Revert Last Migration

```bash
# Using Makefile
make migrate-revert

# Using SQLx CLI
sqlx migrate revert
```

**Note**: Reverting requires a corresponding "down" migration file. SQLx doesn't support this by default - you need to create a new migration to undo changes.

## Troubleshooting

### Migration Already Applied

If you modify a migration that has already been applied:
```
Error: Migration checksum mismatch
```

**Solution**: Create a new migration to make the change instead of modifying existing ones.

### SQL Syntax Error

```
Error: syntax error at or near "..."
```

**Solution**: Test your SQL in `psql` first:
```bash
make db-shell
# Paste and test your SQL
```

### Foreign Key Constraint Violation

```
Error: foreign key constraint does not exist
```

**Solution**: Ensure referenced tables are created in earlier migrations or in the same migration file before the referencing table.

## Migration State

SQLx tracks migrations in the `_sqlx_migrations` table:

```sql
-- View applied migrations
SELECT version, description, installed_on, success 
FROM _sqlx_migrations 
ORDER BY version;
```

Each migration has:
- **version**: Timestamp from filename
- **description**: Migration name
- **checksum**: Hash of SQL content (detects modifications)
- **installed_on**: When it was applied
- **success**: Whether it completed successfully

## Additional Resources

- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
- [SQLx CLI Guide](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)