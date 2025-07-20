use clap::{CommandFactory, Parser};
use pm::{
    handle_command, handle_config_error, Cli, Commands, PmError, init
};

#[tokio::main]
async fn main() {
    // This is the development binary (_pm)

    // Development mode is always enabled for _pm binary
    std::env::set_var("PM_DEV_MODE", "true");

    let cli = Cli::parse();

    // Handle global version flag
    if cli.version {
        println!("_pm {} (dev)", env!("CARGO_PKG_VERSION"));
        return;
    }

    // If no command provided, show help
    let Some(command) = &cli.command else {
        let mut app = Cli::command();
        app.print_help().unwrap();
        
        // Show additional dev mode info
        println!("\n🔧 Development Mode Information:");
        println!("   Binary: _pm (development version)");
        println!("   Config: Uses separate dev config directory");
        println!("   Shell integration: Uses _pm functions");
        return;
    };

    // Handle commands with dev mode adjustments
    if let Err(e) = handle_command_dev(command).await {
        // Check if this is a user cancellation (Ctrl-C)
        if let Some(pm_error) = e.downcast_ref::<PmError>() {
            if matches!(pm_error, PmError::OperationCancelled) {
                // Gracefully exit on cancellation
                std::process::exit(0);
            }
        }
        handle_config_error(e);
    }
}

/// Handle commands with development mode adjustments
async fn handle_command_dev(command: &Commands) -> anyhow::Result<()> {
    match command {
        Commands::Init { skip, replace } => {
            // Development mode is always enabled for _pm
            init::handle_init(*skip, *replace, true).await
        }
        _ => {
            // For other commands, use the common handler
            handle_command(command).await
        }
    }
}