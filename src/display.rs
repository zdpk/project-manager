use crate::constants::*;
use crate::Project;
use chrono::{DateTime, Utc};

pub fn format_relative_time(time: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(time);

    if duration.num_minutes() < 1 {
        "방금 전".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}분 전", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}시간 전", duration.num_hours())
    } else if duration.num_days() == 1 {
        "어제".to_string()
    } else if duration.num_days() < 7 {
        format!("{}일 전", duration.num_days())
    } else if duration.num_weeks() == 1 {
        "1주일 전".to_string()
    } else if duration.num_weeks() < 4 {
        format!("{}주일 전", duration.num_weeks())
    } else if duration.num_days() < 365 {
        let months = duration.num_days() / 30;
        if months == 1 {
            "1달 전".to_string()
        } else {
            format!("{}달 전", months)
        }
    } else {
        let years = duration.num_days() / 365;
        if years == 1 {
            "1년 전".to_string()
        } else {
            format!("{}년 전", years)
        }
    }
}

pub fn display_project_simple(project: &Project, access_time: Option<DateTime<Utc>>) {
    let tags_display = if project.tags.is_empty() {
        "".to_string()
    } else {
        format!("[{}]", project.tags.join(", "))
    };

    let last_updated_display = if let Some(git_time) = project.git_updated_at {
        format!("Git: {}", format_relative_time(git_time))
    } else {
        format!("PM: {}", format_relative_time(project.updated_at))
    };

    let access_display = if let Some(access_time) = access_time {
        format!(" (접근: {})", format_relative_time(access_time))
    } else {
        "".to_string()
    };

    println!(
        "{:<width_name$} {:<width_tags$} {:<width_time$}{}",
        project.name,
        tags_display,
        last_updated_display,
        access_display,
        width_name = PROJECT_NAME_WIDTH,
        width_tags = PROJECT_TAGS_WIDTH,
        width_time = PROJECT_TIME_WIDTH
    );
}

pub fn display_project_detailed(
    project: &Project,
    access_time: Option<DateTime<Utc>>,
    access_count: u32,
) {
    println!("\n{}", project.name);
    if !project.tags.is_empty() {
        println!("  Tags: {}", project.tags.join(", "));
    }
    println!("  Path: {}", project.path.display());
    if let Some(desc) = &project.description {
        println!("  Description: {}", desc);
    }
    println!("  ID: {}", project.id);
    println!(
        "  Created: {}",
        project.created_at.format("%Y-%m-%d %H:%M:%S")
    );
    println!(
        "  Updated: {}",
        project.updated_at.format("%Y-%m-%d %H:%M:%S")
    );
    if let Some(git_time) = project.git_updated_at {
        println!(
            "  Git Updated: {} ({})",
            git_time.format("%Y-%m-%d %H:%M:%S"),
            format_relative_time(git_time)
        );
    }
    if let Some(access_time) = access_time {
        println!(
            "  Last Accessed: {} ({})",
            access_time.format("%Y-%m-%d %H:%M:%S"),
            format_relative_time(access_time)
        );
    }
    if access_count > 0 {
        println!("  Access Count: {}", access_count);
    }
}

pub fn display_project_list_header(count: usize) {
    println!("📋 Active Projects ({} found)", count);
}

pub fn display_no_projects() {
    println!("📋 No projects found");
    println!("\n💡 {}", SUGGESTION_ADD_FIRST_PROJECT);
}

pub fn display_no_matches() {
    println!("📋 No projects match your filters");
    println!("\n💡 Try:");
    println!("  - No filters: pm ls");
    println!("  - Longer time period: pm ls -r 30d");
    println!("  - Different tags: pm ls --tags-any frontend,backend");
}

pub fn display_switch_info(
    project_name: &str,
    access_count: u32,
    last_accessed: Option<DateTime<Utc>>,
) {
    println!("🔄 Switching to project: {}", project_name);
    println!("📊 Access count: {} times", access_count);

    if let Some(last_time) = last_accessed {
        println!("⏰ Last accessed: {}", format_relative_time(last_time));
    }
}

pub fn display_switch_success(project_path: &std::path::Path, no_editor: bool) {
    println!("📂 Working directory: {}", project_path.display());

    if no_editor {
        println!("✅ Project switched (editor not opened)");
    } else {
        println!("🚀 Opening editor...");
    }
}

pub fn display_suggestions(suggestions: &[String]) {
    if !suggestions.is_empty() {
        println!("\n💡 Did you mean one of these?");
        for suggestion in suggestions.iter().take(3) {
            println!("  - {}", suggestion);
        }
    } else {
        println!("\n💡 {}", SUGGESTION_USE_PM_LS);
    }
}

pub fn display_error(context: &str, error: &str) {
    eprintln!("❌ {}: {}", context, error);
}

pub fn display_warning(message: &str) {
    eprintln!("⚠️  {}", message);
}

pub fn display_success(message: &str) {
    println!("✅ {}", message);
}

pub fn display_info(message: &str) {
    println!("💡 {}", message);
}

pub fn display_project_added(project_name: &str, tags: &[String]) {
    println!("✅ Project '{}' added successfully!", project_name);
    if !tags.is_empty() {
        println!("🏷️  Tags: {}", tags.join(", "));
    }
}

pub fn display_editor_error(error: &str) {
    eprintln!(
        "❌ Failed to execute editor '{}': {}",
        DEFAULT_EDITOR, error
    );
    eprintln!("\n💡 Suggestions:");
    eprintln!("  - {}", SUGGESTION_INSTALL_HELIX);
    eprintln!("  - {}", SUGGESTION_USE_NO_EDITOR);
    eprintln!("  - {}", SUGGESTION_SET_EDITOR_ENV);
}

pub fn display_init_success(
    github_username: &str,
    projects_root: &std::path::Path,
    config_path: &std::path::Path,
) {
    println!("\n✅ {}", SUCCESS_PM_INITIALIZED);
    println!("👤 GitHub username: {}", github_username);
    println!("📁 Projects root: {}", projects_root.display());
    println!("⚙️  Config file: {}", config_path.display());
    println!("\n🎯 Next steps:");
    println!("  pm add <path>     # Add your first project");
    println!("  pm ls             # List projects");
    println!("  pm s <name>       # Switch to project");
}
