#!/usr/bin/env cargo-script

//! Quick test to create a new bash extension with new structure

use pm::extensions::creation::{ExtensionCreationConfig, ExtensionCreator, ExtensionTemplateType};
use pm::extensions::platform::{Architecture, OperatingSystem, Platform, PlatformSelection};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🧪 Creating test Bash extension with new structure...");

    let target_path = PathBuf::from("./test-new-bash-structure");
    if target_path.exists() {
        std::fs::remove_dir_all(&target_path)?;
    }

    let platforms = PlatformSelection {
        platforms: vec![
            Platform::new(OperatingSystem::Darwin, Architecture::Aarch64),
            Platform::new(OperatingSystem::Linux, Architecture::X86_64),
        ],
    };

    let config = ExtensionCreationConfig {
        name: "test-hooks".to_string(),
        description: "Git hooks management with new structure".to_string(),
        author: "PM Team".to_string(),
        email: Some("pm@example.com".to_string()),
        template_type: ExtensionTemplateType::Bash,
        platforms,
        target_directory: target_path.clone(),
        init_git: false,
        create_github_repo: false,
    };

    ExtensionCreator::create_extension(config).await?;
    
    println!("✅ Extension created successfully!");
    println!("📁 Location: {}", target_path.display());
    
    // List the generated structure
    println!("\n📋 Generated structure:");
    list_directory(&target_path, 0)?;
    
    Ok(())
}

fn list_directory(path: &PathBuf, indent: usize) -> anyhow::Result<()> {
    let entries = std::fs::read_dir(path)?;
    let indent_str = "  ".repeat(indent);
    
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        if entry.file_type()?.is_dir() {
            println!("{}📁 {}/", indent_str, file_name_str);
            list_directory(&entry.path(), indent + 1)?;
        } else {
            println!("{}📄 {}", indent_str, file_name_str);
        }
    }
    
    Ok(())
}