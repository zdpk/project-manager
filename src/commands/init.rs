use std::path::PathBuf;
use crate::config::{get_config_path, save_config, Config};
use crate::constants::*;
use crate::display::*;
use crate::error::PmError;
use crate::InitMode;
use anyhow::Result;
use inquire::{Text, Select};
use crate::commands::project;

fn interactive_mode_selection() -> Result<InitMode> {
    let mode_options = vec![
        "🔍 Auto-detect existing workspace and repositories",
        "🌐 Setup GitHub integration for cloning repositories",
        "🚀 Both auto-detection and GitHub integration",
        "⚙️ Manual setup only",
    ];

    let selected = Select::new("Choose your setup preference:", mode_options)
        .prompt()?;

    // Map the selected option to the corresponding mode
    match selected {
        "🔍 Auto-detect existing workspace and repositories" => Ok(InitMode::Detect),
        "🌐 Setup GitHub integration for cloning repositories" => Ok(InitMode::Load),
        "🚀 Both auto-detection and GitHub integration" => Ok(InitMode::All),
        "⚙️ Manual setup only" => Ok(InitMode::None),
        _ => Ok(InitMode::Detect), // Fallback
    }
}

pub async fn handle_init(mode: Option<&InitMode>) -> Result<()> {
    let config_path = get_config_path()?;

    if config_path.exists() {
        display_success(&format!("{} is already initialized", APP_NAME.to_uppercase()));
        println!("📁 Configuration file: {}", config_path.display());
        println!("\n💡 To reinitialize, delete the config file first:");
        println!("   rm {}", config_path.display());
        return Ok(());
    }

    println!("🚀 Initializing {}...\n", APP_NAME.to_uppercase());

    // Determine the mode to use
    let selected_mode = match mode {
        Some(m) => *m,  // Use explicitly specified mode
        None => {       // Show interactive selection
            println!("Select your initialization preference:\n");
            interactive_mode_selection()?
        }
    };

    // Step 1: Basic configuration
    let github_username = match Text::new("GitHub username:")
        .with_help_message("Used for repository cloning and GitHub integration")
        .prompt() {
        Ok(username) => username,
        Err(e) => {
            display_error("Failed to get GitHub username", &e.to_string());
            display_info("You can set this later in the config file");
            return Err(PmError::InitializationFailed.into());
        }
    };

    // Step 2: Determine workspace directory based on mode
    let projects_root_dir = match selected_mode {
        InitMode::Detect | InitMode::All => {
            // Auto-detect ~/workspace or use home directory
            let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
            let workspace_dir = home_dir.join("workspace");
            
            if workspace_dir.exists() {
                println!("✅ Auto-detected workspace directory: {}", workspace_dir.display());
                workspace_dir
            } else {
                println!("📁 ~/workspace not found, using default location");
                let default_path = Text::new("Projects root directory path:")
                    .with_default(DEFAULT_WORKSPACE_DIR)
                    .with_help_message("Where your projects will be stored")
                    .prompt()?;
                PathBuf::from(shellexpand::tilde(&default_path).to_string())
            }
        }
        _ => {
            // Manual setup for Load and None modes
            let default_path = Text::new("Projects root directory path:")
                .with_default(DEFAULT_WORKSPACE_DIR)
                .with_help_message("Where your projects will be stored")
                .prompt()?;
            PathBuf::from(shellexpand::tilde(&default_path).to_string())
        }
    };

    // Create the projects root directory if it doesn't exist
    if !projects_root_dir.exists() {
        println!("📁 Creating projects root directory: {}", projects_root_dir.display());
        if let Err(e) = std::fs::create_dir_all(&projects_root_dir) {
            display_error("Failed to create directory", &e.to_string());
            println!("   Path: {}", projects_root_dir.display());
            return Err(PmError::DirectoryCreationFailed.into());
        }
    }

    // Step 3: Create and save initial configuration
    let config = Config {
        version: crate::constants::CONFIG_VERSION.to_string(),
        github_username: github_username.clone(),
        projects_root_dir: projects_root_dir.clone(),
        ..Default::default()
    };

    save_config(&config).await?;
    display_init_success(&github_username, &projects_root_dir, &config_path);

    // Step 4: Execute setup actions based on mode
    match selected_mode {
        InitMode::Detect => {
            println!("\n🔍 Auto-detecting existing repositories...");
            if let Err(e) = project::handle_scan(Some(&projects_root_dir), false).await {
                display_warning(&format!("Auto-detection failed: {}", e));
                println!("💡 You can run 'pm scan' later to detect repositories");
            }
        }
        InitMode::Load => {
            println!("\n🌐 GitHub integration ready!");
            println!("💡 Use 'pm load owner/repo' to clone and add repositories");
            
            // Optionally prompt for first repository
            let load_repo = inquire::Confirm::new("Would you like to clone a repository now?")
                .with_default(false)
                .prompt()
                .unwrap_or(false);
                
            if load_repo {
                let repo = Text::new("Repository (owner/repo format):")
                    .with_help_message("e.g., microsoft/vscode or your-username/my-project")
                    .prompt()?;
                    
                if let Err(e) = project::handle_load(&repo, None).await {
                    display_warning(&format!("Failed to load repository: {}", e));
                    println!("💡 You can try again with: pm load {}", repo);
                }
            }
        }
        InitMode::All => {
            // First auto-detect
            println!("\n🔍 Auto-detecting existing repositories...");
            if let Err(e) = project::handle_scan(Some(&projects_root_dir), false).await {
                display_warning(&format!("Auto-detection failed: {}", e));
            }
            
            // Then offer GitHub integration
            println!("\n🌐 GitHub integration ready!");
            let load_repo = inquire::Confirm::new("Would you like to clone a repository now?")
                .with_default(false)
                .prompt()
                .unwrap_or(false);
                
            if load_repo {
                let repo = Text::new("Repository (owner/repo format):")
                    .with_help_message("e.g., microsoft/vscode or your-username/my-project")
                    .prompt()?;
                    
                if let Err(e) = project::handle_load(&repo, None).await {
                    display_warning(&format!("Failed to load repository: {}", e));
                    println!("💡 You can try again with: pm load {}", repo);
                }
            }
        }
        InitMode::None => {
            println!("\n✅ Manual setup complete!");
            println!("💡 Next steps:");
            println!("   - Add projects: pm add <path>");
            println!("   - Scan for repos: pm scan");
            println!("   - Clone from GitHub: pm load owner/repo");
        }
    }

    println!("\n🎉 {} initialized successfully!", APP_NAME.to_uppercase());
    println!("📖 Use 'pm --help' to see all available commands");
    
    Ok(())
}