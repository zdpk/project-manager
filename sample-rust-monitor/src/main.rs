use anyhow::Result;
use clap::{Parser, Subcommand};

/// SAMPLE-RUST-MONITOR - System monitoring tool built in Rust for PM demonstration
#[derive(Parser)]
#[command(name = "pm-ext-sample-rust-monitor", about = "System monitoring tool built in Rust for PM demonstration", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Example command - replace with your extension's functionality
    Example {
        /// Example argument
        #[arg(short, long)]
        message: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Example { message }) => {
            let msg = message.unwrap_or_else(|| "Hello from PM extension!".to_string());
            println!("🚀 {}: {}", env!("CARGO_PKG_NAME"), msg);
            
            // Get PM context from environment variables
            if let Ok(project) = std::env::var("PM_CURRENT_PROJECT") {
                println!("📁 Current PM project: {}", project);
            }
            
            if let Ok(config_path) = std::env::var("PM_CONFIG_PATH") {
                println!("⚙️  PM config: {}", config_path);
            }
        }
        None => {
            println!("🔧 SAMPLE-RUST-MONITOR v{}", env!("CARGO_PKG_VERSION"));
            println!("📖 Use --help to see available commands");
        }
    }
    
    Ok(())
}
