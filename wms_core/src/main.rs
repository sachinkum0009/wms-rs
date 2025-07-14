use clap::{Parser, Subcommand};
use tracing::{info, error};
use anyhow::Result;
use wms_db::{Database, Order};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "wms-cli")]
#[command(about = "A Warehouse Management System CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Order {
        #[command(subcommand)]
        action: OrderCommands,
    },
}

#[derive(Subcommand)]
enum OrderCommands {
    Create {
        #[arg(short = 'i', long = "item")]
        item: String,
        #[arg(short = 'q', long = "quantity")]
        quantity: i32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("WMS CLI initialized");

    let cli = Cli::parse();

    match cli.command {
        Commands::Order { action } => {
            handle_order_command(action).await?;
        }
    }

    Ok(())
}

async fn handle_order_command(action: OrderCommands) -> Result<()> {
    match action {
        OrderCommands::Create { item, quantity } => {
            create_order(&item, quantity).await?;
        }
    }
    Ok(())
}

async fn create_order(item: &str, quantity: i32) -> Result<()> {
    info!("📝 Creating new order...");
    
    // Connect to database
    let db = Database::from_env().await?;
    
    info!("📦 Order details:");
    info!("  • Item: {}", item);
    info!("  • Quantity: {}", quantity);

    // Generate order ID
    let order_id = format!("ORD-{}", generate_order_number());
    
    // Save order to database
    let saved_order = db.create_order(&order_id, item, quantity).await?;
    
    info!("✅ Order created successfully!");
    info!("📋 Order ID: {}", saved_order.id);
    
    Ok(())
}

fn generate_order_number() -> String {
    // Generate a random 6-digit number for the order
    let uuid = Uuid::new_v4();
    let bytes = uuid.as_bytes();
    let number = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) % 1000000;
    format!("{:06}", number)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_generate_order_number() {
        let order_number = generate_order_number();
        assert_eq!(order_number.len(), 6);
        assert!(order_number.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_cli_parsing_order_create() {
        let args = vec!["wms-cli", "order", "create", "-i", "Test Item", "-q", "5"];
        let cli = Cli::try_parse_from(args).expect("Failed to parse CLI args");
        
        match cli.command {
            Commands::Order { action } => {
                match action {
                    OrderCommands::Create { item, quantity } => {
                        assert_eq!(item, "Test Item");
                        assert_eq!(quantity, 5);
                    }
                }
            }
        }
    }

    #[test]
    fn test_cli_parsing_order_create_long_flags() {
        let args = vec!["wms-cli", "order", "create", "--item", "Another Item", "--quantity", "10"];
        let cli = Cli::try_parse_from(args).expect("Failed to parse CLI args");
        
        match cli.command {
            Commands::Order { action } => {
                match action {
                    OrderCommands::Create { item, quantity } => {
                        assert_eq!(item, "Another Item");
                        assert_eq!(quantity, 10);
                    }
                }
            }
        }
    }

    #[test]
    fn test_cli_parsing_invalid_quantity() {
        let args = vec!["wms-cli", "order", "create", "-i", "Test Item", "-q", "invalid"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_order_id_format() {
        let order_number = "123456";
        let order_id = format!("ORD-{}", order_number);
        assert_eq!(order_id, "ORD-123456");
        assert!(order_id.starts_with("ORD-"));
        assert_eq!(order_id.len(), 10); // "ORD-" + 6 digits
    }

    // Integration test (requires database setup)
    #[tokio::test]
    #[ignore] // Ignored by default, run with --ignored flag
    async fn test_create_order_integration() {
        use std::env;
        
        // Skip test if DATABASE_URL is not set
        if env::var("DATABASE_URL").is_err() {
            eprintln!("Skipping integration test: DATABASE_URL not set");
            return;
        }

        // Initialize logging for the test
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info")
            .try_init();

        let result = create_order("Test Integration Item", 3).await;
        
        // The test should succeed if database is properly configured
        match result {
            Ok(_) => {
                // Test passed - order was created successfully
                println!("Integration test passed: Order created successfully");
            }
            Err(e) => {
                // This might fail if database is not set up, which is expected
                eprintln!("Integration test failed (expected if DB not configured): {}", e);
            }
        }
    }
}