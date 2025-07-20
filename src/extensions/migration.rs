use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::extensions::{ExtensionType, get_extensions_dir};

/// Migration support for converting old extension structure to new folder-based structure
pub struct ExtensionMigrator;

/// Legacy extension structure information
#[derive(Debug)]
pub struct LegacyExtensionInfo {
    pub name: String,
    pub directory: PathBuf,
    pub manifest_path: Option<PathBuf>,
    pub binary_file: Option<PathBuf>,
    pub manifest_content: Option<String>,
}

impl ExtensionMigrator {
    /// Scan for extensions that need migration
    pub async fn scan_for_legacy_extensions() -> Result<Vec<LegacyExtensionInfo>> {
        let extensions_dir = get_extensions_dir()?;
        
        if !extensions_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut legacy_extensions = Vec::new();
        let mut entries = fs::read_dir(&extensions_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            
            if entry_path.is_dir() {
                if let Some(extension_name) = entry_path.file_name().and_then(|n| n.to_str()) {
                    if let Some(legacy_info) = Self::analyze_extension_directory(&entry_path, extension_name).await? {
                        legacy_extensions.push(legacy_info);
                    }
                }
            }
        }
        
        Ok(legacy_extensions)
    }
    
    /// Analyze an extension directory to determine if it needs migration
    async fn analyze_extension_directory(ext_dir: &Path, name: &str) -> Result<Option<LegacyExtensionInfo>> {
        let manifest_yml = ext_dir.join("manifest.yml");
        let extension_yml = ext_dir.join("extension.yml");
        let binary_file = ext_dir.join("binary");
        let pm_ext_file = ext_dir.join(format!("pm-ext-{}", name));
        
        // Check if this is a legacy extension (has old manifest or binary structure)
        let has_old_manifest = manifest_yml.exists() && !extension_yml.exists();
        let has_old_binary = binary_file.exists();
        let has_old_script = pm_ext_file.exists();
        let has_new_structure = Self::has_new_folder_structure(ext_dir).await?;
        
        if (has_old_manifest || has_old_binary || has_old_script) && !has_new_structure {
            // This is a legacy extension that needs migration
            let manifest_path = if manifest_yml.exists() {
                Some(manifest_yml.clone())
            } else {
                None
            };
            
            let binary_file_path = if binary_file.exists() {
                Some(binary_file)
            } else if pm_ext_file.exists() {
                Some(pm_ext_file)
            } else {
                None
            };
            
            let manifest_content = if let Some(ref path) = manifest_path {
                Some(fs::read_to_string(path).await.unwrap_or_default())
            } else {
                None
            };
            
            Ok(Some(LegacyExtensionInfo {
                name: name.to_string(),
                directory: ext_dir.to_path_buf(),
                manifest_path,
                binary_file: binary_file_path,
                manifest_content,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Check if an extension directory already has the new folder structure
    async fn has_new_folder_structure(ext_dir: &Path) -> Result<bool> {
        let bash_dir = ext_dir.join("bash");
        let python_dir = ext_dir.join("python");
        let bin_dir = ext_dir.join("bin");
        let extension_yml = ext_dir.join("extension.yml");
        
        Ok(extension_yml.exists() && (bash_dir.exists() || python_dir.exists() || bin_dir.exists()))
    }
    
    /// Migrate a single legacy extension to the new structure
    pub async fn migrate_extension(legacy_info: &LegacyExtensionInfo) -> Result<()> {
        println!("🔄 Migrating extension '{}'...", legacy_info.name);
        
        // Determine extension type from binary file or manifest
        let extension_type = Self::determine_extension_type(legacy_info).await?;
        println!("📋 Detected type: {}", extension_type);
        
        // Create backup of the original directory
        Self::create_backup(&legacy_info.directory).await?;
        
        // Create new folder structure
        Self::create_new_structure(&legacy_info.directory, &extension_type).await?;
        
        // Migrate files based on type
        Self::migrate_files(legacy_info, &extension_type).await?;
        
        // Create or update extension.yml manifest
        Self::create_new_manifest(legacy_info, &extension_type).await?;
        
        // Clean up old files
        Self::cleanup_old_files(legacy_info).await?;
        
        println!("✅ Successfully migrated extension '{}'", legacy_info.name);
        Ok(())
    }
    
    /// Determine the extension type from legacy files
    async fn determine_extension_type(legacy_info: &LegacyExtensionInfo) -> Result<ExtensionType> {
        // Check if there's a binary file
        if let Some(ref binary_path) = legacy_info.binary_file {
            // Check shebang to determine script type (most reliable)
            if binary_path.is_file() {
                if let Ok(content) = fs::read_to_string(binary_path).await {
                    let first_line = content.lines().next().unwrap_or("");
                    if first_line.starts_with("#!/bin/bash") || first_line.starts_with("#!/usr/bin/bash") {
                        return Ok(ExtensionType::Bash);
                    } else if first_line.starts_with("#!/usr/bin/env python") || first_line.starts_with("#!/usr/bin/python") {
                        return Ok(ExtensionType::Python);
                    }
                    
                    // Check if it's a bash script without shebang but with bash syntax
                    if content.contains("case ") && content.contains(";;") {
                        return Ok(ExtensionType::Bash);
                    }
                    
                    // Check if it's a python script without shebang but with python syntax
                    if content.contains("def ") || content.contains("import ") {
                        return Ok(ExtensionType::Python);
                    }
                }
            }
            
            // Check file extension
            if let Some(ext) = binary_path.extension().and_then(|e| e.to_str()) {
                match ext {
                    "sh" => return Ok(ExtensionType::Bash),
                    "py" => return Ok(ExtensionType::Python),
                    _ => {}
                }
            }
            
            // If binary file is named "binary", it's likely a bash script in legacy format
            if binary_path.file_name().and_then(|n| n.to_str()) == Some("binary") {
                return Ok(ExtensionType::Bash);
            }
        }
        
        // Check manifest for type hints
        if let Some(ref manifest_content) = legacy_info.manifest_content {
            if manifest_content.contains("bash") || manifest_content.contains(".sh") {
                return Ok(ExtensionType::Bash);
            } else if manifest_content.contains("python") || manifest_content.contains(".py") {
                return Ok(ExtensionType::Python);
            }
        }
        
        // Default to bash for backward compatibility
        println!("⚠️  Could not determine extension type, defaulting to Bash");
        Ok(ExtensionType::Bash)
    }
    
    /// Create backup of the original directory
    async fn create_backup(ext_dir: &Path) -> Result<()> {
        let backup_dir = ext_dir.with_extension("backup");
        
        if backup_dir.exists() {
            fs::remove_dir_all(&backup_dir).await?;
        }
        
        Self::copy_directory_recursive(ext_dir, &backup_dir).await?;
        println!("💾 Created backup at: {}", backup_dir.display());
        Ok(())
    }
    
    /// Create the new folder structure
    async fn create_new_structure(ext_dir: &Path, extension_type: &ExtensionType) -> Result<()> {
        match extension_type {
            ExtensionType::Bash => {
                fs::create_dir_all(ext_dir.join("bash")).await?;
            }
            ExtensionType::Python => {
                fs::create_dir_all(ext_dir.join("python")).await?;
            }
            ExtensionType::Binary => {
                fs::create_dir_all(ext_dir.join("bin")).await?;
            }
            ExtensionType::Mixed => {
                fs::create_dir_all(ext_dir.join("bash")).await?;
                fs::create_dir_all(ext_dir.join("python")).await?;
                fs::create_dir_all(ext_dir.join("bin")).await?;
            }
        }
        Ok(())
    }
    
    /// Migrate files to the new structure
    async fn migrate_files(legacy_info: &LegacyExtensionInfo, extension_type: &ExtensionType) -> Result<()> {
        if let Some(ref binary_path) = legacy_info.binary_file {
            match extension_type {
                ExtensionType::Bash => {
                    let target_dir = legacy_info.directory.join("bash");
                    
                    // Create a wrapper script that handles the new command structure
                    let wrapper_content = Self::create_legacy_bash_wrapper(binary_path, &legacy_info.name).await?;
                    let target_file = target_dir.join("main.sh");
                    fs::write(&target_file, wrapper_content).await?;
                    Self::make_executable(&target_file).await?;
                    
                    // Create additional scripts
                    Self::create_default_bash_scripts(&target_dir, &legacy_info.name).await?;
                }
                ExtensionType::Python => {
                    let target_dir = legacy_info.directory.join("python");
                    let target_file = target_dir.join("main.py");
                    fs::copy(binary_path, &target_file).await?;
                    Self::make_executable(&target_file).await?;
                    
                    // Create additional scripts
                    Self::create_default_python_scripts(&target_dir, &legacy_info.name).await?;
                }
                ExtensionType::Binary => {
                    let target_dir = legacy_info.directory.join("bin");
                    let target_file = target_dir.join("main");
                    fs::copy(binary_path, &target_file).await?;
                    Self::make_executable(&target_file).await?;
                }
                ExtensionType::Mixed => {
                    // Handle mixed type (shouldn't happen in migration, but handle gracefully)
                    let target_dir = legacy_info.directory.join("bash");
                    let target_file = target_dir.join("main.sh");
                    fs::copy(binary_path, &target_file).await?;
                    Self::make_executable(&target_file).await?;
                }
            }
        }
        Ok(())
    }
    
    /// Create new extension.yml manifest
    async fn create_new_manifest(legacy_info: &LegacyExtensionInfo, extension_type: &ExtensionType) -> Result<()> {
        let manifest_path = legacy_info.directory.join("extension.yml");
        
        // Try to parse existing manifest for metadata
        let (description, author, version) = if let Some(ref content) = legacy_info.manifest_content {
            Self::extract_manifest_metadata(content)
        } else {
            (
                format!("Migrated {} extension", legacy_info.name),
                "Unknown".to_string(),
                "0.1.0".to_string()
            )
        };
        
        let commands = match extension_type {
            ExtensionType::Bash => {
                r#"commands:
  - name: main
    help: Main functionality of the extension
    file: main.sh"#
            }
            ExtensionType::Python => {
                r#"commands:
  - name: main
    help: Main functionality of the extension
    file: main.py"#
            }
            ExtensionType::Binary => {
                r#"commands:
  - name: main
    help: Main functionality of the extension"#
            }
            ExtensionType::Mixed => {
                r#"commands:
  - name: main
    help: Main functionality of the extension
    file: main.sh"#
            }
        };
        
        let manifest_content = format!(
            r#"name: {}
version: {}
description: {}
author: {}
homepage: https://github.com/{}/pm-ext-{}
pm_version: ">=0.1.0"
type: {}
{}
"#,
            legacy_info.name,
            version,
            description,
            author,
            author,
            legacy_info.name,
            extension_type.to_string(),
            commands
        );
        
        fs::write(&manifest_path, manifest_content).await?;
        println!("📄 Created new manifest: extension.yml");
        Ok(())
    }
    
    /// Extract metadata from old manifest content
    fn extract_manifest_metadata(content: &str) -> (String, String, String) {
        let mut description = "Migrated extension".to_string();
        let mut author = "Unknown".to_string();
        let mut version = "0.1.0".to_string();
        
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("description:") {
                if let Some(desc) = line.strip_prefix("description:") {
                    description = desc.trim().trim_matches('"').to_string();
                }
            } else if line.starts_with("author:") {
                if let Some(auth) = line.strip_prefix("author:") {
                    author = auth.trim().trim_matches('"').to_string();
                }
            } else if line.starts_with("version:") {
                if let Some(ver) = line.strip_prefix("version:") {
                    version = ver.trim().trim_matches('"').to_string();
                }
            }
        }
        
        (description, author, version)
    }
    
    /// Clean up old files after migration
    async fn cleanup_old_files(legacy_info: &LegacyExtensionInfo) -> Result<()> {
        // Remove old manifest.yml
        if let Some(ref manifest_path) = legacy_info.manifest_path {
            if manifest_path.exists() {
                fs::remove_file(manifest_path).await?;
                println!("🗑️  Removed old manifest.yml");
            }
        }
        
        // Remove old binary file
        if let Some(ref binary_path) = legacy_info.binary_file {
            if binary_path.exists() {
                fs::remove_file(binary_path).await?;
                println!("🗑️  Removed old binary file");
            }
        }
        
        Ok(())
    }
    
    /// Copy directory recursively
    async fn copy_directory_recursive(source: &Path, target: &Path) -> Result<()> {
        fs::create_dir_all(target).await?;
        
        let mut entries = fs::read_dir(source).await?;
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            let file_name = entry.file_name();
            let target_path = target.join(&file_name);
            
            if entry_path.is_dir() {
                Box::pin(Self::copy_directory_recursive(&entry_path, &target_path)).await?;
            } else {
                fs::copy(&entry_path, &target_path).await?;
            }
        }
        
        Ok(())
    }
    
    /// Make file executable
    async fn make_executable(path: &Path) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path).await?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(path, perms).await?;
        }
        Ok(())
    }
    
    /// Create default bash helper scripts
    async fn create_default_bash_scripts(bash_dir: &Path, extension_name: &str) -> Result<()> {
        // Create example.sh
        let example_content = format!(
            r#"#!/bin/bash

# {extension_name} - Example Command
# Generated during migration

set -e

echo "🎯 This is an example command for the {extension_name} extension"
echo "🔧 You can modify this file to implement additional functionality"
"#,
            extension_name = extension_name.to_uppercase()
        );
        
        let example_path = bash_dir.join("example.sh");
        fs::write(&example_path, example_content).await?;
        Self::make_executable(&example_path).await?;
        
        // Create help.sh
        let help_content = format!(
            r#"#!/bin/bash

# {extension_name} - Help Command
# Generated during migration

echo "Usage: pm {extension_name} [COMMAND] [OPTIONS]"
echo ""
echo "Commands:"
echo "  main       Main functionality (migrated from legacy extension)"
echo "  example    Example command"
echo "  help       Show this help"
echo ""
echo "Extension: {extension_name}"
echo "This extension was migrated from the legacy format."
"#,
            extension_name = extension_name
        );
        
        let help_path = bash_dir.join("help.sh");
        fs::write(&help_path, help_content).await?;
        Self::make_executable(&help_path).await?;
        
        Ok(())
    }
    
    /// Create default python helper scripts
    async fn create_default_python_scripts(python_dir: &Path, extension_name: &str) -> Result<()> {
        // Create example.py
        let example_content = format!(
            r#"#!/usr/bin/env python3

"""
{extension_name} - Example Command
Generated during migration
"""

print("🎯 This is an example command for the {extension_name} extension")
print("🔧 You can modify this file to implement additional functionality")
"#,
            extension_name = extension_name
        );
        
        let example_path = python_dir.join("example.py");
        fs::write(&example_path, example_content).await?;
        Self::make_executable(&example_path).await?;
        
        // Create help.py
        let help_content = format!(
            r#"#!/usr/bin/env python3

"""
{extension_name} - Help Command
Generated during migration
"""

print("Usage: pm {extension_name} [COMMAND] [OPTIONS]")
print("")
print("Commands:")
print("  main       Main functionality (migrated from legacy extension)")
print("  example    Example command")
print("  help       Show this help")
print("")
print("Extension: {extension_name}")
print("This extension was migrated from the legacy format.")
"#,
            extension_name = extension_name
        );
        
        let help_path = python_dir.join("help.py");
        fs::write(&help_path, help_content).await?;
        Self::make_executable(&help_path).await?;
        
        Ok(())
    }
    
    /// Migrate all legacy extensions
    pub async fn migrate_all() -> Result<()> {
        let legacy_extensions = Self::scan_for_legacy_extensions().await?;
        
        if legacy_extensions.is_empty() {
            println!("✅ No legacy extensions found that need migration");
            return Ok(());
        }
        
        println!("🔍 Found {} legacy extension(s) that need migration:", legacy_extensions.len());
        for ext in &legacy_extensions {
            println!("  - {}", ext.name);
        }
        
        println!("");
        
        for legacy_ext in legacy_extensions {
            match Self::migrate_extension(&legacy_ext).await {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("❌ Failed to migrate '{}': {}", legacy_ext.name, e);
                }
            }
        }
        
        println!("🎉 Migration completed!");
        Ok(())
    }
    
    /// Create a wrapper script that adapts legacy script to new command structure
    async fn create_legacy_bash_wrapper(legacy_binary_path: &Path, extension_name: &str) -> Result<String> {
        // Read the original script content
        let legacy_content = fs::read_to_string(legacy_binary_path).await?;
        
        // Create a wrapper that preserves the original functionality
        // but adapts it to the new command structure
        let wrapper_content = format!(
            r#"#!/bin/bash

# {extension_name} - Main Command (Migrated from Legacy Extension)
# This wrapper preserves the original extension functionality
# while adapting it to the new PM extension structure

set -e

# Original extension content below:
# =====================================

{legacy_content}
"#,
            extension_name = extension_name.to_uppercase(),
            legacy_content = legacy_content
        );
        
        Ok(wrapper_content)
    }
}