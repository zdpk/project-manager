use crate::constants::*;
use crate::utils::is_git_repository;
use crate::{MachineMetadata, Project};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[schemars(
    title = "PM Configuration",
    description = "Configuration file for PM (Project Manager)"
)]
pub struct Config {
    #[schemars(
        description = "Configuration file version",
        example = "config_version_example"
    )]
    pub version: String,
    #[schemars(
        description = "Directory where PM configuration files are stored",
        example = "config_path_example"
    )]
    pub config_path: PathBuf,
    #[serde(default)]
    #[schemars(description = "Application settings")]
    pub settings: ConfigSettings,
    #[schemars(description = "Project storage (managed by PM)")]
    pub projects: HashMap<Uuid, Project>,
    #[schemars(description = "Machine-specific metadata")]
    pub machine_metadata: HashMap<String, MachineMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Default, JsonSchema)]
#[schemars(
    title = "Configuration Settings",
    description = "Application-specific settings"
)]
pub struct ConfigSettings {
    #[serde(default = "default_show_git_status")]
    #[schemars(description = "Show git status in project listings")]
    pub show_git_status: bool,
    #[serde(default = "default_recent_projects_limit")]
    #[schemars(description = "Maximum number of recent projects to display")]
    pub recent_projects_limit: u32,
}


fn default_show_git_status() -> bool {
    true
}

fn default_recent_projects_limit() -> u32 {
    10
}

// Schema example functions
fn config_version_example() -> &'static str {
    "0.1.1"
}

fn config_path_example() -> &'static str {
    "~/.config/pm"
}



impl Default for Config {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION.to_string(),
            config_path: PathBuf::new(),
            settings: ConfigSettings::default(),
            projects: HashMap::new(),
            machine_metadata: HashMap::new(),
        }
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join(CONFIG_FILENAME))
}

pub fn get_config_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Failed to find home directory")?;
    let config_dir = home_dir.join(CONFIG_DIR_NAME);
    
    // Use different subdirectory for dev mode (_pm binary)
    let subdir_name = if is_dev_mode() {
        "_pm"
    } else {
        CONFIG_SUBDIR_NAME
    };
    
    let pm_dir = config_dir.join(subdir_name);
    
    Ok(pm_dir)
}

/// Detect if running in development mode based on binary name
fn is_dev_mode() -> bool {
    std::env::args()
        .next()
        .map(|path| {
            std::path::Path::new(&path)
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("_pm") || name.contains("_pm"))
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

pub fn get_schema_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("config.schema.json"))
}

pub async fn generate_schema() -> Result<()> {
    let schema = schema_for!(Config);
    let schema_content = serde_json::to_string_pretty(&schema)?;

    let schema_path = get_schema_path()?;
    let parent_dir = schema_path
        .parent()
        .context("Failed to get parent directory")?;

    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).await?;
    }

    fs::write(schema_path, schema_content).await?;
    Ok(())
}

pub async fn load_config() -> Result<Config> {
    let path = get_config_path()?;
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Configuration file not found. Run '{} init' to initialize.", 
            crate::utils::get_binary_name()
        ));
    }
    let content = fs::read_to_string(path).await?;
    let mut config: Config = serde_yaml::from_str(&content)?;

    // Migration: Check if any projects need git repository status update
    let mut needs_migration = false;
    for project in config.projects.values_mut() {
        // Check if the project struct is missing the is_git_repository field (will default to false)
        if !project.is_git_repository && is_git_repository(&project.path) {
            project.is_git_repository = true;
            needs_migration = true;
        }
    }

    // Save config if migration was needed
    if needs_migration {
        save_config(&config).await?;
    }

    // Validate config using schema
    validate_config(&config)?;

    Ok(config)
}

pub async fn save_config(config: &Config) -> Result<()> {
    let path = get_config_path()?;
    let parent_dir = path.parent().context("Failed to get parent directory")?;
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).await?;
    }

    // Validate config before saving
    validate_config(config)?;

    // Generate YAML content with header comment
    let yaml_content = serde_yaml::to_string(config)?;
    let content = format!(
        "# PM Configuration File\n# Schema: ~/.config/pm/config.schema.json\n# Generated by PM v{}\n\n{}",
        CONFIG_VERSION,
        yaml_content
    );

    fs::write(path, content).await?;

    // Auto-generate schema file
    if let Err(e) = generate_schema().await {
        eprintln!("Warning: Failed to generate schema file: {}", e);
    }

    Ok(())
}

impl Config {
    pub fn add_project(&mut self, project: Project) {
        self.projects.insert(project.id, project);
    }

    pub fn find_project_by_name(&self, name: &str) -> Option<&Project> {
        self.projects.values().find(|p| p.name == name)
    }

    pub fn find_project_by_name_mut(&mut self, name: &str) -> Option<&mut Project> {
        self.projects.values_mut().find(|p| p.name == name)
    }

    pub fn find_project_by_path(&self, path: &Path) -> Option<&Project> {
        self.projects.values().find(|p| path.starts_with(&p.path))
    }

    pub fn record_project_access(&mut self, project_id: Uuid) {
        let machine_id = get_machine_id();
        let metadata = self.machine_metadata.entry(machine_id).or_default();

        // Update last accessed time
        metadata.last_accessed.insert(project_id, Utc::now());

        // Update access count
        let count = metadata.access_counts.entry(project_id).or_insert(0);
        *count += 1;
    }

    pub fn remove_project(&mut self, project_id: Uuid) -> anyhow::Result<()> {
        // Remove project from main collection
        self.projects.remove(&project_id);
        
        // Remove from all machine metadata
        for metadata in self.machine_metadata.values_mut() {
            metadata.last_accessed.remove(&project_id);
            metadata.access_counts.remove(&project_id);
        }
        
        Ok(())
    }

    pub fn get_project_access_info(&self, project_id: Uuid) -> (Option<DateTime<Utc>>, u32) {
        let machine_id = get_machine_id();

        if let Some(metadata) = self.machine_metadata.get(&machine_id) {
            let last_accessed = metadata.last_accessed.get(&project_id).copied();
            let access_count = metadata
                .access_counts
                .get(&project_id)
                .copied()
                .unwrap_or(0);
            (last_accessed, access_count)
        } else {
            (None, 0)
        }
    }

    #[allow(dead_code)]
    pub fn get_total_access_count(&self, project_id: Uuid) -> u32 {
        self.machine_metadata
            .values()
            .map(|metadata| {
                metadata
                    .access_counts
                    .get(&project_id)
                    .copied()
                    .unwrap_or(0)
            })
            .sum()
    }
}

fn validate_config(_config: &Config) -> Result<()> {
    // For now, we'll do basic validation without JSON schema
    // Full schema validation will be implemented in Phase 2

    // Basic validations would go here
    Ok(())
}

pub fn get_machine_id() -> String {
    use std::env;

    // Try to get a unique machine identifier
    if let Ok(hostname) = env::var("HOSTNAME") {
        if !hostname.is_empty() {
            return hostname;
        }
    }

    if let Ok(computername) = env::var("COMPUTERNAME") {
        if !computername.is_empty() {
            return computername;
        }
    }

    // Try using system hostname
    if let Ok(hostname) = std::process::Command::new("hostname")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
    {
        if !hostname.is_empty() {
            return hostname;
        }
    }

    // Fallback to username@unknown
    format!(
        "{}@unknown",
        env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string())
    )
}
