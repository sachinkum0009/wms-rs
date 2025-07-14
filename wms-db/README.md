# WMS Database Module

This module provides PostgreSQL database connectivity and management for the WMS (Warehouse Management System) project.

## Features

- 🔌 **Async PostgreSQL Connection**: Built with `sqlx` for high-performance async database operations
- 🏊 **Connection Pooling**: Efficient connection pool management with configurable limits
- 🔧 **Environment Configuration**: Load database settings from environment variables
- ✅ **Health Checks**: Built-in database health monitoring
- 🔄 **Migrations**: Database schema migration support with `sqlx migrate`
- 📊 **Logging**: Comprehensive logging with `tracing`
- 🧪 **Testing**: Integration tests for database functionality

## Quick Start

### 1. Setup PostgreSQL

Start a PostgreSQL instance using Docker:

```bash
# Start PostgreSQL with Docker
docker run --name wms-postgres \
  -e POSTGRES_DB=wms_dev \
  -e POSTGRES_USER=wms_user \
  -e POSTGRES_PASSWORD=wms_password \
  -p 5432:5432 \
  -d postgres:15

# Or use Docker Compose (create docker-compose.yml):
# version: '3.8'
# services:
#   postgres:
#     image: postgres:15
#     environment:
#       POSTGRES_DB: wms_dev
#       POSTGRES_USER: wms_user
#       POSTGRES_PASSWORD: wms_password
#     ports:
#       - "5432:5432"
#     volumes:
#       - postgres_data:/var/lib/postgresql/data
# volumes:
#   postgres_data:
```

### 2. Configure Environment

Copy the example environment file and update it:

```bash
cp .env.example .env
# Edit .env with your actual database credentials
```

### 3. Run Migrations

```bash
# From the wms-db directory
sqlx migrate run --database-url "postgresql://wms_user:wms_password@localhost:5432/wms_dev"
```

### 4. Test the Connection

```bash
# Run unit tests
cargo test

# Run integration tests (requires running PostgreSQL)
cargo test -- --ignored
```

## Usage

### Basic Connection

```rust
use wms_db::{Database, init_logging};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    init_logging();
    
    // Connect to database using environment variables
    let db = Database::from_env().await?;
    
    // Run health check
    db.health_check().await?;
    
    // Use the connection pool
    let pool = db.pool();
    let row = sqlx::query!("SELECT 1 as test")
        .fetch_one(pool)
        .await?;
    
    println!("Database test result: {}", row.test.unwrap());
    
    // Close connection gracefully
    db.close().await;
    
    Ok(())
}
```

### Custom Configuration

```rust
use wms_db::{Database, DatabaseConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = DatabaseConfig {
        database_url: "postgresql://user:pass@localhost:5432/mydb".to_string(),
        max_connections: 20,
        min_connections: 5,
        connection_timeout: Duration::from_secs(60),
        idle_timeout: Duration::from_secs(300),
    };
    
    let db = Database::new(config).await?;
    db.health_check().await?;
    
    Ok(())
}
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection URL | **Required** |
| `DB_MAX_CONNECTIONS` | Maximum connections in pool | `10` |
| `DB_MIN_CONNECTIONS` | Minimum connections in pool | `1` |
| `DB_CONNECTION_TIMEOUT_SECS` | Connection timeout in seconds | `30` |
| `DB_IDLE_TIMEOUT_SECS` | Idle connection timeout in seconds | `600` |
| `RUST_LOG` | Logging configuration | `wms_db=info,sqlx=warn` |

## Migrations

Migrations are located in the `migrations/` directory and use sqlx's migration system.

### Creating a New Migration

```bash
# Install sqlx-cli if not already installed
cargo install sqlx-cli

# Create a new migration
sqlx migrate add create_users_table

# Edit the generated migration file in migrations/
# Then run migrations
sqlx migrate run
```

### Migration Commands

```bash
# Run all pending migrations
sqlx migrate run

# Revert the last migration
sqlx migrate revert

# Check migration status
sqlx migrate info
```

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

Integration tests require a running PostgreSQL instance:

```bash
# Set up test database
export DATABASE_URL="postgresql://wms_user:wms_password@localhost:5432/wms_test"

# Run integration tests
cargo test -- --ignored
```

## Logging

The module uses `tracing` for structured logging. Set the `RUST_LOG` environment variable to control log levels:

```bash
# Enable debug logging for wms-db
export RUST_LOG="wms_db=debug,sqlx=info"

# Or in your .env file
RUST_LOG=wms_db=debug,sqlx=info
```

## Security Considerations

- 🔐 Database credentials are masked in logs
- 🔗 Use connection pooling to prevent connection exhaustion
- ⚡ Configure appropriate timeouts for production use
- 🔄 Regularly rotate database credentials
- 📊 Monitor connection pool metrics in production

## Troubleshooting

### Common Issues

1. **Connection refused**: Ensure PostgreSQL is running and accessible
2. **Authentication failed**: Check credentials in `DATABASE_URL`
3. **Migration errors**: Ensure database exists and user has proper permissions
4. **Pool exhaustion**: Increase `DB_MAX_CONNECTIONS` or check for connection leaks

### Debug Commands

```bash
# Check if PostgreSQL is running
pg_isready -h localhost -p 5432

# Test connection with psql
psql "postgresql://wms_user:wms_password@localhost:5432/wms_dev"

# Check migration status
sqlx migrate info --database-url $DATABASE_URL
```

## Dependencies

- [`sqlx`](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [`tokio`](https://tokio.rs/) - Async runtime
- [`anyhow`](https://github.com/dtolnay/anyhow) - Error handling
- [`dotenv`](https://github.com/dotenv-rs/dotenv) - Environment loading
- [`tracing`](https://github.com/tokio-rs/tracing) - Structured logging