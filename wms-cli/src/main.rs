use clap::{Parser, Subcommand};
use color_eyre::Result;
use tracing::{info, error};
use tracing_subscriber;

mod commands;
mod config;

use commands::{inventory, order, system};
use config::Config;

/// WMS CLI - Warehouse Management System Command Line Interface
#[derive(Parser)]
#[command(name = "wms-cli")]
#[command(about = "A CLI tool for interacting with the Warehouse Management System")]
#[command(version = "0.1.0")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,
    
    /// API URL override
    #[arg(short, long)]
    api_url: Option<String>,
    
    /// Database URL override
    #[arg(short, long)]
    database_url: Option<String>,
    
    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inventory management commands
    Inventory {
        #[command(subcommand)]
        action: InventoryCommands,
    },
    /// Order management commands
    Order {
        #[command(subcommand)]
        action: OrderCommands,
    },
    /// System health and status commands
    System {
        #[command(subcommand)]
        action: SystemCommands,
    },
}

#[derive(Subcommand)]
enum InventoryCommands {
    /// List all inventory items
    List,
}

#[derive(Subcommand)]
enum OrderCommands {
    /// Create a new mock order
    Create {
        /// Item name
        #[arg(short, long)]
        item: String,
        /// Quantity
        #[arg(short, long)]
        quantity: u32,
    },
}

#[derive(Subcommand)]
enum SystemCommands {
    /// Check system health status
    Health,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;
    
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Parse CLI arguments
    let cli = Cli::parse();
    
    // Setup logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("wms_cli={},warn", log_level))
        .init();
    
    info!("Starting WMS CLI");
    
    // Load configuration
    let config = Config::load(cli.config.as_deref(), cli.api_url, cli.database_url)?;
    
    // Route commands
    let result = match cli.command {
        Commands::Inventory { action } => {
            match action {
                InventoryCommands::List => inventory::list(&config).await,
            }
        }
        Commands::Order { action } => {
            match action {
                OrderCommands::Create { item, quantity } => {
                    order::create(&config, &item, quantity).await
                }
            }
        }
        Commands::System { action } => {
            match action {
                SystemCommands::Health => system::health(&config).await,
            }
        }
    };
    
    match result {
        Ok(_) => {
            info!("Command completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Command failed: {}", e);
            Err(e)
        }
    }
}