use crate::{Project, MachineMetadata};
use crate::constants::*;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: String,
    pub github_username: String,
    pub projects_root_dir: PathBuf,
    #[serde(default = "default_editor")]
    pub editor: String,
    #[serde(default)]
    pub settings: ConfigSettings,
    pub projects: HashMap<Uuid, Project>,
    pub machine_metadata: HashMap<String, MachineMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfigSettings {
    #[serde(default = "default_auto_open_editor")]
    pub auto_open_editor: bool,
    #[serde(default = "default_show_git_status")]
    pub show_git_status: bool,
    #[serde(default = "default_recent_projects_limit")]
    pub recent_projects_limit: u32,
}

fn default_editor() -> String {
    DEFAULT_EDITOR.to_string()
}

fn default_auto_open_editor() -> bool {
    true
}

fn default_show_git_status() -> bool {
    true
}

fn default_recent_projects_limit() -> u32 {
    10
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION.to_string(),
            github_username: String::new(),
            projects_root_dir: PathBuf::from(shellexpand::tilde(DEFAULT_WORKSPACE_DIR).to_string()),
            editor: default_editor(),
            settings: ConfigSettings::default(),
            projects: HashMap::new(),
            machine_metadata: HashMap::new(),
        }
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Failed to find home directory")?;
    let config_dir = home_dir.join(CONFIG_DIR_NAME);
    let pm_dir = config_dir.join(CONFIG_SUBDIR_NAME);
    Ok(pm_dir.join(CONFIG_FILENAME))
}

pub async fn load_config() -> Result<Config> {
    let path = get_config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(path).await?;
    let config: Config = serde_yaml::from_str(&content)?;
    
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
    
    let content = serde_yaml::to_string(config)?;
    fs::write(path, content).await?;
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

    pub fn find_project_by_path(&self, path: &PathBuf) -> Option<&Project> {
        self.projects.values().find(|p| path.starts_with(&p.path))
    }

    pub fn record_project_access(&mut self, project_id: Uuid) {
        let machine_id = get_machine_id();
        let metadata = self.machine_metadata.entry(machine_id).or_insert_with(MachineMetadata::default);
        
        // Update last accessed time
        metadata.last_accessed.insert(project_id, Utc::now());
        
        // Update access count
        let count = metadata.access_counts.entry(project_id).or_insert(0);
        *count += 1;
    }

    pub fn get_project_access_info(&self, project_id: Uuid) -> (Option<DateTime<Utc>>, u32) {
        let machine_id = get_machine_id();
        
        if let Some(metadata) = self.machine_metadata.get(&machine_id) {
            let last_accessed = metadata.last_accessed.get(&project_id).copied();
            let access_count = metadata.access_counts.get(&project_id).copied().unwrap_or(0);
            (last_accessed, access_count)
        } else {
            (None, 0)
        }
    }

    pub fn get_total_access_count(&self, project_id: Uuid) -> u32 {
        self.machine_metadata.values()
            .map(|metadata| metadata.access_counts.get(&project_id).copied().unwrap_or(0))
            .sum()
    }
}


fn validate_config(config: &Config) -> Result<()> {
    // For now, we'll do basic validation without JSON schema
    // Full schema validation will be implemented in Phase 2
    
    // Basic validations
    if config.github_username.is_empty() {
        return Err(anyhow::anyhow!("GitHub username cannot be empty"));
    }
    
    if !config.projects_root_dir.exists() {
        return Err(anyhow::anyhow!(
            "Projects root directory does not exist: {}",
            config.projects_root_dir.display()
        ));
    }
    
    // Validate GitHub username format (basic check)
    if !config.github_username.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err(anyhow::anyhow!(
            "Invalid GitHub username format: {}",
            config.github_username
        ));
    }
    
    Ok(())
}


fn get_machine_id() -> String {
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
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string()) {
        if !hostname.is_empty() {
            return hostname;
        }
    }
    
    // Fallback to username@unknown
    format!("{}@unknown", env::var("USER").or_else(|_| env::var("USERNAME")).unwrap_or_else(|_| "unknown".to_string()))
}
