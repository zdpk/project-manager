#!/usr/bin/env cargo-script

//! A script to manually create a sample Bash extension for demonstration
//! 
//! Usage: cd test-extensions && cargo run --bin create_sample_bash_extension

use pm::extensions::creation::{ExtensionCreationConfig, ExtensionCreator, ExtensionTemplateType};
use pm::extensions::platform::{Architecture, OperatingSystem, Platform, PlatformSelection};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 Creating sample Bash extension...");

    // Clean up any existing test extension
    let target_path = PathBuf::from("./sample-bash-hooks");
    if target_path.exists() {
        std::fs::remove_dir_all(&target_path)?;
    }

    // Create configuration for a sample Bash extension
    let platforms = PlatformSelection {
        platforms: vec![
            Platform::new(OperatingSystem::Darwin, Architecture::Aarch64),
            Platform::new(OperatingSystem::Linux, Architecture::X86_64),
            Platform::new(OperatingSystem::Linux, Architecture::Aarch64),
        ],
    };

    let config = ExtensionCreationConfig {
        name: "sample-bash-hooks".to_string(),
        description: "Sample Git hooks extension for PM demonstration".to_string(),
        author: "PM Team".to_string(),
        email: Some("pm@example.com".to_string()),
        template_type: ExtensionTemplateType::Bash,
        platforms,
        target_directory: target_path.clone(),
        init_git: false, // Skip git init for demo
        create_github_repo: false,
    };

    // Create the extension
    ExtensionCreator::create_extension(config).await?;

    println!("✅ Sample Bash extension created at: {}", target_path.display());
    println!("");
    println!("📋 Generated files:");
    
    // List all generated files
    if let Ok(entries) = std::fs::read_dir(&target_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    println!("  • {}", entry.file_name().to_string_lossy());
                } else if path.is_dir() {
                    println!("  📁 {}/", entry.file_name().to_string_lossy());
                    // List subdirectory contents
                    if let Ok(sub_entries) = std::fs::read_dir(&path) {
                        for sub_entry in sub_entries {
                            if let Ok(sub_entry) = sub_entry {
                                let sub_path = sub_entry.path();
                                let relative_path = sub_path.strip_prefix(&target_path).unwrap();
                                println!("    • {}", relative_path.display());
                            }
                        }
                    }
                }
            }
        }
    }

    println!("");
    println!("🔍 To examine the generated Bash script:");
    println!("  cat {}/pm-ext-sample-bash-hooks", target_path.display());
    println!("");
    println!("📖 To see the README:");
    println!("  cat {}/README.md", target_path.display());
    println!("");
    println!("🎯 The script is executable and ready to use!");

    // Check if script is executable
    let script_path = target_path.join("pm-ext-sample-bash-hooks");
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
    }

    Ok(())
}