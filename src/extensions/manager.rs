use crate::extensions::{
    discovery, ensure_extensions_dir, find_extension_binary, 
    get_extension_dir, ExtensionManifest, creation
};
use crate::ExtensionAction;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tokio::fs;

/// Extension manager for handling extension operations
pub struct ExtensionManager;

impl ExtensionManager {
    /// Create a new extension manager
    pub fn new() -> Self {
        Self
    }
}

/// Handle extension management commands
pub async fn handle_extension_command(action: &ExtensionAction) -> Result<()> {
    match action {
        ExtensionAction::Create { name, ext_type, directory, description, author, non_interactive } => {
            creation::create_extension(
                name.clone(),
                *ext_type,
                directory.clone(),
                description.clone(),
                author.clone(),
                *non_interactive,
            ).await
        }
        ExtensionAction::Install { name, source, version, local } => {
            handle_install(name, source.as_deref(), version.as_deref(), *local).await
        }
        ExtensionAction::Uninstall { name, force } => {
            handle_uninstall(name, *force).await
        }
        ExtensionAction::List { all } => {
            handle_list(*all).await
        }
        ExtensionAction::Info { name } => {
            handle_info(name).await
        }
        ExtensionAction::Update { name } => {
            handle_update(name.as_deref()).await
        }
        ExtensionAction::Search { query } => {
            handle_search(query).await
        }
    }
}

/// Execute an external extension command
pub async fn execute_extension_command(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(anyhow::anyhow!("No extension command provided"));
    }
    
    let input_name = &args[0];
    let extension_args = &args[1..];
    
    // Resolve extension alias to actual name
    let extension_name = resolve_extension_alias(input_name).await?;
    
    // Find the extension binary
    let binary_path = find_extension_binary(&extension_name)
        .ok_or_else(|| anyhow::anyhow!("Extension '{}' not found", extension_name))?;
    
    // Prepare environment variables for the extension
    let current_project = get_current_project_context().await?;
    let config_path = crate::config::get_config_path()?;
    
    // Execute the extension
    let mut cmd = Command::new(&binary_path);
    cmd.args(extension_args);
    cmd.env("PM_CONFIG_PATH", config_path);
    cmd.env("PM_CURRENT_PROJECT", current_project);
    cmd.env("PM_VERSION", env!("CARGO_PKG_VERSION"));
    cmd.env("PM_EXTENSION_DIR", get_extension_dir(&extension_name)?);
    
    let status = cmd.status()
        .with_context(|| format!("Failed to execute extension '{}'", extension_name))?;
    
    if !status.success() {
        let exit_code = status.code().unwrap_or(-1);
        return Err(anyhow::anyhow!("Extension '{}' exited with code {}", extension_name, exit_code));
    }
    
    Ok(())
}

/// Handle extension installation
async fn handle_install(name: &str, source: Option<&str>, version: Option<&str>, local: bool) -> Result<()> {
    // Ensure extensions directory exists
    ensure_extensions_dir().await?;
    
    if local {
        // Local installation
        handle_local_install(name).await
    } else {
        // Remote installation (placeholder)
        println!("Installing extension '{}'...", name);
        
        // Check if extension is already installed
        if discovery::is_extension_installed(name).await {
            return Err(anyhow::anyhow!("Extension '{}' is already installed", name));
        }
        
        // TODO: Implement actual remote installation logic
        if let Some(source) = source {
            println!("Would install from source: {}", source);
        } else {
            println!("Would install from default registry");
        }
        
        if let Some(version) = version {
            println!("Would install version: {}", version);
        }
        
        // Create extension directory
        let ext_dir = get_extension_dir(name)?;
        fs::create_dir_all(&ext_dir).await?;
        
        println!("✅ Extension '{}' installation placeholder completed", name);
        println!("📁 Extension directory: {}", ext_dir.display());
        
        Ok(())
    }
}

/// Handle local extension installation
async fn handle_local_install(name_or_path: &str) -> Result<()> {
    use std::path::Path;
    
    // Determine source directory
    let source_dir = if name_or_path == "." {
        std::env::current_dir()?
    } else {
        let path = Path::new(name_or_path);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        }
    };
    
    println!("Installing extension from local directory: {}", source_dir.display());
    
    // Check if source directory exists
    if !source_dir.exists() {
        return Err(anyhow::anyhow!("Source directory not found: {}", source_dir.display()));
    }
    
    // Look for extension.yml in source directory
    let manifest_path = source_dir.join("extension.yml");
    if !manifest_path.exists() {
        return Err(anyhow::anyhow!("extension.yml not found in: {}", source_dir.display()));
    }
    
    // Load and validate manifest
    let manifest = ExtensionManifest::load_from_file(&manifest_path).await?;
    let extension_name = &manifest.name;
    
    println!("Found extension: {} v{}", extension_name, manifest.version);
    
    // Check if extension is already installed
    if discovery::is_extension_installed(extension_name).await {
        return Err(anyhow::anyhow!("Extension '{}' is already installed", extension_name));
    }
    
    // Extension commands use namespaced structure (pm <extension> <command>)
    // so there are no conflicts between different extensions
    
    // Determine extension type and install accordingly
    if let Some(ext_type) = determine_extension_type(&source_dir).await? {
        match ext_type.as_str() {
            "bash" => install_bash_extension(&source_dir, &manifest).await?,
            "python" => install_python_extension(&source_dir, &manifest).await?,
            "rust" => install_rust_extension(&source_dir, &manifest).await?,
            _ => return Err(anyhow::anyhow!("Unsupported extension type: {}", ext_type)),
        }
    } else {
        return Err(anyhow::anyhow!("Could not determine extension type from directory structure"));
    }
    
    println!("✅ Extension '{}' installed successfully", extension_name);
    
    Ok(())
}


/// Determine extension type from directory structure
async fn determine_extension_type(source_dir: &Path) -> Result<Option<String>> {
    if source_dir.join("bash").exists() || source_dir.join("example.sh").exists() {
        Ok(Some("bash".to_string()))
    } else if source_dir.join("python").exists() || source_dir.join("main.py").exists() {
        Ok(Some("python".to_string()))
    } else if source_dir.join("Cargo.toml").exists() {
        Ok(Some("rust".to_string()))
    } else {
        Ok(None)
    }
}

/// Install bash extension
async fn install_bash_extension(source_dir: &Path, manifest: &ExtensionManifest) -> Result<()> {
    let target_dir = get_extension_dir(&manifest.name)?;
    fs::create_dir_all(&target_dir).await?;
    
    // Copy manifest
    let target_manifest = target_dir.join("manifest.yml");
    manifest.save_to_file(&target_manifest).await?;
    
    // Copy bash scripts
    let bash_source = if source_dir.join("bash").exists() {
        source_dir.join("bash")
    } else {
        source_dir.to_path_buf()
    };
    
    let bash_target = target_dir.join("bash");
    fs::create_dir_all(&bash_target).await?;
    
    copy_directory(&bash_source, &bash_target).await?;
    
    // Create executable wrapper
    create_bash_wrapper(&target_dir, &manifest.name).await?;
    
    Ok(())
}

/// Install python extension
async fn install_python_extension(source_dir: &Path, manifest: &ExtensionManifest) -> Result<()> {
    let target_dir = get_extension_dir(&manifest.name)?;
    fs::create_dir_all(&target_dir).await?;
    
    // Copy manifest
    let target_manifest = target_dir.join("manifest.yml");
    manifest.save_to_file(&target_manifest).await?;
    
    // Copy python files
    let python_source = if source_dir.join("python").exists() {
        source_dir.join("python")
    } else {
        source_dir.to_path_buf()
    };
    
    let python_target = target_dir.join("python");
    fs::create_dir_all(&python_target).await?;
    
    copy_directory(&python_source, &python_target).await?;
    
    // Create executable wrapper
    create_python_wrapper(&target_dir, &manifest.name).await?;
    
    Ok(())
}

/// Install rust extension
async fn install_rust_extension(source_dir: &Path, manifest: &ExtensionManifest) -> Result<()> {
    let target_dir = get_extension_dir(&manifest.name)?;
    fs::create_dir_all(&target_dir).await?;
    
    // Copy manifest
    let target_manifest = target_dir.join("manifest.yml");
    manifest.save_to_file(&target_manifest).await?;
    
    // Build the rust extension
    println!("Building Rust extension...");
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .current_dir(source_dir);
    
    let output = cmd.output()
        .with_context(|| "Failed to execute cargo build")?;
    
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to build Rust extension: {}", 
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    // Find the built binary
    let cargo_toml_path = source_dir.join("Cargo.toml");
    let cargo_content: String = fs::read_to_string(cargo_toml_path).await?;
    let binary_name = extract_binary_name_from_cargo_toml(&cargo_content)?;
    
    let source_binary = source_dir.join("target/release").join(&binary_name);
    if !source_binary.exists() {
        return Err(anyhow::anyhow!("Built binary not found: {}", source_binary.display()));
    }
    
    // Copy binary to target
    let target_binary = target_dir.join("binary");
    fs::copy(&source_binary, &target_binary).await?;
    
    // Set executable permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&target_binary).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&target_binary, perms).await?;
    }
    
    Ok(())
}

/// Copy directory recursively
fn copy_directory<'a>(source: &'a Path, target: &'a Path) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
    Box::pin(async move {
        if !source.exists() {
            return Ok(());
        }
        
        fs::create_dir_all(target).await?;
        
        let mut entries = fs::read_dir(source).await?;
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            let file_name = entry.file_name();
            let target_path = target.join(&file_name);
            
            if entry_path.is_dir() {
                copy_directory(&entry_path, &target_path).await?;
            } else {
                fs::copy(&entry_path, &target_path).await?;
                
                // Preserve executable permissions
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let source_perms = fs::metadata(&entry_path).await?.permissions();
                    if source_perms.mode() & 0o111 != 0 {
                        fs::set_permissions(&target_path, source_perms).await?;
                    }
                }
            }
        }
        
        Ok(())
    })
}

/// Create bash wrapper script
async fn create_bash_wrapper(target_dir: &Path, extension_name: &str) -> Result<()> {
    let wrapper_script = format!(r#"#!/bin/bash
# Auto-generated wrapper for {} extension

EXTENSION_DIR="$(dirname "$0")"
BASH_DIR="$EXTENSION_DIR/bash"

# Pass command name as first argument
if [ $# -eq 0 ]; then
    COMMAND="help"
else
    COMMAND="$1"
    shift
fi

# Look for command script
if [ -f "$BASH_DIR/$COMMAND.sh" ]; then
    exec "$BASH_DIR/$COMMAND.sh" "$COMMAND" "$@"
elif [ -f "$BASH_DIR/example.sh" ]; then
    exec "$BASH_DIR/example.sh" "$COMMAND" "$@"
else
    echo "Command '$COMMAND' not found in extension '{}'"
    exit 1
fi
"#, extension_name, extension_name);
    
    let wrapper_path = target_dir.join("binary");
    fs::write(&wrapper_path, wrapper_script).await?;
    
    // Set executable permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&wrapper_path).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&wrapper_path, perms).await?;
    }
    
    Ok(())
}

/// Create python wrapper script
async fn create_python_wrapper(target_dir: &Path, extension_name: &str) -> Result<()> {
    let wrapper_script = format!(r#"#!/bin/bash
# Auto-generated wrapper for {} extension

EXTENSION_DIR="$(dirname "$0")"
PYTHON_DIR="$EXTENSION_DIR/python"

# Pass command name as first argument
if [ $# -eq 0 ]; then
    COMMAND="help"
else
    COMMAND="$1"
    shift
fi

# Look for command script
if [ -f "$PYTHON_DIR/$COMMAND.py" ]; then
    exec python3 "$PYTHON_DIR/$COMMAND.py" "$COMMAND" "$@"
elif [ -f "$PYTHON_DIR/main.py" ]; then
    exec python3 "$PYTHON_DIR/main.py" "$COMMAND" "$@"
else
    echo "Command '$COMMAND' not found in extension '{}'"
    exit 1
fi
"#, extension_name, extension_name);
    
    let wrapper_path = target_dir.join("binary");
    fs::write(&wrapper_path, wrapper_script).await?;
    
    // Set executable permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&wrapper_path).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&wrapper_path, perms).await?;
    }
    
    Ok(())
}

/// Extract binary name from Cargo.toml
fn extract_binary_name_from_cargo_toml(content: &str) -> Result<String> {
    // Simple parsing to find [[bin]] name
    for line in content.lines() {
        if line.trim().starts_with("name =") && line.contains("=") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                let name = parts[1].trim().trim_matches('"').trim_matches('\'');
                if !name.is_empty() {
                    return Ok(name.to_string());
                }
            }
        }
    }
    
    // Fallback: extract from [package] name
    for line in content.lines() {
        if line.trim().starts_with("name =") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                let name = parts[1].trim().trim_matches('"').trim_matches('\'');
                if !name.is_empty() {
                    return Ok(name.to_string());
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("Could not extract binary name from Cargo.toml"))
}

/// Handle extension uninstallation
async fn handle_uninstall(name: &str, force: bool) -> Result<()> {
    if !discovery::is_extension_installed(name).await {
        return Err(anyhow::anyhow!("Extension '{}' is not installed", name));
    }
    
    if !force {
        // TODO: Add confirmation prompt
        println!("⚠️  This will remove extension '{}' and all its files", name);
        println!("Use --force to skip this confirmation");
        return Ok(());
    }
    
    let ext_dir = get_extension_dir(name)?;
    fs::remove_dir_all(&ext_dir).await
        .with_context(|| format!("Failed to remove extension directory: {}", ext_dir.display()))?;
    
    println!("✅ Extension '{}' has been uninstalled", name);
    
    Ok(())
}

/// Handle listing extensions
async fn handle_list(all: bool) -> Result<()> {
    if all {
        println!("📦 Available extensions (from registry):");
        println!("  (Registry listing not yet implemented)");
        println!();
    }
    
    println!("📦 Installed extensions:");
    
    let extensions = discovery::discover_extensions().await?;
    
    if extensions.is_empty() {
        println!("  No extensions installed");
        return Ok(());
    }
    
    for (name, info) in extensions {
        println!("  {:<12} {} - {}", name, info.version, info.description);
        
        // Show commands
        if !info.commands.is_empty() {
            let command_names: Vec<String> = info.commands.iter()
                .map(|cmd| cmd.name.clone())
                .collect();
            println!("               Commands: {}", command_names.join(", "));
        }
    }
    
    Ok(())
}

/// Handle showing extension information
async fn handle_info(name: &str) -> Result<()> {
    let extension_info = discovery::load_extension_info(name).await
        .with_context(|| format!("Failed to load extension '{}' info", name))?;
    
    println!("📦 Extension: {}", extension_info.name);
    println!("Version: {}", extension_info.version);
    println!("Description: {}", extension_info.description);
    
    if let Some(author) = &extension_info.author {
        println!("Author: {}", author);
    }
    
    if let Some(homepage) = &extension_info.homepage {
        println!("Homepage: {}", homepage);
    }
    
    println!("\nCommands:");
    for cmd in &extension_info.commands {
        println!("  {:<12} {}", cmd.name, cmd.help);
        
        if let Some(aliases) = &cmd.aliases {
            if !aliases.is_empty() {
                println!("               Aliases: {}", aliases.join(", "));
            }
        }
        
        if let Some(args) = &cmd.args {
            if !args.is_empty() {
                println!("               Args: {}", args.join(" "));
            }
        }
    }
    
    Ok(())
}

/// Handle extension updates
async fn handle_update(name: Option<&str>) -> Result<()> {
    if let Some(name) = name {
        println!("Updating extension '{}'...", name);
        // TODO: Implement update logic for specific extension
        println!("✅ Extension '{}' update placeholder completed", name);
    } else {
        println!("Updating all extensions...");
        let extensions = discovery::discover_extensions().await?;
        
        for (ext_name, _) in extensions {
            println!("  Updating '{}'...", ext_name);
            // TODO: Implement update logic
        }
        
        println!("✅ All extensions update placeholder completed");
    }
    
    Ok(())
}

/// Handle extension search
async fn handle_search(query: &str) -> Result<()> {
    println!("Searching for extensions matching '{}'...", query);
    
    // TODO: Implement search logic against registry
    println!("  (Search functionality not yet implemented)");
    
    Ok(())
}

/// Resolve extension alias to actual extension name
async fn resolve_extension_alias(input: &str) -> Result<String> {
    let extensions = discovery::discover_extensions().await?;
    
    // 1단계: 정확한 매칭 확인 (최우선)
    if extensions.contains_key(input) {
        return Ok(input.to_string());
    }
    
    // 2단계: Prefix 매칭 (알파벳 순 정렬)
    let mut matches: Vec<String> = extensions.keys()
        .filter(|name| name.starts_with(input))
        .cloned()
        .collect();
    matches.sort();
    
    match matches.len() {
        0 => {
            // 3단계: 사용 가능한 확장들 표시
            let available: Vec<String> = extensions.keys().cloned().collect();
            let mut sorted_available = available;
            sorted_available.sort();
            
            Err(anyhow::anyhow!(
                "Extension '{}' not found. Available extensions: {}", 
                input, 
                sorted_available.join(", ")
            ))
        },
        1 => {
            let extension_name = matches[0].clone();
            println!("🔍 Using '{}' extension", extension_name);
            Ok(extension_name)
        },
        _ => {
            // 여러 매칭: 사용자 선택 요청
            interactive_selection(input, matches).await
        }
    }
}

/// Interactive selection for conflicting aliases
async fn interactive_selection(input: &str, matches: Vec<String>) -> Result<String> {
    println!("⚠️  Multiple extensions match '{}':", input);
    
    for (i, ext_name) in matches.iter().enumerate() {
        println!("  {}. {}", i + 1, ext_name);
    }
    
    // Use first match as default for non-interactive environments
    // In a real implementation, you'd read from stdin
    println!("Automatically selecting: {}", matches[0]);
    Ok(matches[0].clone())
}

/// Get current project context as JSON string
async fn get_current_project_context() -> Result<String> {
    // Try to load current project information
    match crate::config::load_config().await {
        Ok(config) => {
            // Get current directory and find matching project
            if let Ok(current_dir) = std::env::current_dir() {
                for (_, project) in config.projects {
                    if project.path == current_dir {
                        let context = serde_json::json!({
                            "id": project.id,
                            "name": project.name,
                            "path": project.path,
                            "tags": project.tags,
                            "description": project.description,
                            "is_git_repository": project.is_git_repository
                        });
                        return Ok(context.to_string());
                    }
                }
            }
        }
        Err(_) => {
            // If config can't be loaded, provide minimal context
        }
    }
    
    // Fallback to current directory info
    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    
    let context = serde_json::json!({
        "path": current_dir,
        "name": current_dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
    });
    
    Ok(context.to_string())
}