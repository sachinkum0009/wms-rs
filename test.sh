#!/bin/bash

echo "Running WMS CLI Tests"
echo "===================="

# Run unit tests (no database required)
echo "1. Running unit tests..."
cargo test --workspace

echo ""
echo "2. Running unit tests with output..."
cargo test --workspace -- --nocapture

echo ""
echo "3. To run integration tests (requires database setup):"
echo "   - Copy .env.example to .env and configure DATABASE_URL"
echo "   - Set up a PostgreSQL database"
echo "   - Run: cargo test --workspace -- --ignored"

echo ""
echo "4. To build and run the CLI:"
echo "   cargo build"
echo "   ./target/debug/wms-cli order create -i 'Your Item' -q 5"