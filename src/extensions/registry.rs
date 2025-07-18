use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Extension registry for tracking installed extensions
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExtensionRegistry {
    /// Map of extension name to installation info
    pub extensions: HashMap<String, ExtensionRegistryEntry>,
}

/// Information about an installed extension
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionRegistryEntry {
    pub name: String,
    pub version: String,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    pub source: Option<String>,
    pub checksum: Option<String>,
}

impl ExtensionRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Load registry from file
    pub async fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }
        
        let content = tokio::fs::read_to_string(path).await?;
        let registry: ExtensionRegistry = serde_yaml::from_str(&content)?;
        Ok(registry)
    }
    
    /// Save registry to file
    pub async fn save_to_file(&self, path: &Path) -> Result<()> {
        let content = serde_yaml::to_string(self)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }
    
    /// Add an extension to the registry
    pub fn add_extension(&mut self, name: String, version: String, source: Option<String>) {
        let entry = ExtensionRegistryEntry {
            name: name.clone(),
            version,
            installed_at: chrono::Utc::now(),
            source,
            checksum: None, // TODO: Calculate and store checksum
        };
        
        self.extensions.insert(name, entry);
    }
    
    /// Remove an extension from the registry
    pub fn remove_extension(&mut self, name: &str) -> Option<ExtensionRegistryEntry> {
        self.extensions.remove(name)
    }
    
    /// Check if an extension is registered
    pub fn is_registered(&self, name: &str) -> bool {
        self.extensions.contains_key(name)
    }
    
    /// Get extension registry entry
    pub fn get_extension(&self, name: &str) -> Option<&ExtensionRegistryEntry> {
        self.extensions.get(name)
    }
    
    /// List all registered extensions
    pub fn list_extensions(&self) -> Vec<&ExtensionRegistryEntry> {
        self.extensions.values().collect()
    }
}

/// Get the registry file path
pub fn get_registry_path() -> Result<std::path::PathBuf> {
    let extensions_dir = crate::extensions::get_extensions_dir()?;
    Ok(extensions_dir.join("registry.yml"))
}

/// Load the extension registry
pub async fn load_registry() -> Result<ExtensionRegistry> {
    let registry_path = get_registry_path()?;
    ExtensionRegistry::load_from_file(&registry_path).await
}

/// Save the extension registry
pub async fn save_registry(registry: &ExtensionRegistry) -> Result<()> {
    let registry_path = get_registry_path()?;
    
    // Ensure parent directory exists
    if let Some(parent) = registry_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    
    registry.save_to_file(&registry_path).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_registry_operations() {
        let mut registry = ExtensionRegistry::new();
        
        // Test adding extension
        registry.add_extension(
            "test-ext".to_string(),
            "1.0.0".to_string(),
            Some("https://example.com/test-ext".to_string())
        );
        
        assert!(registry.is_registered("test-ext"));
        assert!(!registry.is_registered("non-existent"));
        
        // Test getting extension
        let entry = registry.get_extension("test-ext").unwrap();
        assert_eq!(entry.name, "test-ext");
        assert_eq!(entry.version, "1.0.0");
        
        // Test removing extension
        let removed = registry.remove_extension("test-ext");
        assert!(removed.is_some());
        assert!(!registry.is_registered("test-ext"));
    }
    
    #[tokio::test]
    async fn test_registry_file_operations() {
        let temp_dir = tempdir().unwrap();
        let registry_path = temp_dir.path().join("registry.yml");
        
        let mut registry = ExtensionRegistry::new();
        registry.add_extension(
            "test-ext".to_string(),
            "1.0.0".to_string(),
            None
        );
        
        // Test saving
        registry.save_to_file(&registry_path).await.unwrap();
        assert!(registry_path.exists());
        
        // Test loading
        let loaded_registry = ExtensionRegistry::load_from_file(&registry_path).await.unwrap();
        assert!(loaded_registry.is_registered("test-ext"));
    }
}