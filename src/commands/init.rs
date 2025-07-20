use crate::backup::{BackupReason, create_backup, add_backup_entry};
use crate::config::{get_config_path, save_config, Config, ConfigSettings};
use crate::constants::*;
use crate::display::*;
use crate::error::{handle_inquire_error, PmError};
use crate::shell_integration;
use crate::utils;
use anyhow::Result;
use inquire::{Confirm, Select, Text};
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum ConflictAction {
    Skip,
    Replace,
    Cancel,
}

pub async fn handle_init(
    skip: bool,
    replace: bool,
    dev: bool,
) -> Result<()> {
    let config_path = get_config_path()?;
    let mut backup_entry = None;

    // Handle existing config with new skip/replace model
    if config_path.exists() {
        if skip {
            // Non-interactive skip mode
            display_success(&format!(
                "{} is already initialized",
                APP_NAME.to_uppercase()
            ));
            println!("📁 Configuration file: {}", config_path.display());
            println!("\n💡 To reinitialize with backup:");
            println!("   {} init --replace   # Backup existing and recreate", utils::get_binary_name());
            return Ok(());
        } else if replace {
            // Non-interactive replace mode
            println!("💾 Creating backup of existing config...");
            backup_entry = Some(create_backup(&config_path, BackupReason::InitForceRecreate).await?);
        } else {
            // Interactive mode: skip/replace/cancel
            let action = handle_config_conflict_interactive(&config_path).await?;
            
            match action {
                ConflictAction::Skip => {
                    display_success(&format!(
                        "{} is already initialized",
                        APP_NAME.to_uppercase()
                    ));
                    println!("📁 Configuration file: {}", config_path.display());
                    println!("\n💡 To reinitialize with backup:");
                    println!("   {} init --replace   # Backup existing and recreate", utils::get_binary_name());
                    return Ok(());
                }
                ConflictAction::Replace => {
                    println!("💾 Creating backup and recreating config...");
                    backup_entry = Some(create_backup(&config_path, BackupReason::InitConflictResolution).await?);
                }
                ConflictAction::Cancel => {
                    println!("🚫 Initialization cancelled");
                    return Ok(());
                }
            }
        }
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
    
    // Show configuration file path info
    println!("📄 Configuration file: {}", config_path.display());
    if utils::is_dev_mode() {
        println!("   (Development mode - using separate config from production)");
    }
    
    // Step 5: Shell integration setup with backup support
    println!();
    let shell_backup = setup_shell_integration_with_backup(skip, replace).await?;
    
    // Step 6: Save backup metadata if we created any backups
    if let Some(mut backup) = backup_entry {
        if let Some(shell_backup) = shell_backup {
            backup.files.extend(shell_backup.files);
            backup.shell_changes.extend(shell_backup.shell_changes);
        }
        add_backup_entry(backup).await?;
        println!("💾 Backup created successfully");
    }
    
    // Step 7: Development mode setup (for _pm binary)
    if utils::is_dev_mode() {
        setup_dev_environment().await?;
    }
    

    // Show next steps for using PM
    let binary_name = utils::get_binary_name();
    println!("\n🎯 Next steps:");
    println!("  {} add <path>          # Add your first project", binary_name);
    println!("  {} scan                # Scan for existing repositories", binary_name);
    println!("  {} clone <owner>/<repo> # Clone specific repository", binary_name);
    println!("  {} clone               # Browse and select repositories", binary_name);
    
    if dev {
        println!("\n🔧 Development mode enabled:");
        println!("  _PM_BINARY environment variable configured in shell files");
        println!("  _pm shell integration installed for development");
        println!("  Use current development binary for testing");
    }
    
    let binary_name = utils::get_binary_name();
    println!("\n📖 Use '{} --help' to see all available commands", binary_name);

    Ok(())
}

/// Handle configuration file conflicts with interactive user choice
async fn handle_config_conflict_interactive(
    config_path: &Path,
) -> Result<ConflictAction> {
    println!("⚠️  Configuration already exists: {}", config_path.display());
    
    // Interactive prompt with skip/replace choices
    let choices = vec!["Skip (keep existing)", "Replace (backup and recreate)", "Cancel"];
    let choice = handle_inquire_error(
        Select::new("What would you like to do?", choices)
            .prompt()
    )?;
    
    match choice {
        "Skip (keep existing)" => Ok(ConflictAction::Skip),
        "Replace (backup and recreate)" => Ok(ConflictAction::Replace),
        "Cancel" => Ok(ConflictAction::Cancel),
        _ => unreachable!(),
    }
}

/// Setup shell integration with backup support
async fn setup_shell_integration_with_backup(
    skip: bool,
    replace: bool,
) -> Result<Option<crate::backup::BackupEntry>> {
    // Use the new backup-enabled shell integration setup
    match shell_integration::setup_shell_integration_with_backup(skip, replace).await {
        Ok(backup_entry) => Ok(backup_entry),
        Err(e) => {
            display_warning(&format!("Failed to setup shell integration: {}", e));
            println!("💡 You can manually setup shell integration later");
            Ok(None)
        }
    }
}

/// Setup development environment with _PM_BINARY
async fn setup_dev_environment() -> Result<()> {
    println!("🔧 Setting up development environment...");
    
    // Detect current binary path
    let current_exe = std::env::current_exe()?;
    let dev_binary_path = if current_exe.to_string_lossy().contains("target/debug") {
        current_exe
    } else {
        // Try to guess development directory
        let mut path = current_exe.clone();
        path.pop(); // Remove binary name
        
        // Try to find target/debug/pm
        let mut dev_path = path.clone();
        dev_path.push("target");
        dev_path.push("debug");
        dev_path.push("pm");
        
        if dev_path.exists() {
            dev_path
        } else {
            // Fallback to current exe
            println!("⚠️  Could not detect development binary path, using current executable");
            current_exe
        }
    };
    
    // Add environment variable to shell files
    if let Err(e) = shell_integration::add_dev_env_to_shell_files(&dev_binary_path).await {
        display_warning(&format!("Failed to add development environment to shell files: {}", e));
        println!("💡 You can manually set _PM_BINARY environment variable");
        println!("   export _PM_BINARY=\"{}\"", dev_binary_path.display());
    }
    
    // Setup development shell integration for _pm
    println!("\n🔧 Setting up development shell integration...");
    if let Err(e) = shell_integration::setup_dev_shell_integration().await {
        display_warning(&format!("Failed to setup development shell integration: {}", e));
        println!("💡 You can manually setup _pm shell function later");
    } else {
        println!("✅ Development shell integration installed");
        println!("   You can now use '_pm' command for development");
    }
    
    println!("✅ Development environment configured");
    println!("   _PM_BINARY set to: {}", dev_binary_path.display());
    
    Ok(())
}

