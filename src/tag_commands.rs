use crate::config::Config;
use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;

pub async fn add_tags(project_name: &str, tags: &[String], config: &mut Config) -> Result<()> {
    if let Some(project) = config.find_project_by_name_mut(project_name) {
        for tag in tags {
            if !project.tags.contains(tag) {
                project.tags.push(tag.clone());
            }
        }
        project.updated_at = Utc::now();
        println!("✓ Tags added to project '{}'", project_name);
    } else {
        println!("❌ Project not found: {}", project_name);
    }
    Ok(())
}

pub async fn remove_tags(project_name: &str, tags: &[String], config: &mut Config) -> Result<()> {
    if let Some(project) = config.find_project_by_name_mut(project_name) {
        let initial_tags_count = project.tags.len();
        project.tags.retain(|tag| !tags.contains(tag));
        if project.tags.len() < initial_tags_count {
            project.updated_at = Utc::now();
            println!("✓ Tags removed from project '{}'", project_name);
        } else {
            println!(
                "No matching tags found to remove from project '{}'.",
                project_name
            );
        }
    } else {
        println!("❌ Project not found: {}", project_name);
    }
    Ok(())
}

pub async fn list_tags(config: &Config) -> Result<()> {
    let mut tag_counts: HashMap<String, u32> = HashMap::new();

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
    Ok(())
}

pub async fn show_tags(project_name: Option<&str>, config: &Config) -> Result<()> {
    let project_to_show = if let Some(name) = project_name {
        config.find_project_by_name(name)
    } else {
        let current_path = std::env::current_dir()?;
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
    Ok(())
}
