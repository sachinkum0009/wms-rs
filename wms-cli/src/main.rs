use anyhow::Result;
use clap::{Parser, Subcommand};
use color_eyre::eyre;
use tracing::{info, error};

mod commands;

use commands::{system, inventory, order};

#[derive(Parser)]
#[command(name = "wms-cli")]
#[command(about = "A CLI tool for interacting with the Warehouse Management System")]
#[command(version = "0.1.0")]
#[command(author = "WMS Development Team")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// System management commands
    System {
        #[command(subcommand)]
        system_command: SystemCommands,
    },
    /// Inventory management commands
    Inventory {
        #[command(subcommand)]
        inventory_command: InventoryCommands,
    },
    /// Order management commands
    Order {
        #[command(subcommand)]
        order_command: OrderCommands,
    },
}

#[derive(Subcommand)]
enum SystemCommands {
    /// Check system health including database connectivity
    Health,
}

#[derive(Subcommand)]
enum InventoryCommands {
    /// List all inventory items
    List,
}

#[derive(Subcommand)]
enum OrderCommands {
    /// Create a new order
    Create {
        /// Name of the item to order
        #[arg(short, long)]
        item: String,
        /// Quantity to order
        #[arg(short, long)]
        quantity: u32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize color-eyre for better error reporting
    color_eyre::install()?;

    // Initialize logging
    init_logging();

    // Load environment variables
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::System { system_command } => {
            match system_command {
                SystemCommands::Health => system::health().await,
            }
        }
        Commands::Inventory { inventory_command } => {
            match inventory_command {
                InventoryCommands::List => inventory::list().await,
            }
        }
        Commands::Order { order_command } => {
            match order_command {
                OrderCommands::Create { item, quantity } => {
                    order::create(item.clone(), *quantity).await
                }
            }
        }
    }
}

fn init_logging() {
    // Set default log level if not specified
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "wms_cli=info,wms_db=info");
    }

    // Initialize tracing subscriber with colored output
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .with_ansi(true)
        .init();

    info!("WMS CLI initialized");
}