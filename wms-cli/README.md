# WMS CLI Tool

A command-line interface for interacting with the Warehouse Management System (WMS). This tool provides a way for developers and admins to manage inventory, submit orders, and check system health from the terminal.

## Features
- **System Health Checks**: Monitor database connectivity and system status
- **Inventory Management**: List inventory items (stub)
- **Order Management**: Create mock orders (stub)
- **Colorful Output and Logging**

## Prerequisites
- Rust (edition 2021)
- PostgreSQL database (or just a .env for stub/demo)

## Installation

### Build from Source
```sh
cargo build -p wms-cli --release
```

## Usage

```sh
# Show help
cargo run -p wms-cli -- --help

# List inventory items
cargo run -p wms-cli -- inventory list

# Create a mock order
cargo run -p wms-cli -- order create --item "Widget X" --quantity 42

# Check system health
cargo run -p wms-cli -- system health
```

## Environment Variables
Create a `.env` (see `.env.example`):

```
DATABASE_URL=postgresql://user:pass@localhost:5432/wms_dev
API_URL=http://localhost:8080/api
RUST_LOG=wms_cli=info
```

## Command Reference
- `inventory list`: Print inventory (fake data for now)
- `order create --item ITEM --quantity AMOUNT`: Submit order (fake for now)
- `system health`: Ping DB/API for system health

## Extending the CLI
- Add new subcommand modules under `src/commands/`
- Register in `commands/mod.rs` & `main.rs` enum

## License
MIT/Apache-2.0 (per parent project)
