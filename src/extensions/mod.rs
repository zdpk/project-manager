pub mod creation;
pub mod discovery;
pub mod manifest;
pub mod manager;
pub mod registry;

pub use discovery::{discover_extensions, find_extension_binary};
pub use manifest::{ExtensionManifest, ExtensionCommand};
pub use manager::{ExtensionManager, handle_extension_command, execute_extension_command};
pub use registry::ExtensionRegistry;

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

/// Get the binary path for a specific extension
pub fn get_extension_binary_path(name: &str) -> Result<PathBuf> {
    Ok(get_extension_dir(name)?.join("binary"))
}

/// Get the manifest path for a specific extension
pub fn get_extension_manifest_path(name: &str) -> Result<PathBuf> {
    Ok(get_extension_dir(name)?.join("manifest.yml"))
}

/// Check if extension directory exists and create if needed
pub async fn ensure_extensions_dir() -> Result<PathBuf> {
    let ext_dir = get_extensions_dir()?;
    if !ext_dir.exists() {
        tokio::fs::create_dir_all(&ext_dir).await?;
    }
    Ok(ext_dir)
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