use crate::extensions::{
    discovery, ensure_extensions_dir,
    get_extension_dir, get_extension_manifest_path, ExtensionManifest, ExtensionType,
    ExtensionMigrator
};
use crate::ExtensionAction;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tokio::fs;

/// Extension manager for handling extension operations
pub struct ExtensionManager;

impl Default for ExtensionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtensionManager {
    /// Create a new extension manager
    pub fn new() -> Self {
        Self
    }
}

/// Handle extension management commands
pub async fn handle_extension_command(action: &ExtensionAction) -> Result<()> {
    match action {
        ExtensionAction::New => {
            crate::extensions::ExtensionCreator::interactive_create().await
        }
        ExtensionAction::Install { name, source, local, version } => {
            handle_install(name, source.as_deref(), *local, version.as_deref()).await
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
        ExtensionAction::Migrate { name, force } => {
            handle_migrate(name.as_deref(), *force).await
        }
    }
}

/// Resolve extension alias to full extension name
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
            let mut available: Vec<String> = extensions.keys().cloned().collect();
            available.sort();
            Err(anyhow::anyhow!(
                "No extension matches '{}'\n💡 Available extensions: {}", 
                input, 
                available.join(", ")
            ))
        },
        1 => {
            // 고유 매칭 - 바로 실행 (확인 메시지 표시)
            println!("🔍 Using '{}' extension", matches[0]);
            Ok(matches[0].clone())
        },
        _ => {
            // 충돌 - 대화형 선택
            interactive_selection(input, matches).await
        }
    }
}

/// Interactive selection when alias matches multiple extensions
async fn interactive_selection(input: &str, matches: Vec<String>) -> Result<String> {
    use std::io::{self, Write};
    
    println!("Multiple extensions match '{}':", input);
    for (i, name) in matches.iter().enumerate() {
        println!("  {}) {}", i + 1, name);
    }
    
    loop {
        print!("Select (1-{}): ", matches.len());
        io::stdout().flush()?;
        
        let mut selection = String::new();
        io::stdin().read_line(&mut selection)?;
        
        if let Ok(choice) = selection.trim().parse::<usize>() {
            if choice >= 1 && choice <= matches.len() {
                let selected = matches[choice - 1].clone();
                println!("🔍 Using '{}' extension", selected);
                return Ok(selected);
            }
        }
        
        println!("Invalid selection. Please enter a number between 1 and {}", matches.len());
    }
}

/// Execute an external extension command
pub async fn execute_extension_command(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(anyhow::anyhow!("No extension command provided"));
    }
    
    // Resolve extension alias to full name
    let extension_name = resolve_extension_alias(&args[0]).await?;
    let command_name = args.get(1).map_or("help", |v| v); // Default to help if no command
    let command_args = if args.len() > 2 { &args[2..] } else { &[] };
    
    // Load extension manifest to understand structure
    let manifest_path = get_extension_manifest_path(&extension_name)?;
    if !manifest_path.exists() {
        return Err(anyhow::anyhow!("Extension '{}' not found", extension_name));
    }
    
    let manifest = ExtensionManifest::load_from_file(&manifest_path).await?;
    
    // Find the specific command
    let command = manifest.find_command(command_name)
        .ok_or_else(|| anyhow::anyhow!("Command '{}' not found in extension '{}'", command_name, extension_name))?;
    
    // Execute based on command type
    execute_command(&extension_name, &manifest, command, command_args).await
}

/// Execute a specific command based on its type
async fn execute_command(
    extension_name: &str, 
    manifest: &ExtensionManifest, 
    command: &crate::extensions::ExtensionCommand, 
    args: &[String]
) -> Result<()> {
    let cmd_type = command.get_effective_type(&manifest.extension_type);
    
    // Prepare environment variables
    let current_project = get_current_project_context().await?;
    let config_path = crate::config::get_config_path()?;
    let extension_dir = get_extension_dir(extension_name)?;
    
    match cmd_type {
        ExtensionType::Bash => {
            execute_bash_command(extension_name, command, args, &current_project, &config_path, &extension_dir).await
        }
        ExtensionType::Python => {
            execute_python_command(extension_name, command, args, &current_project, &config_path, &extension_dir).await
        }
        ExtensionType::Binary => {
            execute_binary_command(extension_name, command, args, &current_project, &config_path, &extension_dir).await
        }
        ExtensionType::Mixed => {
            return Err(anyhow::anyhow!("Mixed type should not be reached in execution"));
        }
    }
}

/// Execute a bash script command
async fn execute_bash_command(
    extension_name: &str,
    command: &crate::extensions::ExtensionCommand,
    args: &[String],
    current_project: &str,
    config_path: &std::path::Path,
    extension_dir: &std::path::Path,
) -> Result<()> {
    let script_file = command.get_file().unwrap_or("main.sh");
    let script_path = crate::extensions::get_bash_scripts_dir(extension_name)?.join(script_file);
    
    if !script_path.exists() {
        return Err(anyhow::anyhow!("Bash script not found: {}", script_path.display()));
    }
    
    let mut cmd = Command::new("bash");
    cmd.arg(&script_path);
    cmd.arg(&command.name);  // Pass command name as first argument
    cmd.args(args);
    cmd.env("PM_CONFIG_PATH", config_path);
    cmd.env("PM_CURRENT_PROJECT", current_project);
    cmd.env("PM_VERSION", env!("CARGO_PKG_VERSION"));
    cmd.env("PM_EXTENSION_DIR", extension_dir);
    cmd.env("PM_EXTENSION_NAME", extension_name);
    cmd.env("PM_COMMAND_NAME", &command.name);
    
    let status = cmd.status()
        .with_context(|| format!("Failed to execute bash script: {}", script_path.display()))?;
    
    if !status.success() {
        let exit_code = status.code().unwrap_or(-1);
        return Err(anyhow::anyhow!("Bash script exited with code {}", exit_code));
    }
    
    Ok(())
}

/// Execute a python script command
async fn execute_python_command(
    extension_name: &str,
    command: &crate::extensions::ExtensionCommand,
    args: &[String],
    current_project: &str,
    config_path: &std::path::Path,
    extension_dir: &std::path::Path,
) -> Result<()> {
    let script_file = command.get_file().unwrap_or("main.py");
    let script_path = crate::extensions::get_python_scripts_dir(extension_name)?.join(script_file);
    
    if !script_path.exists() {
        return Err(anyhow::anyhow!("Python script not found: {}", script_path.display()));
    }
    
    let mut cmd = Command::new("python3");
    cmd.arg(&script_path);
    cmd.arg(&command.name);  // Pass command name as first argument
    cmd.args(args);
    cmd.env("PM_CONFIG_PATH", config_path);
    cmd.env("PM_CURRENT_PROJECT", current_project);
    cmd.env("PM_VERSION", env!("CARGO_PKG_VERSION"));
    cmd.env("PM_EXTENSION_DIR", extension_dir);
    cmd.env("PM_EXTENSION_NAME", extension_name);
    cmd.env("PM_COMMAND_NAME", &command.name);
    
    let status = cmd.status()
        .with_context(|| format!("Failed to execute python script: {}", script_path.display()))?;
    
    if !status.success() {
        let exit_code = status.code().unwrap_or(-1);
        return Err(anyhow::anyhow!("Python script exited with code {}", exit_code));
    }
    
    Ok(())
}

/// Execute a binary command
async fn execute_binary_command(
    extension_name: &str,
    command: &crate::extensions::ExtensionCommand,
    args: &[String],
    current_project: &str,
    config_path: &std::path::Path,
    extension_dir: &std::path::Path,
) -> Result<()> {
    let binary_file = command.get_file().unwrap_or(ExtensionManifest::get_default_binary_file());
    let binary_path = crate::extensions::get_bin_dir(extension_name)?.join(binary_file);
    
    if !binary_path.exists() {
        return Err(anyhow::anyhow!("Binary not found: {}", binary_path.display()));
    }
    
    let mut cmd = Command::new(&binary_path);
    cmd.arg(&command.name); // Pass the command name to the binary
    cmd.args(args);
    cmd.env("PM_CONFIG_PATH", config_path);
    cmd.env("PM_CURRENT_PROJECT", current_project);
    cmd.env("PM_VERSION", env!("CARGO_PKG_VERSION"));
    cmd.env("PM_EXTENSION_DIR", extension_dir);
    cmd.env("PM_EXTENSION_NAME", extension_name);
    cmd.env("PM_COMMAND_NAME", &command.name);
    
    let status = cmd.status()
        .with_context(|| format!("Failed to execute binary: {}", binary_path.display()))?;
    
    if !status.success() {
        let exit_code = status.code().unwrap_or(-1);
        return Err(anyhow::anyhow!("Binary exited with code {}", exit_code));
    }
    
    Ok(())
}

/// Handle extension installation
async fn handle_install(name: &str, source: Option<&str>, local: bool, version: Option<&str>) -> Result<()> {
    // Ensure extensions directory exists
    ensure_extensions_dir().await?;
    
    // Handle local installation
    if local {
        return install_from_local_path(name).await;
    }
    
    println!("🔄 Installing extension '{}'...", name);
    
    // Check if extension is already installed
    if discovery::is_extension_installed(name).await {
        return Err(anyhow::anyhow!("Extension '{}' is already installed. Use --force to reinstall.", name));
    }
    
    // Install based on source type
    if let Some(source) = source {
        install_from_source(name, source, version).await?
    } else {
        install_from_registry(name, version).await?
    }
    
    // Verify installation
    if discovery::is_extension_installed(name).await {
        println!("✅ Extension '{}' installed successfully!", name);
        
        // Show available commands
        if let Ok(commands) = discovery::get_extension_commands(name).await {
            println!("📋 Available commands: {}", commands.join(", "));
        }
    } else {
        return Err(anyhow::anyhow!("Installation verification failed for extension '{}'", name));
    }
    
    Ok(())
}

/// Install extension from local source (directory or archive)
async fn install_from_source(name: &str, source: &str, _version: Option<&str>) -> Result<()> {
    let source_path = Path::new(source);
    
    if !source_path.exists() {
        return Err(anyhow::anyhow!("Source path does not exist: {}", source));
    }
    
    println!("📂 Installing from local source: {}", source);
    
    if source_path.is_dir() {
        // Install from directory
        install_from_directory(name, source_path).await
    } else if source_path.is_file() {
        // Install from archive (tar.gz, zip, etc.)
        install_from_archive(name, source_path).await
    } else {
        Err(anyhow::anyhow!("Invalid source type: {}", source))
    }
}

/// Install extension from registry (GitHub releases, etc.)
async fn install_from_registry(name: &str, version: Option<&str>) -> Result<()> {
    println!("🌐 Installing from registry...");
    
    // TODO: Implement registry logic
    // For now, show what would be done
    let version_str = version.unwrap_or("latest");
    println!("Would download {} version {} from registry", name, version_str);
    
    Err(anyhow::anyhow!("Registry installation not yet implemented. Use --source to install from local directory."))
}

/// Install extension from a local directory
async fn install_from_directory(name: &str, source_dir: &Path) -> Result<()> {
    // Check if source directory has extension.yml
    let source_manifest = source_dir.join("extension.yml");
    if !source_manifest.exists() {
        return Err(anyhow::anyhow!("extension.yml not found in source directory: {}", source_dir.display()));
    }
    
    // Load and validate manifest
    let manifest = ExtensionManifest::load_from_file(&source_manifest).await?;
    
    // Verify name matches
    if manifest.name != name {
        return Err(anyhow::anyhow!(
            "Extension name mismatch: requested '{}' but manifest contains '{}'", 
            name, manifest.name
        ));
    }
    
    // Create target extension directory
    let target_dir = get_extension_dir(name)?;
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).await?;
    }
    fs::create_dir_all(&target_dir).await?;
    
    // Copy extension files based on type
    copy_extension_files(source_dir, &target_dir, &manifest).await?;
    
    println!("✅ Extension files copied successfully");
    Ok(())
}

/// Install extension from an archive file
async fn install_from_archive(name: &str, _archive_path: &Path) -> Result<()> {
    // TODO: Implement archive extraction and installation
    println!("📦 Archive installation not yet implemented for: {}", name);
    Err(anyhow::anyhow!("Archive installation not yet implemented"))
}

/// Copy extension files from source to target directory
async fn copy_extension_files(source_dir: &Path, target_dir: &Path, manifest: &ExtensionManifest) -> Result<()> {
    // Copy manifest file
    let source_manifest = source_dir.join("extension.yml");
    let target_manifest = target_dir.join("extension.yml");
    fs::copy(&source_manifest, &target_manifest).await?;
    
    // Copy type-specific directories
    match manifest.extension_type {
        ExtensionType::Bash => {
            let source_bash = source_dir.join("bash");
            let target_bash = target_dir.join("bash");
            if source_bash.exists() {
                copy_directory(&source_bash, &target_bash).await?;
                // Ensure scripts are executable
                make_scripts_executable(&target_bash).await?;
            }
        }
        ExtensionType::Python => {
            let source_python = source_dir.join("python");
            let target_python = target_dir.join("python");
            if source_python.exists() {
                copy_directory(&source_python, &target_python).await?;
                // Ensure scripts are executable
                make_scripts_executable(&target_python).await?;
            }
        }
        ExtensionType::Binary => {
            let source_bin = source_dir.join("bin");
            let target_bin = target_dir.join("bin");
            if source_bin.exists() {
                copy_directory(&source_bin, &target_bin).await?;
                // Ensure binaries are executable
                make_binaries_executable(&target_bin).await?;
            }
        }
        ExtensionType::Mixed => {
            // Copy all possible directories
            for dir_name in ["bash", "python", "bin"] {
                let source_subdir = source_dir.join(dir_name);
                let target_subdir = target_dir.join(dir_name);
                if source_subdir.exists() {
                    copy_directory(&source_subdir, &target_subdir).await?;
                    if dir_name == "bin" {
                        make_binaries_executable(&target_subdir).await?;
                    } else {
                        make_scripts_executable(&target_subdir).await?;
                    }
                }
            }
        }
    }
    
    // Copy additional files (README, LICENSE, etc.) if they exist
    for file_name in ["README.md", "LICENSE", ".gitignore"] {
        let source_file = source_dir.join(file_name);
        let target_file = target_dir.join(file_name);
        if source_file.exists() {
            fs::copy(&source_file, &target_file).await.ok(); // Ignore errors for optional files
        }
    }
    
    Ok(())
}

/// Copy directory recursively
async fn copy_directory(source: &Path, target: &Path) -> Result<()> {
    fs::create_dir_all(target).await?;
    
    let mut entries = fs::read_dir(source).await?;
    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        let file_name = entry.file_name();
        let target_path = target.join(&file_name);
        
        if entry_path.is_dir() {
            Box::pin(copy_directory(&entry_path, &target_path)).await?;
        } else {
            fs::copy(&entry_path, &target_path).await?;
        }
    }
    
    Ok(())
}

/// Make scripts executable (Unix only)
async fn make_scripts_executable(dir: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        
        let mut entries = fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "sh" || ext == "py" {
                        let mut perms = fs::metadata(&path).await?.permissions();
                        perms.set_mode(0o755);
                        fs::set_permissions(&path, perms).await?;
                    }
                }
            }
        }
    }
    
    #[cfg(windows)]
    {
        // On Windows, executability is determined by file extension
        // No additional permissions needed
    }
    
    Ok(())
}

/// Make binaries executable (Unix only)
async fn make_binaries_executable(dir: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        
        let mut entries = fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                let mut perms = fs::metadata(&path).await?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&path, perms).await?;
            }
        }
    }
    
    #[cfg(windows)]
    {
        // On Windows, executability is determined by file extension
        // No additional permissions needed
    }
    
    Ok(())
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

/// Handle extension migration
async fn handle_migrate(name: Option<&str>, force: bool) -> Result<()> {
    if let Some(extension_name) = name {
        // Migrate specific extension
        handle_migrate_single(extension_name, force).await
    } else {
        // Migrate all legacy extensions
        handle_migrate_all(force).await
    }
}

/// Handle migration of a specific extension
async fn handle_migrate_single(name: &str, force: bool) -> Result<()> {
    println!("🔍 Scanning for legacy extension '{}'...", name);
    
    let legacy_extensions = ExtensionMigrator::scan_for_legacy_extensions().await?;
    
    if let Some(legacy_ext) = legacy_extensions.iter().find(|ext| ext.name == name) {
        if !force {
            println!("⚠️  This will migrate extension '{}' to the new folder structure.", name);
            println!("   A backup will be created at '{}.backup'", legacy_ext.directory.display());
            println!("   Use --force to skip this confirmation.");
            return Ok(());
        }
        
        ExtensionMigrator::migrate_extension(legacy_ext).await?;
    } else {
        // Check if extension already uses new structure
        if discovery::is_extension_installed(name).await {
            println!("✅ Extension '{}' is already using the new structure", name);
        } else {
            return Err(anyhow::anyhow!("Extension '{}' not found", name));
        }
    }
    
    Ok(())
}

/// Handle migration of all legacy extensions
async fn handle_migrate_all(force: bool) -> Result<()> {
    println!("🔍 Scanning for legacy extensions...");
    
    let legacy_extensions = ExtensionMigrator::scan_for_legacy_extensions().await?;
    
    if legacy_extensions.is_empty() {
        println!("✅ No legacy extensions found that need migration");
        return Ok(());
    }
    
    println!("Found {} legacy extension(s):", legacy_extensions.len());
    for ext in &legacy_extensions {
        println!("  - {} ({})", ext.name, ext.directory.display());
    }
    
    if !force {
        println!("");
        println!("⚠️  This will migrate all {} extension(s) to the new folder structure.", legacy_extensions.len());
        println!("   Backups will be created for each extension.");
        println!("   Use --force to skip this confirmation.");
        return Ok(());
    }
    
    println!("");
    ExtensionMigrator::migrate_all().await?;
    
    Ok(())
}

/// Install extension from local path
async fn install_from_local_path(path: &str) -> Result<()> {
    use std::env;
    use std::path::PathBuf;
    
    // Resolve the source path
    let source_path = if path == "." {
        env::current_dir()?
    } else {
        PathBuf::from(shellexpand::tilde(path).into_owned())
    };
    
    if !source_path.exists() {
        return Err(anyhow::anyhow!("Directory '{}' does not exist", source_path.display()));
    }
    
    // Look for extension.yml manifest
    let manifest_path = source_path.join("extension.yml");
    if !manifest_path.exists() {
        return Err(anyhow::anyhow!("extension.yml not found in '{}'", source_path.display()));
    }
    
    // Load and validate manifest
    let manifest = ExtensionManifest::load_from_file(&manifest_path).await?;
    let extension_name = &manifest.name;
    
    println!("🔄 Installing extension '{}' from local path '{}'...", extension_name, source_path.display());
    
    // Check if extension is already installed
    if discovery::is_extension_installed(extension_name).await {
        return Err(anyhow::anyhow!("Extension '{}' is already installed. Uninstall it first.", extension_name));
    }
    
    // Check for command conflicts
    check_command_conflicts(extension_name, &manifest).await?;
    
    // Handle different extension types
    match manifest.extension_type {
        ExtensionType::Bash | ExtensionType::Python => {
            install_interpreted_extension(&source_path, &manifest).await?;
        }
        ExtensionType::Binary => {
            install_rust_extension(&source_path, &manifest).await?;
        }
        ExtensionType::Mixed => {
            return Err(anyhow::anyhow!("Mixed type extensions are not yet supported for local installation"));
        }
    }
    
    // Verify installation
    if discovery::is_extension_installed(extension_name).await {
        println!("✅ Extension '{}' installed successfully!", extension_name);
        
        // Show available commands
        if let Ok(commands) = discovery::get_extension_commands(extension_name).await {
            let binary_name = crate::utils::get_binary_name();
            println!("📋 Available commands:");
            for command in commands {
                println!("  {} {} {}", binary_name, extension_name, command);
            }
        }
    } else {
        return Err(anyhow::anyhow!("Installation verification failed for extension '{}'", extension_name));
    }
    
    Ok(())
}

/// Check for command name conflicts with existing extensions
async fn check_command_conflicts(extension_name: &str, manifest: &ExtensionManifest) -> Result<()> {
    // Get list of installed extensions
    let installed_extensions = discovery::discover_extensions().await?;
    
    for (installed_ext_name, _) in installed_extensions {
        if installed_ext_name == extension_name {
            continue; // Skip self
        }
        
        if let Ok(existing_commands) = discovery::get_extension_commands(&installed_ext_name).await {
            for new_command in &manifest.commands {
                if existing_commands.contains(&new_command.name) {
                    return Err(anyhow::anyhow!(
                        "Command '{}' already exists in extension '{}'. Please choose a different command name.",
                        new_command.name, installed_ext_name
                    ));
                }
            }
        }
    }
    
    Ok(())
}

/// Install interpreted extension (Bash/Python)
async fn install_interpreted_extension(source_path: &Path, manifest: &ExtensionManifest) -> Result<()> {
    let extension_name = &manifest.name;
    let target_dir = get_extension_dir(extension_name)?;
    
    // Create target directory
    tokio::fs::create_dir_all(&target_dir).await?;
    
    // Copy manifest file
    let source_manifest = source_path.join("extension.yml");
    let target_manifest = target_dir.join("extension.yml");
    fs::copy(&source_manifest, &target_manifest).await?;
    
    // Copy extension files based on type
    match manifest.extension_type {
        ExtensionType::Bash => {
            let source_bash = source_path.join("bash");
            let target_bash = target_dir.join("bash");
            if source_bash.exists() {
                copy_directory(&source_bash, &target_bash).await?;
                make_scripts_executable(&target_bash).await?;
            } else {
                return Err(anyhow::anyhow!("bash/ directory not found in extension"));
            }
        }
        ExtensionType::Python => {
            let source_python = source_path.join("python");
            let target_python = target_dir.join("python");
            if source_python.exists() {
                copy_directory(&source_python, &target_python).await?;
                make_scripts_executable(&target_python).await?;
            } else {
                return Err(anyhow::anyhow!("python/ directory not found in extension"));
            }
            
            // Copy requirements.txt if it exists
            let source_requirements = source_path.join("requirements.txt");
            let target_requirements = target_dir.join("requirements.txt");
            if source_requirements.exists() {
                fs::copy(&source_requirements, &target_requirements).await?;
            }
        }
        _ => unreachable!(),
    }
    
    // Copy optional files
    for file_name in ["README.md", "LICENSE", ".gitignore"] {
        let source_file = source_path.join(file_name);
        let target_file = target_dir.join(file_name);
        if source_file.exists() {
            fs::copy(&source_file, &target_file).await.ok();
        }
    }
    
    Ok(())
}

/// Install Rust extension (build from source)
async fn install_rust_extension(source_path: &Path, manifest: &ExtensionManifest) -> Result<()> {
    let extension_name = &manifest.name;
    
    // Check if Cargo.toml exists
    let cargo_toml = source_path.join("Cargo.toml");
    if !cargo_toml.exists() {
        return Err(anyhow::anyhow!("Cargo.toml not found. This doesn't appear to be a Rust project."));
    }
    
    println!("🔨 Building Rust extension...");
    
    // Build the extension
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(source_path)
        .output()
        .context("Failed to execute cargo build")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Cargo build failed:\n{}", stderr));
    }
    
    // Find the built binary
    let built_binary_name = format!("pm-ext-{}", extension_name);
    let source_binary = source_path.join("target/release").join(&built_binary_name);
    
    if !source_binary.exists() {
        return Err(anyhow::anyhow!("Built binary not found at: {}", source_binary.display()));
    }
    
    // Create target directory structure
    let target_dir = get_extension_dir(extension_name)?;
    let target_bin_dir = target_dir.join("bin");
    tokio::fs::create_dir_all(&target_bin_dir).await?;
    
    // Copy manifest file
    let source_manifest = source_path.join("extension.yml");
    let target_manifest = target_dir.join("extension.yml");
    fs::copy(&source_manifest, &target_manifest).await?;
    
    // Copy binary
    let target_binary = target_bin_dir.join(&built_binary_name);
    fs::copy(&source_binary, &target_binary).await?;
    
    // Make binary executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&target_binary).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&target_binary, perms).await?;
    }
    
    // Copy optional files
    for file_name in ["README.md", "LICENSE", ".gitignore"] {
        let source_file = source_path.join(file_name);
        let target_file = target_dir.join(file_name);
        if source_file.exists() {
            fs::copy(&source_file, &target_file).await.ok();
        }
    }
    
    println!("✅ Rust extension built and installed successfully!");
    
    Ok(())
}