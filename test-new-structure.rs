#!/usr/bin/env cargo-script

//! Test script for the new folder structure system
//! 
//! Usage: cargo run --bin test-new-structure

use pm::extensions::creation::{ExtensionCreationConfig, ExtensionCreator, ExtensionTemplateType};
use pm::extensions::platform::{Architecture, OperatingSystem, Platform, PlatformSelection};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🧪 Testing new folder structure system...");

    // Test Bash extension
    test_bash_extension().await?;
    
    // Test Python extension
    test_python_extension().await?;
    
    // Test Rust extension
    test_rust_extension().await?;
    
    println!("✅ All tests completed successfully!");
    Ok(())
}

async fn test_bash_extension() -> anyhow::Result<()> {
    println!("\n📂 Testing Bash extension with new structure...");
    
    let target_path = PathBuf::from("./test-new-bash");
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
        description: "Git hooks management tool with new structure".to_string(),
        author: "PM Team".to_string(),
        email: Some("pm@example.com".to_string()),
        template_type: ExtensionTemplateType::Bash,
        platforms,
        target_directory: target_path.clone(),
        init_git: false,
        create_github_repo: false,
    };

    ExtensionCreator::create_extension(config).await?;
    
    // Verify new structure
    verify_bash_structure(&target_path)?;
    
    println!("✅ Bash extension structure created and verified");
    Ok(())
}

async fn test_python_extension() -> anyhow::Result<()> {
    println!("\n📂 Testing Python extension with new structure...");
    
    let target_path = PathBuf::from("./test-new-python");
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
        name: "test-deploy".to_string(),
        description: "Deployment tool with new structure".to_string(),
        author: "PM Team".to_string(),
        email: Some("pm@example.com".to_string()),
        template_type: ExtensionTemplateType::Python,
        platforms,
        target_directory: target_path.clone(),
        init_git: false,
        create_github_repo: false,
    };

    ExtensionCreator::create_extension(config).await?;
    
    // Verify new structure
    verify_python_structure(&target_path)?;
    
    println!("✅ Python extension structure created and verified");
    Ok(())
}

async fn test_rust_extension() -> anyhow::Result<()> {
    println!("\n📂 Testing Rust extension with new structure...");
    
    let target_path = PathBuf::from("./test-new-rust");
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
        name: "test-monitor".to_string(),
        description: "System monitor with new structure".to_string(),
        author: "PM Team".to_string(),
        email: Some("pm@example.com".to_string()),
        template_type: ExtensionTemplateType::Rust,
        platforms,
        target_directory: target_path.clone(),
        init_git: false,
        create_github_repo: false,
    };

    ExtensionCreator::create_extension(config).await?;
    
    // Verify new structure
    verify_rust_structure(&target_path)?;
    
    println!("✅ Rust extension structure created and verified");
    Ok(())
}

fn verify_bash_structure(path: &PathBuf) -> anyhow::Result<()> {
    // Check main directories and files
    assert!(path.join("bash").exists(), "bash/ directory should exist");
    assert!(path.join("bash/main.sh").exists(), "bash/main.sh should exist");
    assert!(path.join("bash/example.sh").exists(), "bash/example.sh should exist");
    assert!(path.join("bash/help.sh").exists(), "bash/help.sh should exist");
    assert!(path.join("extension.yml").exists(), "extension.yml should exist");
    assert!(path.join("README.md").exists(), "README.md should exist");
    
    // Check that scripts are executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let main_perms = std::fs::metadata(path.join("bash/main.sh"))?.permissions();
        assert!(main_perms.mode() & 0o111 != 0, "main.sh should be executable");
    }
    
    println!("  ✓ All Bash structure files exist and are properly configured");
    Ok(())
}

fn verify_python_structure(path: &PathBuf) -> anyhow::Result<()> {
    // Check main directories and files
    assert!(path.join("python").exists(), "python/ directory should exist");
    assert!(path.join("python/main.py").exists(), "python/main.py should exist");
    assert!(path.join("python/example.py").exists(), "python/example.py should exist");
    assert!(path.join("python/help.py").exists(), "python/help.py should exist");
    assert!(path.join("extension.yml").exists(), "extension.yml should exist");
    assert!(path.join("requirements.txt").exists(), "requirements.txt should exist");
    assert!(path.join("README.md").exists(), "README.md should exist");
    
    // Check that scripts are executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let main_perms = std::fs::metadata(path.join("python/main.py"))?.permissions();
        assert!(main_perms.mode() & 0o111 != 0, "main.py should be executable");
    }
    
    println!("  ✓ All Python structure files exist and are properly configured");
    Ok(())
}

fn verify_rust_structure(path: &PathBuf) -> anyhow::Result<()> {
    // Check main directories and files
    assert!(path.join("bin").exists(), "bin/ directory should exist");
    assert!(path.join("src").exists(), "src/ directory should exist");
    assert!(path.join("src/main.rs").exists(), "src/main.rs should exist");
    assert!(path.join("Cargo.toml").exists(), "Cargo.toml should exist");
    assert!(path.join("extension.yml").exists(), "extension.yml should exist");
    assert!(path.join("README.md").exists(), "README.md should exist");
    
    println!("  ✓ All Rust structure files exist and are properly configured");
    Ok(())
}