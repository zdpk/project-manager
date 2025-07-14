use crate::config::{get_config_path, load_config, save_config, Config};
use crate::error::handle_inquire_error;
use anyhow::Result;
use chrono::{DateTime, Utc};
use colored::*;
use inquire::{Confirm, Select};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(clap::ValueEnum, Clone)]
pub enum ExportFormat {
    Yaml,
    Json,
}

// Valid configuration keys for validation
const VALID_KEYS: &[&str] = &[
    "version",
    "config_path",
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
    println!(
        "│ {}│ {}│",
        format!("{:width$}", "Field", width = max_width - 1)
            .cyan()
            .bold(),
        format!("{:width$}", "Value", width = 30).cyan().bold()
    );
    println!("├─────────────────────┼────────────────────────────────┤");

    print_config_row("Version", &config.version, max_width);
    print_config_row(
        "Config Path",
        &config.config_path.display().to_string(),
        max_width,
    );
    print_config_row(
        "Show Git Status",
        &format!(
            "{}",
            if config.settings.show_git_status {
                "✓ enabled".green()
            } else {
                "✗ disabled".red()
            }
        ),
        max_width,
    );
    print_config_row(
        "Recent Limit",
        &format!("{} projects", config.settings.recent_projects_limit),
        max_width,
    );

    println!("└─────────────────────┴────────────────────────────────┘");
    println!();
    println!(
        "📁 Config file: {}",
        config_path.display().to_string().bright_black()
    );

    Ok(())
}

pub async fn handle_edit() -> Result<()> {
    let config_path = get_config_path()?;

    // Determine editor to use from environment
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

    println!("🔧 Opening config file in {}...", editor.cyan());

    // Open the config file in editor
    let status = Command::new(&editor).arg(&config_path).status()?;

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



            // Settings validation
            if config.settings.recent_projects_limit > 0
                && config.settings.recent_projects_limit <= 100
            {
                println!(
                    "  - Settings values: {} within acceptable ranges",
                    "✓".green()
                );
            } else {
                println!(
                    "  - Settings values: {} outside acceptable ranges",
                    "⚠️".yellow()
                );
            }

            println!();
            println!(
                "📁 Config file: {}",
                config_path.display().to_string().bright_black()
            );

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
        println!(
            "📁 Current config will be backed up to: {}",
            backup_path.display()
        );
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
        println!(
            "📦 Backup created: {}",
            backup_path.display().to_string().green()
        );
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
        .map(format_value_for_display)
        .unwrap_or_else(|| "not set".to_string());

    // Parse and validate the new value
    let new_value = parse_value_with_validation(key, value)?;

    // Set the new value
    set_nested_value(&mut config_value, &path_segments, new_value)?;

    // Convert back to Config struct
    config = serde_yaml::from_value(config_value)?;

    // Save the updated config
    save_config(&config).await?;

    println!(
        "✅ Updated {}: {} → {}",
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
    list_config_key(&config_value, "config_path", "path");

    println!();
    println!("{}", "⚙️  Advanced Settings:".yellow().bold());
    list_config_key(&config_value, "settings.show_git_status", "boolean");
    list_config_key(&config_value, "settings.recent_projects_limit", "integer");

    println!();
    println!(
        "💡 Use: {} | {}",
        "pm config get <key>".cyan(),
        "pm config set <key> <value>".cyan()
    );

    Ok(())
}

// Helper functions

#[allow(clippy::format_in_format_args)]
fn print_config_row(label: &str, value: &str, max_width: usize) {
    // Remove ANSI color codes for length calculation  
    let clean_value = strip_ansi_codes(value);
    let clean_len = clean_value.chars().count();
    
    if clean_len > 30 {
        let truncated = format!("{}...", &clean_value.chars().take(27).collect::<String>());
        println!(
            "│ {:<width$}│ {:<30}│",
            label, 
            truncated,
            width = max_width - 1
        );
    } else {
        // For values with color codes, we need special handling
        let padding_spaces = 30 - clean_len;
        println!(
            "│ {:<width$}│ {}{}│",
            label,
            value,
            " ".repeat(padding_spaces),
            width = max_width - 1
        );
    }
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
            let child = map
                .get_mut(&key)
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
        "settings.show_git_status" => {
            match value.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Ok(Value::Bool(true)),
                "false" | "0" | "no" | "off" => Ok(Value::Bool(false)),
                _ => Err(anyhow::anyhow!(
                    "Invalid boolean value. Use: true, false, 1, 0, yes, no, on, off"
                )),
            }
        }
        "settings.recent_projects_limit" => {
            let num: u32 = value
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid number format"))?;
            if num == 0 || num > 100 {
                return Err(anyhow::anyhow!(
                    "Recent projects limit must be between 1 and 100"
                ));
            }
            Ok(Value::Number(num.into()))
        }
        "config_path" => {
            let path = PathBuf::from(shellexpand::tilde(value).into_owned());
            Ok(Value::String(path.display().to_string()))
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
        _ => serde_yaml::to_string(value)
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string(),
    }
}

fn suggest_similar_keys(key: &str) {
    let key_lower = key.to_lowercase();
    let suggestions: Vec<&str> = VALID_KEYS
        .iter()
        .filter(|valid_key| {
            let valid_lower = valid_key.to_lowercase();
            valid_lower.contains(&key_lower)
                || key_lower.contains(&valid_lower)
                || levenshtein_distance(&key_lower, &valid_lower) <= 2
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
        .map(format_value_for_display)
        .unwrap_or_else(|| "not set".to_string());

    println!(
        "  {:<30} {:<20} ({})",
        key.cyan(),
        value_str.green(),
        type_name.bright_black()
    );
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    #[allow(clippy::needless_range_loop)]
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
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[len1][len2]
}

// =====================================================
// Phase 4: Advanced Config Features
// =====================================================

#[derive(Serialize, Deserialize, Debug)]
struct BackupMetadata {
    name: String,
    created_at: DateTime<Utc>,
    description: Option<String>,
    config_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTemplate {
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    config: Config,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigHistory {
    timestamp: DateTime<Utc>,
    action: String,
    details: String,
    config_hash: String,
}

// Helper functions for paths
fn get_backups_dir() -> Result<PathBuf> {
    let config_path = get_config_path()?;
    let config_dir = config_path.parent().unwrap();
    Ok(config_dir.join("backups"))
}

fn get_templates_dir() -> Result<PathBuf> {
    let config_path = get_config_path()?;
    let config_dir = config_path.parent().unwrap();
    Ok(config_dir.join("templates"))
}

fn get_history_file() -> Result<PathBuf> {
    let config_path = get_config_path()?;
    let config_dir = config_path.parent().unwrap();
    Ok(config_dir.join("history.yml"))
}

fn ensure_dir_exists(dir: &Path) -> Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

fn generate_backup_name() -> String {
    Utc::now().format("backup_%Y%m%d_%H%M%S").to_string()
}

fn config_hash(config: &Config) -> Result<String> {
    let config_yaml = serde_yaml::to_string(config)?;
    Ok(format!("{:x}", md5::compute(config_yaml.as_bytes())))
}

// =====================================================
// Backup Commands
// =====================================================

pub async fn handle_backup_create(name: Option<&str>) -> Result<()> {
    let config = load_config().await?;
    let backups_dir = get_backups_dir()?;
    ensure_dir_exists(&backups_dir)?;

    let backup_name = name.unwrap_or(&generate_backup_name()).to_string();
    let backup_file = backups_dir.join(format!("{}.yml", backup_name));

    if backup_file.exists() {
        return Err(anyhow::anyhow!("Backup '{}' already exists", backup_name));
    }

    // Create backup metadata
    let metadata = BackupMetadata {
        name: backup_name.clone(),
        created_at: Utc::now(),
        description: None,
        config_version: config.version.clone(),
    };

    // Save config and metadata
    let backup_data = serde_yaml::to_string(&(metadata, config))?;
    fs::write(&backup_file, backup_data)?;

    // Add to history
    add_to_history(
        &format!("backup_create:{}", backup_name),
        &format!("Created backup '{}'", backup_name),
    )
    .await?;

    println!("✅ Created backup: {}", backup_name.green());
    println!(
        "📁 Location: {}",
        backup_file.display().to_string().bright_black()
    );

    Ok(())
}

pub async fn handle_backup_restore(name: &str) -> Result<()> {
    let backups_dir = get_backups_dir()?;
    let backup_file = backups_dir.join(format!("{}.yml", name));

    if !backup_file.exists() {
        return Err(anyhow::anyhow!("Backup '{}' not found", name));
    }

    // Load backup
    let backup_content = fs::read_to_string(&backup_file)?;
    let (metadata, backup_config): (BackupMetadata, Config) =
        serde_yaml::from_str(&backup_content)?;

    // Confirm restore
    let confirm = handle_inquire_error(Confirm::new(&format!(
        "Restore configuration from backup '{}'? This will overwrite your current config.",
        name
    ))
    .with_default(false)
    .prompt())?;

    if !confirm {
        println!("Restore cancelled.");
        return Ok(());
    }

    // Create automatic backup before restore
    handle_backup_create(Some(&format!(
        "auto_before_restore_{}",
        Utc::now().format("%Y%m%d_%H%M%S")
    )))
    .await?;

    // Restore config
    save_config(&backup_config).await?;

    // Add to history
    add_to_history(
        &format!("backup_restore:{}", name),
        &format!("Restored from backup '{}'", name),
    )
    .await?;

    println!("✅ Restored configuration from backup: {}", name.green());
    println!(
        "📅 Backup created: {}",
        metadata.created_at.format("%Y-%m-%d %H:%M:%S UTC")
    );

    Ok(())
}

pub async fn handle_backup_list() -> Result<()> {
    let backups_dir = get_backups_dir()?;

    if !backups_dir.exists() {
        println!("📦 No backups found");
        return Ok(());
    }

    let mut backups = Vec::new();

    for entry in fs::read_dir(&backups_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("yml") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok((metadata, _)) =
                    serde_yaml::from_str::<(BackupMetadata, Config)>(&content)
                {
                    backups.push(metadata);
                }
            }
        }
    }

    if backups.is_empty() {
        println!("📦 No valid backups found");
        return Ok(());
    }

    // Sort by creation date (newest first)
    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    println!("{}", "📦 Configuration Backups".blue().bold());
    println!();

    for backup in backups {
        let age = format_duration(Utc::now().signed_duration_since(backup.created_at));
        println!(
            "  {} {}",
            backup.name.cyan().bold(),
            format!("({})", age).bright_black()
        );
        println!(
            "    📅 {}",
            backup.created_at.format("%Y-%m-%d %H:%M:%S UTC")
        );
        if let Some(desc) = backup.description {
            println!("    📝 {}", desc);
        }
        println!();
    }

    println!(
        "💡 Use: {} | {}",
        "pm config backup restore <name>".cyan(),
        "pm config backup delete <name>".cyan()
    );

    Ok(())
}

pub async fn handle_backup_delete(name: &str) -> Result<()> {
    let backups_dir = get_backups_dir()?;
    let backup_file = backups_dir.join(format!("{}.yml", name));

    if !backup_file.exists() {
        return Err(anyhow::anyhow!("Backup '{}' not found", name));
    }

    let confirm = handle_inquire_error(Confirm::new(&format!("Delete backup '{}'? This cannot be undone.", name))
        .with_default(false)
        .prompt())?;

    if !confirm {
        println!("Delete cancelled.");
        return Ok(());
    }

    fs::remove_file(&backup_file)?;

    // Add to history
    add_to_history(
        &format!("backup_delete:{}", name),
        &format!("Deleted backup '{}'", name),
    )
    .await?;

    println!("✅ Deleted backup: {}", name.green());

    Ok(())
}

// =====================================================
// Template Commands
// =====================================================

pub async fn handle_template_list() -> Result<()> {
    let templates_dir = get_templates_dir()?;

    // Show built-in templates first
    println!("{}", "📋 Configuration Templates".blue().bold());
    println!();
    println!("{}", "🏭 Built-in Templates:".yellow().bold());

    let builtin_templates = get_builtin_templates();
    for (name, description) in builtin_templates {
        println!("  {} - {}", name.cyan().bold(), description);
    }

    // Show user templates
    if templates_dir.exists() {
        let mut user_templates = Vec::new();

        for entry in fs::read_dir(&templates_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("yml") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(template) = serde_yaml::from_str::<ConfigTemplate>(&content) {
                        user_templates.push(template);
                    }
                }
            }
        }

        if !user_templates.is_empty() {
            println!();
            println!("{}", "👤 User Templates:".yellow().bold());

            user_templates.sort_by(|a, b| a.name.cmp(&b.name));

            for template in user_templates {
                println!(
                    "  {} - {}",
                    template.name.cyan().bold(),
                    template
                        .description
                        .unwrap_or_else(|| "No description".to_string())
                );
                println!("    📅 Created: {}", template.created_at.format("%Y-%m-%d"));
            }
        }
    }

    println!();
    println!(
        "💡 Use: {} | {}",
        "pm config template apply <name>".cyan(),
        "pm config template save <name>".cyan()
    );

    Ok(())
}

pub async fn handle_template_apply(name: &str) -> Result<()> {
    // Check built-in templates first
    let builtin_templates = get_builtin_templates();
    if builtin_templates
        .iter()
        .any(|(template_name, _)| template_name == &name)
    {
        return apply_builtin_template(name).await;
    }

    // Check user templates
    let templates_dir = get_templates_dir()?;
    let template_file = templates_dir.join(format!("{}.yml", name));

    if !template_file.exists() {
        return Err(anyhow::anyhow!("Template '{}' not found", name));
    }

    let template_content = fs::read_to_string(&template_file)?;
    let template: ConfigTemplate = serde_yaml::from_str(&template_content)?;

    // Confirm application
    let confirm = handle_inquire_error(Confirm::new(&format!(
        "Apply template '{}'? This will overwrite your current config.",
        name
    ))
    .with_default(false)
    .prompt())?;

    if !confirm {
        println!("Template application cancelled.");
        return Ok(());
    }

    // Create backup before applying template
    handle_backup_create(Some(&format!(
        "auto_before_template_{}",
        Utc::now().format("%Y%m%d_%H%M%S")
    )))
    .await?;

    // Apply template
    save_config(&template.config).await?;

    // Add to history
    add_to_history(
        &format!("template_apply:{}", name),
        &format!("Applied template '{}'", name),
    )
    .await?;

    println!("✅ Applied template: {}", name.green());
    if let Some(desc) = template.description {
        println!("📝 {}", desc);
    }

    Ok(())
}

pub async fn handle_template_save(name: &str, description: Option<&str>) -> Result<()> {
    let config = load_config().await?;
    let templates_dir = get_templates_dir()?;
    ensure_dir_exists(&templates_dir)?;

    let template_file = templates_dir.join(format!("{}.yml", name));

    if template_file.exists() {
        let confirm = handle_inquire_error(Confirm::new(&format!("Template '{}' already exists. Overwrite?", name))
            .with_default(false)
            .prompt())?;

        if !confirm {
            println!("Template save cancelled.");
            return Ok(());
        }
    }

    let template = ConfigTemplate {
        name: name.to_string(),
        description: description.map(|s| s.to_string()),
        created_at: Utc::now(),
        config,
    };

    let template_yaml = serde_yaml::to_string(&template)?;
    fs::write(&template_file, template_yaml)?;

    // Add to history
    add_to_history(
        &format!("template_save:{}", name),
        &format!("Saved template '{}'", name),
    )
    .await?;

    println!("✅ Saved template: {}", name.green());
    println!(
        "📁 Location: {}",
        template_file.display().to_string().bright_black()
    );

    Ok(())
}

pub async fn handle_template_delete(name: &str) -> Result<()> {
    let templates_dir = get_templates_dir()?;
    let template_file = templates_dir.join(format!("{}.yml", name));

    if !template_file.exists() {
        return Err(anyhow::anyhow!("Template '{}' not found", name));
    }

    let confirm = handle_inquire_error(Confirm::new(&format!(
        "Delete template '{}'? This cannot be undone.",
        name
    ))
    .with_default(false)
    .prompt())?;

    if !confirm {
        println!("Delete cancelled.");
        return Ok(());
    }

    fs::remove_file(&template_file)?;

    // Add to history
    add_to_history(
        &format!("template_delete:{}", name),
        &format!("Deleted template '{}'", name),
    )
    .await?;

    println!("✅ Deleted template: {}", name.green());

    Ok(())
}

// Helper functions for templates and other Phase 4 features will be continued...

fn get_builtin_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("minimal", "Minimal configuration with basic settings"),
        ("developer", "Standard developer configuration"),
        ("team", "Team collaboration settings"),
        ("enterprise", "Enterprise environment configuration"),
    ]
}

async fn apply_builtin_template(name: &str) -> Result<()> {
    let config = match name {
        "minimal" => create_minimal_config(),
        "developer" => create_developer_config(),
        "team" => create_team_config(),
        "enterprise" => create_enterprise_config(),
        _ => return Err(anyhow::anyhow!("Unknown built-in template: {}", name)),
    };

    // Confirm application
    let confirm = handle_inquire_error(Confirm::new(&format!(
        "Apply built-in template '{}'? This will overwrite your current config.",
        name
    ))
    .with_default(false)
    .prompt())?;

    if !confirm {
        println!("Template application cancelled.");
        return Ok(());
    }

    // Create backup before applying template
    handle_backup_create(Some(&format!(
        "auto_before_builtin_{}",
        Utc::now().format("%Y%m%d_%H%M%S")
    )))
    .await?;

    // Apply template
    save_config(&config).await?;

    // Add to history
    add_to_history(
        &format!("template_apply:{}", name),
        &format!("Applied built-in template '{}'", name),
    )
    .await?;

    println!("✅ Applied built-in template: {}", name.green());

    Ok(())
}

fn create_minimal_config() -> Config {
    let mut config = Config::default();
    config.settings.recent_projects_limit = 5;
    config.settings.show_git_status = false;
    config
}

fn create_developer_config() -> Config {
    let mut config = Config::default();
    config.settings.recent_projects_limit = 20;
    config.settings.show_git_status = true;
    config
}

fn create_team_config() -> Config {
    let mut config = Config::default();
    config.settings.recent_projects_limit = 15;
    config.settings.show_git_status = true;
    config
}

fn create_enterprise_config() -> Config {
    let mut config = Config::default();
    config.settings.recent_projects_limit = 50;
    config.settings.show_git_status = true;
    config
}


// =====================================================
// Setup Command
// =====================================================

pub async fn handle_setup(quick: bool) -> Result<()> {
    if quick {
        return setup_quick().await;
    }

    println!("{}", "🚀 PM Configuration Setup".blue().bold());
    println!("Let's configure your project manager settings!\n");

    let mut config = Config::default();

    // Projects root directory


    // Show git status
    let show_git = handle_inquire_error(Confirm::new("Show git status in project lists?")
        .with_default(true)
        .prompt())?;
    config.settings.show_git_status = show_git;

    // Recent projects limit
    let recent_limit_options = vec![5, 10, 15, 20, 25, 30];
    let recent_limit = handle_inquire_error(Select::new("Recent projects limit:", recent_limit_options).prompt())?;
    config.settings.recent_projects_limit = recent_limit as u32;


    // Save configuration
    save_config(&config).await?;

    // Add to history
    add_to_history("setup", "Interactive configuration setup completed").await?;

    println!("\n✅ Configuration setup complete!");
    println!(
        "💡 You can modify these settings anytime with: {}",
        "pm config edit".cyan()
    );

    Ok(())
}

async fn setup_quick() -> Result<()> {
    println!("{}", "⚡ Quick Setup".blue().bold());

    let mut config = Config::default();

    // Set sensible defaults
    config.settings.show_git_status = true;
    config.settings.recent_projects_limit = 15;


    save_config(&config).await?;

    // Add to history
    add_to_history("setup_quick", "Quick configuration setup completed").await?;

    println!("✅ Quick setup complete with default settings!");
    println!("💡 Run {} for interactive setup", "pm config setup".cyan());

    Ok(())
}

// =====================================================
// Export/Import Commands
// =====================================================

pub async fn handle_export(format: &ExportFormat, file: Option<&Path>) -> Result<()> {
    let config = load_config().await?;

    let content = match format {
        ExportFormat::Yaml => serde_yaml::to_string(&config)?,
        ExportFormat::Json => serde_json::to_string_pretty(&config)?,
    };

    match file {
        Some(path) => {
            fs::write(path, content)?;
            println!(
                "✅ Configuration exported to: {}",
                path.display().to_string().green()
            );
        }
        None => {
            println!("{}", content);
        }
    }

    Ok(())
}

pub async fn handle_import(file: &Path, force: bool) -> Result<()> {
    if !file.exists() {
        return Err(anyhow::anyhow!("File not found: {}", file.display()));
    }

    let content = fs::read_to_string(file)?;

    // Try to parse as YAML first, then JSON
    let config: Config = serde_yaml::from_str(&content)
        .or_else(|_| serde_json::from_str(&content))
        .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))?;

    if !force {
        // Create backup before import
        handle_backup_create(Some(&format!(
            "auto_before_import_{}",
            Utc::now().format("%Y%m%d_%H%M%S")
        )))
        .await?;

        let confirm = handle_inquire_error(Confirm::new(&format!(
            "Import configuration from '{}'? This will overwrite your current config.",
            file.display()
        ))
        .with_default(false)
        .prompt())?;

        if !confirm {
            println!("Import cancelled.");
            return Ok(());
        }
    }

    save_config(&config).await?;

    // Add to history
    add_to_history(
        &format!("import:{}", file.display()),
        &format!("Imported configuration from '{}'", file.display()),
    )
    .await?;

    println!(
        "✅ Configuration imported from: {}",
        file.display().to_string().green()
    );

    Ok(())
}

// =====================================================
// Diff and History Commands
// =====================================================

pub async fn handle_diff(backup_name: Option<&str>) -> Result<()> {
    let current_config = load_config().await?;

    let (backup_config, backup_display_name) = if let Some(name) = backup_name {
        // Compare with specific backup
        let backups_dir = get_backups_dir()?;
        let backup_file = backups_dir.join(format!("{}.yml", name));

        if !backup_file.exists() {
            return Err(anyhow::anyhow!("Backup '{}' not found", name));
        }

        let backup_content = fs::read_to_string(&backup_file)?;
        let (_, backup_config): (BackupMetadata, Config) = serde_yaml::from_str(&backup_content)?;
        (backup_config, name.to_string())
    } else {
        // Compare with latest backup
        let backups_dir = get_backups_dir()?;

        if !backups_dir.exists() {
            println!("📦 No backups found to compare with");
            return Ok(());
        }

        let mut latest_backup = None;
        let mut latest_time = DateTime::from_timestamp(0, 0).unwrap();

        for entry in fs::read_dir(&backups_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("yml") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok((metadata, config)) =
                        serde_yaml::from_str::<(BackupMetadata, Config)>(&content)
                    {
                        if metadata.created_at > latest_time {
                            latest_time = metadata.created_at;
                            latest_backup = Some((config, metadata.name));
                        }
                    }
                }
            }
        }

        match latest_backup {
            Some((config, name)) => (config, format!("{} (latest)", name)),
            None => {
                println!("📦 No valid backups found to compare with");
                return Ok(());
            }
        }
    };

    println!(
        "{}",
        format!("🔍 Configuration Diff vs {}", backup_display_name)
            .blue()
            .bold()
    );
    println!();

    // Compare configurations
    show_config_diff(&backup_config, &current_config)?;

    Ok(())
}

pub async fn handle_history(limit: usize) -> Result<()> {
    let history_file = get_history_file()?;

    if !history_file.exists() {
        println!("📊 No configuration history found");
        return Ok(());
    }

    let history_content = fs::read_to_string(&history_file)?;
    let mut history: Vec<ConfigHistory> =
        serde_yaml::from_str(&history_content).unwrap_or_default();

    if history.is_empty() {
        println!("📊 No configuration history found");
        return Ok(());
    }

    // Sort by timestamp (newest first) and limit
    history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    history.truncate(limit);

    println!("{}", "📊 Configuration History".blue().bold());
    println!();

    for entry in history {
        let age = format_duration(Utc::now().signed_duration_since(entry.timestamp));
        println!(
            "  {} {}",
            entry.action.cyan().bold(),
            format!("({})", age).bright_black()
        );
        println!("    📅 {}", entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("    📝 {}", entry.details);
        println!();
    }

    Ok(())
}

// =====================================================
// Helper Functions
// =====================================================

async fn add_to_history(action: &str, details: &str) -> Result<()> {
    let history_file = get_history_file()?;
    let config = load_config().await?;

    let mut history: Vec<ConfigHistory> = if history_file.exists() {
        let content = fs::read_to_string(&history_file)?;
        serde_yaml::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    };

    let entry = ConfigHistory {
        timestamp: Utc::now(),
        action: action.to_string(),
        details: details.to_string(),
        config_hash: config_hash(&config).unwrap_or_else(|_| "unknown".to_string()),
    };

    history.push(entry);

    // Keep only last 100 entries
    if history.len() > 100 {
        history.truncate(100);
    }

    let history_yaml = serde_yaml::to_string(&history)?;

    // Ensure parent directory exists
    if let Some(parent) = history_file.parent() {
        ensure_dir_exists(parent)?;
    }

    fs::write(&history_file, history_yaml)?;

    Ok(())
}

fn show_config_diff(old: &Config, new: &Config) -> Result<()> {
    // Simple field-by-field comparison


    if old.settings.show_git_status != new.settings.show_git_status {
        println!(
            "  {} {} → {}",
            "settings.show_git_status:".yellow(),
            old.settings.show_git_status.to_string().red(),
            new.settings.show_git_status.to_string().green()
        );
    }

    if old.settings.recent_projects_limit != new.settings.recent_projects_limit {
        println!(
            "  {} {} → {}",
            "settings.recent_projects_limit:".yellow(),
            old.settings.recent_projects_limit.to_string().red(),
            new.settings.recent_projects_limit.to_string().green()
        );
    }

    Ok(())
}

fn format_duration(duration: chrono::TimeDelta) -> String {
    let total_seconds = duration.num_seconds();

    if total_seconds < 60 {
        format!("{}s ago", total_seconds)
    } else if total_seconds < 3600 {
        format!("{}m ago", total_seconds / 60)
    } else if total_seconds < 86400 {
        format!("{}h ago", total_seconds / 3600)
    } else {
        format!("{}d ago", total_seconds / 86400)
    }
}
