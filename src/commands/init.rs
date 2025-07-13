use crate::commands::project;
use crate::config::{get_config_path, save_config, Config, ConfigSettings};
use crate::constants::*;
use crate::display::*;
use crate::error::{handle_inquire_error, PmError};
use crate::InitMode;
use anyhow::Result;
use inquire::{Confirm, Select, Text};
use std::path::PathBuf;

fn interactive_mode_selection() -> Result<InitMode> {
    let mode_options = vec![
        "🔍 Auto-detect existing workspace and repositories",
        "🌐 Setup GitHub integration for cloning repositories",
        "🚀 Both auto-detection and GitHub integration",
        "⚙️ Manual setup only",
    ];

    let selected = handle_inquire_error(Select::new("Choose your setup preference:", mode_options).prompt())?;

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

    // Determine the mode to use
    let selected_mode = match mode {
        Some(m) => *m, // Use explicitly specified mode
        None => {
            // Show interactive selection
            println!("Select your initialization preference:\n");
            interactive_mode_selection()?
        }
    };

    // Step 1: GitHub username configuration
    let github_username = handle_inquire_error(Text::new("GitHub username:")
        .with_help_message("Used for repository cloning and GitHub integration (required)")
        .prompt())?;

    // Step 2: Projects directory configuration
    let projects_root_dir = {
        let default_workspace = dirs::home_dir()
            .map(|home| home.join("workspace"))
            .unwrap_or_else(|| PathBuf::from("~/workspace"));

        let dir_input = handle_inquire_error(Text::new("Projects root directory:")
            .with_default(&default_workspace.to_string_lossy())
            .with_help_message("Where your projects will be stored (press Enter for default)")
            .prompt())?;

        PathBuf::from(shellexpand::tilde(&dir_input).to_string())
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

    // Create the projects root directory if it doesn't exist
    if !projects_root_dir.exists() {
        println!(
            "\n📁 Creating projects root directory: {}",
            projects_root_dir.display()
        );
        if let Err(e) = std::fs::create_dir_all(&projects_root_dir) {
            display_error("Failed to create directory", &e.to_string());
            println!("   Path: {}", projects_root_dir.display());
            return Err(PmError::DirectoryCreationFailed.into());
        }
    }

    // Step 5: Create and save configuration
    let config = Config {
        version: crate::constants::CONFIG_VERSION.to_string(),
        github_username: github_username.clone(),
        projects_root_dir: projects_root_dir.clone(),
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
    display_init_success(&github_username, &projects_root_dir, &config_path);

    // Step 4: Execute setup actions based on mode
    let mut projects_added = 0;
    let mut scan_failed = false;
    match selected_mode {
        InitMode::Detect => {
            println!("\n🔍 Auto-detecting existing repositories...");
            match project::handle_scan(Some(&projects_root_dir), false).await {
                Ok(count) => {
                    projects_added += count;
                }
                Err(e) => {
                    scan_failed = true;
                    display_warning(&format!("Auto-detection failed: {}", e));
                    println!("💡 You can run 'pm scan' later to detect repositories");
                }
            }
        }
        InitMode::Load => {
            println!("\n🌐 GitHub integration ready!");
            println!("💡 Use 'pm load <owner>/<repo>' to clone and add repositories");

            // Optionally prompt for first repository
            let load_repo = handle_inquire_error(inquire::Confirm::new("Would you like to clone a repository now?")
                .with_default(false)
                .prompt())
                .unwrap_or(false);

            if load_repo {
                let repo = handle_inquire_error(Text::new("Repository (owner/repo format):")
                    .with_help_message("e.g., microsoft/vscode or your-username/my-project")
                    .prompt()
)?;

                if let Err(e) = project::handle_load(&repo, None).await {
                    display_warning(&format!("Failed to load repository: {}", e));
                    println!("💡 You can try again with: pm load {}", repo);
                } else {
                    projects_added += 1;
                }
            }
        }
        InitMode::All => {
            // First auto-detect
            println!("\n🔍 Auto-detecting existing repositories...");
            match project::handle_scan(Some(&projects_root_dir), false).await {
                Ok(count) => {
                    projects_added += count;
                }
                Err(e) => {
                    scan_failed = true;
                    display_warning(&format!("Auto-detection failed: {}", e));
                }
            }

            // Then offer GitHub integration
            println!("\n🌐 GitHub integration ready!");
            let load_repo = handle_inquire_error(inquire::Confirm::new("Would you like to clone a repository now?")
                .with_default(false)
                .prompt())
                .unwrap_or(false);

            if load_repo {
                let repo = handle_inquire_error(Text::new("Repository (owner/repo format):")
                    .with_help_message("e.g., microsoft/vscode or your-username/my-project")
                    .prompt()
)?;

                if let Err(e) = project::handle_load(&repo, None).await {
                    display_warning(&format!("Failed to load repository: {}", e));
                    println!("💡 You can try again with: pm load {}", repo);
                } else {
                    projects_added += 1;
                }
            }
        }
        InitMode::None => {
            println!("\n✅ Manual setup complete!");
        }
    }

    // Show appropriate next steps based on what was accomplished
    if projects_added > 0 {
        println!("\n🎯 Next steps:");
        println!("  pm ls             # List your projects");
        println!("  pm s <name>       # Switch to project");
        println!("  pm add <path>     # Add more projects");
    } else if !scan_failed {
        // Only show next steps if scan didn't fail
        match selected_mode {
            InitMode::None => {
                println!("\n🎯 Next steps:");
                println!("  pm add <path>     # Add your first project");
                println!("  pm scan           # Scan for existing repositories");
                println!("  pm load <owner>/<repo> # Clone from GitHub");
            }
            _ => {
                println!("\n🎯 Next steps:");
                println!("  pm add <path>     # Add your first project");
                println!("  pm scan           # Try scanning again");
                println!("  pm load <owner>/<repo> # Clone from GitHub");
            }
        }
    }

    println!("\n📖 Use 'pm --help' to see all available commands");

    Ok(())
}
