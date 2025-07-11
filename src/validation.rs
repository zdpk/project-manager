use std::path::PathBuf;
use chrono::Duration;
use anyhow::{Result, Context};
use crate::constants::*;

pub fn validate_path(path: &PathBuf) -> Result<PathBuf> {
    if !path.exists() {
        anyhow::bail!(
            "Path does not exist: {}\n\n💡 Suggestions:\n  - {}\n  - Create the directory first: mkdir -p {}",
            path.display(),
            SUGGESTION_CHECK_PATH,
            path.display()
        );
    }

    if !path.is_dir() {
        anyhow::bail!(
            "Path is not a directory: {}\n\n💡 Please provide a directory path, not a file.",
            path.display()
        );
    }

    path.canonicalize()
        .with_context(|| format!("Failed to resolve absolute path for: {}", path.display()))
}

pub fn parse_time_duration(duration_str: &str) -> Result<Duration, String> {
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

pub fn validate_project_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Project name cannot be empty".to_string());
    }

    if name.len() > 100 {
        return Err("Project name too long (max 100 characters)".to_string());
    }

    // Check for invalid characters
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    if name.chars().any(|c| invalid_chars.contains(&c)) {
        return Err("Project name contains invalid characters".to_string());
    }

    Ok(())
}

pub fn validate_tags(tags: &[String]) -> Result<(), String> {
    for tag in tags {
        if tag.is_empty() {
            return Err("Tag cannot be empty".to_string());
        }

        if tag.len() > 50 {
            return Err(format!("Tag '{}' too long (max 50 characters)", tag));
        }

        // Check for invalid characters
        if tag.contains(',') {
            return Err(format!("Tag '{}' cannot contain commas", tag));
        }

        if tag.chars().any(|c| c.is_whitespace()) {
            return Err(format!("Tag '{}' cannot contain whitespace", tag));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time_duration() {
        assert_eq!(parse_time_duration("30s").unwrap(), Duration::seconds(30));
        assert_eq!(parse_time_duration("15m").unwrap(), Duration::minutes(15));
        assert_eq!(parse_time_duration("2h").unwrap(), Duration::hours(2));
        assert_eq!(parse_time_duration("7d").unwrap(), Duration::days(7));
        assert_eq!(parse_time_duration("2w").unwrap(), Duration::weeks(2));
        assert_eq!(parse_time_duration("1y").unwrap(), Duration::days(365));
        
        // Default to days if no unit specified
        assert_eq!(parse_time_duration("7").unwrap(), Duration::days(7));
        
        // Error cases
        assert!(parse_time_duration("").is_err());
        assert!(parse_time_duration("abc").is_err());
        assert!(parse_time_duration("7x").is_err());
    }

    #[test]
    fn test_validate_project_name() {
        assert!(validate_project_name("valid-project").is_ok());
        assert!(validate_project_name("project_123").is_ok());
        assert!(validate_project_name("").is_err());
        assert!(validate_project_name("a".repeat(101).as_str()).is_err());
        assert!(validate_project_name("invalid/name").is_err());
        assert!(validate_project_name("invalid:name").is_err());
    }

    #[test]
    fn test_validate_tags() {
        assert!(validate_tags(&["rust".to_string(), "cli".to_string()]).is_ok());
        assert!(validate_tags(&["".to_string()]).is_err());
        assert!(validate_tags(&["a".repeat(51)]).is_err());
        assert!(validate_tags(&["invalid,tag".to_string()]).is_err());
        assert!(validate_tags(&["invalid tag".to_string()]).is_err());
    }
}