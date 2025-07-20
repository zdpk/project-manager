#!/usr/bin/env cargo-script

//! A script to manually create a sample Python extension for demonstration
//! 
//! Usage: cd test-extensions && cargo run --bin create_sample_python_extension

use pm::extensions::creation::{ExtensionCreationConfig, ExtensionCreator, ExtensionTemplateType};
use pm::extensions::platform::{Architecture, OperatingSystem, Platform, PlatformSelection};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🐍 Creating sample Python extension...");

    // Clean up any existing test extension
    let target_path = PathBuf::from("./sample-python-deploy");
    if target_path.exists() {
        std::fs::remove_dir_all(&target_path)?;
    }

    // Create configuration for a sample Python extension
    let platforms = PlatformSelection {
        platforms: vec![
            Platform::new(OperatingSystem::Darwin, Architecture::Aarch64),
            Platform::new(OperatingSystem::Linux, Architecture::X86_64),
            Platform::new(OperatingSystem::Linux, Architecture::Aarch64),
            Platform::new(OperatingSystem::Windows, Architecture::X86_64),
        ],
    };

    let config = ExtensionCreationConfig {
        name: "sample-python-deploy".to_string(),
        description: "Deployment automation tool for PM demonstration".to_string(),
        author: "PM Team".to_string(),
        email: Some("pm@example.com".to_string()),
        template_type: ExtensionTemplateType::Python,
        platforms,
        target_directory: target_path.clone(),
        init_git: false, // Skip git init for demo
        create_github_repo: false,
    };

    // Create the extension
    ExtensionCreator::create_extension(config).await?;

    println!("✅ Sample Python extension created at: {}", target_path.display());
    println!("");
    println!("📋 Generated files:");
    
    // List all generated files recursively
    fn list_files(dir: &PathBuf, prefix: String) -> std::io::Result<()> {
        let entries = std::fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            
            if path.is_file() {
                println!("{}• {}", prefix, name);
            } else if path.is_dir() {
                println!("{}📁 {}/", prefix, name);
                list_files(&path, format!("{}  ", prefix))?;
            }
        }
        Ok(())
    }
    
    list_files(&target_path, "  ".to_string())?;

    println!("");
    println!("🔍 To examine the generated Python script:");
    println!("  cat {}/pm-ext-sample-python-deploy", target_path.display());
    println!("");
    println!("📦 To examine the requirements.txt:");
    println!("  cat {}/requirements.txt", target_path.display());
    println!("");
    println!("📖 To see the README:");
    println!("  cat {}/README.md", target_path.display());
    println!("");
    println!("🎯 The script is executable and ready to use!");

    // Check if script is executable
    let script_path = target_path.join("pm-ext-sample-python-deploy");
    if script_path.exists() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = std::fs::metadata(&script_path)?;
            let permissions = metadata.permissions();
            if permissions.mode() & 0o111 != 0 {
                println!("✅ Script is executable (permissions: {:o})", permissions.mode());
            } else {
                println!("❌ Script is not executable");
            }
        }
        
        // Test if Python script has valid syntax
        println!("");
        println!("🧪 Testing Python script syntax...");
        let output = std::process::Command::new("python3")
            .arg("-m")
            .arg("py_compile")
            .arg(&script_path)
            .output();
            
        match output {
            Ok(result) => {
                if result.status.success() {
                    println!("✅ Python syntax check passed");
                } else {
                    println!("❌ Python syntax check failed:");
                    println!("{}", String::from_utf8_lossy(&result.stderr));
                }
            }
            Err(e) => {
                println!("⚠️  Could not run Python syntax check: {}", e);
            }
        }
    }

    Ok(())
}