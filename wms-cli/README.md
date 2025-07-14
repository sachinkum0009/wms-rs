# WMS CLI

A command-line interface tool for interacting with the Warehouse Management System.

## Features

- **Inventory Management**: List and view inventory items
- **Order Processing**: Create and manage orders
- **System Health**: Monitor system status and connectivity
- **Configuration**: Flexible configuration via environment variables or CLI options
- **Logging**: Configurable logging with verbose mode support

## Installation

From the workspace root:

```bash
cargo build --release -p wms-cli
```

The binary will be available at `target/release/wms-cli`.

## Usage

### Basic Commands

```bash
# Show help
wms-cli --help

# List inventory items
wms-cli inventory list

# Create a new order
wms-cli order create --item "Widget A" --quantity 10

# Check system health
wms-cli system health
```

### Configuration Options

The CLI can be configured through:

1. **Environment variables** (see `.env.example`)
2. **Command-line arguments**
3. **Configuration file** (future feature)

#### Environment Variables

```bash
# Set API URL
export WMS_API_URL="https://api.wms.example.com"

# Set Database URL
export WMS_DATABASE_URL="postgres://user:pass@localhost/wms_db"
```

#### Command-line Options

```bash
# Override API URL
wms-cli --api-url "http://localhost:3000" inventory list

# Override Database URL
wms-cli --database-url "postgres://localhost/test_db" system health

# Enable verbose logging
wms-cli --verbose inventory list

# Use custom config file (future feature)
wms-cli --config ./config.toml inventory list
```

### Examples

#### List Inventory
```bash
$ wms-cli inventory list
📦 WMS Inventory List
=====================

• Widget A - Qty: 150 (electronics)
• Widget B - Qty: 75 (electronics)
• Tool Set - Qty: 25 (hardware)
• Safety Helmet - Qty: 200 (safety)
• Work Gloves - Qty: 500 (safety)

✅ Total items displayed: 5
💡 This is mock data. Connect to actual WMS API for real inventory.
```

#### Create Order
```bash
$ wms-cli order create --item "Safety Helmet" --quantity 5
🛒 Creating New Order
====================

Item: Safety Helmet
Quantity: 5
Timestamp: 2024-01-15 14:30:45 UTC

✅ Order created successfully!
📋 Order ID: ORD-A1B2C3D4
💡 This is a mock order. Connect to actual WMS API for real order processing.
```

#### System Health Check
```bash
$ wms-cli system health
🏥 WMS System Health Check
==========================

🔗 API Connectivity... ✅ OK
🗄️  Database Connectivity... ✅ OK
💾 System Resources... ✅ OK

🎉 Overall System Status: HEALTHY
💡 This is a mock health check. Connect to actual services for real status.
```

## Development

### Running from Source

```bash
# From workspace root
cargo run -p wms-cli -- inventory list

# With verbose logging
cargo run -p wms-cli -- --verbose system health
```

### Architecture

The CLI is built with a modular structure:

- `main.rs` - Entry point and CLI argument parsing
- `config.rs` - Configuration management
- `commands/` - Individual command implementations
  - `inventory.rs` - Inventory management commands
  - `order.rs` - Order processing commands
  - `system.rs` - System health and status commands

### Adding New Commands

1. Create a new module in `commands/`
2. Add the module to `commands/mod.rs`
3. Define the command structure in `main.rs`
4. Implement the command handler function

## Current Limitations

- All commands currently return mock/stubbed data
- No actual API or database connectivity
- Configuration file support is not yet implemented
- Limited error handling for network operations

## Future Enhancements

- [ ] Real API integration
- [ ] Database connectivity
- [ ] Configuration file support
- [ ] Authentication/authorization
- [ ] Enhanced error handling
- [ ] Interactive mode
- [ ] Output formatting options (JSON, CSV, etc.)
- [ ] Command completion and suggestions