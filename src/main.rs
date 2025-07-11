use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

mod config;
mod utils;
mod tag_commands;
mod constants;
mod display;
mod validation;
mod error;
mod commands;

use config::load_config;
use commands::{init, project, tag};
use error::handle_error;
use constants::*;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub git_updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MachineMetadata {
    pub last_accessed: std::collections::HashMap<Uuid, DateTime<Utc>>,
    pub access_counts: std::collections::HashMap<Uuid, u32>,
}






#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { path, name, tags, description } => {
            if let Err(e) = project::handle_add(path, name, tags, description).await {
                handle_error(e, ERROR_CONFIG_LOAD);
            }
        }
        Commands::List { tags, tags_any, recent, limit, detailed } => {
            if let Err(e) = project::handle_list(tags, tags_any, recent, limit, *detailed).await {
                handle_error(e, ERROR_CONFIG_LOAD);
            }
        }
        Commands::Switch { name, no_editor } => {
            let mut config = match load_config().await {
                Ok(config) => config,
                Err(e) => {
                    handle_error(e, ERROR_CONFIG_LOAD);
                    return;
                }
            };

            if let Err(e) = project::handle_switch(&mut config, name, *no_editor).await {
                handle_error(e, ERROR_PROJECT_NOT_FOUND);
            }
        }
        Commands::Project(project_command) => match project_command {
            ProjectCommands::Add { path, name, tags, description } => {
                if let Err(e) = project::handle_add(path, name, tags, description).await {
                    handle_error(e, ERROR_CONFIG_LOAD);
                }
            }
            ProjectCommands::List { tags, tags_any, recent, limit, detailed } => {
                if let Err(e) = project::handle_list(tags, tags_any, recent, limit, *detailed).await {
                    handle_error(e, ERROR_CONFIG_LOAD);
                }
            }
            ProjectCommands::Switch { name, no_editor } => {
                let mut config = match load_config().await {
                    Ok(config) => config,
                    Err(e) => {
                        handle_error(e, ERROR_CONFIG_LOAD);
                    }
                };

                if let Err(e) = project::handle_switch(&mut config, name, *no_editor).await {
                    handle_error(e, ERROR_PROJECT_NOT_FOUND);
                }
            }
        },
        Commands::Tag { action } => match action {
            TagAction::Add { project_name, tags } => {
                if let Err(e) = tag::handle_tag_add(project_name, tags).await {
                    handle_error(e, "Failed to add tags");
                }
            }
            TagAction::Remove { project_name, tags } => {
                if let Err(e) = tag::handle_tag_remove(project_name, tags).await {
                    handle_error(e, "Failed to remove tags");
                }
            }
            TagAction::List {} => {
                if let Err(e) = tag::handle_tag_list().await {
                    handle_error(e, "Failed to list tags");
                }
            }
            TagAction::Show { project_name } => {
                if let Err(e) = tag::handle_tag_show(project_name.as_deref()).await {
                    handle_error(e, "Failed to show tags");
                }
            }
        },
        Commands::Init {} => {
            if let Err(e) = init::handle_init().await {
                handle_error(e, "Failed to initialize PM");
            }
        }
    }
}