use crate::config::{load_config, save_config, Config};
use crate::constants::*;
use crate::display::*;
use crate::error::PmError;
use crate::utils::get_last_git_commit_time;
use crate::validation::{parse_time_duration, validate_path};
use crate::Project;
use anyhow::Result;
use chrono::Utc;
use colored::*;
use git2::Repository;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Confirm, MultiSelect};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use walkdir::WalkDir;

// Type alias for complex project data tuple
type ProjectData = (Project, Option<chrono::DateTime<chrono::Utc>>, u32);

pub async fn handle_add(
    path: &PathBuf,
    name: &Option<String>,
    tags: &[String],
    description: &Option<String>,
) -> Result<()> {
    let mut config = load_config().await?;

    let resolved_path = if path.is_absolute() {
        path.clone()
    } else {
        config.projects_root_dir.join(path)
    };

    // Check if directory exists
    let absolute_path = if !resolved_path.exists() {
        // Directory doesn't exist - prompt user to create it
        match Confirm::new(&format!(
            "Directory '{}' doesn't exist. Create it?",
            resolved_path.display()
        ))
        .with_default(true)
        .prompt()
        {
            Ok(create_dir) => {
                if !create_dir {
                    println!("❌ Directory creation cancelled. Project not added.");
                    return Ok(());
                }
            }
            Err(_) => {
                // Non-interactive environment - create directory automatically
                println!("ℹ️  Non-interactive mode: Creating directory automatically");
            }
        }

        // Create the directory
        fs::create_dir_all(&resolved_path)?;
        println!("✅ Created directory: {}", resolved_path.display());

        // Now validate the created path
        validate_path(&resolved_path)?
    } else {
        // Directory exists - validate it
        validate_path(&resolved_path)?
    };

    // Check for duplicate projects (path-based)
    if config.projects.values().any(|p| p.path == absolute_path) {
        println!(
            "ℹ️  Project already exists at this path: {}",
            absolute_path.display()
        );
        if let Some(existing_project) = config.projects.values().find(|p| p.path == absolute_path) {
            println!("   Project name: '{}'", existing_project.name);
            println!(
                "   Tags: {}",
                if existing_project.tags.is_empty() {
                    "none".to_string()
                } else {
                    existing_project.tags.join(", ")
                }
            );
        }
        println!("💡 Use 'pm project list' to see all projects");
        return Ok(());
    }

    let project_name = name.clone().unwrap_or_else(|| {
        absolute_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unnamed-project")
            .to_string()
    });

    // Check for duplicate project names
    if config.projects.values().any(|p| p.name == project_name) {
        display_error(
            ERROR_DUPLICATE_PROJECT,
            &format!("with name '{}'", project_name),
        );
        display_info(&format!(
            "Use a different name with: pm add {} --name <new-name>",
            path.display()
        ));
        return Err(PmError::DuplicateProject.into());
    }

    println!(
        "📂 Adding project '{}' at: {}",
        project_name,
        absolute_path.display()
    );

    let git_updated_at = match get_last_git_commit_time(&absolute_path) {
        Ok(time) => time,
        Err(_) => {
            println!("ℹ️  Not a Git repository (no .git directory found)");
            None
        }
    };

    let project = Project {
        id: Uuid::new_v4(),
        name: project_name.clone(),
        path: absolute_path.clone(),
        tags: tags.to_vec(),
        description: description.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        git_updated_at,
    };

    config.add_project(project);
    save_config(&config).await?;

    // Success message
    println!("✅ Successfully added project '{}'", project_name);
    if !tags.is_empty() {
        println!("   Tags: {}", tags.join(", "));
    }
    if let Some(desc) = description {
        println!("   Description: {}", desc);
    }
    println!("   Path: {}", absolute_path.display());

    Ok(())
}

pub async fn handle_list(
    tags: &[String],
    tags_any: &[String],
    recent: &Option<String>,
    limit: &Option<usize>,
    detailed: bool,
) -> Result<()> {
    let config = load_config().await?;

    if config.projects.is_empty() {
        display_no_projects();
        return Ok(());
    }

    let project_ids: Vec<uuid::Uuid> = config.projects.keys().cloned().collect();

    // Update git_updated_at for projects in the background
    update_git_times_by_ids(&project_ids).await;

    // Get filtered project data
    let filtered_project_data = get_filtered_project_data(&config, tags, tags_any, recent)?;

    if filtered_project_data.is_empty() {
        display_no_matches();
        return Ok(());
    }

    // Apply limit
    let limited_project_data = if let Some(limit_count) = limit {
        filtered_project_data
            .into_iter()
            .take(*limit_count)
            .collect()
    } else {
        filtered_project_data
    };

    display_project_list_header(limited_project_data.len());

    for (project, last_accessed, access_count) in limited_project_data {
        if detailed {
            display_project_detailed(&project, last_accessed, access_count);
        } else {
            display_project_simple(&project, last_accessed);
        }
    }

    Ok(())
}

pub async fn handle_switch(config: &mut Config, name: &str, no_editor: bool) -> Result<()> {
    if config.projects.is_empty() {
        display_no_projects();
        return Err(PmError::NoProjectsFound.into());
    }

    if let Some(project) = config.find_project_by_name(name) {
        let project_id = project.id;
        let project_name = project.name.clone();
        let project_path = project.path.clone();

        // Check if project path still exists
        if !project_path.exists() {
            display_error(
                ERROR_PROJECT_NOT_FOUND,
                &format!("path no longer exists: {}", project_path.display()),
            );
            println!("\n💡 Suggestions:");
            println!("  - Update the project path");
            println!("  - Remove the project: pm project remove {}", project_name);
            return Err(PmError::ProjectPathNotFound.into());
        }

        // Record access before switching
        config.record_project_access(project_id);

        // Get access info for display
        let (last_accessed, access_count) = config.get_project_access_info(project_id);

        display_switch_info(&project_name, access_count, last_accessed);

        if let Err(e) = std::env::set_current_dir(&project_path) {
            display_error(ERROR_DIRECTORY_CHANGE, &e.to_string());
            println!("   Path: {}", project_path.display());
            return Err(PmError::DirectoryChangeFailed.into());
        }

        // Save config with updated access tracking
        if let Err(e) = save_config(config).await {
            display_warning(&format!("Failed to save access tracking: {}", e));
            // Continue anyway, don't fail the switch operation
        }

        display_switch_success(&project_path, no_editor);

        if !no_editor {
            let mut cmd = std::process::Command::new(DEFAULT_EDITOR);
            match cmd.status() {
                Ok(status) => {
                    if !status.success() {
                        display_warning(&format!("Editor exited with status: {}", status));
                    }
                }
                Err(e) => {
                    display_editor_error(&e.to_string());
                }
            }
        }

        Ok(())
    } else {
        display_error(ERROR_PROJECT_NOT_FOUND, &format!("'{}'", name));

        let suggestions = suggest_similar_projects(config, name);
        display_suggestions(&suggestions);

        Err(PmError::ProjectNotFound.into())
    }
}

fn suggest_similar_projects(config: &Config, target: &str) -> Vec<String> {
    config
        .projects
        .values()
        .map(|p| &p.name)
        .filter(|name| {
            // Simple similarity check - contains substring or starts with same chars
            name.to_lowercase().contains(&target.to_lowercase())
                || target.to_lowercase().contains(&name.to_lowercase())
                || name.chars().take(3).collect::<String>().to_lowercase()
                    == target.chars().take(3).collect::<String>().to_lowercase()
        })
        .cloned()
        .collect()
}

async fn update_git_times_by_ids(project_ids: &[uuid::Uuid]) {
    use crate::config::load_config;
    use crate::constants::GIT_UPDATE_INTERVAL_HOURS;

    for &project_id in project_ids {
        tokio::spawn(async move {
            if let Ok(config) = load_config().await {
                if let Some(project) = config.projects.get(&project_id) {
                    let needs_update = project.git_updated_at.is_none()
                        || (Utc::now() - project.git_updated_at.unwrap()).num_hours()
                            >= GIT_UPDATE_INTERVAL_HOURS;
                    if needs_update {
                        let project_path = project.path.clone();
                        if let Ok(Some(git_time)) = get_last_git_commit_time(&project_path) {
                            if let Ok(mut config) = load_config().await {
                                if let Some(p) = config.projects.get_mut(&project_id) {
                                    p.git_updated_at = Some(git_time);
                                    let _ = save_config(&config).await;
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

fn get_filtered_project_data(
    config: &Config,
    tags: &[String],
    tags_any: &[String],
    recent: &Option<String>,
) -> Result<Vec<ProjectData>> {
    let mut project_data: Vec<ProjectData> = config
        .projects
        .values()
        .filter(|project| {
            // Tags filter (AND logic - all tags must match)
            if !tags.is_empty() {
                let project_tags: HashSet<String> = project.tags.iter().cloned().collect();
                if !tags.iter().all(|tag| project_tags.contains(tag)) {
                    return false;
                }
            }

            // Tags any filter (OR logic - any tag can match)
            if !tags_any.is_empty() {
                let project_tags: HashSet<String> = project.tags.iter().cloned().collect();
                if !tags_any.iter().any(|tag| project_tags.contains(tag)) {
                    return false;
                }
            }

            // Recent filter
            if let Some(recent_str) = recent {
                match parse_time_duration(recent_str) {
                    Ok(duration) => {
                        let cutoff = Utc::now() - duration;
                        let last_activity = project.git_updated_at.unwrap_or(project.updated_at);
                        if last_activity < cutoff {
                            return false;
                        }
                    }
                    Err(_) => {
                        display_warning(&format!(
                            "Invalid time format: {}. Using default of {} days.",
                            recent_str, DEFAULT_RECENT_DAYS
                        ));
                        let cutoff = Utc::now() - chrono::Duration::days(DEFAULT_RECENT_DAYS);
                        let last_activity = project.git_updated_at.unwrap_or(project.updated_at);
                        if last_activity < cutoff {
                            return false;
                        }
                    }
                }
            }

            true
        })
        .cloned()
        .map(|project| {
            let (last_accessed, access_count) = config.get_project_access_info(project.id);
            (project, last_accessed, access_count)
        })
        .collect();

    // Sort projects: git_updated_at (later), updated_at, created_at
    project_data.sort_by(|a, b| {
        b.0.git_updated_at
            .cmp(&a.0.git_updated_at)
            .then_with(|| b.0.updated_at.cmp(&a.0.updated_at))
            .then_with(|| b.0.created_at.cmp(&a.0.created_at))
    });

    Ok(project_data)
}

#[derive(Debug, Clone)]
struct GitRepoInfo {
    path: PathBuf,
    name: String,
    is_git: bool,
    remote_url: Option<String>,
}

pub async fn handle_scan(directory: Option<&Path>, show_all: bool) -> Result<()> {
    let config = load_config().await?;

    // Determine scan directory
    let scan_dir = if let Some(dir) = directory {
        dir.to_path_buf()
    } else {
        // Default to ~/workspace, fallback to home directory
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
        let workspace_dir = home_dir.join("workspace");
        if workspace_dir.exists() {
            workspace_dir
        } else {
            println!("📁 ~/workspace directory not found, using home directory");
            home_dir
        }
    };

    if !scan_dir.exists() {
        return Err(anyhow::anyhow!(
            "Directory does not exist: {}",
            scan_dir.display()
        ));
    }

    println!(
        "🔍 Scanning for Git repositories in: {}",
        scan_dir.display()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Scanning directories...");

    let mut repositories = Vec::new();

    // Walk through directory structure with smart filtering
    for entry in WalkDir::new(&scan_dir)
        .max_depth(3)
        .into_iter()
        .filter_entry(|e| {
            // Skip directories we don't want to traverse into
            if e.file_type().is_dir() {
                !should_skip_directory(e.path())
            } else {
                true // Always process files
            }
        })
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            let path = entry.path();

            // Skip the scan directory itself
            if path == scan_dir {
                continue;
            }

            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unnamed")
                .to_string();

            pb.set_message(format!("Checking: {}", name));

            // Only check directories that pass our project root validation
            if is_project_root(path) {
                let has_git_dir = path.join(".git").exists();
                let remote_url = if has_git_dir {
                    get_git_remote_url(path)
                } else {
                    None
                };

                repositories.push(GitRepoInfo {
                    path: path.to_path_buf(),
                    name,
                    is_git: has_git_dir,
                    remote_url,
                });
            }
        }
    }

    pb.finish_and_clear();

    if repositories.is_empty() {
        println!("❌ No repositories found in {}", scan_dir.display());
        return Ok(());
    }

    // Filter out already tracked projects
    let existing_paths: HashSet<PathBuf> =
        config.projects.values().map(|p| p.path.clone()).collect();

    let new_repos: Vec<GitRepoInfo> = repositories
        .into_iter()
        .filter(|repo| !existing_paths.contains(&repo.path))
        .collect();

    if new_repos.is_empty() {
        println!("✅ All found repositories are already tracked by PM");
        return Ok(());
    }

    println!("📦 Found {} new repositories:", new_repos.len());

    if show_all {
        // Just display all repositories
        for repo in &new_repos {
            println!(
                "  {} {} {}",
                if repo.is_git { "🔗" } else { "📁" },
                repo.name,
                repo.path.display().to_string().bright_black()
            );
            if let Some(url) = &repo.remote_url {
                println!("    🌐 {}", url.bright_black());
            }
        }
        return Ok(());
    }

    // Interactive selection
    let options: Vec<String> = new_repos
        .iter()
        .map(|repo| {
            let prefix = if repo.is_git { "🔗" } else { "📁" };
            format!("{} {} ({})", prefix, repo.name, repo.path.display())
        })
        .collect();

    if options.is_empty() {
        println!("✅ No new repositories to add");
        return Ok(());
    }

    let selection = MultiSelect::new("Select repositories to add to PM:", options).prompt()?;

    if selection.is_empty() {
        println!("❌ No repositories selected");
        return Ok(());
    }

    // Add selected repositories
    let mut config = load_config().await?;
    let mut added_count = 0;

    for selected in selection {
        // Find the repository by matching the display string
        if let Some(repo) = new_repos.iter().find(|r| {
            let expected = format!(
                "{} {} ({})",
                if r.is_git { "🔗" } else { "📁" },
                r.name,
                r.path.display()
            );
            expected == selected
        }) {
            let git_updated_at = if repo.is_git {
                get_last_git_commit_time(&repo.path).ok().flatten()
            } else {
                None
            };

            let project = Project {
                id: Uuid::new_v4(),
                name: repo.name.clone(),
                path: repo.path.clone(),
                tags: vec!["scanned".to_string()],
                description: repo.remote_url.clone(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                git_updated_at,
            };

            config.add_project(project);
            added_count += 1;
            println!("✅ Added: {}", repo.name);
        }
    }

    save_config(&config).await?;
    println!("🎉 Successfully added {} repositories to PM", added_count);

    Ok(())
}

pub async fn handle_load(repo: &str, directory: Option<&Path>) -> Result<()> {
    // Parse owner/repo format
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Repository format should be 'owner/repo'"));
    }

    let owner = parts[0];
    let repo_name = parts[1];

    let config = load_config().await?;

    // Determine target directory
    let target_dir = if let Some(dir) = directory {
        dir.to_path_buf()
    } else {
        // Default: <root_dir>/<owner>/<repo>
        config.projects_root_dir.join(owner).join(repo_name)
    };

    if target_dir.exists() {
        return Err(anyhow::anyhow!(
            "Directory already exists: {}",
            target_dir.display()
        ));
    }

    // Create parent directories if needed
    if let Some(parent) = target_dir.parent() {
        fs::create_dir_all(parent)?;
    }

    let clone_url = format!("https://github.com/{}/{}.git", owner, repo_name);
    println!("📥 Cloning {} to {}", clone_url, target_dir.display());

    // Clone the repository
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Cloning repository...");

    let _repo = Repository::clone(&clone_url, &target_dir)
        .map_err(|e| anyhow::anyhow!("Failed to clone repository: {}", e))?;

    pb.finish_and_clear();

    // Add to PM
    let git_updated_at = get_last_git_commit_time(&target_dir).ok().flatten();

    let project = Project {
        id: Uuid::new_v4(),
        name: repo_name.to_string(),
        path: target_dir.clone(),
        tags: vec!["github".to_string()],
        description: Some(format!("Cloned from {}", clone_url)),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        git_updated_at,
    };

    let mut config = load_config().await?;
    config.add_project(project);
    save_config(&config).await?;

    println!("✅ Successfully cloned and added {} to PM", repo_name);
    println!("📁 Location: {}", target_dir.display());

    Ok(())
}

fn get_git_remote_url(path: &Path) -> Option<String> {
    if let Ok(repo) = Repository::open(path) {
        if let Ok(remote) = repo.find_remote("origin") {
            return remote.url().map(|s| s.to_string());
        }
    }
    None
}

fn should_skip_directory(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return true; // Skip if we can't get the directory name
    };

    // Always skip these directories
    let always_skip = [
        // Git metadata and version control
        ".git",
        ".svn",
        ".hg",
        ".bzr",
        // Dependencies and build artifacts
        "node_modules",
        "vendor",
        "target",
        "build",
        "dist",
        "out",
        // Caches and temporary files
        ".cache",
        ".npm",
        ".yarn",
        ".pnpm",
        "__pycache__",
        ".pytest_cache",
        // IDE and editor directories
        ".vscode",
        ".idea",
        ".vs",
        ".eclipse",
        ".netbeans",
        // System and temporary
        ".DS_Store",
        "tmp",
        "temp",
        ".tmp",
        ".temp",
        // Other common excludes
        "coverage",
        ".nyc_output",
        ".next",
        ".nuxt",
        ".gradle",
    ];

    if always_skip.contains(&name) {
        return true;
    }

    // Skip any hidden directory (starts with .)
    if name.starts_with('.') {
        return true;
    }

    false
}

fn is_project_root(path: &Path) -> bool {
    // Skip if this directory should be excluded
    if should_skip_directory(path) {
        return false;
    }

    // Check if it's a git repository at the root level (has .git subdirectory)
    let has_git_dir = path.join(".git").exists();

    // Check for project files
    let has_project_files = contains_project_files(path);

    // Consider it a project root if it has either git or project files
    has_git_dir || has_project_files
}

fn contains_project_files(path: &Path) -> bool {
    let project_indicators = [
        "package.json",
        "Cargo.toml",
        "pyproject.toml",
        "go.mod",
        "pom.xml",
        "build.gradle",
        "Makefile",
        ".project",
        "composer.json",
        "requirements.txt",
        "setup.py",
        "Gemfile",
        "mix.exs",
        "deno.json",
    ];

    project_indicators
        .iter()
        .any(|&file| path.join(file).exists())
}
