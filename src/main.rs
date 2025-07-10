use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

mod config;
mod utils;

use config::{get_config_path, load_config, save_config, Config};
use utils::get_last_git_commit_time;

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
    Ls {},
    /// Switch to a project directory and open editor
    S {
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
    Ls {},
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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
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
            ProjectCommands::Ls {} => {
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

                // Sort projects: git_updated_at (later), updated_at, created_at
                projects.sort_by(|a, b| {
                    b.git_updated_at.cmp(&a.git_updated_at)
                        .then_with(|| b.updated_at.cmp(&a.updated_at))
                        .then_with(|| b.created_at.cmp(&a.created_at))
                });

                println!("Active Projects ({} found)", projects.len());
                for project in projects {
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
            ProjectCommands::S { name, no_editor } => {
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
                if let Some(project) = config.find_project_by_name_mut(project_name) {
                    for tag in tags {
                        if !project.tags.contains(tag) {
                            project.tags.push(tag.clone());
                        }
                    }
                    project.updated_at = Utc::now();
                    save_config(&config).await.unwrap();
                    println!("✓ Tags added to project '{}'", project_name);
                } else {
                    println!("❌ Project not found: {}", project_name);
                }
            }
            TagAction::Remove { project_name, tags } => {
                let mut config = load_config().await.unwrap();
                if let Some(project) = config.find_project_by_name_mut(project_name) {
                    let initial_tags_count = project.tags.len();
                    project.tags.retain(|tag| !tags.contains(tag));
                    if project.tags.len() < initial_tags_count {
                        project.updated_at = Utc::now();
                        save_config(&config).await.unwrap();
                        println!("✓ Tags removed from project '{}'", project_name);
                    } else {
                        println!("No matching tags found to remove from project '{}'.", project_name);
                    }
                } else {
                    println!("❌ Project not found: {}", project_name);
                }
            }
            TagAction::Ls {} => {
                let config = load_config().await.unwrap();
                let mut tag_counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

                for project in config.projects.values() {
                    for tag in &project.tags {
                        *tag_counts.entry(tag.clone()).or_insert(0) += 1;
                    }
                }

                let mut sorted_tags: Vec<(String, u32)> = tag_counts.into_iter().collect();
                sorted_tags.sort_by(|a, b| b.1.cmp(&a.1));

                println!("All Tags (by usage count):");
                for (tag, count) in sorted_tags {
                    println!("  - {} ({} projects)", tag, count);
                }
            }
            TagAction::Show { project_name } => {
                let config = load_config().await.unwrap();
                let project_to_show = if let Some(name) = project_name {
                    config.find_project_by_name(&name)
                } else {
                    let current_path = std::env::current_dir().unwrap();
                    config.find_project_by_path(&current_path)
                };

                if let Some(project) = project_to_show {
                    println!("Tags for '{}':", project.name);
                    if project.tags.is_empty() {
                        println!("  (No tags)");
                    } else {
                        for tag in &project.tags {
                            println!("  - {}", tag);
                        }
                    }
                } else {
                    println!("❌ Project not found. Please specify a project name or navigate into a project directory.");
                }
            }
        },
        Commands::Init {} => {
            let config_path = get_config_path().unwrap();
            if config_path.exists() {
                println!("❌ pm이 이미 초기화되었습니다. 설정 파일: {:?}", config_path);
                return;
            }

            println!("pm을 초기화합니다...");

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