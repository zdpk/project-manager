use crate::config::{get_config_path, load_config, save_config, Config};
use crate::constants::*;
use crate::display::*;
use crate::error::PmError;
use anyhow::Result;
use colored::*;
use serde_yaml::Value;
use std::fs;
use std::process::Command;

// Valid configuration keys for validation
const VALID_KEYS: &[&str] = &[
    "version",
    "github_username",
    "projects_root_dir", 
    "editor",
    "settings.auto_open_editor",
    "settings.show_git_status",
    "settings.recent_projects_limit",
];

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

pub async fn handle_get(key: &str) -> Result<()> {
    let config = load_config().await?;
    
    // Convert config to YAML Value for easier nested access
    let config_value = serde_yaml::to_value(&config)?;
    
    // Parse the key path
    let path_segments: Vec<&str> = key.split('.').collect();
    
    // Get the value at the specified path
    match get_nested_value(&config_value, &path_segments) {
        Some(value) => {
            match value {
                Value::String(s) => println!("{}", s),
                Value::Bool(b) => println!("{}", b),
                Value::Number(n) => println!("{}", n),
                Value::Mapping(_) => {
                    // If it's a mapping, show the YAML representation
                    let yaml_str = serde_yaml::to_string(value)?;
                    print!("{}", yaml_str);
                }
                _ => println!("{}", serde_yaml::to_string(value)?),
            }
        }
        None => {
            println!("❌ Key '{}' not found", key.red());
            println!("💡 Use 'pm config list' to see available keys");
            
            // Suggest similar keys
            suggest_similar_keys(key);
            
            return Err(anyhow::anyhow!("Key not found: {}", key));
        }
    }
    
    Ok(())
}

pub async fn handle_set(key: &str, value: &str) -> Result<()> {
    // Validate the key
    if !VALID_KEYS.contains(&key) {
        println!("❌ Invalid key path '{}'", key.red());
        println!("💡 Use 'pm config list' to see available keys");
        suggest_similar_keys(key);
        return Err(anyhow::anyhow!("Invalid key: {}", key));
    }
    
    let mut config = load_config().await?;
    
    // Convert config to YAML Value for manipulation
    let mut config_value = serde_yaml::to_value(&config)?;
    
    // Parse the key path
    let path_segments: Vec<&str> = key.split('.').collect();
    
    // Get current value for comparison
    let old_value = get_nested_value(&config_value, &path_segments)
        .map(|v| format_value_for_display(v))
        .unwrap_or_else(|| "not set".to_string());
    
    // Parse and validate the new value
    let new_value = parse_value_with_validation(key, value)?;
    
    // Set the new value
    set_nested_value(&mut config_value, &path_segments, new_value)?;
    
    // Convert back to Config struct
    config = serde_yaml::from_value(config_value)?;
    
    // Save the updated config
    save_config(&config).await?;
    
    println!("✅ Updated {}: {} → {}", 
        key.cyan(),
        old_value.bright_black(),
        value.green()
    );
    
    Ok(())
}

pub async fn handle_list() -> Result<()> {
    let config = load_config().await?;
    let config_value = serde_yaml::to_value(&config)?;
    
    println!("{}", "📋 Available Configuration Keys".blue().bold());
    println!();
    
    println!("{}", "🔧 Basic Settings:".yellow().bold());
    list_config_key(&config_value, "version", "string");
    list_config_key(&config_value, "github_username", "string");
    list_config_key(&config_value, "projects_root_dir", "path");
    list_config_key(&config_value, "editor", "string");
    
    println!();
    println!("{}", "⚙️  Advanced Settings:".yellow().bold());
    list_config_key(&config_value, "settings.auto_open_editor", "boolean");
    list_config_key(&config_value, "settings.show_git_status", "boolean");
    list_config_key(&config_value, "settings.recent_projects_limit", "integer");
    
    println!();
    println!("💡 Use: {} | {}", 
        "pm config get <key>".cyan(),
        "pm config set <key> <value>".cyan()
    );
    
    Ok(())
}

// Helper functions

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
    // Simple ANSI code removal
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

fn get_nested_value<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = value;
    for segment in path {
        current = current.get(segment)?;
    }
    Some(current)
}

fn set_nested_value(value: &mut Value, path: &[&str], new_value: Value) -> Result<()> {
    if path.is_empty() {
        return Err(anyhow::anyhow!("Empty path"));
    }
    
    if path.len() == 1 {
        // Set the value directly
        if let Value::Mapping(ref mut map) = value {
            map.insert(Value::String(path[0].to_string()), new_value);
        } else {
            return Err(anyhow::anyhow!("Cannot set value on non-mapping"));
        }
    } else {
        // Navigate to the parent and recursively set
        if let Value::Mapping(ref mut map) = value {
            let key = Value::String(path[0].to_string());
            let child = map.get_mut(&key)
                .ok_or_else(|| anyhow::anyhow!("Key not found: {}", path[0]))?;
            set_nested_value(child, &path[1..], new_value)?;
        } else {
            return Err(anyhow::anyhow!("Cannot navigate into non-mapping"));
        }
    }
    
    Ok(())
}

fn parse_value_with_validation(key: &str, value: &str) -> Result<Value> {
    match key {
        "settings.auto_open_editor" | "settings.show_git_status" => {
            match value.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Ok(Value::Bool(true)),
                "false" | "0" | "no" | "off" => Ok(Value::Bool(false)),
                _ => Err(anyhow::anyhow!("Invalid boolean value. Use: true, false, 1, 0, yes, no, on, off")),
            }
        }
        "settings.recent_projects_limit" => {
            let num: u32 = value.parse()
                .map_err(|_| anyhow::anyhow!("Invalid number format"))?;
            if num == 0 || num > 100 {
                return Err(anyhow::anyhow!("Recent projects limit must be between 1 and 100"));
            }
            Ok(Value::Number(num.into()))
        }
        "github_username" => {
            if value.is_empty() {
                return Err(anyhow::anyhow!("GitHub username cannot be empty"));
            }
            if !value.chars().all(|c| c.is_alphanumeric() || c == '-') {
                return Err(anyhow::anyhow!("Invalid GitHub username format"));
            }
            Ok(Value::String(value.to_string()))
        }
        _ => {
            // For other string values, just store as string
            Ok(Value::String(value.to_string()))
        }
    }
}

fn format_value_for_display(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        _ => serde_yaml::to_string(value).unwrap_or_else(|_| "unknown".to_string()).trim().to_string(),
    }
}

fn suggest_similar_keys(key: &str) {
    let key_lower = key.to_lowercase();
    let suggestions: Vec<&str> = VALID_KEYS.iter()
        .filter(|valid_key| {
            let valid_lower = valid_key.to_lowercase();
            valid_lower.contains(&key_lower) || 
            key_lower.contains(&valid_lower) ||
            levenshtein_distance(&key_lower, &valid_lower) <= 2
        })
        .cloned()
        .collect();
    
    if !suggestions.is_empty() {
        println!("🔍 Did you mean:");
        for suggestion in suggestions {
            println!("   {}", suggestion.cyan());
        }
    }
}

fn list_config_key(config_value: &Value, key: &str, type_name: &str) {
    let path_segments: Vec<&str> = key.split('.').collect();
    let value = get_nested_value(config_value, &path_segments);
    
    let value_str = value
        .map(|v| format_value_for_display(v))
        .unwrap_or_else(|| "not set".to_string());
    
    println!("  {:<30} {:<20} ({})", 
        key.cyan(),
        value_str.green(),
        type_name.bright_black()
    );
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    
    matrix[len1][len2]
}