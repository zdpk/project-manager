use chrono::{DateTime, Utc, Duration};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

mod config;
mod utils;
mod tag_commands;

use config::{get_config_path, load_config, save_config, Config};
use utils::get_last_git_commit_time;
use tag_commands::{add_tags, remove_tags, list_tags, show_tags};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage projects (add, list, switch)
    #[command(subcommand, alias = "p")]
    Project(ProjectCommands),

    /// Manage project tags
    Tag {
        #[command(subcommand)]
        action: TagAction,
    },
    /// Initialize the pm tool
    Init {},
    
    /// Add a new project to manage
    Add {
        /// Path to the project directory
        path: PathBuf,

        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,

        #[arg(short, long)]
        description: Option<String>,
    },
    
    /// List managed projects  
    #[command(alias = "ls")]
    List {
        /// Filter projects by tags (comma-separated, all tags must match)
        #[arg(short = 't', long, value_delimiter = ',')]
        tags: Vec<String>,

        /// Filter projects by tags (comma-separated, any tag can match)
        #[arg(long, value_delimiter = ',')]
        tags_any: Vec<String>,

        /// Show only projects updated within the last time period (e.g., 7d, 2w, 1m, 1y)
        #[arg(short = 'r', long)]
        recent: Option<String>,

        /// Limit the number of results
        #[arg(short = 'l', long)]
        limit: Option<usize>,

        /// Show detailed information
        #[arg(short = 'd', long)]
        detailed: bool,
    },
    
    /// Switch to a project directory and open editor
    #[command(alias = "s")]
    Switch {
        name: String,

        #[arg(long)]
        no_editor: bool,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    /// Add a new project to manage
    Add {
        /// Path to the project directory
        path: PathBuf,

        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,

        #[arg(short, long)]
        description: Option<String>,
    },
    /// List managed projects  
    #[command(alias = "ls")]
    List {
        /// Filter projects by tags (comma-separated, all tags must match)
        #[arg(short = 't', long, value_delimiter = ',')]
        tags: Vec<String>,

        /// Filter projects by tags (comma-separated, any tag can match)
        #[arg(long, value_delimiter = ',')]
        tags_any: Vec<String>,

        /// Show only projects updated within the last time period (e.g., 7d, 2w, 1m, 1y)
        #[arg(short = 'r', long)]
        recent: Option<String>,

        /// Limit the number of results
        #[arg(short = 'l', long)]
        limit: Option<usize>,

        /// Show detailed information
        #[arg(short = 'd', long)]
        detailed: bool,
    },
    /// Switch to a project directory and open editor
    #[command(alias = "s")]
    Switch {
        name: String,

        #[arg(long)]
        no_editor: bool,
    },
}

#[derive(Subcommand)]
enum TagAction {
    /// Add tags to a project
    Add {
        /// The name of the project
        project_name: String,
        /// The tags to add
        #[arg(required = true)]
        tags: Vec<String>,
    },
    /// Remove tags from a project
    #[command(alias = "rm")]
    Remove {
        /// The name of the project
        project_name: String,
        /// The tags to remove
        #[arg(required = true)]
        tags: Vec<String>,
    },
    /// List all unique tags and their counts
    #[command(alias = "ls")]
    List {},
    /// Show tags for a specific project
    Show {
        /// The name of the project (optional, defaults to current directory's project)
        project_name: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    id: Uuid,
    name: String,
    path: PathBuf,
    tags: Vec<String>,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    git_updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MachineMetadata {
    last_accessed: std::collections::HashMap<Uuid, DateTime<Utc>>,
    access_counts: std::collections::HashMap<Uuid, u32>,
}

fn parse_time_duration(duration_str: &str) -> Result<Duration, String> {
    if duration_str.is_empty() {
        return Err("Duration cannot be empty".to_string());
    }

    let (number_part, unit_part) = if let Some(last_char) = duration_str.chars().last() {
        if last_char.is_alphabetic() {
            let (num_str, unit_str) = duration_str.split_at(duration_str.len() - 1);
            (num_str, unit_str)
        } else {
            (duration_str, "d") // default to days
        }
    } else {
        return Err("Invalid duration format".to_string());
    };

    let number: i64 = number_part.parse()
        .map_err(|_| format!("Invalid number: {}", number_part))?;

    match unit_part.to_lowercase().as_str() {
        "s" | "sec" | "second" | "seconds" => Ok(Duration::seconds(number)),
        "m" | "min" | "minute" | "minutes" => Ok(Duration::minutes(number)),
        "h" | "hour" | "hours" => Ok(Duration::hours(number)),
        "d" | "day" | "days" => Ok(Duration::days(number)),
        "w" | "week" | "weeks" => Ok(Duration::weeks(number)),
        "y" | "year" | "years" => Ok(Duration::days(number * 365)),
        _ => Err(format!("Unknown time unit: {}. Use s, m, h, d, w, or y", unit_part))
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { path, name, tags, description } => {
            let mut config = load_config().await.unwrap();

            let resolved_path = if path.is_absolute() {
                path.clone()
            } else {
                config.projects_root_dir.join(path)
            };

            let absolute_path = resolved_path.canonicalize().unwrap();
            println!("Adding project at: {:?}", absolute_path);

            let project_name = name.clone().unwrap_or_else(|| {
                absolute_path.file_name().unwrap().to_str().unwrap().to_string()
            });

            let project = Project {
                id: Uuid::new_v4(),
                name: project_name,
                path: absolute_path.clone(),
                tags: tags.clone(),
                description: description.clone(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                git_updated_at: get_last_git_commit_time(&absolute_path).unwrap(),
            };

            config.add_project(project);
            save_config(&config).await.unwrap();

            println!("✅ Project '{}' added successfully.", config.projects.get(&config.projects.keys().last().unwrap()).unwrap().name);
        }
        Commands::List { tags, tags_any, recent, limit, detailed } => {
            let mut config = load_config().await.unwrap();
            let mut projects: Vec<&mut Project> = config.projects.values_mut().collect();

            // Update git_updated_at for projects in the background
            for project in projects.iter_mut() {
                let needs_update = project.git_updated_at.is_none() || 
                                   (Utc::now() - project.git_updated_at.unwrap()).num_hours() >= 1;
                if needs_update {
                    let project_path = project.path.clone();
                    let project_id = project.id;
                    tokio::spawn(async move {
                        if let Ok(Some(git_time)) = get_last_git_commit_time(&project_path) {
                            let mut config = load_config().await.unwrap();
                            if let Some(p) = config.projects.get_mut(&project_id) {
                                p.git_updated_at = Some(git_time);
                                save_config(&config).await.unwrap();
                            }
                        }
                    });
                }
            }

            // Apply filters
            let mut filtered_projects: Vec<&mut Project> = projects.into_iter().filter(|project| {
                // Tags filter (AND logic - all tags must match)
                if !tags.is_empty() {
                    let project_tags: std::collections::HashSet<String> = project.tags.iter().cloned().collect();
                    if !tags.iter().all(|tag| project_tags.contains(tag)) {
                        return false;
                    }
                }

                // Tags any filter (OR logic - any tag can match)
                if !tags_any.is_empty() {
                    let project_tags: std::collections::HashSet<String> = project.tags.iter().cloned().collect();
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
                            eprintln!("⚠️  Invalid time format: {}. Using default of 7 days.", recent_str);
                            let cutoff = Utc::now() - Duration::days(7);
                            let last_activity = project.git_updated_at.unwrap_or(project.updated_at);
                            if last_activity < cutoff {
                                return false;
                            }
                        }
                    }
                }

                true
            }).collect();

            // Sort projects: git_updated_at (later), updated_at, created_at
            filtered_projects.sort_by(|a, b| {
                b.git_updated_at.cmp(&a.git_updated_at)
                    .then_with(|| b.updated_at.cmp(&a.updated_at))
                    .then_with(|| b.created_at.cmp(&a.created_at))
            });

            // Apply limit
            if let Some(limit_count) = limit {
                filtered_projects.truncate(*limit_count);
            }

            println!("Active Projects ({} found)", filtered_projects.len());
            for project in filtered_projects {
                if *detailed {
                    // Detailed view
                    println!("\n{}", project.name);
                    if !project.tags.is_empty() {
                        println!("  Tags: {}", project.tags.join(", "));
                    }
                    println!("  Path: {}", project.path.display());
                    if let Some(desc) = &project.description {
                        println!("  Description: {}", desc);
                    }
                    println!("  ID: {}", project.id);
                    println!("  Created: {}", project.created_at.format("%Y-%m-%d %H:%M:%S"));
                    println!("  Updated: {}", project.updated_at.format("%Y-%m-%d %H:%M:%S"));
                    if let Some(git_time) = project.git_updated_at {
                        println!("  Git Updated: {}", git_time.format("%Y-%m-%d %H:%M:%S"));
                    }
                } else {
                    // Simple view
                    let tags_display = if project.tags.is_empty() {
                        "".to_string()
                    } else {
                        format!("[{}]", project.tags.join(", "))
                    };
                    let last_updated_display = if let Some(git_time) = project.git_updated_at {
                        format!("Git: {}", git_time.format("%Y-%m-%d %H:%M"))
                    } else {
                        format!("PM: {}", project.updated_at.format("%Y-%m-%d %H:%M"))
                    };
                    println!(
                        "{:<20} {:<30} {:<20}",
                        project.name,
                        tags_display,
                        last_updated_display
                    );
                }
            }
        }
        Commands::Switch { name, no_editor } => {
            let config = load_config().await.unwrap();
            if let Some(project) = config.find_project_by_name(name) {
                println!("Switching to project: {}", project.name);
                std::env::set_current_dir(&project.path).unwrap();
                println!("Changed directory to: {:?}", &project.path);

                if !no_editor {
                    println!("Opening editor...");
                    let mut cmd = std::process::Command::new("hx");
                    cmd.status().expect("Failed to execute editor.");
                } else {
                    println!("Editor not opened due to --no-editor flag.");
                }

            } else {
                println!("❌ Project not found: {}", name);
            }
        }
        Commands::Project(project_command) => match project_command {
            ProjectCommands::Add { path, name, tags, description } => {
                let mut config = load_config().await.unwrap();

                let resolved_path = if path.is_absolute() {
                    path.clone()
                } else {
                    config.projects_root_dir.join(path)
                };

                let absolute_path = resolved_path.canonicalize().unwrap();
                println!("Adding project at: {:?}", absolute_path);

                let project_name = name.clone().unwrap_or_else(|| {
                    absolute_path.file_name().unwrap().to_str().unwrap().to_string()
                });

                let project = Project {
                    id: Uuid::new_v4(),
                    name: project_name,
                    path: absolute_path.clone(),
                    tags: tags.clone(),
                    description: description.clone(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    git_updated_at: get_last_git_commit_time(&absolute_path).unwrap(),
                };

                config.add_project(project);
                save_config(&config).await.unwrap();

                println!("✅ Project '{}' added successfully.", config.projects.get(&config.projects.keys().last().unwrap()).unwrap().name);
            }
            ProjectCommands::List { tags, tags_any, recent, limit, detailed } => {
                let mut config = load_config().await.unwrap();
                let mut projects: Vec<&mut Project> = config.projects.values_mut().collect();

                // Update git_updated_at for projects in the background
                for project in projects.iter_mut() {
                    let needs_update = project.git_updated_at.is_none() || 
                                       (Utc::now() - project.git_updated_at.unwrap()).num_hours() >= 1;
                    if needs_update {
                        let project_path = project.path.clone();
                        let project_id = project.id;
                        tokio::spawn(async move {
                            if let Ok(Some(git_time)) = get_last_git_commit_time(&project_path) {
                                let mut config = load_config().await.unwrap();
                                if let Some(p) = config.projects.get_mut(&project_id) {
                                    p.git_updated_at = Some(git_time);
                                    save_config(&config).await.unwrap();
                                }
                            }
                        });
                    }
                }

                // Apply filters
                let mut filtered_projects: Vec<&mut Project> = projects.into_iter().filter(|project| {
                    // Tags filter (AND logic - all tags must match)
                    if !tags.is_empty() {
                        let project_tags: std::collections::HashSet<String> = project.tags.iter().cloned().collect();
                        if !tags.iter().all(|tag| project_tags.contains(tag)) {
                            return false;
                        }
                    }

                    // Tags any filter (OR logic - any tag can match)
                    if !tags_any.is_empty() {
                        let project_tags: std::collections::HashSet<String> = project.tags.iter().cloned().collect();
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
                                eprintln!("⚠️  Invalid time format: {}. Using default of 7 days.", recent_str);
                                let cutoff = Utc::now() - Duration::days(7);
                                let last_activity = project.git_updated_at.unwrap_or(project.updated_at);
                                if last_activity < cutoff {
                                    return false;
                                }
                            }
                        }
                    }

                    true
                }).collect();

                // Sort projects: git_updated_at (later), updated_at, created_at
                filtered_projects.sort_by(|a, b| {
                    b.git_updated_at.cmp(&a.git_updated_at)
                        .then_with(|| b.updated_at.cmp(&a.updated_at))
                        .then_with(|| b.created_at.cmp(&a.created_at))
                });

                // Apply limit
                if let Some(limit_count) = limit {
                    filtered_projects.truncate(*limit_count);
                }

                println!("Active Projects ({} found)", filtered_projects.len());
                for project in filtered_projects {
                    if *detailed {
                        // Detailed view
                        println!("\n{}", project.name);
                        if !project.tags.is_empty() {
                            println!("  Tags: {}", project.tags.join(", "));
                        }
                        println!("  Path: {}", project.path.display());
                        if let Some(desc) = &project.description {
                            println!("  Description: {}", desc);
                        }
                        println!("  ID: {}", project.id);
                        println!("  Created: {}", project.created_at.format("%Y-%m-%d %H:%M:%S"));
                        println!("  Updated: {}", project.updated_at.format("%Y-%m-%d %H:%M:%S"));
                        if let Some(git_time) = project.git_updated_at {
                            println!("  Git Updated: {}", git_time.format("%Y-%m-%d %H:%M:%S"));
                        }
                    } else {
                        // Simple view
                        let tags_display = if project.tags.is_empty() {
                            "".to_string()
                        } else {
                            format!("[{}]", project.tags.join(", "))
                        };
                        let last_updated_display = if let Some(git_time) = project.git_updated_at {
                            format!("Git: {}", git_time.format("%Y-%m-%d %H:%M"))
                        } else {
                            format!("PM: {}", project.updated_at.format("%Y-%m-%d %H:%M"))
                        };
                        println!(
                            "{:<20} {:<30} {:<20}",
                            project.name,
                            tags_display,
                            last_updated_display
                        );
                    }
                }
            }
            ProjectCommands::Switch { name, no_editor } => {
                let config = load_config().await.unwrap();
                if let Some(project) = config.find_project_by_name(name) {
                    println!("Switching to project: {}", project.name);
                    std::env::set_current_dir(&project.path).unwrap();
                    println!("Changed directory to: {:?}", &project.path);

                    if !no_editor {
                        println!("Opening editor...");
                        let mut cmd = std::process::Command::new("hx");
                        cmd.status().expect("Failed to execute editor.");
                    } else {
                        println!("Editor not opened due to --no-editor flag.");
                    }

                } else {
                    println!("❌ Project not found: {}", name);
                }
            }
        },
        Commands::Tag { action } => match action {
            TagAction::Add { project_name, tags } => {
                let mut config = load_config().await.unwrap();
                add_tags(project_name, tags, &mut config).await.unwrap();
                save_config(&config).await.unwrap();
            }
            TagAction::Remove { project_name, tags } => {
                let mut config = load_config().await.unwrap();
                remove_tags(project_name, tags, &mut config).await.unwrap();
                save_config(&config).await.unwrap();
            }
            TagAction::List {} => {
                let config = load_config().await.unwrap();
                list_tags(&config).await.unwrap();
            }
            TagAction::Show { project_name } => {
                let config = load_config().await.unwrap();
                show_tags(project_name.as_deref(), &config).await.unwrap();
            }
        },
        Commands::Init {} => {
            let config_path = get_config_path().unwrap();
            if config_path.exists() {
                println!("❌ pm이 이미 초기화되었습니다. 설정 파일: {:?}", config_path);
                return;
            }

            let github_username = inquire::Text::new("GitHub username:")
                .prompt()
                .expect("Failed to get GitHub username");

            let projects_root_dir_str = inquire::Text::new("Projects root directory path:")
                .with_default("~/workspace")
                .prompt()
                .expect("Failed to get projects root directory path");

            let projects_root_dir = PathBuf::from(shellexpand::tilde(&projects_root_dir_str).to_string());

            let config = Config {
                github_username,
                projects_root_dir,
                ..Default::default()
            };

            save_config(&config).await.unwrap();

            println!("✅ pm이 성공적으로 초기화되었습니다.");
            println!("   설정 파일: {:?}", config_path);
        }
    }
}