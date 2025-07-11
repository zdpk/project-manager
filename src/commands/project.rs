use std::path::PathBuf;
use chrono::Utc;
use uuid::Uuid;
use crate::config::{load_config, save_config, Config};
use crate::constants::*;
use crate::display::*;
use crate::error::PmError;
use crate::utils::get_last_git_commit_time;
use crate::validation::{validate_path, parse_time_duration};
use crate::Project;
use anyhow::Result;
use std::collections::HashSet;

pub async fn handle_add(path: &PathBuf, name: &Option<String>, tags: &[String], description: &Option<String>) -> Result<()> {
    let mut config = load_config().await?;

    let resolved_path = if path.is_absolute() {
        path.clone()
    } else {
        config.projects_root_dir.join(path)
    };

    let absolute_path = validate_path(&resolved_path)?;

    // Check for duplicate projects
    if config.projects.values().any(|p| p.path == absolute_path) {
        display_error(ERROR_DUPLICATE_PROJECT, &format!("at path: {}", absolute_path.display()));
        display_info(SUGGESTION_USE_PM_LS);
        return Err(PmError::DuplicateProject.into());
    }

    let project_name = name.clone().unwrap_or_else(|| {
        absolute_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unnamed-project")
            .to_string()
    });

    // Check for duplicate project names
    if config.projects.values().any(|p| p.name == project_name) {
        display_error(ERROR_DUPLICATE_PROJECT, &format!("with name '{}'", project_name));
        display_info(&format!("Use a different name with: pm add {} --name <new-name>", path.display()));
        return Err(PmError::DuplicateProject.into());
    }

    println!("📂 Adding project at: {}", absolute_path.display());

    let git_updated_at = match get_last_git_commit_time(&absolute_path) {
        Ok(time) => time,
        Err(_) => {
            display_warning("Not a Git repository or no commits found");
            None
        }
    };

    let project = Project {
        id: Uuid::new_v4(),
        name: project_name.clone(),
        path: absolute_path,
        tags: tags.to_vec(),
        description: description.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        git_updated_at,
    };

    config.add_project(project);
    save_config(&config).await?;

    display_project_added(&project_name, tags);
    Ok(())
}

pub async fn handle_list(tags: &[String], tags_any: &[String], recent: &Option<String>, limit: &Option<usize>, detailed: bool) -> Result<()> {
    let mut config = load_config().await?;

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
        filtered_project_data.into_iter().take(*limit_count).collect()
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
            display_error(ERROR_PROJECT_NOT_FOUND, &format!("path no longer exists: {}", project_path.display()));
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
        if let Err(e) = save_config(&config).await {
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
                },
                Err(e) => {
                    display_editor_error(&e.to_string());
                }
            }
        }

        Ok(())
    } else {
        display_error(ERROR_PROJECT_NOT_FOUND, &format!("'{}'", name));
        
        let suggestions = suggest_similar_projects(&config, name);
        display_suggestions(&suggestions);
        
        Err(PmError::ProjectNotFound.into())
    }
}

fn suggest_similar_projects(config: &Config, target: &str) -> Vec<String> {
    config.projects.values()
        .map(|p| &p.name)
        .filter(|name| {
            // Simple similarity check - contains substring or starts with same chars
            name.to_lowercase().contains(&target.to_lowercase()) ||
            target.to_lowercase().contains(&name.to_lowercase()) ||
            name.chars().take(3).collect::<String>().to_lowercase() == 
            target.chars().take(3).collect::<String>().to_lowercase()
        })
        .map(|s| s.clone())
        .collect()
}

async fn update_git_times_by_ids(project_ids: &[uuid::Uuid]) {
    use crate::config::load_config;
    use crate::constants::GIT_UPDATE_INTERVAL_HOURS;
    
    for &project_id in project_ids {
        tokio::spawn(async move {
            if let Ok(config) = load_config().await {
                if let Some(project) = config.projects.get(&project_id) {
                    let needs_update = project.git_updated_at.is_none() || 
                                       (Utc::now() - project.git_updated_at.unwrap()).num_hours() >= GIT_UPDATE_INTERVAL_HOURS;
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

fn get_filtered_project_data(config: &Config, tags: &[String], tags_any: &[String], recent: &Option<String>) -> Result<Vec<(Project, Option<chrono::DateTime<chrono::Utc>>, u32)>> {
    let mut project_data: Vec<(Project, Option<chrono::DateTime<chrono::Utc>>, u32)> = config.projects.values()
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
                    },
                    Err(_) => {
                        display_warning(&format!("Invalid time format: {}. Using default of {} days.", recent_str, DEFAULT_RECENT_DAYS));
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
        b.0.git_updated_at.cmp(&a.0.git_updated_at)
            .then_with(|| b.0.updated_at.cmp(&a.0.updated_at))
            .then_with(|| b.0.created_at.cmp(&a.0.created_at))
    });

    Ok(project_data)
}