pub mod creation;
pub mod discovery;
pub mod manifest;
pub mod manager;
pub mod migration;
pub mod platform;
pub mod registry;
pub mod templates;

pub use creation::{ExtensionCreator, ExtensionCreationConfig};
pub use discovery::{discover_extensions, find_extension_executable};
pub use manifest::{ExtensionManifest, ExtensionCommand, ExtensionType};
pub use manager::{ExtensionManager, handle_extension_command, execute_extension_command};
pub use migration::{ExtensionMigrator, LegacyExtensionInfo};
pub use platform::{Platform, PlatformSelection, OperatingSystem, Architecture};
pub use registry::ExtensionRegistry;
pub use templates::{TemplateContext, WorkflowTemplate, ExtensionTemplate};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Extension information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub homepage: Option<String>,
    pub commands: Vec<ExtensionCommand>,
}

/// Get the extensions directory path
pub fn get_extensions_dir() -> Result<PathBuf> {
    let config_dir = crate::config::get_config_dir()?;
    Ok(config_dir.join("extension"))
}

/// Get the path for a specific extension directory
pub fn get_extension_dir(name: &str) -> Result<PathBuf> {
    Ok(get_extensions_dir()?.join(name))
}

/// Get the binary path for a specific extension (deprecated - use get_extension_executable_path)
#[deprecated(note = "Use get_extension_executable_path instead")]
pub fn get_extension_binary_path(name: &str) -> Result<PathBuf> {
    Ok(get_extension_dir(name)?.join("binary"))
}

/// Get the executable path for a specific extension and command
pub fn get_extension_executable_path(name: &str, extension_type: &ExtensionType, file: Option<&str>) -> Result<PathBuf> {
    let ext_dir = get_extension_dir(name)?;
    
    match extension_type {
        ExtensionType::Bash => {
            let file_name = file.unwrap_or("main.sh");
            Ok(ext_dir.join("bash").join(file_name))
        }
        ExtensionType::Python => {
            let file_name = file.unwrap_or("main.py");
            Ok(ext_dir.join("python").join(file_name))
        }
        ExtensionType::Binary => {
            let file_name = file.unwrap_or("main");
            Ok(ext_dir.join("bin").join(file_name))
        }
        ExtensionType::Mixed => {
            return Err(anyhow::anyhow!("Cannot get executable path for mixed type without specific command type"));
        }
    }
}

/// Get the bash scripts directory for an extension
pub fn get_bash_scripts_dir(name: &str) -> Result<PathBuf> {
    Ok(get_extension_dir(name)?.join("bash"))
}

/// Get the python scripts directory for an extension
pub fn get_python_scripts_dir(name: &str) -> Result<PathBuf> {
    Ok(get_extension_dir(name)?.join("python"))
}

/// Get the binary directory for an extension
pub fn get_bin_dir(name: &str) -> Result<PathBuf> {
    Ok(get_extension_dir(name)?.join("bin"))
}

/// Get the type-specific directory for an extension
pub fn get_type_dir(name: &str, extension_type: &ExtensionType) -> Result<PathBuf> {
    let ext_dir = get_extension_dir(name)?;
    let type_dir = ExtensionManifest::get_type_directory(extension_type);
    Ok(ext_dir.join(type_dir))
}

/// Get the manifest path for a specific extension
pub fn get_extension_manifest_path(name: &str) -> Result<PathBuf> {
    Ok(get_extension_dir(name)?.join("extension.yml"))
}

/// Check if extension directory exists and create if needed
pub async fn ensure_extensions_dir() -> Result<PathBuf> {
    let ext_dir = get_extensions_dir()?;
    if !ext_dir.exists() {
        tokio::fs::create_dir_all(&ext_dir).await?;
    }
    Ok(ext_dir)
}

/// Ensure all necessary directories exist for an extension
pub async fn ensure_extension_dirs(name: &str, extension_type: &ExtensionType) -> Result<()> {
    let ext_dir = get_extension_dir(name)?;
    tokio::fs::create_dir_all(&ext_dir).await?;
    
    match extension_type {
        ExtensionType::Bash => {
            tokio::fs::create_dir_all(get_bash_scripts_dir(name)?).await?;
        }
        ExtensionType::Python => {
            tokio::fs::create_dir_all(get_python_scripts_dir(name)?).await?;
        }
        ExtensionType::Binary => {
            tokio::fs::create_dir_all(get_bin_dir(name)?).await?;
        }
        ExtensionType::Mixed => {
            // Create all directories for mixed extensions
            tokio::fs::create_dir_all(get_bash_scripts_dir(name)?).await?;
            tokio::fs::create_dir_all(get_python_scripts_dir(name)?).await?;
            tokio::fs::create_dir_all(get_bin_dir(name)?).await?;
        }
    }
    
    Ok(())
}

/// Check if a binary is executable
pub fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(path) {
            let permissions = metadata.permissions();
            // Check if any execute bit is set
            permissions.mode() & 0o111 != 0
        } else {
            false
        }
    }
    
    #[cfg(windows)]
    {
        // On Windows, check if file exists and has .exe extension or is executable
        path.exists() && (
            path.extension().map_or(false, |ext| ext == "exe") ||
            std::fs::metadata(path).map_or(false, |m| !m.is_dir())
        )
    }
}