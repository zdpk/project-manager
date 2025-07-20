use pm::extensions::creation::{ExtensionCreationConfig, ExtensionCreator, ExtensionTemplateType};
use pm::extensions::platform::{Architecture, OperatingSystem, Platform, PlatformSelection};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🧪 Quick test of new structure");

    let target_path = PathBuf::from("./quick-test-bash");
    if target_path.exists() {
        std::fs::remove_dir_all(&target_path)?;
    }

    let platforms = PlatformSelection {
        platforms: vec![Platform::new(OperatingSystem::Linux, Architecture::X86_64)],
    };

    let config = ExtensionCreationConfig {
        name: "quick-test".to_string(),
        description: "Quick test extension".to_string(),
        author: "Test".to_string(),
        email: None,
        template_type: ExtensionTemplateType::Bash,
        platforms,
        target_directory: target_path.clone(),
        init_git: false,
        create_github_repo: false,
    };

    ExtensionCreator::create_extension(config).await?;
    
    // Check structure
    println!("📁 Checking structure...");
    if target_path.join("bash").exists() {
        println!("✅ bash/ directory exists");
    } else {
        println!("❌ bash/ directory missing");
    }
    
    if target_path.join("bash/main.sh").exists() {
        println!("✅ bash/main.sh exists");
    } else {
        println!("❌ bash/main.sh missing");
    }
    
    if target_path.join("bash/example.sh").exists() {
        println!("✅ bash/example.sh exists");
    } else {
        println!("❌ bash/example.sh missing");
    }
    
    if target_path.join("extension.yml").exists() {
        println!("✅ extension.yml exists");
        
        // Check manifest content
        let content = std::fs::read_to_string(target_path.join("extension.yml"))?;
        println!("📄 Manifest content preview:");
        let lines: Vec<&str> = content.lines().take(10).collect();
        for line in lines {
            println!("   {}", line);
        }
    } else {
        println!("❌ extension.yml missing");
    }
    
    Ok(())
}