use pm::extensions::creation::{ExtensionCreationConfig, ExtensionCreator, ExtensionTemplateType};
use pm::extensions::platform::{Architecture, OperatingSystem, Platform, PlatformSelection};
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_bash_extension_new_structure() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let target_path = temp_dir.path().join("test-bash");

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

    ExtensionCreator::create_extension(config).await.expect("Failed to create extension");
    
    // Verify new structure
    verify_bash_structure(&target_path);
}

#[tokio::test]
async fn test_python_extension_new_structure() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let target_path = temp_dir.path().join("test-python");

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

    ExtensionCreator::create_extension(config).await.expect("Failed to create extension");
    
    // Verify new structure
    verify_python_structure(&target_path);
}

#[tokio::test]
async fn test_rust_extension_new_structure() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let target_path = temp_dir.path().join("test-rust");

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

    ExtensionCreator::create_extension(config).await.expect("Failed to create extension");
    
    // Verify new structure
    verify_rust_structure(&target_path);
}

fn verify_bash_structure(path: &PathBuf) {
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
        let main_perms = std::fs::metadata(path.join("bash/main.sh")).unwrap().permissions();
        assert!(main_perms.mode() & 0o111 != 0, "main.sh should be executable");
    }
}

fn verify_python_structure(path: &PathBuf) {
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
        let main_perms = std::fs::metadata(path.join("python/main.py")).unwrap().permissions();
        assert!(main_perms.mode() & 0o111 != 0, "main.py should be executable");
    }
}

fn verify_rust_structure(path: &PathBuf) {
    // Check main directories and files
    assert!(path.join("bin").exists(), "bin/ directory should exist");
    assert!(path.join("src").exists(), "src/ directory should exist");
    assert!(path.join("src/main.rs").exists(), "src/main.rs should exist");
    assert!(path.join("Cargo.toml").exists(), "Cargo.toml should exist");
    assert!(path.join("extension.yml").exists(), "extension.yml should exist");
    assert!(path.join("README.md").exists(), "README.md should exist");
}

#[tokio::test]
async fn test_extension_manifest_structure() {
    use pm::extensions::ExtensionManifest;
    
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let target_path = temp_dir.path().join("test-manifest");

    let platforms = PlatformSelection {
        platforms: vec![Platform::new(OperatingSystem::Linux, Architecture::X86_64)],
    };

    let config = ExtensionCreationConfig {
        name: "test-manifest".to_string(),
        description: "Test manifest structure".to_string(),
        author: "PM Team".to_string(),
        email: None,
        template_type: ExtensionTemplateType::Bash,
        platforms,
        target_directory: target_path.clone(),
        init_git: false,
        create_github_repo: false,
    };

    ExtensionCreator::create_extension(config).await.expect("Failed to create extension");
    
    // Verify manifest can be loaded and has correct structure
    let manifest_path = target_path.join("extension.yml");
    let manifest = ExtensionManifest::load_from_file(&manifest_path).await.expect("Failed to load manifest");
    
    assert_eq!(manifest.name, "test-manifest");
    assert_eq!(manifest.extension_type, pm::extensions::ExtensionType::Bash);
    assert!(!manifest.commands.is_empty());
    
    // Check that commands have the expected structure
    let example_cmd = manifest.find_command("example").expect("Should have example command");
    assert_eq!(example_cmd.get_file(), Some("example.sh"));
    
    let help_cmd = manifest.find_command("help").expect("Should have help command");
    assert_eq!(help_cmd.get_file(), Some("help.sh"));
}