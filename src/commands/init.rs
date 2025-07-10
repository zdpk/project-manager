use std::path::PathBuf;
use crate::config::{get_config_path, save_config, Config};
use crate::constants::*;
use crate::display::*;
use crate::error::PmError;
use anyhow::Result;

pub async fn handle_init() -> Result<()> {
    let config_path = get_config_path()?;

    if config_path.exists() {
        display_success(&format!("{} is already initialized", APP_NAME.to_uppercase()));
        println!("📁 Configuration file: {}", config_path.display());
        println!("\n💡 To reinitialize, delete the config file first:");
        println!("   rm {}", config_path.display());
        return Ok(());
    }

    println!("🚀 Initializing {}...\n", APP_NAME.to_uppercase());

    let github_username = match inquire::Text::new("GitHub username:").prompt() {
        Ok(username) => username,
        Err(e) => {
            display_error("Failed to get GitHub username", &e.to_string());
            display_info("You can also set this later in the config file");
            return Err(PmError::InitializationFailed.into());
        }
    };

    let projects_root_dir_str = match inquire::Text::new("Projects root directory path:")
        .with_default(DEFAULT_WORKSPACE_DIR)
        .prompt()
    {
        Ok(path) => path,
        Err(e) => {
            display_error("Failed to get projects root directory", &e.to_string());
            return Err(PmError::InitializationFailed.into());
        }
    };

    let projects_root_dir = PathBuf::from(shellexpand::tilde(&projects_root_dir_str).to_string());

    // Validate and create the projects root directory if it doesn't exist
    if !projects_root_dir.exists() {
        println!("📁 Creating projects root directory: {}", projects_root_dir.display());
        if let Err(e) = std::fs::create_dir_all(&projects_root_dir) {
            display_error("Failed to create directory", &e.to_string());
            println!("   Path: {}", projects_root_dir.display());
            return Err(PmError::DirectoryCreationFailed.into());
        }
    }

    let config = Config {
        github_username: github_username.clone(),
        projects_root_dir: projects_root_dir.clone(),
        ..Default::default()
    };

    save_config(&config).await?;

    display_init_success(&github_username, &projects_root_dir, &config_path);
    Ok(())
}