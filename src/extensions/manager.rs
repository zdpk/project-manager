use crate::extensions::{
    discovery, ensure_extensions_dir, find_extension_binary, 
    get_extension_dir
};
use crate::ExtensionAction;
use anyhow::{Context, Result};
use std::process::Command;
use tokio::fs;

/// Extension manager for handling extension operations
pub struct ExtensionManager;

impl ExtensionManager {
    /// Create a new extension manager
    pub fn new() -> Self {
        Self
    }
}

/// Handle extension management commands
pub async fn handle_extension_command(action: &ExtensionAction) -> Result<()> {
    match action {
        ExtensionAction::Install { name, source, version } => {
            handle_install(name, source.as_deref(), version.as_deref()).await
        }
        ExtensionAction::Uninstall { name, force } => {
            handle_uninstall(name, *force).await
        }
        ExtensionAction::List { all } => {
            handle_list(*all).await
        }
        ExtensionAction::Info { name } => {
            handle_info(name).await
        }
        ExtensionAction::Update { name } => {
            handle_update(name.as_deref()).await
        }
        ExtensionAction::Search { query } => {
            handle_search(query).await
        }
    }
}

/// Execute an external extension command
pub async fn execute_extension_command(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(anyhow::anyhow!("No extension command provided"));
    }
    
    let extension_name = &args[0];
    let extension_args = &args[1..];
    
    // Find the extension binary
    let binary_path = find_extension_binary(extension_name)
        .ok_or_else(|| anyhow::anyhow!("Extension '{}' not found", extension_name))?;
    
    // Prepare environment variables for the extension
    let current_project = get_current_project_context().await?;
    let config_path = crate::config::get_config_path()?;
    
    // Execute the extension
    let mut cmd = Command::new(&binary_path);
    cmd.args(extension_args);
    cmd.env("PM_CONFIG_PATH", config_path);
    cmd.env("PM_CURRENT_PROJECT", current_project);
    cmd.env("PM_VERSION", env!("CARGO_PKG_VERSION"));
    cmd.env("PM_EXTENSION_DIR", get_extension_dir(extension_name)?);
    
    let status = cmd.status()
        .with_context(|| format!("Failed to execute extension '{}'", extension_name))?;
    
    if !status.success() {
        let exit_code = status.code().unwrap_or(-1);
        return Err(anyhow::anyhow!("Extension '{}' exited with code {}", extension_name, exit_code));
    }
    
    Ok(())
}

/// Handle extension installation
async fn handle_install(name: &str, source: Option<&str>, version: Option<&str>) -> Result<()> {
    println!("Installing extension '{}'...", name);
    
    // Ensure extensions directory exists
    ensure_extensions_dir().await?;
    
    // Check if extension is already installed
    if discovery::is_extension_installed(name).await {
        return Err(anyhow::anyhow!("Extension '{}' is already installed", name));
    }
    
    // TODO: Implement actual installation logic based on source
    // For now, just show what would be done
    if let Some(source) = source {
        println!("Would install from source: {}", source);
    } else {
        println!("Would install from default registry");
    }
    
    if let Some(version) = version {
        println!("Would install version: {}", version);
    }
    
    // Create extension directory
    let ext_dir = get_extension_dir(name)?;
    fs::create_dir_all(&ext_dir).await?;
    
    println!("✅ Extension '{}' installation placeholder completed", name);
    println!("📁 Extension directory: {}", ext_dir.display());
    
    Ok(())
}

/// Handle extension uninstallation
async fn handle_uninstall(name: &str, force: bool) -> Result<()> {
    if !discovery::is_extension_installed(name).await {
        return Err(anyhow::anyhow!("Extension '{}' is not installed", name));
    }
    
    if !force {
        // TODO: Add confirmation prompt
        println!("⚠️  This will remove extension '{}' and all its files", name);
        println!("Use --force to skip this confirmation");
        return Ok(());
    }
    
    let ext_dir = get_extension_dir(name)?;
    fs::remove_dir_all(&ext_dir).await
        .with_context(|| format!("Failed to remove extension directory: {}", ext_dir.display()))?;
    
    println!("✅ Extension '{}' has been uninstalled", name);
    
    Ok(())
}

/// Handle listing extensions
async fn handle_list(all: bool) -> Result<()> {
    if all {
        println!("📦 Available extensions (from registry):");
        println!("  (Registry listing not yet implemented)");
        println!();
    }
    
    println!("📦 Installed extensions:");
    
    let extensions = discovery::discover_extensions().await?;
    
    if extensions.is_empty() {
        println!("  No extensions installed");
        return Ok(());
    }
    
    for (name, info) in extensions {
        println!("  {:<12} {} - {}", name, info.version, info.description);
        
        // Show commands
        if !info.commands.is_empty() {
            let command_names: Vec<String> = info.commands.iter()
                .map(|cmd| cmd.name.clone())
                .collect();
            println!("               Commands: {}", command_names.join(", "));
        }
    }
    
    Ok(())
}

/// Handle showing extension information
async fn handle_info(name: &str) -> Result<()> {
    let extension_info = discovery::load_extension_info(name).await
        .with_context(|| format!("Failed to load extension '{}' info", name))?;
    
    println!("📦 Extension: {}", extension_info.name);
    println!("Version: {}", extension_info.version);
    println!("Description: {}", extension_info.description);
    
    if let Some(author) = &extension_info.author {
        println!("Author: {}", author);
    }
    
    if let Some(homepage) = &extension_info.homepage {
        println!("Homepage: {}", homepage);
    }
    
    println!("\nCommands:");
    for cmd in &extension_info.commands {
        println!("  {:<12} {}", cmd.name, cmd.help);
        
        if let Some(aliases) = &cmd.aliases {
            if !aliases.is_empty() {
                println!("               Aliases: {}", aliases.join(", "));
            }
        }
        
        if let Some(args) = &cmd.args {
            if !args.is_empty() {
                println!("               Args: {}", args.join(" "));
            }
        }
    }
    
    Ok(())
}

/// Handle extension updates
async fn handle_update(name: Option<&str>) -> Result<()> {
    if let Some(name) = name {
        println!("Updating extension '{}'...", name);
        // TODO: Implement update logic for specific extension
        println!("✅ Extension '{}' update placeholder completed", name);
    } else {
        println!("Updating all extensions...");
        let extensions = discovery::discover_extensions().await?;
        
        for (ext_name, _) in extensions {
            println!("  Updating '{}'...", ext_name);
            // TODO: Implement update logic
        }
        
        println!("✅ All extensions update placeholder completed");
    }
    
    Ok(())
}

/// Handle extension search
async fn handle_search(query: &str) -> Result<()> {
    println!("Searching for extensions matching '{}'...", query);
    
    // TODO: Implement search logic against registry
    println!("  (Search functionality not yet implemented)");
    
    Ok(())
}

/// Get current project context as JSON string
async fn get_current_project_context() -> Result<String> {
    // Try to load current project information
    match crate::config::load_config().await {
        Ok(config) => {
            // Get current directory and find matching project
            if let Ok(current_dir) = std::env::current_dir() {
                for (_, project) in config.projects {
                    if project.path == current_dir {
                        let context = serde_json::json!({
                            "id": project.id,
                            "name": project.name,
                            "path": project.path,
                            "tags": project.tags,
                            "description": project.description,
                            "is_git_repository": project.is_git_repository
                        });
                        return Ok(context.to_string());
                    }
                }
            }
        }
        Err(_) => {
            // If config can't be loaded, provide minimal context
        }
    }
    
    // Fallback to current directory info
    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    
    let context = serde_json::json!({
        "path": current_dir,
        "name": current_dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
    });
    
    Ok(context.to_string())
}