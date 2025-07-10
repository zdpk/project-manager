use crate::{Project, MachineMetadata};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub github_username: String,
    pub projects_root_dir: PathBuf,
    pub projects: HashMap<Uuid, Project>,
    pub machine_metadata: HashMap<String, MachineMetadata>,
}

pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("Failed to find config directory")?;
    let pm_dir = config_dir.join("pm");
    Ok(pm_dir.join("config.json"))
}

pub async fn load_config() -> Result<Config> {
    let path = get_config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(path).await?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

pub async fn save_config(config: &Config) -> Result<()> {
    let path = get_config_path()?;
    let parent_dir = path.parent().context("Failed to get parent directory")?;
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).await?;
    }
    let content = serde_json::to_string_pretty(config)?;
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
}
