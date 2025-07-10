use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::process::Command;
use std::path::Path;

pub fn get_last_git_commit_time(path: &Path) -> Result<Option<DateTime<Utc>>> {
    if !path.join(".git").exists() {
        return Ok(None);
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("log")
        .arg("-1")
        .arg("--format=%aI")
        .output()
        .context("Failed to execute git command")?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        let timestamp_str = stdout.trim();
        if timestamp_str.is_empty() {
            Ok(None)
        } else {
            let datetime = DateTime::parse_from_rfc3339(timestamp_str)?;
            Ok(Some(datetime.with_timezone(&Utc)))
        }
    } else {
        let stderr = String::from_utf8(output.stderr)?;
        eprintln!("Error getting git commit time: {}", stderr);
        Ok(None)
    }
}
