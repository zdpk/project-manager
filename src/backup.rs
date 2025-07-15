use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use crate::config::get_config_path;
use crate::shell_integration::ShellType;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackupMetadata {
    pub backups: Vec<BackupEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackupEntry {
    pub timestamp: DateTime<Utc>,
    pub id: String,
    pub reason: BackupReason,
    pub files: Vec<BackupFile>,
    pub shell_changes: Vec<ShellChange>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BackupReason {
    InitForceRecreate,
    InitConflictResolution,
    ManualBackup,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackupFile {
    pub original_path: PathBuf,
    pub backup_path: PathBuf,
    pub file_type: BackupFileType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BackupFileType {
    Config,
    ShellIntegration(ShellType),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShellChange {
    pub file: PathBuf,
    pub action: ShellChangeAction,
    pub content: String,
    pub line_number: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ShellChangeAction {
    Append,
    Prepend,
    Insert,
    Replace,
}

impl BackupMetadata {
    pub fn new() -> Self {
        Self {
            backups: Vec::new(),
        }
    }
}

impl Default for BackupMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for BackupReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupReason::InitForceRecreate => write!(f, "Init Force Recreate"),
            BackupReason::InitConflictResolution => write!(f, "Init Conflict Resolution"),
            BackupReason::ManualBackup => write!(f, "Manual Backup"),
        }
    }
}

/// Get the backup directory path
pub fn get_backup_dir() -> Result<PathBuf> {
    let config_path = get_config_path()?;
    let config_dir = config_path.parent()
        .ok_or_else(|| anyhow!("Failed to get config directory"))?;
    Ok(config_dir.join("backups"))
}

/// Get the backup metadata file path
pub fn get_backup_metadata_path() -> Result<PathBuf> {
    Ok(get_backup_dir()?.join("metadata.json"))
}

/// Load backup metadata from disk
pub async fn load_backup_metadata() -> Result<BackupMetadata> {
    let metadata_path = get_backup_metadata_path()?;
    
    if !metadata_path.exists() {
        return Ok(BackupMetadata::new());
    }
    
    let content = fs::read_to_string(&metadata_path).await?;
    let metadata: BackupMetadata = serde_json::from_str(&content)
        .map_err(|e| anyhow!("Failed to parse backup metadata: {}", e))?;
    
    Ok(metadata)
}

/// Save backup metadata to disk
pub async fn save_backup_metadata(metadata: &BackupMetadata) -> Result<()> {
    let metadata_path = get_backup_metadata_path()?;
    let backup_dir = get_backup_dir()?;
    
    // Ensure backup directory exists
    if !backup_dir.exists() {
        fs::create_dir_all(&backup_dir).await?;
    }
    
    let content = serde_json::to_string_pretty(metadata)?;
    fs::write(&metadata_path, content).await?;
    
    Ok(())
}

/// Create a new backup entry
pub async fn create_backup(
    file_path: &Path,
    reason: BackupReason,
) -> Result<BackupEntry> {
    let timestamp = Utc::now();
    let id = timestamp.format("%Y-%m-%d_%H-%M-%S").to_string();
    
    let backup_dir = get_backup_dir()?.join(&id);
    fs::create_dir_all(&backup_dir).await?;
    
    let file_name = file_path.file_name()
        .ok_or_else(|| anyhow!("Invalid file path"))?;
    let backup_path = backup_dir.join(file_name);
    
    // Copy the file to backup directory
    fs::copy(file_path, &backup_path).await?;
    
    let file_type = if file_path.to_string_lossy().contains("config.yml") {
        BackupFileType::Config
    } else if file_path.to_string_lossy().contains("pm.zsh") {
        BackupFileType::ShellIntegration(ShellType::Zsh)
    } else if file_path.to_string_lossy().contains("pm.fish") {
        BackupFileType::ShellIntegration(ShellType::Fish)
    } else if file_path.to_string_lossy().contains("pm.bash") {
        BackupFileType::ShellIntegration(ShellType::Bash)
    } else {
        BackupFileType::Config // Default fallback
    };
    
    let backup_file = BackupFile {
        original_path: file_path.to_path_buf(),
        backup_path,
        file_type,
    };
    
    Ok(BackupEntry {
        timestamp,
        id,
        reason,
        files: vec![backup_file],
        shell_changes: Vec::new(),
    })
}

/// Add a backup entry to metadata and save
pub async fn add_backup_entry(backup_entry: BackupEntry) -> Result<()> {
    let mut metadata = load_backup_metadata().await?;
    metadata.backups.push(backup_entry);
    save_backup_metadata(&metadata).await?;
    Ok(())
}

/// Create multiple file backup
pub async fn create_multi_file_backup(
    files: &[&Path],
    reason: BackupReason,
) -> Result<BackupEntry> {
    let timestamp = Utc::now();
    let id = timestamp.format("%Y-%m-%d_%H-%M-%S").to_string();
    
    let backup_dir = get_backup_dir()?.join(&id);
    fs::create_dir_all(&backup_dir).await?;
    
    let mut backup_files = Vec::new();
    
    for file_path in files {
        if !file_path.exists() {
            continue;
        }
        
        let file_name = file_path.file_name()
            .ok_or_else(|| anyhow!("Invalid file path: {}", file_path.display()))?;
        let backup_path = backup_dir.join(file_name);
        
        // Copy the file to backup directory
        fs::copy(file_path, &backup_path).await?;
        
        let file_type = determine_file_type(file_path);
        
        backup_files.push(BackupFile {
            original_path: file_path.to_path_buf(),
            backup_path,
            file_type,
        });
    }
    
    Ok(BackupEntry {
        timestamp,
        id,
        reason,
        files: backup_files,
        shell_changes: Vec::new(),
    })
}

/// Determine the backup file type based on file path
fn determine_file_type(file_path: &Path) -> BackupFileType {
    let path_str = file_path.to_string_lossy();
    
    if path_str.contains("config.yml") {
        BackupFileType::Config
    } else if path_str.contains("pm.zsh") {
        BackupFileType::ShellIntegration(ShellType::Zsh)
    } else if path_str.contains("pm.fish") {
        BackupFileType::ShellIntegration(ShellType::Fish)
    } else if path_str.contains("pm.bash") {
        BackupFileType::ShellIntegration(ShellType::Bash)
    } else {
        BackupFileType::Config // Default fallback
    }
}

/// Record a shell change for rollback purposes
pub fn create_shell_change(
    file: PathBuf,
    action: ShellChangeAction,
    content: String,
    line_number: Option<usize>,
) -> ShellChange {
    ShellChange {
        file,
        action,
        content,
        line_number,
    }
}

/// Get the most recent backup
pub async fn get_latest_backup() -> Result<Option<BackupEntry>> {
    let metadata = load_backup_metadata().await?;
    
    let latest = metadata.backups
        .into_iter()
        .max_by(|a, b| a.timestamp.cmp(&b.timestamp));
    
    Ok(latest)
}

/// Check if backup directory exists and is accessible
pub async fn is_backup_system_available() -> bool {
    match get_backup_dir() {
        Ok(backup_dir) => {
            if let Err(_) = fs::create_dir_all(&backup_dir).await {
                return false;
            }
            true
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_backup_metadata_serialization() {
        let metadata = BackupMetadata::new();
        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: BackupMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(metadata.backups.len(), deserialized.backups.len());
    }
    
    #[test]
    fn test_backup_reason_display() {
        assert_eq!(BackupReason::InitForceRecreate.to_string(), "Init Force Recreate");
        assert_eq!(BackupReason::InitConflictResolution.to_string(), "Init Conflict Resolution");
        assert_eq!(BackupReason::ManualBackup.to_string(), "Manual Backup");
    }
    
    #[test]
    fn test_determine_file_type() {
        let config_path = Path::new("/home/user/.config/pm/config.yml");
        assert!(matches!(determine_file_type(config_path), BackupFileType::Config));
        
        let zsh_path = Path::new("/home/user/.config/pm/pm.zsh");
        assert!(matches!(determine_file_type(zsh_path), BackupFileType::ShellIntegration(ShellType::Zsh)));
        
        let fish_path = Path::new("/home/user/.config/fish/functions/pm.fish");
        assert!(matches!(determine_file_type(fish_path), BackupFileType::ShellIntegration(ShellType::Fish)));
    }
}