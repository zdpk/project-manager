#!/usr/bin/env cargo-script

//! A script to manually create a sample Rust extension for demonstration
//! 
//! Usage: cd test-extensions && cargo run --bin create_sample_rust_extension

use pm::extensions::creation::{ExtensionCreationConfig, ExtensionCreator, ExtensionTemplateType};
use pm::extensions::platform::{Architecture, OperatingSystem, Platform, PlatformSelection};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🦀 Creating sample Rust extension...");

    // Clean up any existing test extension
    let target_path = PathBuf::from("./sample-rust-monitor");
    if target_path.exists() {
        std::fs::remove_dir_all(&target_path)?;
    }

    // Create configuration for a sample Rust extension
    let platforms = PlatformSelection {
        platforms: vec![
            Platform::new(OperatingSystem::Darwin, Architecture::Aarch64),
            Platform::new(OperatingSystem::Linux, Architecture::X86_64),
            Platform::new(OperatingSystem::Linux, Architecture::Aarch64),
            Platform::new(OperatingSystem::Windows, Architecture::X86_64),
        ],
    };

    let config = ExtensionCreationConfig {
        name: "sample-rust-monitor".to_string(),
        description: "System monitoring tool built in Rust for PM demonstration".to_string(),
        author: "PM Team".to_string(),
        email: Some("pm@example.com".to_string()),
        template_type: ExtensionTemplateType::Rust,
        platforms,
        target_directory: target_path.clone(),
        init_git: false, // Skip git init for demo
        create_github_repo: false,
    };

    // Create the extension
    ExtensionCreator::create_extension(config).await?;

    println!("✅ Sample Rust extension created at: {}", target_path.display());
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
    println!("🔍 To examine the Cargo.toml:");
    println!("  cat {}/Cargo.toml", target_path.display());
    println!("");
    println!("🦀 To examine the main Rust source:");
    println!("  cat {}/src/main.rs", target_path.display());
    println!("");
    println!("📖 To see the README:");
    println!("  cat {}/README.md", target_path.display());
    println!("");
    println!("🔨 Testing Rust project compilation...");

    // Test if Rust project compiles
    let cargo_check = std::process::Command::new("cargo")
        .arg("check")
        .current_dir(&target_path)
        .output();

    match cargo_check {
        Ok(result) => {
            if result.status.success() {
                println!("✅ Rust project compiles successfully");
                println!("🔧 Building release binary...");
                
                // Build the binary
                let cargo_build = std::process::Command::new("cargo")
                    .arg("build")
                    .arg("--release")
                    .current_dir(&target_path)
                    .output();
                    
                match cargo_build {
                    Ok(build_result) => {
                        if build_result.status.success() {
                            println!("✅ Release binary built successfully");
                            
                            // Check if binary exists
                            let binary_path = target_path.join("target/release").join("sample-rust-monitor");
                            if binary_path.exists() {
                                println!("✅ Binary created at: {}", binary_path.display());
                                
                                #[cfg(unix)]
                                {
                                    use std::os::unix::fs::PermissionsExt;
                                    let metadata = std::fs::metadata(&binary_path)?;
                                    let permissions = metadata.permissions();
                                    if permissions.mode() & 0o111 != 0 {
                                        println!("✅ Binary is executable (permissions: {:o})", permissions.mode());
                                    }
                                }
                            } else {
                                println!("❌ Binary not found at expected location");
                            }
                        } else {
                            println!("❌ Build failed:");
                            println!("{}", String::from_utf8_lossy(&build_result.stderr));
                        }
                    }
                    Err(e) => {
                        println!("⚠️  Could not run cargo build: {}", e);
                    }
                }
            } else {
                println!("❌ Rust project compilation failed:");
                println!("{}", String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(e) => {
            println!("⚠️  Could not run cargo check: {}", e);
        }
    }

    println!("");
    println!("🎯 The Rust extension is ready to use!");
    println!("   To test manually: cd {} && cargo run -- --help", target_path.display());

    Ok(())
}