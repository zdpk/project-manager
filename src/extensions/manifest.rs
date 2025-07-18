use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Extension manifest structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub homepage: Option<String>,
    pub pm_version: Option<String>,
    pub commands: Vec<ExtensionCommand>,
}

/// Extension command specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionCommand {
    pub name: String,
    pub help: String,
    pub aliases: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
}

impl ExtensionManifest {
    /// Load manifest from file
    pub async fn load_from_file(path: &Path) -> Result<Self> {
        let content = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read manifest file: {}", path.display()))?;
        
        let manifest: ExtensionManifest = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse manifest file: {}", path.display()))?;
        
        manifest.validate()?;
        Ok(manifest)
    }
    
    /// Save manifest to file
    pub async fn save_to_file(&self, path: &Path) -> Result<()> {
        let content = serde_yaml::to_string(self)
            .context("Failed to serialize manifest")?;
        
        tokio::fs::write(path, content)
            .await
            .with_context(|| format!("Failed to write manifest file: {}", path.display()))?;
        
        Ok(())
    }
    
    /// Validate manifest contents
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(anyhow::anyhow!("Extension name cannot be empty"));
        }
        
        if self.version.is_empty() {
            return Err(anyhow::anyhow!("Extension version cannot be empty"));
        }
        
        if self.description.is_empty() {
            return Err(anyhow::anyhow!("Extension description cannot be empty"));
        }
        
        if self.commands.is_empty() {
            return Err(anyhow::anyhow!("Extension must define at least one command"));
        }
        
        // Validate commands
        for command in &self.commands {
            command.validate()?;
        }
        
        // Check for duplicate command names
        let mut command_names = std::collections::HashSet::new();
        for command in &self.commands {
            if !command_names.insert(&command.name) {
                return Err(anyhow::anyhow!("Duplicate command name: {}", command.name));
            }
            
            // Check aliases for duplicates
            if let Some(aliases) = &command.aliases {
                for alias in aliases {
                    if !command_names.insert(alias) {
                        return Err(anyhow::anyhow!("Duplicate command name/alias: {}", alias));
                    }
                }
            }
        }
        
        // Validate version format (basic semver check)
        if !is_valid_semver(&self.version) {
            return Err(anyhow::anyhow!("Invalid version format: {}", self.version));
        }
        
        // Validate PM version requirement if specified
        if let Some(pm_version) = &self.pm_version {
            if !is_valid_version_requirement(pm_version) {
                return Err(anyhow::anyhow!("Invalid pm_version requirement: {}", pm_version));
            }
        }
        
        Ok(())
    }
    
    /// Get all command names including aliases
    pub fn get_all_command_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for command in &self.commands {
            names.push(command.name.clone());
            if let Some(aliases) = &command.aliases {
                names.extend(aliases.clone());
            }
        }
        names
    }
    
    /// Find command by name or alias
    pub fn find_command(&self, name: &str) -> Option<&ExtensionCommand> {
        self.commands.iter().find(|cmd| {
            cmd.name == name || 
            cmd.aliases.as_ref().map_or(false, |aliases| aliases.contains(&name.to_string()))
        })
    }
}

impl ExtensionCommand {
    /// Validate command specification
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(anyhow::anyhow!("Command name cannot be empty"));
        }
        
        if self.help.is_empty() {
            return Err(anyhow::anyhow!("Command help cannot be empty"));
        }
        
        // Validate command name format (no spaces, special characters)
        if !self.name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return Err(anyhow::anyhow!("Invalid command name format: {}", self.name));
        }
        
        // Validate aliases
        if let Some(aliases) = &self.aliases {
            for alias in aliases {
                if alias.is_empty() {
                    return Err(anyhow::anyhow!("Command alias cannot be empty"));
                }
                if !alias.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
                    return Err(anyhow::anyhow!("Invalid command alias format: {}", alias));
                }
            }
        }
        
        Ok(())
    }
}

/// Basic semver validation
fn is_valid_semver(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    
    parts.iter().all(|part| {
        part.chars().all(|c| c.is_ascii_digit()) && !part.is_empty()
    })
}

/// Basic version requirement validation
fn is_valid_version_requirement(requirement: &str) -> bool {
    // Simple validation for now - accept patterns like ">=1.0.0", "^1.0.0", "~1.0.0", "1.0.0"
    let requirement = requirement.trim();
    
    if requirement.starts_with(">=") || requirement.starts_with("<=") ||
       requirement.starts_with('>') || requirement.starts_with('<') ||
       requirement.starts_with('^') || requirement.starts_with('~') {
        let version_part = requirement.trim_start_matches(['>', '<', '=', '^', '~']);
        is_valid_semver(version_part.trim())
    } else {
        is_valid_semver(requirement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_semver_validation() {
        assert!(is_valid_semver("1.0.0"));
        assert!(is_valid_semver("10.20.30"));
        assert!(!is_valid_semver("1.0"));
        assert!(!is_valid_semver("1.0.0.0"));
        assert!(!is_valid_semver("v1.0.0"));
        assert!(!is_valid_semver("1.0.0-alpha"));
    }
    
    #[test]
    fn test_version_requirement_validation() {
        assert!(is_valid_version_requirement("1.0.0"));
        assert!(is_valid_version_requirement(">=1.0.0"));
        assert!(is_valid_version_requirement("^1.0.0"));
        assert!(is_valid_version_requirement("~1.0.0"));
        assert!(!is_valid_version_requirement(">="));
        assert!(!is_valid_version_requirement("^"));
    }
}