use crate::config::{get_config_path, load_config, save_config, Config};
use crate::constants::*;
use crate::display::*;
use crate::error::PmError;
use anyhow::Result;
use colored::*;
use std::fs;
use std::process::Command;

pub async fn handle_show() -> Result<()> {
    let config = load_config().await?;
    let config_path = get_config_path()?;
    
    println!("{}", "📋 PM Configuration".blue().bold());
    println!();
    
    // Create a nice table-like output
    let max_width = 20;
    
    println!("┌─────────────────────┬────────────────────────────────┐");
    println!("│ {}│ {}│", 
        format!("{:width$}", "Field", width = max_width - 1).cyan().bold(),
        format!("{:width$}", "Value", width = 30).cyan().bold()
    );
    println!("├─────────────────────┼────────────────────────────────┤");
    
    print_config_row("Version", &config.version, max_width);
    print_config_row("GitHub Username", &config.github_username, max_width);
    print_config_row("Projects Root", &config.projects_root_dir.display().to_string(), max_width);
    print_config_row("Editor", &config.editor, max_width);
    print_config_row("Auto Open Editor", &format!("{}", if config.settings.auto_open_editor { "✓ enabled".green() } else { "✗ disabled".red() }), max_width);
    print_config_row("Show Git Status", &format!("{}", if config.settings.show_git_status { "✓ enabled".green() } else { "✗ disabled".red() }), max_width);
    print_config_row("Recent Limit", &format!("{} projects", config.settings.recent_projects_limit), max_width);
    
    println!("└─────────────────────┴────────────────────────────────┘");
    println!();
    println!("📁 Config file: {}", config_path.display().to_string().bright_black());
    
    Ok(())
}

pub async fn handle_edit() -> Result<()> {
    let config = load_config().await?;
    let config_path = get_config_path()?;
    
    // Determine editor to use
    let editor = std::env::var("EDITOR")
        .unwrap_or_else(|_| config.editor.clone());
    
    println!("🔧 Opening config file in {}...", editor.cyan());
    
    // Open the config file in editor
    let status = Command::new(&editor)
        .arg(&config_path)
        .status()?;
    
    if !status.success() {
        return Err(anyhow::anyhow!("Editor exited with non-zero status"));
    }
    
    // Validate the config after editing
    match load_config().await {
        Ok(_) => {
            println!("✅ Config validated successfully after edit");
        }
        Err(e) => {
            println!("❌ Config validation failed: {}", e.to_string().red());
            println!("💡 Please fix the errors and try again");
            return Err(e);
        }
    }
    
    Ok(())
}

pub async fn handle_validate() -> Result<()> {
    println!("🔍 Validating configuration...");
    
    let config_path = get_config_path()?;
    
    // Check if config file exists
    if !config_path.exists() {
        println!("❌ Configuration file not found: {}", config_path.display());
        println!("💡 Run 'pm init' to create a configuration file");
        return Err(anyhow::anyhow!("Config file not found"));
    }
    
    // Try to load and validate config
    match load_config().await {
        Ok(config) => {
            println!("✅ Configuration is valid");
            println!();
            println!("{}", "📋 Validation summary:".blue().bold());
            
            // GitHub username validation
            if config.github_username.is_empty() {
                println!("  - GitHub username: {} empty", "⚠️".yellow());
            } else if config.github_username.chars().all(|c| c.is_alphanumeric() || c == '-') {
                println!("  - GitHub username format: {} valid", "✓".green());
            } else {
                println!("  - GitHub username format: {} invalid characters", "❌".red());
            }
            
            // Projects root directory validation
            if config.projects_root_dir.exists() {
                println!("  - Projects root directory: {} exists", "✓".green());
            } else {
                println!("  - Projects root directory: {} does not exist", "❌".red());
            }
            
            // Editor validation
            if Command::new(&config.editor).arg("--version").output().is_ok() {
                println!("  - Editor command: {} found in PATH", "✓".green());
            } else {
                println!("  - Editor command: {} not found or invalid", "⚠️".yellow());
            }
            
            // Settings validation
            if config.settings.recent_projects_limit > 0 && config.settings.recent_projects_limit <= 100 {
                println!("  - Settings values: {} within acceptable ranges", "✓".green());
            } else {
                println!("  - Settings values: {} outside acceptable ranges", "⚠️".yellow());
            }
            
            println!();
            println!("📁 Config file: {}", config_path.display().to_string().bright_black());
            
            Ok(())
        }
        Err(e) => {
            println!("❌ Configuration validation failed:");
            println!("   {}", e.to_string().red());
            println!();
            println!("💡 Common issues:");
            println!("   - Invalid YAML syntax");
            println!("   - Missing required fields");
            println!("   - Invalid path format");
            println!("   - Run 'pm config edit' to fix manually");
            
            Err(e)
        }
    }
}

pub async fn handle_reset() -> Result<()> {
    let config_path = get_config_path()?;
    
    println!("⚠️  This will reset your configuration to defaults.");
    
    if config_path.exists() {
        // Create backup
        let backup_path = config_path.with_extension("yml.backup");
        println!("📁 Current config will be backed up to: {}", backup_path.display());
        println!();
        
        // Ask for confirmation
        print!("Continue? (y/N): ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Operation cancelled.");
            return Ok(());
        }
        
        // Create backup
        fs::copy(&config_path, &backup_path)?;
        println!("📦 Backup created: {}", backup_path.display().to_string().green());
    }
    
    // Create default config
    let default_config = Config::default();
    save_config(&default_config).await?;
    
    println!("✅ Configuration reset to defaults");
    println!("💡 Run 'pm init' to set up your preferences again");
    
    Ok(())
}

fn print_config_row(label: &str, value: &str, max_width: usize) {
    // Remove ANSI color codes for length calculation
    let clean_value = strip_ansi_codes(value);
    let truncated_value = if clean_value.len() > 30 {
        format!("{}...", &clean_value[..27])
    } else {
        clean_value.clone()
    };
    
    println!("│ {}│ {}│", 
        format!("{:width$}", label, width = max_width - 1),
        if clean_value.len() > 30 {
            format!("{:width$}", truncated_value, width = 30)
        } else {
            format!("{:width$}", value, width = 30)
        }
    );
}

fn strip_ansi_codes(s: &str) -> String {
    // Simple ANSI code removal - in a real implementation you might want to use a crate
    let mut result = String::new();
    let mut in_escape = false;
    
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape && c == 'm' {
            in_escape = false;
        } else if !in_escape {
            result.push(c);
        }
    }
    
    result
}