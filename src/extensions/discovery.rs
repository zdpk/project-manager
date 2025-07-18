use crate::extensions::{ExtensionInfo, ExtensionManifest, get_extensions_dir, get_extension_binary_path, get_extension_manifest_path, is_executable};
use anyhow::Result;
use std::collections::HashMap;
use tokio::fs;

/// Discover all installed extensions
pub async fn discover_extensions() -> Result<HashMap<String, ExtensionInfo>> {
    let mut extensions = HashMap::new();
    let extensions_dir = get_extensions_dir()?;
    
    if !extensions_dir.exists() {
        return Ok(extensions);
    }
    
    let mut entries = fs::read_dir(&extensions_dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        
        if entry_path.is_dir() {
            if let Some(extension_name) = entry_path.file_name().and_then(|n| n.to_str()) {
                if let Ok(extension_info) = load_extension_info(extension_name).await {
                    extensions.insert(extension_name.to_string(), extension_info);
                }
            }
        }
    }
    
    Ok(extensions)
}

/// Load extension information from its directory
pub async fn load_extension_info(name: &str) -> Result<ExtensionInfo> {
    let binary_path = get_extension_binary_path(name)?;
    let manifest_path = get_extension_manifest_path(name)?;
    
    // Check if binary exists and is executable
    if !binary_path.exists() {
        return Err(anyhow::anyhow!("Extension binary not found: {}", binary_path.display()));
    }
    
    if !is_executable(&binary_path) {
        return Err(anyhow::anyhow!("Extension binary is not executable: {}", binary_path.display()));
    }
    
    // Check if manifest exists
    if !manifest_path.exists() {
        return Err(anyhow::anyhow!("Extension manifest not found: {}", manifest_path.display()));
    }
    
    // Load and validate manifest
    let manifest = ExtensionManifest::load_from_file(&manifest_path).await?;
    
    // Verify that manifest name matches directory name
    if manifest.name != name {
        return Err(anyhow::anyhow!(
            "Extension name mismatch: directory '{}' vs manifest '{}'", 
            name, 
            manifest.name
        ));
    }
    
    Ok(ExtensionInfo {
        name: manifest.name,
        version: manifest.version,
        description: manifest.description,
        author: manifest.author,
        homepage: manifest.homepage,
        commands: manifest.commands,
    })
}

/// Find extension binary path if it exists
pub fn find_extension_binary(name: &str) -> Option<std::path::PathBuf> {
    if let Ok(binary_path) = get_extension_binary_path(name) {
        if binary_path.exists() && is_executable(&binary_path) {
            return Some(binary_path);
        }
    }
    None
}

/// Check if an extension is installed
pub async fn is_extension_installed(name: &str) -> bool {
    let binary_path = match get_extension_binary_path(name) {
        Ok(path) => path,
        Err(_) => return false,
    };
    
    let manifest_path = match get_extension_manifest_path(name) {
        Ok(path) => path,
        Err(_) => return false,
    };
    
    binary_path.exists() && 
    is_executable(&binary_path) && 
    manifest_path.exists()
}

/// Get extension command names (including aliases)
pub async fn get_extension_commands(name: &str) -> Result<Vec<String>> {
    let extension_info = load_extension_info(name).await?;
    Ok(extension_info.commands.iter()
        .flat_map(|cmd| {
            let mut names = vec![cmd.name.clone()];
            if let Some(aliases) = &cmd.aliases {
                names.extend(aliases.clone());
            }
            names
        })
        .collect())
}

/// Find extension that provides a specific command
pub async fn find_extension_for_command(command: &str) -> Result<Option<String>> {
    let extensions = discover_extensions().await?;
    
    for (ext_name, ext_info) in extensions {
        for cmd in &ext_info.commands {
            if cmd.name == command {
                return Ok(Some(ext_name));
            }
            if let Some(aliases) = &cmd.aliases {
                if aliases.contains(&command.to_string()) {
                    return Ok(Some(ext_name));
                }
            }
        }
    }
    
    Ok(None)
}

/// List all available commands from all extensions
pub async fn list_all_extension_commands() -> Result<HashMap<String, String>> {
    let mut commands = HashMap::new();
    let extensions = discover_extensions().await?;
    
    for (ext_name, ext_info) in extensions {
        for cmd in &ext_info.commands {
            commands.insert(cmd.name.clone(), ext_name.clone());
            
            if let Some(aliases) = &cmd.aliases {
                for alias in aliases {
                    commands.insert(alias.clone(), ext_name.clone());
                }
            }
        }
    }
    
    Ok(commands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;
    
    #[tokio::test]
    async fn test_discover_extensions() {
        // This test would require setting up a temporary extension directory
        // Implementation would depend on the test environment setup
    }
}