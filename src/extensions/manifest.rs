use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fmt;

/// Extension type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionType {
    Bash,
    Python,
    Binary,
    Mixed, // For extensions with multiple types
}

impl fmt::Display for ExtensionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtensionType::Bash => write!(f, "bash"),
            ExtensionType::Python => write!(f, "python"),
            ExtensionType::Binary => write!(f, "binary"),
            ExtensionType::Mixed => write!(f, "mixed"),
        }
    }
}

/// Extension manifest structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub homepage: Option<String>,
    pub pm_version: Option<String>,
    #[serde(rename = "type")]
    pub extension_type: ExtensionType,
    pub commands: Vec<ExtensionCommand>,
}

/// Extension command specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionCommand {
    pub name: String,
    pub help: String,
    pub aliases: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
    /// For mixed-type extensions, specify the command type
    #[serde(rename = "type")]
    pub command_type: Option<ExtensionType>,
    /// File to execute (for bash/python), binary subcommand args (for binary)
    pub file: Option<String>,
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
            command.validate(&self.extension_type)?;
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
            cmd.aliases.as_ref().is_some_and(|aliases| aliases.contains(&name.to_string()))
        })
    }
    
    /// Get the default file name for binary commands
    pub fn get_default_binary_file() -> &'static str {
        "main"
    }
    
    /// Get the directory name for a given extension type
    pub fn get_type_directory(extension_type: &ExtensionType) -> &'static str {
        match extension_type {
            ExtensionType::Bash => "bash",
            ExtensionType::Python => "python",
            ExtensionType::Binary => "bin",
            ExtensionType::Mixed => panic!("Mixed type should not have a single directory"),
        }
    }
}

impl ExtensionCommand {
    /// Validate command specification
    pub fn validate(&self, extension_type: &ExtensionType) -> Result<()> {
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
        
        // Validate command type and file based on extension type
        match extension_type {
            ExtensionType::Mixed => {
                // For mixed extensions, command_type must be specified
                if self.command_type.is_none() {
                    return Err(anyhow::anyhow!("Command type must be specified for mixed extensions"));
                }
                
                // Validate file based on command type
                if let Some(cmd_type) = &self.command_type {
                    self.validate_file_for_type(cmd_type)?;
                }
            }
            other_type => {
                // For single-type extensions, validate file based on extension type
                self.validate_file_for_type(other_type)?;
            }
        }
        
        Ok(())
    }
    
    /// Validate file specification for a given type
    fn validate_file_for_type(&self, cmd_type: &ExtensionType) -> Result<()> {
        match cmd_type {
            ExtensionType::Bash => {
                if let Some(file) = &self.file {
                    if !file.ends_with(".sh") {
                        return Err(anyhow::anyhow!("Bash command file must end with .sh: {}", file));
                    }
                } else {
                    return Err(anyhow::anyhow!("Bash commands must specify a file"));
                }
            }
            ExtensionType::Python => {
                if let Some(file) = &self.file {
                    if !file.ends_with(".py") {
                        return Err(anyhow::anyhow!("Python command file must end with .py: {}", file));
                    }
                } else {
                    return Err(anyhow::anyhow!("Python commands must specify a file"));
                }
            }
            ExtensionType::Binary => {
                // Binary commands don't require a file (defaults to "main")
                // If file is specified, it should be a valid binary name
                if let Some(file) = &self.file {
                    if file.is_empty() {
                        return Err(anyhow::anyhow!("Binary file name cannot be empty"));
                    }
                }
            }
            ExtensionType::Mixed => {
                return Err(anyhow::anyhow!("Mixed type should not be validated directly"));
            }
        }
        Ok(())
    }
    
    /// Get the effective command type for this command
    pub fn get_effective_type<'a>(&'a self, extension_type: &'a ExtensionType) -> &'a ExtensionType {
        match extension_type {
            ExtensionType::Mixed => self.command_type.as_ref().unwrap_or(extension_type),
            other => other,
        }
    }
    
    /// Get the file to execute for this command
    pub fn get_file(&self) -> Option<&str> {
        self.file.as_deref()
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
    
    #[test]
    fn test_extension_type_display() {
        assert_eq!(ExtensionType::Bash.to_string(), "bash");
        assert_eq!(ExtensionType::Python.to_string(), "python");
        assert_eq!(ExtensionType::Binary.to_string(), "binary");
        assert_eq!(ExtensionType::Mixed.to_string(), "mixed");
    }
    
    #[test]
    fn test_extension_type_directory_mapping() {
        assert_eq!(ExtensionManifest::get_type_directory(&ExtensionType::Bash), "bash");
        assert_eq!(ExtensionManifest::get_type_directory(&ExtensionType::Python), "python");
        assert_eq!(ExtensionManifest::get_type_directory(&ExtensionType::Binary), "bin");
    }
    
    #[test]
    fn test_extension_command_file_validation() {
        let mut bash_cmd = ExtensionCommand {
            name: "test".to_string(),
            help: "Test command".to_string(),
            aliases: None,
            args: None,
            command_type: None,
            file: Some("test.sh".to_string()),
        };
        
        // Should pass for bash extension
        assert!(bash_cmd.validate(&ExtensionType::Bash).is_ok());
        
        // Should fail for bash with wrong extension
        bash_cmd.file = Some("test.py".to_string());
        assert!(bash_cmd.validate(&ExtensionType::Bash).is_err());
        
        // Should pass for python with .py extension
        bash_cmd.file = Some("test.py".to_string());
        assert!(bash_cmd.validate(&ExtensionType::Python).is_ok());
    }
    
    #[test]
    fn test_extension_command_effective_type() {
        let bash_cmd = ExtensionCommand {
            name: "test".to_string(),
            help: "Test".to_string(),
            aliases: None,
            args: None,
            command_type: None,
            file: Some("test.sh".to_string()),
        };
        
        // For single-type extension, should return extension type
        assert_eq!(bash_cmd.get_effective_type(&ExtensionType::Bash), &ExtensionType::Bash);
        
        // For mixed extension with command type specified
        let mixed_cmd = ExtensionCommand {
            name: "test".to_string(),
            help: "Test".to_string(),
            aliases: None,
            args: None,
            command_type: Some(ExtensionType::Python),
            file: Some("test.py".to_string()),
        };
        
        assert_eq!(mixed_cmd.get_effective_type(&ExtensionType::Mixed), &ExtensionType::Python);
    }
}