use clap::{CommandFactory, Parser};
use pm::{
    handle_command, handle_config_error, utils, Cli, Commands, PmError
};

#[tokio::main]
async fn main() {
    // Production binary should not be built with dev features
    if cfg!(feature = "dev") {
        eprintln!("❌ Error: Production binary built with dev features!");
        eprintln!("💡 Correct usage:");
        eprintln!("   Production: cargo build --bin pm");
        eprintln!("   Development: cargo build --bin _pm --features dev");
        std::process::exit(1);
    }

    let cli = Cli::parse();

    // Handle global version flag
    if cli.version {
        println!("pm {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // If no command provided, show help
    let Some(command) = &cli.command else {
        let mut app = Cli::command();
        app.print_help().unwrap();
        return;
    };

    // Handle commands
    if let Err(e) = handle_command(command).await {
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