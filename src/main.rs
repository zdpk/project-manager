use chrono::{DateTime, Utc};
use clap::{CommandFactory, Parser, Subcommand};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

mod backup;
mod commands;
mod config;
mod constants;
mod display;
mod error;
mod shell_integration;
mod tag_commands;
mod utils;
mod validation;

use error::PmError;

use commands::config::ExportFormat;
use commands::{backup as backup_cmd, config as config_cmd, init, project, status, tag};
use config::load_config;
use constants::*;
use display::display_error;
use error::handle_error;

fn handle_config_error(e: anyhow::Error) -> ! {
    if e.to_string().contains("Configuration file not found") {
        display_error("PM not initialized", "Configuration file not found");
        println!("\n💡 Please initialize PM first:");
        println!("   pm init");
        std::process::exit(1);
    } else {
        handle_error(e, ERROR_CONFIG_LOAD);
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_help_subcommand = true, disable_version_flag = true)]
struct Cli {
    /// Show version information
    #[arg(short = 'v', long = "version")]
    version: bool,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new project to manage (alias: a)
    #[command(alias = "a")]
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

    /// Clone repositories from GitHub (interactive browse or direct clone) (alias: cl)
    #[command(alias = "cl")]
    Clone {
        /// Repository in format owner/repo (optional for interactive browse)
        repo: Option<String>,

        /// Target directory (defaults to <current_dir>/<owner>/<repo>)
        #[arg(short, long)]
        directory: Option<PathBuf>,
    },

    /// List managed projects (alias: ls)
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

        /// Show verbose information
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// Switch to a project directory (alias: sw)
    #[command(alias = "sw")]
    Switch {
        name: String,
    },

    /// Scan for Git repositories and add them to PM (alias: sc)
    #[command(alias = "sc")]
    Scan {
        /// Directory to scan (defaults to current directory)
        #[arg(short, long)]
        directory: Option<PathBuf>,

        /// Show all repositories found, don't prompt for selection
        #[arg(long)]
        show_all: bool,
    },

    /// Manage project tags (alias: t)
    #[command(alias = "t")]
    Tag {
        #[command(subcommand)]
        action: TagAction,
    },

    /// Remove projects from PM (alias: rm)
    #[command(alias = "rm")]
    Remove {
        /// Project name (optional for interactive mode)
        project: Option<String>,
        
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Manage configuration (alias: cf)
    #[command(alias = "cf")]
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },

    /// Manage backups (list, restore, clean) (alias: bk)
    #[command(alias = "bk")]
    Backup {
        #[command(subcommand)]
        action: BackupAction,
    },

    /// Initialize PM with basic configuration
    Init {
        /// Skip initialization if config exists (non-interactive)
        #[arg(long)]
        skip: bool,
        
        /// Replace existing config with backup (non-interactive)
        #[arg(long)]
        replace: bool,
        
        /// Development mode with _PM_BINARY setup (hidden from help)
        #[arg(long, hide = true)]
        dev: bool,
    },

    /// Show current project status (for prompt integration)
    Status {
        /// Output format (text, json)
        #[arg(long, default_value = "text")]
        format: String,
        
        /// Quiet mode for prompt integration
        #[arg(short, long)]
        quiet: bool,
    },

}


#[derive(Subcommand)]
enum BackupAction {
    /// List all available backups
    List,
    
    /// Restore a specific backup or select interactively
    Restore {
        /// Backup ID to restore (optional for interactive mode)
        backup_id: Option<String>,
        
        /// Skip confirmation prompt
        #[arg(short = 'f', long)]
        force: bool,
    },
    
    /// Clean old backups (keep most recent N)
    Clean {
        /// Number of backups to keep
        #[arg(default_value = "5")]
        keep: usize,
        
        /// Skip confirmation prompt
        #[arg(short = 'f', long)]
        force: bool,
    },
    
    /// Show backup system status
    Status,
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

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show {},

    /// Edit configuration file
    Edit {},

    /// Validate configuration file
    Validate {},

    /// Reset configuration to defaults
    Reset {},

    /// Get a specific configuration value
    Get {
        /// Configuration key (supports dot notation like 'settings.show_git_status')
        key: String,
    },

    /// Set a configuration value
    Set {
        /// Configuration key (supports dot notation like 'settings.show_git_status')
        key: String,
        /// New value
        value: String,
    },

    /// List all available configuration keys
    List {},

    /// Backup and restore operations
    #[command(subcommand)]
    Backup(BackupCommands),

    /// Template operations
    #[command(subcommand)]
    Template(TemplateCommands),

    /// Interactive configuration setup
    Setup {
        /// Use quick setup with defaults
        #[arg(long)]
        quick: bool,
    },

    /// Export configuration
    Export {
        /// Output format
        #[arg(long, value_enum, default_value = "yaml")]
        format: ExportFormat,
        /// Output file path
        #[arg(long)]
        file: Option<PathBuf>,
    },

    /// Import configuration from file
    Import {
        /// Input file path
        file: PathBuf,
        /// Force import without backup
        #[arg(long)]
        force: bool,
    },

    /// Show differences between current config and backup
    Diff {
        /// Backup name to compare with (defaults to latest)
        backup: Option<String>,
    },

    /// Show configuration change history
    History {
        /// Limit number of entries
        #[arg(long, default_value = "10")]
        limit: usize,
    },
}

#[derive(Subcommand)]
enum BackupCommands {
    /// Create a backup of current configuration
    Create {
        /// Backup name (optional, defaults to timestamp)
        #[arg(long)]
        name: Option<String>,
    },
    /// Restore configuration from backup
    Restore {
        /// Backup name
        name: String,
    },
    /// List all available backups
    List {},
    /// Delete a backup
    Delete {
        /// Backup name
        name: String,
    },
}

#[derive(Subcommand)]
enum TemplateCommands {
    /// List available templates
    List {},
    /// Apply a template
    Apply {
        /// Template name
        name: String,
    },
    /// Save current configuration as template
    Save {
        /// Template name
        name: String,
        /// Template description
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a template
    Delete {
        /// Template name
        name: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[schemars(title = "Project", description = "A managed project")]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub git_updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[schemars(description = "Whether this project is a Git repository")]
    pub is_git_repository: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, JsonSchema)]
#[schemars(
    title = "Machine Metadata",
    description = "Machine-specific project metadata"
)]
pub struct MachineMetadata {
    pub last_accessed: std::collections::HashMap<Uuid, DateTime<Utc>>,
    pub access_counts: std::collections::HashMap<Uuid, u32>,
}


#[tokio::main]
async fn main() {
    // Detect development mode based on binary name
    let is_dev_mode = std::env::args().next()
        .map(|arg0| std::path::Path::new(&arg0)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name == "_pm")
            .unwrap_or(false))
        .unwrap_or(false);

    // Set development mode environment variable for config loading
    if is_dev_mode {
        std::env::set_var("PM_DEV_MODE", "true");
    }

    let cli = Cli::parse();

    // Handle global version flag
    if cli.version {
        let binary_name = if is_dev_mode { "_pm" } else { env!("CARGO_PKG_NAME") };
        println!("{} {} {}", binary_name, env!("CARGO_PKG_VERSION"), 
                 if is_dev_mode { "(dev)" } else { "" });
        return;
    }

    // If no command provided, show help
    let Some(command) = &cli.command else {
        let mut app = Cli::command();
        app.print_help().unwrap();
        return;
    };

    match command {
        Commands::Add {
            path,
            name,
            tags,
            description,
        } => {
            if let Err(e) = project::handle_add(path, name, tags, description).await {
                handle_config_error(e);
            }
        }
        Commands::Clone { repo, directory } => {
            if let Err(e) = project::handle_clone(repo.as_deref(), directory.as_deref()).await {
                // Check if this is a user cancellation (Ctrl-C)
                if let Some(pm_error) = e.downcast_ref::<PmError>() {
                    if matches!(pm_error, PmError::OperationCancelled) {
                        // Gracefully exit on cancellation
                        std::process::exit(0);
                    }
                }
                handle_config_error(e);
            }
        }
        Commands::List {
            tags,
            tags_any,
            recent,
            limit,
            verbose,
        } => {
            if let Err(e) = project::handle_list(tags, tags_any, recent, limit, *verbose).await {
                handle_config_error(e);
            }
        }
        Commands::Switch { name } => match load_config().await {
            Ok(mut config) => {
                if let Err(e) = project::handle_switch(&mut config, name).await {
                    handle_error(e, ERROR_PROJECT_NOT_FOUND);
                }
            }
            Err(e) => {
                handle_config_error(e);
            }
        },
        Commands::Scan {
            directory,
            show_all,
        } => {
            if let Err(e) = project::handle_scan(directory.as_deref(), *show_all).await {
                handle_config_error(e);
            }
        }
        Commands::Tag { action } => match action {
            TagAction::Add { project_name, tags } => {
                if let Err(e) = tag::handle_tag_add(project_name, tags).await {
                    handle_config_error(e);
                }
            }
            TagAction::Remove { project_name, tags } => {
                if let Err(e) = tag::handle_tag_remove(project_name, tags).await {
                    handle_config_error(e);
                }
            }
            TagAction::List {} => {
                if let Err(e) = tag::handle_tag_list().await {
                    handle_config_error(e);
                }
            }
            TagAction::Show { project_name } => {
                if let Err(e) = tag::handle_tag_show(project_name.as_deref()).await {
                    handle_config_error(e);
                }
            }
        },
        Commands::Remove { project, yes } => {
            if let Err(e) = project::handle_remove(project.as_deref(), *yes).await {
                handle_config_error(e);
            }
        }
        Commands::Config { command } => match command.as_ref().unwrap_or(&ConfigCommands::Show {}) {
            ConfigCommands::Show {} => {
                if let Err(e) = config_cmd::handle_show().await {
                    handle_config_error(e);
                }
            }
            ConfigCommands::Edit {} => {
                if let Err(e) = config_cmd::handle_edit().await {
                    handle_config_error(e);
                }
            }
            ConfigCommands::Validate {} => {
                if let Err(e) = config_cmd::handle_validate().await {
                    handle_config_error(e);
                }
            }
            ConfigCommands::Reset {} => {
                if let Err(e) = config_cmd::handle_reset().await {
                    handle_config_error(e);
                }
            }
            ConfigCommands::Get { key } => {
                if let Err(e) = config_cmd::handle_get(key).await {
                    handle_config_error(e);
                }
            }
            ConfigCommands::Set { key, value } => {
                if let Err(e) = config_cmd::handle_set(key, value).await {
                    handle_config_error(e);
                }
            }
            ConfigCommands::List {} => {
                if let Err(e) = config_cmd::handle_list().await {
                    handle_error(e, "Failed to list config keys");
                }
            }
            ConfigCommands::Backup(backup_command) => match backup_command {
                BackupCommands::Create { name } => {
                    if let Err(e) = config_cmd::handle_backup_create(name.as_deref()).await {
                        handle_error(e, "Failed to create backup");
                    }
                }
                BackupCommands::Restore { name } => {
                    if let Err(e) = config_cmd::handle_backup_restore(name).await {
                        handle_error(e, "Failed to restore backup");
                    }
                }
                BackupCommands::List {} => {
                    if let Err(e) = config_cmd::handle_backup_list().await {
                        handle_error(e, "Failed to list backups");
                    }
                }
                BackupCommands::Delete { name } => {
                    if let Err(e) = config_cmd::handle_backup_delete(name).await {
                        handle_error(e, "Failed to delete backup");
                    }
                }
            },
            ConfigCommands::Template(template_command) => match template_command {
                TemplateCommands::List {} => {
                    if let Err(e) = config_cmd::handle_template_list().await {
                        handle_error(e, "Failed to list templates");
                    }
                }
                TemplateCommands::Apply { name } => {
                    if let Err(e) = config_cmd::handle_template_apply(name).await {
                        handle_error(e, "Failed to apply template");
                    }
                }
                TemplateCommands::Save { name, description } => {
                    if let Err(e) =
                        config_cmd::handle_template_save(name, description.as_deref()).await
                    {
                        handle_error(e, "Failed to save template");
                    }
                }
                TemplateCommands::Delete { name } => {
                    if let Err(e) = config_cmd::handle_template_delete(name).await {
                        handle_error(e, "Failed to delete template");
                    }
                }
            },
            ConfigCommands::Setup { quick } => {
                if let Err(e) = config_cmd::handle_setup(*quick).await {
                    handle_error(e, "Failed to setup config");
                }
            }
            ConfigCommands::Export { format, file } => {
                if let Err(e) = config_cmd::handle_export(format, file.as_deref()).await {
                    handle_error(e, "Failed to export config");
                }
            }
            ConfigCommands::Import { file, force } => {
                if let Err(e) = config_cmd::handle_import(file, *force).await {
                    handle_error(e, "Failed to import config");
                }
            }
            ConfigCommands::Diff { backup } => {
                if let Err(e) = config_cmd::handle_diff(backup.as_deref()).await {
                    handle_error(e, "Failed to show config diff");
                }
            }
            ConfigCommands::History { limit } => {
                if let Err(e) = config_cmd::handle_history(*limit).await {
                    handle_error(e, "Failed to show config history");
                }
            }
        },
        Commands::Backup { action } => match action {
            BackupAction::List => {
                if let Err(e) = backup_cmd::handle_backup_list().await {
                    handle_error(e, "Failed to list backups");
                }
            }
            BackupAction::Restore { backup_id, force } => {
                if let Some(id) = backup_id {
                    if let Err(e) = backup_cmd::handle_backup_restore(id, *force).await {
                        handle_error(e, "Failed to restore backup");
                    }
                } else {
                    if let Err(e) = backup_cmd::handle_backup_restore_interactive().await {
                        handle_error(e, "Failed to restore backup");
                    }
                }
            }
            BackupAction::Clean { keep, force } => {
                if let Err(e) = backup_cmd::handle_backup_clean(*keep, *force).await {
                    handle_error(e, "Failed to clean backups");
                }
            }
            BackupAction::Status => {
                if let Err(e) = backup_cmd::handle_backup_status().await {
                    handle_error(e, "Failed to show backup status");
                }
            }
        },
        Commands::Init { skip, replace, dev } => {
            if let Err(e) = init::handle_init(*skip, *replace, *dev).await {
                handle_error(e, "Failed to initialize PM");
            }
        }
        Commands::Status { format, quiet } => {
            if let Err(e) = status::handle_status(format, *quiet).await {
                handle_error(e, "Failed to get project status");
            }
        }
    }
}
