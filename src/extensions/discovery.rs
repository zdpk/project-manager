use crate::extensions::{
    ExtensionInfo, ExtensionManifest, ExtensionType, get_extensions_dir, 
    get_extension_manifest_path, get_extension_executable_path,
    get_bash_scripts_dir, get_python_scripts_dir, get_bin_dir, is_executable
};
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
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
    let manifest_path = get_extension_manifest_path(name)?;
    
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
    
    // Validate that required files exist based on extension type
    validate_extension_files(name, &manifest).await?;
    
    Ok(ExtensionInfo {
        name: manifest.name,
        version: manifest.version,
        description: manifest.description,
        author: manifest.author,
        homepage: manifest.homepage,
        commands: manifest.commands,
    })
}

/// Validate that all required files exist for an extension
async fn validate_extension_files(name: &str, manifest: &ExtensionManifest) -> Result<()> {
    for command in &manifest.commands {
        let cmd_type = command.get_effective_type(&manifest.extension_type);
        
        match cmd_type {
            ExtensionType::Bash => {
                let file = command.get_file().unwrap_or("main.sh");
                let script_path = get_bash_scripts_dir(name)?.join(file);
                if !script_path.exists() {
                    return Err(anyhow::anyhow!("Bash script not found: {}", script_path.display()));
                }
                if !is_executable(&script_path) {
                    return Err(anyhow::anyhow!("Bash script is not executable: {}", script_path.display()));
                }
            }
            ExtensionType::Python => {
                let file = command.get_file().unwrap_or("main.py");
                let script_path = get_python_scripts_dir(name)?.join(file);
                if !script_path.exists() {
                    return Err(anyhow::anyhow!("Python script not found: {}", script_path.display()));
                }
                if !is_executable(&script_path) {
                    return Err(anyhow::anyhow!("Python script is not executable: {}", script_path.display()));
                }
            }
            ExtensionType::Binary => {
                let file = command.get_file().unwrap_or(ExtensionManifest::get_default_binary_file());
                let binary_path = get_bin_dir(name)?.join(file);
                if !binary_path.exists() {
                    return Err(anyhow::anyhow!("Binary not found: {}", binary_path.display()));
                }
                if !is_executable(&binary_path) {
                    return Err(anyhow::anyhow!("Binary is not executable: {}", binary_path.display()));
                }
            }
            ExtensionType::Mixed => {
                return Err(anyhow::anyhow!("Mixed type should not be used in validation"));
            }
        }
    }
    Ok(())
}

// Deprecated function removed - use find_extension_executable instead

/// Find extension executable path for a specific command
pub async fn find_extension_executable(name: &str, command: &str) -> Option<PathBuf> {
    // First try to load the extension info
    let _extension_info = match load_extension_info(name).await {
        Ok(info) => info,
        Err(_) => return None,
    };
    
    // Load manifest to get extension type
    let manifest_path = match get_extension_manifest_path(name) {
        Ok(path) => path,
        Err(_) => return None,
    };
    
    let manifest = match ExtensionManifest::load_from_file(&manifest_path).await {
        Ok(manifest) => manifest,
        Err(_) => return None,
    };
    
    // Find the specific command
    if let Some(cmd) = manifest.find_command(command) {
        let cmd_type = cmd.get_effective_type(&manifest.extension_type);
        let executable_path = match get_extension_executable_path(name, cmd_type, cmd.get_file()) {
            Ok(path) => path,
            Err(_) => return None,
        };
        
        if executable_path.exists() && is_executable(&executable_path) {
            return Some(executable_path);
        }
    }
    
    None
}

/// Check if an extension is installed
pub async fn is_extension_installed(name: &str) -> bool {
    // Try to load extension info - this will validate all files exist
    load_extension_info(name).await.is_ok()
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
pub async fn find_extension_for_command(command: &str) -> Result<Option<(String, ExtensionManifest)>> {
    let extensions_dir = get_extensions_dir()?;
    
    if !extensions_dir.exists() {
        return Ok(None);
    }
    
    let mut entries = fs::read_dir(&extensions_dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        
        if entry_path.is_dir() {
            if let Some(extension_name) = entry_path.file_name().and_then(|n| n.to_str()) {
                if let Ok(manifest_path) = get_extension_manifest_path(extension_name) {
                    if manifest_path.exists() {
                        if let Ok(manifest) = ExtensionManifest::load_from_file(&manifest_path).await {
                            if manifest.find_command(command).is_some() {
                                return Ok(Some((extension_name.to_string(), manifest)));
                            }
                        }
                    }
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
    
    
    
    
    #[tokio::test]
    async fn test_discover_extensions() {
        // This test would require setting up a temporary extension directory
        // Implementation would depend on the test environment setup
    }
}