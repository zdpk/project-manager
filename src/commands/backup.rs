use crate::backup::{load_backup_metadata, save_backup_metadata, get_backup_dir};
use crate::display::*;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use colored::Colorize;
use inquire::{Confirm, Select};
use std::path::Path;
use tokio::fs;

/// List all available backups
pub async fn handle_backup_list() -> Result<()> {
    let metadata = load_backup_metadata().await?;
    
    if metadata.backups.is_empty() {
        println!("📦 No backups found");
        println!("💡 Backups are created automatically during:");
        println!("   • {} init --backup or --force", crate::utils::get_binary_name());
        println!("   • Conflict resolution during initialization");
        return Ok(());
    }
    
    println!("📦 Available Backups ({} total):\n", metadata.backups.len());
    
    // Sort backups by timestamp (newest first)
    let mut sorted_backups = metadata.backups.clone();
    sorted_backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    for backup in &sorted_backups {
        let age = format_backup_age(&backup.timestamp);
        println!("🗓️  {} ({})", backup.id.bright_blue(), backup.reason);
        println!("   📅 Created: {} ({})", 
            backup.timestamp.format("%Y-%m-%d %H:%M:%S UTC"), 
            age
        );
        println!("   📁 Files: {}", backup.files.len());
        
        for file in &backup.files {
            let file_type_icon = match &file.file_type {
                crate::backup::BackupFileType::Config => "⚙️",
                crate::backup::BackupFileType::ShellIntegration(shell_type) => {
                    match shell_type {
                        crate::shell_integration::ShellType::Fish => "🐠",
                        crate::shell_integration::ShellType::Zsh => "🐚",
                        crate::shell_integration::ShellType::Bash => "💻",
                        crate::shell_integration::ShellType::Unknown => "❓",
                    }
                }
            };
            
            println!("     {} {} → {}", 
                file_type_icon,
                file.original_path.display(), 
                file.backup_path.display()
            );
        }
        
        if !backup.shell_changes.is_empty() {
            println!("   🔧 Shell changes: {}", backup.shell_changes.len());
            for change in &backup.shell_changes {
                println!("     📝 {} ({})", 
                    change.file.display(),
                    format!("{:?}", change.action).to_lowercase()
                );
            }
        }
        
        println!();
    }
    
    Ok(())
}

/// Restore a specific backup
pub async fn handle_backup_restore(backup_id: &str, force: bool) -> Result<()> {
    let metadata = load_backup_metadata().await?;
    let backup = metadata.backups.iter()
        .find(|b| b.id == backup_id)
        .ok_or_else(|| anyhow!("Backup not found: {}", backup_id))?;
    
    println!("🔄 Restoring backup: {}", backup_id.bright_blue());
    println!("   📅 Created: {} ({})", 
        backup.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
        backup.reason
    );
    println!("   📁 Files: {}", backup.files.len());
    
    // Show what will be restored
    for file in &backup.files {
        println!("   📄 {} ← {}", 
            file.original_path.display(), 
            file.backup_path.display()
        );
    }
    
    if !backup.shell_changes.is_empty() {
        println!("   🔧 Shell changes: {} (will be rolled back)", backup.shell_changes.len());
    }
    
    // Confirmation
    if !force {
        let confirmed = Confirm::new("Do you want to restore this backup?")
            .with_default(false)
            .prompt()?;
        
        if !confirmed {
            println!("🚫 Backup restore cancelled");
            return Ok(());
        }
    }
    
    let mut restored_files = 0;
    let mut failed_files = 0;
    
    // Restore files
    for file in &backup.files {
        if !file.backup_path.exists() {
            display_warning(&format!("Backup file not found: {}", file.backup_path.display()));
            failed_files += 1;
            continue;
        }
        
        // Ensure parent directory exists
        if let Some(parent) = file.original_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent).await {
                    display_warning(&format!("Failed to create directory {}: {}", parent.display(), e));
                    failed_files += 1;
                    continue;
                }
            }
        }
        
        // Restore the file
        if let Err(e) = fs::copy(&file.backup_path, &file.original_path).await {
            display_warning(&format!("Failed to restore {}: {}", file.original_path.display(), e));
            failed_files += 1;
        } else {
            println!("✅ Restored: {}", file.original_path.display());
            restored_files += 1;
        }
    }
    
    // TODO: Implement shell changes rollback
    if !backup.shell_changes.is_empty() {
        display_warning("Shell changes rollback not yet implemented");
        println!("💡 You may need to manually revert shell configuration changes");
    }
    
    // Summary
    if failed_files == 0 {
        display_success(&format!("Backup restored successfully! ({} files)", restored_files));
    } else {
        display_warning(&format!("Backup restored with {} errors. {} files restored successfully.", 
            failed_files, restored_files));
    }
    
    Ok(())
}

/// Clean old backups (keep most recent N)
pub async fn handle_backup_clean(keep_count: usize, force: bool) -> Result<()> {
    let mut metadata = load_backup_metadata().await?;
    
    if metadata.backups.len() <= keep_count {
        println!("📦 No backups to clean (current: {}, keep: {})", 
            metadata.backups.len(), keep_count);
        return Ok(());
    }
    
    // Sort by timestamp (newest first)
    metadata.backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    let to_remove = metadata.backups.split_off(keep_count);
    
    println!("🗑️  Found {} backups to clean (keeping {} most recent)", 
        to_remove.len(), keep_count);
    
    // Show what will be removed
    for backup in &to_remove {
        let age = format_backup_age(&backup.timestamp);
        println!("   📦 {} ({}, {})", backup.id, backup.reason, age);
    }
    
    // Confirmation
    if !force {
        let confirmed = Confirm::new(&format!("Delete {} old backups?", to_remove.len()))
            .with_default(false)
            .prompt()?;
        
        if !confirmed {
            println!("🚫 Backup cleanup cancelled");
            return Ok(());
        }
    }
    
    let mut removed_count = 0;
    let mut failed_count = 0;
    
    // Remove backup directories
    for backup in &to_remove {
        let backup_dir = get_backup_dir()?.join(&backup.id);
        if backup_dir.exists() {
            if let Err(e) = fs::remove_dir_all(&backup_dir).await {
                display_warning(&format!("Failed to remove backup {}: {}", backup.id, e));
                failed_count += 1;
            } else {
                println!("🗑️ Removed backup: {}", backup.id);
                removed_count += 1;
            }
        } else {
            println!("⚠️  Backup directory not found: {}", backup_dir.display());
            removed_count += 1; // Still count as removed from metadata
        }
    }
    
    // Update metadata
    save_backup_metadata(&metadata).await?;
    
    // Summary
    if failed_count == 0 {
        display_success(&format!("Cleaned {} old backups successfully", removed_count));
    } else {
        display_warning(&format!("Cleaned {} backups with {} errors", removed_count, failed_count));
    }
    
    Ok(())
}

/// Interactive backup selection for restore
pub async fn handle_backup_restore_interactive() -> Result<()> {
    let metadata = load_backup_metadata().await?;
    
    if metadata.backups.is_empty() {
        println!("📦 No backups available for restore");
        return Ok(());
    }
    
    // Sort backups by timestamp (newest first)
    let mut sorted_backups = metadata.backups.clone();
    sorted_backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    // Create selection options
    let options: Vec<String> = sorted_backups.iter()
        .map(|backup| {
            let age = format_backup_age(&backup.timestamp);
            format!("{} - {} ({}, {} files)", 
                backup.id, backup.reason, age, backup.files.len())
        })
        .collect();
    
    let choice = Select::new("Select a backup to restore:", options)
        .prompt()?;
    
    // Extract backup ID from choice
    let backup_id = choice.split(" - ").next()
        .ok_or_else(|| anyhow!("Failed to parse backup selection"))?;
    
    handle_backup_restore(backup_id, false).await
}

/// Format backup age in a human-readable way
fn format_backup_age(timestamp: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*timestamp);
    
    if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minutes ago", duration.num_minutes())
    } else {
        "just now".to_string()
    }
}

/// Show backup system status
pub async fn handle_backup_status() -> Result<()> {
    let backup_dir = get_backup_dir()?;
    
    println!("📦 Backup System Status\n");
    
    // Check if backup system is available
    let system_available = crate::backup::is_backup_system_available().await;
    println!("🔧 System Status: {}", 
        if system_available { "✅ Available".green() } else { "❌ Unavailable".red() }
    );
    
    println!("📁 Backup Directory: {}", backup_dir.display());
    
    if backup_dir.exists() {
        // Calculate total backup size
        let mut total_size = 0u64;
        let mut total_files = 0u32;
        
        let mut entries = fs::read_dir(&backup_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                if let Ok(size) = calculate_dir_size(&entry.path()).await {
                    total_size += size;
                    total_files += 1;
                }
            }
        }
        
        println!("💾 Storage Used: {} ({} backup directories)", 
            format_size(total_size), total_files);
    } else {
        println!("💾 Storage Used: Directory not created yet");
    }
    
    // Load and show backup metadata
    match load_backup_metadata().await {
        Ok(metadata) => {
            println!("📊 Backup Count: {}", metadata.backups.len());
            
            if !metadata.backups.is_empty() {
                let newest = metadata.backups.iter()
                    .max_by(|a, b| a.timestamp.cmp(&b.timestamp))
                    .unwrap();
                let oldest = metadata.backups.iter()
                    .min_by(|a, b| a.timestamp.cmp(&b.timestamp))
                    .unwrap();
                
                println!("📅 Newest Backup: {} ({})", 
                    newest.id, format_backup_age(&newest.timestamp));
                println!("📅 Oldest Backup: {} ({})", 
                    oldest.id, format_backup_age(&oldest.timestamp));
            }
        }
        Err(_) => {
            println!("📊 Backup Count: 0 (metadata not found)");
        }
    }
    
    Ok(())
}

/// Calculate directory size recursively
fn calculate_dir_size(path: &Path) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<u64>> + Send + '_>> {
    Box::pin(async move {
        let mut total_size = 0;
        let mut entries = fs::read_dir(path).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            if metadata.is_file() {
                total_size += metadata.len();
            } else if metadata.is_dir() {
                total_size += calculate_dir_size(&entry.path()).await?;
            }
        }
        
        Ok(total_size)
    })
}

/// Format file size in human-readable format
fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    const THRESHOLD: u64 = 1024;
    
    if bytes < THRESHOLD {
        return format!("{} B", bytes);
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD as f64;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}