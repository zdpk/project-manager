use crate::config::Config;
use crate::utils::{get_active_git_hooks, get_pm_hooks_status, has_pm_hooks_template};
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

/// Install PM hooks in the current project
pub async fn install_hooks(_config: &Config, project_path: Option<&Path>) -> Result<()> {
    let target_path = if let Some(path) = project_path {
        path.to_path_buf()
    } else {
        std::env::current_dir().context("Failed to get current directory")?
    };

    // Check if it's a git repository
    if !target_path.join(".git").exists() {
        return Err(anyhow!("Not a git repository: {}", target_path.display()));
    }

    // Check if PM hooks template exists
    if !has_pm_hooks_template(&target_path) {
        return Err(anyhow!(
            "No PM hooks template found in {}. Please create a hooks/ directory with hook scripts.",
            target_path.display()
        ));
    }

    let hooks_dir = target_path.join(".git").join("hooks");
    let pm_hooks_dir = target_path.join("hooks");

    // Create .git/hooks directory if it doesn't exist
    if !hooks_dir.exists() {
        fs::create_dir_all(&hooks_dir).context("Failed to create hooks directory")?;
    }

    let hook_names = ["pre-commit", "commit-msg", "pre-push"];
    let mut installed_count = 0;

    for hook_name in hook_names {
        let hook_path = hooks_dir.join(hook_name);
        let pm_hook_path = pm_hooks_dir.join(hook_name);

        if !pm_hook_path.exists() {
            println!("⚠️  PM hook '{}' not found in hooks/ directory", hook_name);
            continue;
        }

        // Remove existing hook if it exists
        if hook_path.exists() {
            fs::remove_file(&hook_path).context("Failed to remove existing hook")?;
        }

        // Create relative symlink
        let relative_path = format!("../../hooks/{}", hook_name);
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&relative_path, &hook_path)
                .context(format!("Failed to create symlink for {}", hook_name))?;
        }
        #[cfg(not(unix))]
        {
            // For non-Unix systems, copy the file instead
            fs::copy(&pm_hook_path, &hook_path)
                .context(format!("Failed to copy hook {}", hook_name))?;
        }

        println!("✅ Installed hook: {}", hook_name);
        installed_count += 1;
    }

    if installed_count > 0 {
        println!(
            "🎉 Successfully installed {} PM hooks in {}",
            installed_count,
            target_path.display()
        );
    } else {
        println!("❌ No hooks were installed");
    }

    Ok(())
}

/// Uninstall PM hooks from the current project
pub async fn uninstall_hooks(_config: &Config, project_path: Option<&Path>) -> Result<()> {
    let target_path = if let Some(path) = project_path {
        path.to_path_buf()
    } else {
        std::env::current_dir().context("Failed to get current directory")?
    };

    let hooks_dir = target_path.join(".git").join("hooks");
    if !hooks_dir.exists() {
        return Err(anyhow!("Not a git repository: {}", target_path.display()));
    }

    let hook_names = ["pre-commit", "commit-msg", "pre-push"];
    let mut removed_count = 0;

    for hook_name in hook_names {
        let hook_path = hooks_dir.join(hook_name);

        if hook_path.exists() {
            // Check if it's a PM hook (symlink or contains PM signature)
            let is_pm_hook = if let Ok(link_target) = fs::read_link(&hook_path) {
                link_target.to_string_lossy().contains("hooks/")
            } else if let Ok(content) = fs::read_to_string(&hook_path) {
                content.contains("PM projects") || content.contains("Pre-commit hook for PM")
            } else {
                false
            };

            if is_pm_hook {
                fs::remove_file(&hook_path).context("Failed to remove hook")?;
                println!("✅ Removed hook: {}", hook_name);
                removed_count += 1;
            } else {
                println!("⚠️  Skipping '{}' (not a PM hook)", hook_name);
            }
        }
    }

    if removed_count > 0 {
        println!(
            "🎉 Successfully removed {} PM hooks from {}",
            removed_count,
            target_path.display()
        );
    } else {
        println!("❌ No PM hooks were found to remove");
    }

    Ok(())
}

/// List hooks status for the current project
pub async fn list_hooks(_config: &Config, project_path: Option<&Path>) -> Result<()> {
    let target_path = if let Some(path) = project_path {
        path.to_path_buf()
    } else {
        std::env::current_dir().context("Failed to get current directory")?
    };

    println!("🔍 Hooks status for: {}", target_path.display());
    println!();

    // Check if it's a git repository
    if !target_path.join(".git").exists() {
        println!("❌ Not a git repository");
        return Ok(());
    }

    // PM hooks status
    let pm_hooks_status = get_pm_hooks_status(&target_path);
    println!("📋 PM Hooks: {}", pm_hooks_status);

    // Active git hooks
    let active_hooks = get_active_git_hooks(&target_path);
    if active_hooks.is_empty() {
        println!("📋 Active Git Hooks: None");
    } else {
        println!("📋 Active Git Hooks: {}", active_hooks.join(", "));
    }

    // PM hooks template availability
    if has_pm_hooks_template(&target_path) {
        println!("📋 PM Hooks Template: Available");
    } else {
        println!("📋 PM Hooks Template: Not found");
    }

    Ok(())
}

/// Create PM hooks template in the current project
pub async fn init_hooks(_config: &Config, project_path: Option<&Path>) -> Result<()> {
    let target_path = if let Some(path) = project_path {
        path.to_path_buf()
    } else {
        std::env::current_dir().context("Failed to get current directory")?
    };

    let hooks_dir = target_path.join("hooks");

    if hooks_dir.exists() {
        return Err(anyhow!(
            "Hooks directory already exists: {}",
            hooks_dir.display()
        ));
    }

    // Create hooks directory
    fs::create_dir_all(&hooks_dir).context("Failed to create hooks directory")?;

    // Create hook templates (use embedded content)
    let pre_commit_content = r#"#!/bin/bash
# Pre-commit hook for PM projects
# This hook runs before each commit to ensure code quality

set -e

echo "🔍 Running pre-commit checks..."

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ cargo not found. Please install Rust."
    exit 1
fi

# Run cargo fmt check
echo "📝 Checking code formatting..."
if ! cargo fmt --all --check; then
    echo "❌ Code formatting issues found. Run 'cargo fmt --all' to fix."
    exit 1
fi

# Run cargo clippy
echo "🔧 Running clippy lints..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy lints failed. Please fix the issues."
    exit 1
fi

# Run tests
echo "🧪 Running tests..."
if ! cargo test --all; then
    echo "❌ Tests failed. Please fix failing tests."
    exit 1
fi

echo "✅ Pre-commit checks passed!"
"#;

    let commit_msg_content = r#"#!/bin/bash
# Commit message hook for PM projects
# This hook validates commit message format

set -e

commit_msg_file=$1
commit_msg=$(cat "$commit_msg_file")

echo "📝 Validating commit message format..."

# Check if commit message is empty
if [[ -z "${commit_msg// }" ]]; then
    echo "❌ Empty commit message"
    exit 1
fi

# Check minimum length
if [[ ${#commit_msg} -lt 10 ]]; then
    echo "❌ Commit message too short (minimum 10 characters)"
    echo "Current: ${#commit_msg} characters"
    exit 1
fi

# Check maximum length of first line
first_line=$(echo "$commit_msg" | head -n1)
if [[ ${#first_line} -gt 72 ]]; then
    echo "❌ First line too long (maximum 72 characters)"
    echo "Current: ${#first_line} characters"
    exit 1
fi

# Check for conventional commit format (optional)
if [[ "$commit_msg" =~ ^(feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert)(\(.+\))?:\ .+ ]]; then
    echo "✅ Conventional commit format detected"
else
    echo "💡 Consider using conventional commit format: <type>(<scope>): <description>"
fi

echo "✅ Commit message validation passed!"
"#;

    let pre_push_content = r#"#!/bin/bash
# Pre-push hook for PM projects
# This hook runs before pushing to remote repository

set -e

echo "🚀 Running pre-push checks..."

# Check if there are any uncommitted changes
if ! git diff --quiet; then
    echo "❌ Uncommitted changes found. Please commit or stash them."
    exit 1
fi

# Check if there are any unstaged changes
if ! git diff --cached --quiet; then
    echo "❌ Unstaged changes found. Please add and commit them."
    exit 1
fi

# Run a final build check
echo "🔨 Running final build check..."
if ! cargo build --release; then
    echo "❌ Release build failed. Please fix build errors."
    exit 1
fi

# Check if documentation builds
echo "📚 Checking documentation build..."
if ! cargo doc --no-deps --quiet; then
    echo "❌ Documentation build failed. Please fix doc errors."
    exit 1
fi

echo "✅ Pre-push checks passed!"
"#;

    let readme_content = r#"# PM Project Git Hooks

This directory contains Git hooks templates for PM projects to ensure code quality and consistency.

## Available Hooks

### pre-commit
- Runs before each commit
- Checks code formatting with `cargo fmt`
- Runs linting with `cargo clippy`
- Runs tests with `cargo test`

### commit-msg
- Validates commit message format
- Ensures minimum/maximum length requirements
- Suggests conventional commit format

### pre-push
- Runs before pushing to remote
- Checks for uncommitted changes
- Runs release build verification
- Checks documentation build

## Installation

To install these hooks in your project:

```bash
# Make hooks executable
chmod +x hooks/*

# Install hooks (creates symlinks)
ln -sf ../../hooks/pre-commit .git/hooks/pre-commit
ln -sf ../../hooks/commit-msg .git/hooks/commit-msg
ln -sf ../../hooks/pre-push .git/hooks/pre-push
```

Or use the PM command:
```bash
pm hooks install
```

## Customization

You can customize these hooks by:
1. Modifying the templates in the `hooks/` directory
2. Creating project-specific hooks
3. Disabling specific checks by commenting out sections

## Best Practices

1. **Keep hooks fast**: Avoid long-running operations
2. **Make them fail-safe**: Exit on first error
3. **Provide clear feedback**: Show what's being checked
4. **Allow bypassing**: Use `git commit --no-verify` if needed
5. **Version control**: Keep hooks in the repository for team sharing
"#;

    fs::write(hooks_dir.join("pre-commit"), pre_commit_content)
        .context("Failed to create pre-commit hook")?;
    fs::write(hooks_dir.join("commit-msg"), commit_msg_content)
        .context("Failed to create commit-msg hook")?;
    fs::write(hooks_dir.join("pre-push"), pre_push_content)
        .context("Failed to create pre-push hook")?;
    fs::write(hooks_dir.join("README.md"), readme_content).context("Failed to create README.md")?;

    // Make hooks executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o755);
        fs::set_permissions(hooks_dir.join("pre-commit"), perms.clone())
            .context("Failed to set permissions for pre-commit")?;
        fs::set_permissions(hooks_dir.join("commit-msg"), perms.clone())
            .context("Failed to set permissions for commit-msg")?;
        fs::set_permissions(hooks_dir.join("pre-push"), perms)
            .context("Failed to set permissions for pre-push")?;
    }

    println!("✅ Created PM hooks template in {}", hooks_dir.display());
    println!("💡 Run 'pm hooks install' to install these hooks");

    Ok(())
}
