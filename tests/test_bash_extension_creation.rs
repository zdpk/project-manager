use anyhow::Result;
use pm::extensions::creation::{ExtensionCreationConfig, ExtensionCreator, ExtensionTemplateType};
use pm::extensions::platform::{Architecture, OperatingSystem, Platform, PlatformSelection};
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn test_bash_extension_creation() -> Result<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let target_path = temp_dir.path().join("test-hooks");

    // Create test configuration for a Bash extension
    let platforms = PlatformSelection {
        platforms: vec![
            Platform::new(OperatingSystem::Darwin, Architecture::Aarch64),
            Platform::new(OperatingSystem::Linux, Architecture::X86_64),
            Platform::new(OperatingSystem::Linux, Architecture::Aarch64),
        ],
    };

    let config = ExtensionCreationConfig {
        name: "test-hooks".to_string(),
        description: "Test hooks extension for PM".to_string(),
        author: "test-author".to_string(),
        email: Some("test@example.com".to_string()),
        template_type: ExtensionTemplateType::Bash,
        platforms,
        target_directory: target_path.clone(),
        init_git: false, // Skip git init for test
        create_github_repo: false,
    };

    // Create the extension
    ExtensionCreator::create_extension(config).await?;

    // Verify that the target directory was created
    assert!(target_path.exists(), "Extension directory should be created");

    // Test 1: Verify new folder structure is created
    let expected_files = vec![
        "bash/main.sh",            // Main Bash script
        "bash/example.sh",         // Example script
        "bash/help.sh",            // Help script
        "README.md",                // Documentation
        "extension.yml",            // New manifest format
        "LICENSE",                  // License file
        ".gitignore",              // Git ignore
        ".github/workflows/release.yml", // GitHub Actions workflow
    ];

    for file in expected_files {
        let file_path = target_path.join(file);
        assert!(
            file_path.exists(),
            "Expected file '{}' should exist at {}",
            file,
            file_path.display()
        );
    }

    // Test 2: Verify the main Bash script is executable and contains expected content
    let script_path = target_path.join("bash/main.sh");
    let script_content = fs::read_to_string(&script_path).await?;

    // Check script header and metadata
    assert!(script_content.contains("#!/bin/bash"), "Should have bash shebang");
    assert!(script_content.contains("TEST-HOOKS"), "Should contain extension title");
    assert!(script_content.contains("Author: test-author"), "Should contain author");
    assert!(script_content.contains("Generated with PM Extension Template"), "Should contain template attribution");

    // Check script functionality
    assert!(script_content.contains("print_info() {"), "Should have print_info function");
    assert!(script_content.contains("print_success() {"), "Should have print_success function");
    assert!(script_content.contains("print_warning() {"), "Should have print_warning function");
    assert!(script_content.contains("print_error() {"), "Should have print_error function");
    assert!(script_content.contains("test-hooks Extension - Main Command"), "Should contain main command output");

    // Check PM integration
    assert!(script_content.contains("PM_CURRENT_PROJECT"), "Should check PM_CURRENT_PROJECT");
    assert!(script_content.contains("PM_CONFIG_PATH"), "Should check PM_CONFIG_PATH");

    // Check that it's a main command script (not the old command dispatcher)
    assert!(!script_content.contains("case "), "Main script should not have case statement (separate files now)");

    // Test 3: Verify script is executable (Unix systems only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(&script_path).await?;
        let permissions = metadata.permissions();
        assert!(
            permissions.mode() & 0o111 != 0,
            "Script should be executable (permissions: {:o})",
            permissions.mode()
        );
    }

    // Test 4: Verify README.md content
    let readme_path = target_path.join("README.md");
    let readme_content = fs::read_to_string(&readme_path).await?;

    assert!(readme_content.contains("# PM Extension: test-hooks"), "Should have correct title");
    assert!(readme_content.contains("Test hooks extension for PM"), "Should contain description");
    assert!(readme_content.contains("## Installation"), "Should have installation section");
    assert!(readme_content.contains("pm ext install test-hooks"), "Should show install command");
    assert!(readme_content.contains("pm test-hooks example"), "Should show usage example");
    assert!(readme_content.contains("pm ext install test-hooks --source ./"), "Should have local install instruction");

    // Test 5: Verify extension.yml manifest
    let manifest_path = target_path.join("extension.yml");
    let manifest_content = fs::read_to_string(&manifest_path).await?;

    assert!(manifest_content.contains("name: test-hooks"), "Should have correct name");
    assert!(manifest_content.contains("description: Test hooks extension for PM"), "Should have description");
    assert!(manifest_content.contains("author: test-author"), "Should have author");
    assert!(manifest_content.contains("type: bash"), "Should specify bash type");
    assert!(manifest_content.contains("pm_version: \">=0.1.0\""), "Should specify min PM version");

    // Check commands structure
    assert!(manifest_content.contains("commands:"), "Should have commands section");
    assert!(manifest_content.contains("file: main.sh"), "Should specify main.sh file");
    assert!(manifest_content.contains("file: example.sh"), "Should specify example.sh file");
    assert!(manifest_content.contains("file: help.sh"), "Should specify help.sh file");

    // Test 6: Verify GitHub Actions workflow
    let workflow_path = target_path.join(".github/workflows/release.yml");
    let workflow_content = fs::read_to_string(&workflow_path).await?;

    assert!(workflow_content.contains("name: Release Extension"), "Should have workflow name");
    assert!(workflow_content.contains("aarch64-apple-darwin"), "Should include Darwin ARM64 target");
    assert!(workflow_content.contains("x86_64-unknown-linux-gnu"), "Should include Linux x86_64 target");
    assert!(workflow_content.contains("aarch64-unknown-linux-gnu"), "Should include Linux ARM64 target");
    assert!(workflow_content.contains("cargo build --release"), "Should build with cargo");

    // Test 7: Verify LICENSE file
    let license_path = target_path.join("LICENSE");
    let license_content = fs::read_to_string(&license_path).await?;

    assert!(license_content.contains("MIT License"), "Should be MIT license");
    assert!(license_content.contains("test-author"), "Should contain author name");

    // Test 8: Verify .gitignore file
    let gitignore_path = target_path.join(".gitignore");
    let gitignore_content = fs::read_to_string(&gitignore_path).await?;

    assert!(gitignore_content.contains("# Rust"), "Should have Rust section");
    assert!(gitignore_content.contains("/target/"), "Should ignore target directory");
    assert!(gitignore_content.contains(".DS_Store"), "Should ignore macOS files");
    assert!(gitignore_content.contains(".pm/"), "Should ignore PM directory");

    println!("✅ All tests passed! Bash extension template generation working correctly.");

    Ok(())
}

#[tokio::test]
async fn test_bash_script_syntax_validation() -> Result<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let target_path = temp_dir.path().join("syntax-test");

    // Create minimal configuration
    let platforms = PlatformSelection {
        platforms: vec![Platform::new(OperatingSystem::Linux, Architecture::X86_64)],
    };

    let config = ExtensionCreationConfig {
        name: "syntax-test".to_string(),
        description: "Syntax validation test".to_string(),
        author: "test-user".to_string(),
        email: None,
        template_type: ExtensionTemplateType::Bash,
        platforms,
        target_directory: target_path.clone(),
        init_git: false,
        create_github_repo: false,
    };

    // Create the extension
    ExtensionCreator::create_extension(config).await?;

    // Verify script syntax using bash -n (syntax check only)
    // Check all bash scripts syntax on Unix systems
    #[cfg(unix)]
    {
        use std::process::Command;
        
        for script_name in ["main.sh", "example.sh", "help.sh"] {
            let script_path = target_path.join("bash").join(script_name);
            let output = Command::new("bash")
                .arg("-n")  // Check syntax only, don't execute
                .arg(&script_path)
                .output()?;

            assert!(
                output.status.success(),
                "Bash script {} should have valid syntax. Error: {}",
                script_name,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    // Verify the script has proper structure
    let main_script_path = target_path.join("bash/main.sh");
    let script_content = fs::read_to_string(&main_script_path).await?;
    
    // Check for proper function definitions in main script
    let functions = ["print_info", "print_success", "print_warning", "print_error"];
    for func in functions {
        assert!(
            script_content.contains(&format!("{}() {{", func)),
            "Function {} should be properly defined",
            func
        );
    }

    // Check for proper variable usage
    assert!(script_content.contains("set -e"), "Should use set -e for error handling");

    println!("✅ Bash script syntax validation passed!");

    Ok(())
}

#[tokio::test]
async fn test_extension_creation_error_handling() -> Result<()> {
    // Test that creating an extension in an existing directory fails
    let temp_dir = TempDir::new()?;
    let target_path = temp_dir.path().join("existing-dir");
    
    // Create the directory first
    fs::create_dir_all(&target_path).await?;

    let platforms = PlatformSelection {
        platforms: vec![Platform::new(OperatingSystem::Linux, Architecture::X86_64)],
    };

    let config = ExtensionCreationConfig {
        name: "error-test".to_string(),
        description: "Error handling test".to_string(),
        author: "test-user".to_string(),
        email: None,
        template_type: ExtensionTemplateType::Bash,
        platforms,
        target_directory: target_path,
        init_git: false,
        create_github_repo: false,
    };

    // This should fail because directory already exists
    let result = ExtensionCreator::create_extension(config).await;
    assert!(result.is_err(), "Should fail when directory already exists");

    let error_message = result.unwrap_err().to_string();
    assert!(
        error_message.contains("already exists"),
        "Error message should mention directory already exists. Got: {}",
        error_message
    );

    println!("✅ Error handling test passed!");

    Ok(())
}

#[tokio::test]
async fn test_extension_with_different_platforms() -> Result<()> {
    // Test extension creation with all supported platforms
    let temp_dir = TempDir::new()?;
    let target_path = temp_dir.path().join("multi-platform");

    let platforms = PlatformSelection {
        platforms: Platform::all_supported(),
    };

    let config = ExtensionCreationConfig {
        name: "multi-platform".to_string(),
        description: "Multi-platform extension test".to_string(),
        author: "test-author".to_string(),
        email: Some("test@example.com".to_string()),
        template_type: ExtensionTemplateType::Bash,
        platforms,
        target_directory: target_path.clone(),
        init_git: false,
        create_github_repo: false,
    };

    ExtensionCreator::create_extension(config).await?;

    // Verify workflow includes all platforms
    let workflow_path = target_path.join(".github/workflows/release.yml");
    let workflow_content = fs::read_to_string(&workflow_path).await?;

    // Check for all expected platform targets
    let expected_targets = [
        "aarch64-apple-darwin",     // macOS Apple Silicon
        "aarch64-unknown-linux-gnu", // Linux ARM64
        "x86_64-unknown-linux-gnu",  // Linux x86_64
        "aarch64-pc-windows-msvc",   // Windows ARM64
        "x86_64-pc-windows-msvc",    // Windows x86_64
    ];

    for target in expected_targets {
        assert!(
            workflow_content.contains(target),
            "Workflow should include target: {}",
            target
        );
    }

    // Verify README includes all platforms
    let readme_path = target_path.join("README.md");
    let readme_content = fs::read_to_string(&readme_path).await?;

    // For Bash extensions, we use universal compatibility so no platform-specific mentions
    assert!(readme_content.contains("pm ext install multi-platform --source ./"), "README should mention local installation");
    assert!(readme_content.contains("bash/"), "README should mention bash folder structure");

    println!("✅ Multi-platform test passed!");

    Ok(())
}