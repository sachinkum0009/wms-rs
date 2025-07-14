# WMS CLI Tool

A command-line interface for interacting with the Warehouse Management System (WMS). This tool provides developers and operators with a simple way to interact with the system from the terminal.

## Features

- 🏥 **System Health Checks**: Monitor database connectivity and system status
- 📦 **Inventory Management**: List and manage inventory items (planned)
- 📋 **Order Management**: Create and manage orders (planned)
- 🎨 **Colored Output**: Beautiful, readable terminal output with emojis
- 🔍 **Rich Error Handling**: Detailed error messages and stack traces
- 📝 **Structured Logging**: Comprehensive logging with configurable levels

## Installation

### Prerequisites

- Rust 1.70+ installed
- PostgreSQL database accessible
- `.env` file configured (see Configuration section)

### Build from Source

```bash
# Clone the repository (if not already done)
git clone <repository-url>
cd <repository-name>

# Build the CLI tool
cargo build --release -p wms-cli

# The binary will be available at target/release/wms-cli
```

### Development Build

```bash
# Build in debug mode
cargo build -p wms-cli

# Run directly with cargo
cargo run -p wms-cli -- --help
```

## Configuration

### Environment Variables

Create a `.env` file in the project root with the following configuration:

```bash
# Copy the example file
cp .env.example .env

# Edit the .env file with your database settings
DATABASE_URL=postgresql://wms_user:wms_password@localhost:5432/wms_dev
```

Required environment variables:

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection URL | `postgresql://user:pass@localhost:5432/wms_dev` |

Optional environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `DB_MAX_CONNECTIONS` | Maximum database connections | `10` |
| `DB_MIN_CONNECTIONS` | Minimum database connections | `1` |
| `DB_CONNECTION_TIMEOUT_SECS` | Connection timeout (seconds) | `30` |
| `DB_IDLE_TIMEOUT_SECS` | Idle connection timeout (seconds) | `600` |
| `RUST_LOG` | Logging configuration | `wms_cli=info,wms_db=info` |

## Usage

### Basic Commands

```bash
# Show help
wms-cli --help

# Show version
wms-cli --version
```

### System Commands

```bash
# Check system health (database connectivity)
wms-cli system health
```

### Inventory Commands

```bash
# List all inventory items (placeholder)
wms-cli inventory list
```

### Order Commands

```bash
# Create a new order
wms-cli order create --item "Widget A" --quantity 10

# Create a new order (short flags)
wms-cli order create -i "Gadget X" -q 5
```

## Command Reference

### `wms-cli system health`

Checks the overall system health including:
- Database connectivity
- Database health check query
- Connection pool status

**Example output:**
```
2024-01-01T12:00:00.000Z  INFO wms_cli: WMS CLI initialized
2024-01-01T12:00:00.000Z  INFO wms_cli::commands::system: Running system health check...
2024-01-01T12:00:00.000Z  INFO wms_cli::commands::system: ✅ Database connection established
2024-01-01T12:00:00.000Z  INFO wms_db: Running database health check...
2024-01-01T12:00:00.000Z  INFO wms_db: Database health check passed
2024-01-01T12:00:00.000Z  INFO wms_cli::commands::system: ✅ Database health check passed
2024-01-01T12:00:00.000Z  INFO wms_cli::commands::system: 🎉 System health check completed successfully
```

### `wms-cli inventory list`

Lists all inventory items in the system.

> **Note**: This is currently a placeholder implementation that shows sample data. The actual implementation will query the database for real inventory items.

### `wms-cli order create`

Creates a new order with the specified item and quantity.

**Arguments:**
- `--item, -i`: Name of the item to order (required)
- `--quantity, -q`: Quantity to order (required, must be > 0)

> **Note**: This is currently a placeholder implementation that generates a sample order ID. The actual implementation will store orders in the database.

## Development

### Project Structure

```
wms-cli/
├── src/
│   ├── main.rs              # CLI entry point and argument parsing
│   └── commands/
│       ├── mod.rs           # Commands module
│       ├── system.rs        # System health commands
│       ├── inventory.rs     # Inventory management commands
│       └── order.rs         # Order management commands
├── Cargo.toml               # Dependencies and metadata
└── README.md                # This file
```

### Adding New Commands

1. Create a new module in `src/commands/`
2. Add the module to `src/commands/mod.rs`
3. Define the command structure in `src/main.rs`
4. Implement the command logic in your new module

### Running Tests

```bash
# Run all tests
cargo test -p wms-cli

# Run with output
cargo test -p wms-cli -- --nocapture
```

### Logging

The CLI uses structured logging with `tracing`. Log levels can be controlled via the `RUST_LOG` environment variable:

```bash
# Debug level for CLI and database
RUST_LOG=wms_cli=debug,wms_db=debug cargo run -p wms-cli -- system health

# Info level (default)
RUST_LOG=wms_cli=info,wms_db=info cargo run -p wms-cli -- system health

# Minimal logging
RUST_LOG=error cargo run -p wms-cli -- system health
```

## Troubleshooting

### Common Issues

1. **Database connection failed**
   - Ensure PostgreSQL is running
   - Check `DATABASE_URL` in your `.env` file
   - Verify database credentials and permissions

2. **Command not found**
   - Build the project: `cargo build -p wms-cli`
   - Use the full path: `./target/debug/wms-cli`
   - Or run with cargo: `cargo run -p wms-cli -- --help`

3. **Permission denied**
   - Make the binary executable: `chmod +x target/release/wms-cli`

### Debug Commands

```bash
# Test database connection
cargo run -p wms-cli -- system health

# Check with debug logging
RUST_LOG=debug cargo run -p wms-cli -- system health

# Verify environment variables
env | grep -E "(DATABASE_URL|RUST_LOG)"
```

## Dependencies

- **clap**: Command-line argument parsing with derive macros
- **tokio**: Async runtime for database operations
- **anyhow**: Error handling and context
- **color-eyre**: Rich error reporting with color and context
- **tracing**: Structured logging
- **tracing-subscriber**: Log formatting and filtering
- **wms-db**: Database connectivity and operations
- **dotenv**: Environment variable loading
- **fastrand**: Random number generation for order IDs

## Future Enhancements

- [ ] Real inventory database queries
- [ ] Order persistence and retrieval
- [ ] User authentication and authorization
- [ ] Configuration file support
- [ ] Interactive mode with prompts
- [ ] Export functionality (JSON, CSV)
- [ ] Batch operations support
- [ ] Real-time system monitoring

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under the same license as the parent WMS project.