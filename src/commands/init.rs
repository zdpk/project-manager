use crate::config::{get_config_path, save_config, Config, ConfigSettings};
use crate::constants::*;
use crate::display::*;
use crate::error::{handle_inquire_error, PmError};
use anyhow::Result;
use inquire::{Confirm, Select, Text};
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


    // Step 3: Editor configuration
    let editor_options = vec![
        "code (Visual Studio Code)",
        "hx (Helix)",
        "nvim (Neovim)",
        "vim (Vim)",
        "nano (Nano)",
        "emacs (Emacs)",
        "Other (custom command)",
    ];

    let selected_editor = handle_inquire_error(Select::new("Choose your preferred editor:", editor_options).prompt())?;

    let editor = match selected_editor {
        "code (Visual Studio Code)" => "code".to_string(),
        "hx (Helix)" => "hx".to_string(),
        "nvim (Neovim)" => "nvim".to_string(),
        "vim (Vim)" => "vim".to_string(),
        "nano (Nano)" => "nano".to_string(),
        "emacs (Emacs)" => "emacs".to_string(),
        "Other (custom command)" => handle_inquire_error(Text::new("Enter custom editor command:")
            .with_help_message("e.g., 'subl', 'atom', 'idea'")
            .prompt())?,
        _ => "code".to_string(), // fallback
    };

    // Step 4: Additional settings
    let auto_open_editor = handle_inquire_error(Confirm::new("Automatically open editor when switching to projects?")
        .with_default(true)
        .prompt())?;

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

    // Step 5: Create and save configuration
    let config = Config {
        version: crate::constants::CONFIG_VERSION.to_string(),
        config_path: config_dir_path.clone(),
        editor,
        settings: ConfigSettings {
            auto_open_editor,
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
