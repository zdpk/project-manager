use crate::config::{get_config_path, save_config, Config, ConfigSettings};
use crate::constants::*;
use crate::display::*;
use crate::error::{handle_inquire_error, PmError};
use anyhow::Result;
use inquire::{Confirm, Text};
use std::path::PathBuf;

pub async fn handle_init() -> Result<()> {
    let config_path = get_config_path()?;

    if config_path.exists() {
        display_success(&format!(
            "{} is already initialized",
            APP_NAME.to_uppercase()
        ));
        println!("📁 Configuration file: {}", config_path.display());
        println!("\n💡 To reinitialize, delete the config file first:");
        println!("   rm {}", config_path.display());
        return Ok(());
    }

    println!("🚀 Initializing {}...\n", APP_NAME.to_uppercase());

    // Step 1: Configuration directory setup
    let config_dir_path = {
        let default_config_dir = dirs::home_dir()
            .map(|home| home.join(".config").join("pm"))
            .unwrap_or_else(|| PathBuf::from("~/.config/pm"));

        let config_input = handle_inquire_error(Text::new("Configuration directory:")
            .with_default(&default_config_dir.to_string_lossy())
            .with_help_message("Where PM configuration files will be stored (press Enter for default)")
            .prompt())?;

        PathBuf::from(shellexpand::tilde(&config_input).to_string())
    };


    // Step 3: Additional settings

    let show_git_status = handle_inquire_error(Confirm::new("Show git status in project listings?")
        .with_default(true)
        .prompt())?;


    // Create the config directory if it doesn't exist
    if !config_dir_path.exists() {
        println!(
            "📂 Creating configuration directory: {}",
            config_dir_path.display()
        );
        if let Err(e) = std::fs::create_dir_all(&config_dir_path) {
            display_error("Failed to create config directory", &e.to_string());
            println!("   Path: {}", config_dir_path.display());
            return Err(PmError::DirectoryCreationFailed.into());
        }
    }

    // Step 4: Create and save configuration
    let config = Config {
        version: crate::constants::CONFIG_VERSION.to_string(),
        config_path: config_dir_path.clone(),
        settings: ConfigSettings {
            show_git_status,
            recent_projects_limit: 10, // default
        },
        projects: std::collections::HashMap::new(),
        machine_metadata: std::collections::HashMap::new(),
    };

    save_config(&config).await?;
    display_init_success(&config_dir_path, &config_path);

    // Show next steps for using PM
    println!("\n🎯 Next steps:");
    println!("  pm add <path>          # Add your first project");
    println!("  pm scan                # Scan for existing repositories");
    println!("  pm clone <owner>/<repo> # Clone specific repository");
    println!("  pm clone               # Browse and select repositories");
    
    println!("\n📖 Use 'pm --help' to see all available commands");

    Ok(())
}
