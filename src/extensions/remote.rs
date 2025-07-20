use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use url::Url;

/// Remote extension registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Registry name
    pub name: String,
    /// Base URL of the registry
    pub url: Url,
    /// Authentication token
    pub token: Option<String>,
    /// Whether this is the default registry
    pub default: bool,
}

/// Extension metadata from remote registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteExtensionMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: ExtensionAuthor,
    pub license: String,
    pub repository: Option<RepositoryInfo>,
    pub pm_version: String,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub dist: DistributionInfo,
    pub dependencies: HashMap<String, String>,
    pub config_schema: Option<serde_json::Value>,
    pub commands: Vec<String>,
    pub hooks: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub downloads: u64,
}

/// Extension author information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionAuthor {
    pub name: String,
    pub email: Option<String>,
    pub url: Option<String>,
}

/// Repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    #[serde(rename = "type")]
    pub repo_type: String,
    pub url: String,
}

/// Distribution information for downloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionInfo {
    /// Download URL for the extension archive
    pub tarball: String,
    /// SHA256 integrity hash
    pub integrity: String,
    /// Size in bytes
    pub size: u64,
}

/// Search result from registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub extensions: Vec<SearchExtension>,
    pub total: u64,
    pub facets: SearchFacets,
}

/// Extension in search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchExtension {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub downloads: u64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Search facets for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFacets {
    pub categories: HashMap<String, u64>,
    pub licenses: HashMap<String, u64>,
    pub authors: HashMap<String, u64>,
}

/// Search parameters
#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    pub query: Option<String>,
    pub category: Option<String>,
    pub author: Option<String>,
    pub keywords: Vec<String>,
    pub sort: Option<String>, // "downloads", "updated", "created", "name"
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Registry client for interacting with remote registries
pub struct RegistryClient {
    client: Client,
    config: RegistryConfig,
}

impl RegistryClient {
    /// Create a new registry client
    pub fn new(config: RegistryConfig) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        
        // Add authentication header if token is provided
        if let Some(token) = &config.token {
            let auth_value = format!("Bearer {}", token);
            if let Ok(header_value) = reqwest::header::HeaderValue::from_str(&auth_value) {
                headers.insert(reqwest::header::AUTHORIZATION, header_value);
            }
        }
        
        // Add user agent
        if let Ok(user_agent) = reqwest::header::HeaderValue::from_str("pm-cli/1.0.0") {
            headers.insert(reqwest::header::USER_AGENT, user_agent);
        }
        
        let client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { client, config }
    }
    
    /// Get extension metadata
    pub async fn get_extension(&self, name: &str) -> Result<RemoteExtensionMetadata> {
        let url = self.config.url
            .join(&format!("api/v1/extensions/{}", name))
            .context("Failed to construct extension URL")?;
        
        let response = self.client
            .get(url)
            .send()
            .await
            .context("Failed to fetch extension metadata")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch extension '{}': HTTP {}",
                name,
                response.status()
            ));
        }
        
        let metadata: RemoteExtensionMetadata = response
            .json()
            .await
            .context("Failed to parse extension metadata")?;
        
        Ok(metadata)
    }
    
    /// Get specific version of extension metadata
    pub async fn get_extension_version(&self, name: &str, version: &str) -> Result<RemoteExtensionMetadata> {
        let url = self.config.url
            .join(&format!("api/v1/extensions/{}/{}", name, version))
            .context("Failed to construct extension version URL")?;
        
        let response = self.client
            .get(url)
            .send()
            .await
            .context("Failed to fetch extension version metadata")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch extension '{}' version '{}': HTTP {}",
                name,
                version,
                response.status()
            ));
        }
        
        let metadata: RemoteExtensionMetadata = response
            .json()
            .await
            .context("Failed to parse extension version metadata")?;
        
        Ok(metadata)
    }
    
    /// Search for extensions
    pub async fn search(&self, params: &SearchParams) -> Result<SearchResult> {
        let mut url = self.config.url
            .join("api/v1/extensions")
            .context("Failed to construct search URL")?;
        
        // Build query parameters
        let mut query_pairs = url.query_pairs_mut();
        
        if let Some(q) = &params.query {
            query_pairs.append_pair("q", q);
        }
        
        if let Some(category) = &params.category {
            query_pairs.append_pair("category", category);
        }
        
        if let Some(author) = &params.author {
            query_pairs.append_pair("author", author);
        }
        
        for keyword in &params.keywords {
            query_pairs.append_pair("keyword", keyword);
        }
        
        if let Some(sort) = &params.sort {
            query_pairs.append_pair("sort", sort);
        }
        
        if let Some(limit) = params.limit {
            query_pairs.append_pair("limit", &limit.to_string());
        }
        
        if let Some(offset) = params.offset {
            query_pairs.append_pair("offset", &offset.to_string());
        }
        
        drop(query_pairs);
        
        let response = self.client
            .get(url)
            .send()
            .await
            .context("Failed to search extensions")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to search extensions: HTTP {}",
                response.status()
            ));
        }
        
        let result: SearchResult = response
            .json()
            .await
            .context("Failed to parse search results")?;
        
        Ok(result)
    }
    
    /// Download extension archive
    pub async fn download_extension(&self, metadata: &RemoteExtensionMetadata, target_path: &PathBuf) -> Result<()> {
        let response = self.client
            .get(&metadata.dist.tarball)
            .send()
            .await
            .context("Failed to download extension archive")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to download extension: HTTP {}",
                response.status()
            ));
        }
        
        // Get the content
        let content = response
            .bytes()
            .await
            .context("Failed to read download content")?;
        
        // Verify size
        if content.len() as u64 != metadata.dist.size {
            return Err(anyhow::anyhow!(
                "Downloaded size mismatch: expected {}, got {}",
                metadata.dist.size,
                content.len()
            ));
        }
        
        // Verify integrity (SHA256)
        let hash = sha256::digest(&content[..]);
        let expected_hash = metadata.dist.integrity
            .strip_prefix("sha256-")
            .unwrap_or(&metadata.dist.integrity);
        
        if hash != expected_hash {
            return Err(anyhow::anyhow!(
                "Integrity check failed: expected {}, got {}",
                expected_hash,
                hash
            ));
        }
        
        // Ensure parent directory exists
        if let Some(parent) = target_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .context("Failed to create download directory")?;
        }
        
        // Write to file
        tokio::fs::write(target_path, &content).await
            .context("Failed to write downloaded file")?;
        
        Ok(())
    }
    
    /// Check registry connectivity
    pub async fn ping(&self) -> Result<bool> {
        let url = self.config.url
            .join("api/v1/")
            .context("Failed to construct ping URL")?;
        
        match self.client.get(url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

/// Registry manager for handling multiple registries
pub struct RegistryManager {
    registries: HashMap<String, RegistryConfig>,
    default_registry: Option<String>,
}

impl RegistryManager {
    /// Create a new registry manager
    pub fn new() -> Self {
        Self {
            registries: HashMap::new(),
            default_registry: None,
        }
    }
    
    /// Load registry configuration from file
    pub async fn load_from_config(config_path: &PathBuf) -> Result<Self> {
        if !config_path.exists() {
            return Ok(Self::new());
        }
        
        let content = tokio::fs::read_to_string(config_path).await
            .context("Failed to read registry config")?;
        
        let config: RegistryManagerConfig = toml::from_str(&content)
            .context("Failed to parse registry config")?;
        
        let mut manager = Self::new();
        
        for (name, registry_config) in config.registries {
            manager.add_registry(name, registry_config);
        }
        
        manager.default_registry = config.default_registry;
        
        Ok(manager)
    }
    
    /// Save registry configuration to file
    pub async fn save_to_config(&self, config_path: &PathBuf) -> Result<()> {
        let config = RegistryManagerConfig {
            registries: self.registries.clone(),
            default_registry: self.default_registry.clone(),
        };
        
        let content = toml::to_string_pretty(&config)
            .context("Failed to serialize registry config")?;
        
        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .context("Failed to create config directory")?;
        }
        
        tokio::fs::write(config_path, content).await
            .context("Failed to write registry config")?;
        
        Ok(())
    }
    
    /// Add a registry
    pub fn add_registry(&mut self, name: String, config: RegistryConfig) {
        let is_default = config.default;
        self.registries.insert(name.clone(), config);
        
        if is_default || self.default_registry.is_none() {
            self.default_registry = Some(name);
        }
    }
    
    /// Remove a registry
    pub fn remove_registry(&mut self, name: &str) -> Option<RegistryConfig> {
        let removed = self.registries.remove(name);
        
        // If we removed the default registry, pick a new default
        if self.default_registry.as_ref() == Some(&name.to_string()) {
            self.default_registry = self.registries.keys().next().cloned();
        }
        
        removed
    }
    
    /// Get a registry client
    pub fn get_client(&self, registry_name: Option<&str>) -> Result<RegistryClient> {
        let name = registry_name
            .or(self.default_registry.as_deref())
            .context("No registry specified and no default registry configured")?;
        
        let config = self.registries.get(name)
            .context(format!("Registry '{}' not found", name))?;
        
        Ok(RegistryClient::new(config.clone()))
    }
    
    /// List all registries
    pub fn list_registries(&self) -> Vec<(&String, &RegistryConfig)> {
        self.registries.iter().collect()
    }
    
    /// Get default registry name
    pub fn get_default_registry(&self) -> Option<&String> {
        self.default_registry.as_ref()
    }
}

/// Configuration structure for registry manager
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegistryManagerConfig {
    registries: HashMap<String, RegistryConfig>,
    default_registry: Option<String>,
}

/// Get the registry configuration file path
pub fn get_registry_config_path() -> Result<PathBuf> {
    let config_dir = crate::config::get_config_dir()?;
    Ok(config_dir.join("registries.toml"))
}

/// Load the default registry manager
pub async fn load_registry_manager() -> Result<RegistryManager> {
    let config_path = get_registry_config_path()?;
    let mut manager = RegistryManager::load_from_config(&config_path).await?;
    
    // Add default PM registry if no registries are configured
    if manager.registries.is_empty() {
        let default_registry = RegistryConfig {
            name: "pm".to_string(),
            url: Url::parse("https://registry.pm.dev/")
                .expect("Invalid default registry URL"),
            token: std::env::var("PM_REGISTRY_TOKEN").ok(),
            default: true,
        };
        
        manager.add_registry("pm".to_string(), default_registry);
    }
    
    Ok(manager)
}

/// Save the registry manager configuration
pub async fn save_registry_manager(manager: &RegistryManager) -> Result<()> {
    let config_path = get_registry_config_path()?;
    manager.save_to_config(&config_path).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_search_params() {
        let params = SearchParams {
            query: Some("test".to_string()),
            category: Some("development".to_string()),
            limit: Some(10),
            ..Default::default()
        };
        
        assert_eq!(params.query, Some("test".to_string()));
        assert_eq!(params.limit, Some(10));
    }
    
    #[tokio::test]
    async fn test_registry_manager() {
        let mut manager = RegistryManager::new();
        
        let config = RegistryConfig {
            name: "test".to_string(),
            url: Url::parse("https://test.example.com/").unwrap(),
            token: None,
            default: true,
        };
        
        manager.add_registry("test".to_string(), config);
        
        assert_eq!(manager.get_default_registry(), Some(&"test".to_string()));
        assert!(manager.get_client(None).is_ok());
        
        let removed = manager.remove_registry("test");
        assert!(removed.is_some());
        assert_eq!(manager.get_default_registry(), None);
    }
    
    #[tokio::test]
    async fn test_registry_config_serialization() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("registries.toml");
        
        let mut manager = RegistryManager::new();
        let config = RegistryConfig {
            name: "test".to_string(),
            url: Url::parse("https://test.example.com/").unwrap(),
            token: Some("secret".to_string()),
            default: true,
        };
        
        manager.add_registry("test".to_string(), config);
        
        // Test saving
        manager.save_to_config(&config_path).await.unwrap();
        assert!(config_path.exists());
        
        // Test loading
        let loaded_manager = RegistryManager::load_from_config(&config_path).await.unwrap();
        assert_eq!(loaded_manager.get_default_registry(), Some(&"test".to_string()));
        
        let registries = loaded_manager.list_registries();
        let (name, loaded_config) = registries.first().unwrap();
        assert_eq!(name, &"test");
        assert_eq!(loaded_config.token, Some("secret".to_string()));
    }
}