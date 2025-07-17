use crate::config::{get_machine_id, load_config, Config};
use crate::utils::{
    detect_project_language, get_active_git_hooks, get_git_current_branch, get_git_remote_url,
    get_last_git_commit_time, get_pm_hooks_status, has_direnv_config, is_direnv_active,
    is_git_repository,
};
use anyhow::Result;
use serde_json::json;
use std::env;
use std::path::{Path, PathBuf};

pub async fn handle_status(format: &str, quiet: bool) -> Result<()> {
    let current_dir = env::current_dir()?;

    // Try to load config and find project
    let config = match load_config().await {
        Ok(config) => config,
        Err(_) => {
            // No config found, not in a PM project
            if quiet {
                std::process::exit(1);
            } else {
                println!("Not in a PM-managed project");
                return Ok(());
            }
        }
    };

    // Find project by current directory
    let project = find_project_by_current_path(&config, &current_dir);

    if let Some(project) = project {
        // Get additional information
        let git_info = get_git_info(&current_dir).await;
        let direnv_info = get_direnv_info(&current_dir);
        let hooks_info = get_hooks_info(&current_dir);
        let machine_id = get_machine_id();
        let machine_metadata = config.machine_metadata.get(&machine_id);

        match format {
            "json" => {
                let output = json!({
                    "project": {
                        "name": project.name,
                        "tags": project.tags,
                        "path": project.path,
                        "description": project.description,
                        "language": detect_project_language(&project.path).unwrap_or_else(|| "unknown".to_string())
                    },
                    "git": git_info,
                    "direnv": direnv_info,
                    "hooks": hooks_info,
                    "metadata": {
                        "access_count": machine_metadata.map(|m| m.access_counts.get(&project.id).unwrap_or(&0)).unwrap_or(&0),
                        "last_accessed": machine_metadata.and_then(|m| m.last_accessed.get(&project.id))
                    }
                });

                if quiet {
                    // Minimal output for prompt integration
                    let minimal = json!({
                        "name": project.name,
                        "tags": project.tags.join(","),
                        "git_branch": git_info.get("branch").and_then(|v| v.as_str()).unwrap_or(""),
                        "git_changes": git_info.get("has_changes").and_then(|v| v.as_bool()).unwrap_or(false)
                    });
                    println!("{}", minimal);
                } else {
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
            }
            _ => {
                // Text format
                if quiet {
                    // Minimal text output for prompt
                    let tags_str = if project.tags.is_empty() {
                        String::new()
                    } else {
                        format!(" ({})", project.tags.join(", "))
                    };

                    let git_str =
                        if let Some(branch) = git_info.get("branch").and_then(|v| v.as_str()) {
                            if git_info
                                .get("has_changes")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false)
                            {
                                format!(" [{}*]", branch)
                            } else {
                                format!(" [{}]", branch)
                            }
                        } else {
                            String::new()
                        };

                    println!("{}{}{}", project.name, tags_str, git_str);
                } else {
                    // Full text output
                    println!("📋 Project: {}", project.name);

                    if !project.tags.is_empty() {
                        println!("🏷️  Tags: {}", project.tags.join(", "));
                    }

                    if let Some(desc) = &project.description {
                        println!("📝 Description: {}", desc);
                    }

                    println!("📁 Path: {}", project.path.display());

                    // Git information
                    if let Some(branch) = git_info.get("branch").and_then(|v| v.as_str()) {
                        let changes_str = if git_info
                            .get("has_changes")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false)
                        {
                            " (with changes)"
                        } else {
                            ""
                        };
                        println!("🌿 Git: {}{}", branch, changes_str);
                    }

                    // Direnv information
                    if direnv_info
                        .get("has_config")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false)
                    {
                        let status = if direnv_info
                            .get("is_active")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false)
                        {
                            "active"
                        } else {
                            "inactive"
                        };
                        println!("🌍 Direnv: {}", status);
                    }

                    // Hooks information
                    if let Some(pm_hooks_status) =
                        hooks_info.get("pm_hooks_status").and_then(|v| v.as_str())
                    {
                        if pm_hooks_status != "No PM hooks template" {
                            println!("🔧 PM Hooks: {}", pm_hooks_status);
                        }
                    }

                    if let Some(active_hooks) =
                        hooks_info.get("active_hooks").and_then(|v| v.as_array())
                    {
                        if !active_hooks.is_empty() {
                            let hooks_list: Vec<String> = active_hooks
                                .iter()
                                .filter_map(|v| v.as_str())
                                .map(|s| s.to_string())
                                .collect();
                            println!("🪝 Active Hooks: {}", hooks_list.join(", "));
                        }
                    }

                    // Access information
                    if let Some(metadata) = machine_metadata {
                        if let Some(count) = metadata.access_counts.get(&project.id) {
                            println!("📊 Access count: {}", count);
                        }
                        if let Some(last_accessed) = metadata.last_accessed.get(&project.id) {
                            println!(
                                "🕒 Last accessed: {}",
                                last_accessed.format("%Y-%m-%d %H:%M:%S")
                            );
                        }
                    }
                }
            }
        }
    } else {
        // Not in a PM project
        if quiet {
            std::process::exit(1);
        } else {
            println!("Current directory is not a PM-managed project");
            println!("💡 Use 'pm add .' to add this directory as a project");
        }
    }

    Ok(())
}

fn find_project_by_current_path<'a>(
    config: &'a Config,
    current_dir: &PathBuf,
) -> Option<&'a crate::Project> {
    // Try exact match first
    if let Some(project) = config.projects.values().find(|p| p.path == *current_dir) {
        return Some(project);
    }

    // Try to find parent project (in case we're in a subdirectory)
    config
        .projects
        .values()
        .find(|&project| current_dir.starts_with(&project.path))
}

async fn get_git_info(path: &PathBuf) -> serde_json::Value {
    let mut git_info = serde_json::Map::new();

    git_info.insert("is_repository".to_string(), json!(is_git_repository(path)));

    if is_git_repository(path) {
        if let Ok(Some(branch)) = get_git_current_branch(path) {
            git_info.insert("branch".to_string(), json!(branch));
        }

        if let Ok(Some(remote_url)) = get_git_remote_url(path) {
            git_info.insert("remote_url".to_string(), json!(remote_url));
        }

        if let Ok(Some(last_commit)) = get_last_git_commit_time(path) {
            git_info.insert("last_commit".to_string(), json!(last_commit));
        }

        // Check for uncommitted changes
        git_info.insert("has_changes".to_string(), json!(has_git_changes(path)));
    }

    json!(git_info)
}

fn has_git_changes(path: &PathBuf) -> bool {
    use std::process::Command;

    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(path)
        .output();

    if let Ok(output) = output {
        !output.stdout.is_empty()
    } else {
        false
    }
}

fn get_direnv_info(path: &Path) -> serde_json::Value {
    json!({
        "has_config": has_direnv_config(path),
        "is_active": is_direnv_active(path)
    })
}

fn get_hooks_info(path: &Path) -> serde_json::Value {
    let active_hooks = get_active_git_hooks(path);
    let pm_hooks_status = get_pm_hooks_status(path);

    json!({
        "active_hooks": active_hooks,
        "pm_hooks_status": pm_hooks_status
    })
}
