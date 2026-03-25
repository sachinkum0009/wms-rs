use clap::{Parser, Subcommand, Args};
use std::error::Error;

#[derive(Parser)]
#[command(name = "wms_cli", about = "CLI for WMS core")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Publish a message to a topic
    Pub(PublishArgs),
    /// Subscribe to a topic
    Sub {
        #[arg(short, long)]
        topic: String,
    },
}

#[derive(Args)]
pub struct PublishArgs {
    /// The topic name (short: -t, long: --topic)
    #[arg(short, long)]
    topic: String,

    /// The message content (short: -m, long: --message)
    #[arg(short, long)]
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Pub(args) => {
            println!("topic: {}, msg: {}", args.topic, args.message);
        }
        Commands::Sub { topic } => {
            println!("Subscribing to topic: {}", topic);
        }
    }
    
    Ok(())
}