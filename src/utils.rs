use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::path::Path;
use std::process::Command;

/// Detect if we're running in development mode based on binary name
pub fn is_dev_mode() -> bool {
    std::env::args()
        .next()
        .map(|arg0| {
            std::path::Path::new(&arg0)
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name == "_pm")
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

/// Get the appropriate binary name for help messages
pub fn get_binary_name() -> &'static str {
    if is_dev_mode() {
        "_pm"
    } else {
        "pm"
    }
}

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

#[allow(dead_code)]
pub fn get_git_remote_url(path: &Path) -> Result<Option<String>> {
    if !path.join(".git").exists() {
        return Ok(None);
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .context("Failed to execute git remote command")?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        let url = stdout.trim();
        if url.is_empty() {
            Ok(None)
        } else {
            Ok(Some(url.to_string()))
        }
    } else {
        Ok(None)
    }
}

#[allow(dead_code)]
pub fn get_git_current_branch(path: &Path) -> Result<Option<String>> {
    if !path.join(".git").exists() {
        return Ok(None);
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("branch")
        .arg("--show-current")
        .output()
        .context("Failed to execute git branch command")?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        let branch = stdout.trim();
        if branch.is_empty() {
            Ok(None)
        } else {
            Ok(Some(branch.to_string()))
        }
    } else {
        Ok(None)
    }
}

#[allow(dead_code)]
pub fn get_git_status(path: &Path) -> Result<Option<String>> {
    if !path.join(".git").exists() {
        return Ok(None);
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("status")
        .arg("--porcelain")
        .output()
        .context("Failed to execute git status command")?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        let status = stdout.trim();
        if status.is_empty() {
            Ok(Some("clean".to_string()))
        } else {
            let lines: Vec<&str> = status.lines().collect();
            Ok(Some(format!("{} changes", lines.len())))
        }
    } else {
        Ok(None)
    }
}

#[allow(dead_code)]
pub fn is_git_repository(path: &Path) -> bool {
    path.join(".git").exists()
}

/// Check if a project has direnv configuration
pub fn has_direnv_config(path: &Path) -> bool {
    path.join(".envrc").exists()
}

/// Check if direnv is currently active in the project
pub fn is_direnv_active(path: &Path) -> bool {
    if !has_direnv_config(path) {
        return false;
    }

    // Check if DIRENV_DIR environment variable is set and matches the project path
    if let Ok(direnv_dir) = std::env::var("DIRENV_DIR") {
        if let Ok(canonical_path) = path.canonicalize() {
            return direnv_dir == canonical_path.to_string_lossy();
        }
    }

    false
}

/// Get list of active git hooks in a project
pub fn get_active_git_hooks(path: &Path) -> Vec<String> {
    let hooks_dir = path.join(".git").join("hooks");
    if !hooks_dir.exists() {
        return Vec::new();
    }

    let mut active_hooks = Vec::new();
    let hook_names = [
        "pre-commit",
        "commit-msg",
        "pre-push",
        "post-commit",
        "post-checkout",
        "pre-rebase",
    ];

    for hook_name in hook_names {
        let hook_path = hooks_dir.join(hook_name);
        if hook_path.exists() && hook_path.is_file() {
            // Check if it's executable
            if let Ok(metadata) = std::fs::metadata(&hook_path) {
                use std::os::unix::fs::PermissionsExt;
                if metadata.permissions().mode() & 0o111 != 0 {
                    active_hooks.push(hook_name.to_string());
                }
            }
        }
    }

    active_hooks
}

/// Check if project has PM hooks template available
pub fn has_pm_hooks_template(path: &Path) -> bool {
    path.join("hooks").exists()
}

/// Get the status of PM hooks installation
pub fn get_pm_hooks_status(path: &Path) -> String {
    let hooks_dir = path.join(".git").join("hooks");
    let pm_hooks_dir = path.join("hooks");

    if !pm_hooks_dir.exists() {
        return "No PM hooks template".to_string();
    }

    if !hooks_dir.exists() {
        return "Not a git repository".to_string();
    }

    let pm_hooks = ["pre-commit", "commit-msg", "pre-push"];
    let mut installed_count = 0;

    for hook_name in pm_hooks {
        let hook_path = hooks_dir.join(hook_name);
        let pm_hook_path = pm_hooks_dir.join(hook_name);

        if hook_path.exists() && pm_hook_path.exists() {
            // Check if it's a symlink pointing to PM hooks
            if let Ok(link_target) = std::fs::read_link(&hook_path) {
                if link_target.to_string_lossy().contains("hooks/") {
                    installed_count += 1;
                }
            }
        }
    }

    match installed_count {
        0 => "Not installed".to_string(),
        count if count == pm_hooks.len() => "Fully installed".to_string(),
        count => format!("Partially installed ({}/{})", count, pm_hooks.len()),
    }
}

#[allow(dead_code)]
pub fn detect_project_language(path: &Path) -> Option<String> {
    let files = std::fs::read_dir(path).ok()?;
    let mut language_counts: std::collections::HashMap<&str, u32> =
        std::collections::HashMap::new();

    for entry in files.flatten() {
        if let Some(extension) = entry.path().extension() {
            if let Some(ext_str) = extension.to_str() {
                let language = match ext_str {
                    "rs" => "Rust",
                    "js" | "jsx" => "JavaScript",
                    "ts" | "tsx" => "TypeScript",
                    "py" => "Python",
                    "go" => "Go",
                    "java" => "Java",
                    "cpp" | "cc" | "cxx" => "C++",
                    "c" => "C",
                    "rb" => "Ruby",
                    "php" => "PHP",
                    "swift" => "Swift",
                    "kt" => "Kotlin",
                    "dart" => "Dart",
                    "scala" => "Scala",
                    "clj" => "Clojure",
                    "hs" => "Haskell",
                    "ml" => "OCaml",
                    "fs" => "F#",
                    "elm" => "Elm",
                    "ex" | "exs" => "Elixir",
                    "erl" => "Erlang",
                    "lua" => "Lua",
                    "r" => "R",
                    "jl" => "Julia",
                    "nim" => "Nim",
                    "zig" => "Zig",
                    "v" => "V",
                    "cr" => "Crystal",
                    "d" => "D",
                    _ => continue,
                };
                *language_counts.entry(language).or_insert(0) += 1;
            }
        }
    }

    language_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(lang, _)| lang.to_string())
}
