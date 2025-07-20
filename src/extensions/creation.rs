use anyhow::{Context, Result};
use chrono::Datelike;
use std::path::{Path, PathBuf};
use tokio::fs;

use super::platform::PlatformSelection;
use super::templates::{TemplateContext, WorkflowTemplate, ExtensionTemplate as TemplateGenerator};

/// Extension creation configuration
#[derive(Debug, Clone)]
pub struct ExtensionCreationConfig {
    pub name: String,
    pub description: String,
    pub author: String,
    pub email: Option<String>,
    pub template_type: ExtensionTemplateType,
    pub platforms: PlatformSelection,
    pub target_directory: PathBuf,
    pub init_git: bool,
    pub create_github_repo: bool,
}

/// Extension template types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ExtensionTemplateType {
    Rust,
    Bash,
    Python,
}

impl ExtensionTemplateType {
    /// Get all available templates
    pub fn all() -> Vec<(&'static str, ExtensionTemplateType)> {
        vec![
            ("Rust (Recommended for performance)", ExtensionTemplateType::Rust),
            ("Bash (Simple scripts)", ExtensionTemplateType::Bash),
            ("Python (Rich ecosystem)", ExtensionTemplateType::Python),
        ]
    }
    
    /// Get template name
    pub fn name(&self) -> &'static str {
        match self {
            ExtensionTemplateType::Rust => "Rust",
            ExtensionTemplateType::Bash => "Bash", 
            ExtensionTemplateType::Python => "Python",
        }
    }
}

/// Extension creator for handling extension generation
pub struct ExtensionCreator;

impl ExtensionCreator {
    /// Interactive extension creation wizard
    pub async fn interactive_create() -> Result<()> {
        println!("🚀 Creating a new PM extension...\n");
        
        // Gather basic information
        let name = inquire::Text::new("Extension name:")
            .with_help_message("Use kebab-case (e.g., 'git-hooks', 'deploy-tool')")
            .prompt()?;
        
        // Validate extension name
        Self::validate_extension_name(&name)?;
        
        let description = inquire::Text::new("Extension description:")
            .with_help_message("Brief description of what your extension does")
            .prompt()?;
        
        let author = inquire::Text::new("Author name:")
            .with_default(&Self::get_default_author())
            .prompt()?;
        
        let email = inquire::Text::new("Author email (optional):")
            .with_default("")
            .prompt()
            .ok()
            .filter(|s| !s.is_empty());
        
        // Template selection
        let template_options = ExtensionTemplateType::all();
        let template_names: Vec<&str> = template_options.iter().map(|(name, _)| *name).collect();
        let selected_name = inquire::Select::new("Choose template:", template_names).prompt()?;
        
        let template_type = template_options.iter()
            .find(|(name, _)| *name == selected_name)
            .map(|(_, template)| template.clone())
            .unwrap_or(ExtensionTemplateType::Rust);
        
        // Platform selection (only for compiled languages)
        let platforms = match template_type {
            ExtensionTemplateType::Rust => {
                // Rust needs platform-specific binaries
                PlatformSelection::interactive_selection()?
            }
            ExtensionTemplateType::Bash | ExtensionTemplateType::Python => {
                // Interpreted languages are platform-universal
                println!("📦 Platform selection skipped (interpreted language - universal compatibility)");
                PlatformSelection::universal()
            }
        };
        
        // Target directory
        let default_dir = format!("./pm-ext-{}", name);
        let target_directory = inquire::Text::new("Extension directory:")
            .with_default(&default_dir)
            .prompt()?;
        
        let target_path = PathBuf::from(shellexpand::tilde(&target_directory).into_owned());
        
        // Git initialization
        let init_git = inquire::Confirm::new("Initialize git repository?")
            .with_default(true)
            .prompt()?;
        
        let create_github_repo = if init_git {
            inquire::Confirm::new("Create GitHub repository?")
                .with_default(false)
                .prompt()?
        } else {
            false
        };
        
        let config = ExtensionCreationConfig {
            name,
            description,
            author,
            email,
            template_type,
            platforms,
            target_directory: target_path,
            init_git,
            create_github_repo,
        };
        
        // Create the extension
        Self::create_extension(config).await?;
        
        Ok(())
    }
    
    /// Create extension from configuration
    pub async fn create_extension(config: ExtensionCreationConfig) -> Result<()> {
        let target_dir = &config.target_directory;
        
        // Check if directory already exists
        if target_dir.exists() {
            return Err(anyhow::anyhow!(
                "Directory '{}' already exists. Please choose a different location.",
                target_dir.display()
            ));
        }
        
        println!("📁 Creating extension directory: {}", target_dir.display());
        fs::create_dir_all(target_dir).await
            .with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;
        
        // Create template context
        let template_context = TemplateContext::new(
            config.name.clone(),
            config.description.clone(),
            config.author.clone(),
            config.platforms.clone(),
            config.template_type.clone(),
        );
        
        // Generate files based on template type
        match config.template_type {
            ExtensionTemplateType::Rust => {
                Self::create_rust_extension(target_dir, &template_context).await?;
            }
            ExtensionTemplateType::Bash => {
                Self::create_bash_extension(target_dir, &template_context).await?;
            }
            ExtensionTemplateType::Python => {
                Self::create_python_extension(target_dir, &template_context).await?;
            }
        }
        
        // Create common files
        Self::create_common_files(target_dir, &template_context).await?;
        
        // Initialize git if requested
        if config.init_git {
            Self::init_git_repository(target_dir).await?;
        }
        
        // Create GitHub repository if requested
        if config.create_github_repo {
            Self::create_github_repository(target_dir, &config).await?;
        }
        
        // Success message
        Self::print_success_message(&config);
        
        Ok(())
    }
    
    /// Create Rust extension files with new bin/ structure
    async fn create_rust_extension(target_dir: &Path, context: &TemplateContext) -> Result<()> {
        // Create src directory for source code
        let src_dir = target_dir.join("src");
        fs::create_dir_all(&src_dir).await?;
        
        // Create bin/ directory for compiled binary (new structure)
        let bin_dir = target_dir.join("bin");
        fs::create_dir_all(&bin_dir).await?;
        
        // Generate Cargo.toml
        let cargo_toml = TemplateGenerator::generate_cargo_toml(context);
        fs::write(target_dir.join("Cargo.toml"), cargo_toml).await?;
        
        // Generate main.rs
        let main_rs = TemplateGenerator::generate_main_rs(context);
        fs::write(src_dir.join("main.rs"), main_rs).await?;
        
        // Create tests directory
        let tests_dir = target_dir.join("tests");
        fs::create_dir_all(&tests_dir).await?;
        
        let integration_test = r#"use std::process::Command;

#[test]
fn test_extension_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute extension");
    
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("PM Extension"));
}
"#;
        fs::write(tests_dir.join("integration_tests.rs"), integration_test).await?;
        
        Ok(())
    }
    
    /// Create Bash extension files with new folder structure
    async fn create_bash_extension(target_dir: &Path, context: &TemplateContext) -> Result<()> {
        // Create bash/ directory
        let bash_dir = target_dir.join("bash");
        fs::create_dir_all(&bash_dir).await?;
        
        // Generate example.sh only (simplified approach)
        let example_script = Self::generate_bash_example_script(context);
        let example_path = bash_dir.join("example.sh");
        fs::write(&example_path, example_script).await?;
        
        // Make script executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&example_path).await?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&example_path, perms).await?;
        }
        
        Ok(())
    }
    
    /// Generate example.sh script for Bash extensions
    fn generate_bash_example_script(context: &TemplateContext) -> String {
        format!(
            r#"#!/bin/bash

# {} - {}
# Author: {}
# Generated with PM Extension Template

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_info() {{ echo -e "${{BLUE}}ℹ️  $1${{NC}}"; }}
print_success() {{ echo -e "${{GREEN}}✅ $1${{NC}}"; }}
print_warning() {{ echo -e "${{YELLOW}}⚠️  $1${{NC}}"; }}
print_error() {{ echo -e "${{RED}}❌ $1${{NC}}"; }}

# Show help if requested
if [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage: pm {} [COMMAND] [OPTIONS]"
    echo ""
    echo "Available Commands:"
    echo "  help       Show this help"
    echo ""
    echo "PM Environment Variables:"
    echo "  PM_CURRENT_PROJECT - Current project context"
    echo "  PM_CONFIG_PATH     - PM configuration path"
    echo "  PM_VERSION         - PM version"
    echo ""
    echo "Extension: {}"
    echo "Description: {}"
    echo "Author: {}"
    exit 0
fi

# Main extension functionality
print_success "{} Extension"

if [ -n "$PM_CURRENT_PROJECT" ]; then
    print_info "Current PM project: $PM_CURRENT_PROJECT"
fi

if [ -n "$PM_CONFIG_PATH" ]; then
    print_info "PM config: $PM_CONFIG_PATH"
fi

# Handle command line arguments
message="${{1:-Hello from PM extension!}}"
if [ "$#" -gt 0 ]; then
    print_success "Message: $message"
else
    print_success "Default message: $message"
fi

echo "🎯 This is the {} extension"
echo "🔧 You can modify this file to implement your extension functionality"
echo "💡 Try: pm {} help"
"#,
            context.name.to_uppercase(),
            context.description,
            context.author,
            context.name,
            context.name,
            context.description,
            context.author,
            context.name,
            context.name,
            context.name
        )
    }
    
    /// Create Python extension files with new folder structure
    async fn create_python_extension(target_dir: &Path, context: &TemplateContext) -> Result<()> {
        // Create python/ directory
        let python_dir = target_dir.join("python");
        fs::create_dir_all(&python_dir).await?;
        
        // Generate main.py
        let main_script = Self::generate_python_main_script(context);
        let main_path = python_dir.join("main.py");
        fs::write(&main_path, main_script).await?;
        
        // Generate example.py
        let example_script = Self::generate_python_example_script(context);
        let example_path = python_dir.join("example.py");
        fs::write(&example_path, example_script).await?;
        
        // Generate help.py
        let help_script = Self::generate_python_help_script(context);
        let help_path = python_dir.join("help.py");
        fs::write(&help_path, help_script).await?;
        
        // Make all scripts executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for script_path in [&main_path, &example_path, &help_path] {
                let mut perms = fs::metadata(script_path).await?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(script_path, perms).await?;
            }
        }
        
        // Create requirements.txt
        let requirements_content = "# Add Python dependencies here\n# Example:\n# requests>=2.25.0\n# click>=8.0.0\n";
        fs::write(target_dir.join("requirements.txt"), requirements_content).await?;
        
        Ok(())
    }
    
    /// Generate main.py script for Python extensions
    fn generate_python_main_script(context: &TemplateContext) -> String {
        format!(
            r#"#!/usr/bin/env python3

"""
{} - {}
Author: {}
Generated with PM Extension Template
"""

import os
import sys


class Colors:
    RED = '\033[0;31m'
    GREEN = '\033[0;32m'
    YELLOW = '\033[1;33m'
    BLUE = '\033[0;34m'
    NC = '\033[0m'


def print_info(message: str) -> None:
    print(f"{{Colors.BLUE}}ℹ️  {{message}}{{Colors.NC}}")


def print_success(message: str) -> None:
    print(f"{{Colors.GREEN}}✅ {{message}}{{Colors.NC}}")


def print_warning(message: str) -> None:
    print(f"{{Colors.YELLOW}}⚠️  {{message}}{{Colors.NC}}")


def print_error(message: str) -> None:
    print(f"{{Colors.RED}}❌ {{message}}{{Colors.NC}}")


def main():
    """Main entry point for the extension."""
    print_success("{} Extension - Main Command")
    
    # Check PM environment variables
    pm_project = os.environ.get('PM_CURRENT_PROJECT')
    if pm_project:
        print_info(f"Current PM project: {{pm_project}}")
    
    pm_config = os.environ.get('PM_CONFIG_PATH')
    if pm_config:
        print_info(f"PM config: {{pm_config}}")
    
    print("🔧 This is the main command for the {} extension")
    print("📝 Replace this with your extension's main functionality")


if __name__ == "__main__":
    main()
"#,
            context.name.to_uppercase(),
            context.description,
            context.author,
            context.name,
            context.name
        )
    }
    
    /// Generate example.py script for Python extensions
    fn generate_python_example_script(context: &TemplateContext) -> String {
        format!(
            r#"#!/usr/bin/env python3

"""
{} - Example Command
Author: {}
Generated with PM Extension Template
"""

import os
import sys


class Colors:
    RED = '\033[0;31m'
    GREEN = '\033[0;32m'
    YELLOW = '\033[1;33m'
    BLUE = '\033[0;34m'
    NC = '\033[0m'


def print_info(message: str) -> None:
    print(f"{{Colors.BLUE}}ℹ️  {{message}}{{Colors.NC}}")


def print_success(message: str) -> None:
    print(f"{{Colors.GREEN}}✅ {{message}}{{Colors.NC}}")


def print_warning(message: str) -> None:
    print(f"{{Colors.YELLOW}}⚠️  {{message}}{{Colors.NC}}")


def print_error(message: str) -> None:
    print(f"{{Colors.RED}}❌ {{message}}{{Colors.NC}}")


def main():
    """Example command implementation."""
    message = sys.argv[1] if len(sys.argv) > 1 else "Hello from PM extension!"
    print_success(message)
    
    # Check PM environment variables
    pm_project = os.environ.get('PM_CURRENT_PROJECT')
    if pm_project:
        print_info(f"Current PM project: {{pm_project}}")
    
    pm_config = os.environ.get('PM_CONFIG_PATH')
    if pm_config:
        print_info(f"PM config: {{pm_config}}")
    
    print("🎯 This is an example command for the {} extension")
    print("🔧 You can modify this file to implement your example functionality")


if __name__ == "__main__":
    main()
"#,
            context.name.to_uppercase(),
            context.author,
            context.name
        )
    }
    
    /// Generate help.py script for Python extensions
    fn generate_python_help_script(context: &TemplateContext) -> String {
        format!(
            r#"#!/usr/bin/env python3

"""
{} - Help Command
Author: {}
Generated with PM Extension Template
"""


def main():
    """Show help information for the extension."""
    print("Usage: pm {} [COMMAND] [OPTIONS]")
    print("")
    print("Commands:")
    print("  main       Main functionality (default)")
    print("  example    Example command")
    print("  help       Show this help")
    print("")
    print("PM Environment Variables:")
    print("  PM_CURRENT_PROJECT - Current project context")
    print("  PM_CONFIG_PATH     - PM configuration path")
    print("  PM_VERSION         - PM version")
    print("")
    print("Extension: {}")
    print("Description: {}")
    print("Author: {}")


if __name__ == "__main__":
    main()
"#,
            context.name.to_uppercase(),
            context.author,
            context.name,
            context.name,
            context.description,
            context.author
        )
    }
    
    /// Create common files (README, .gitignore, workflows, etc.)
    async fn create_common_files(target_dir: &Path, context: &TemplateContext) -> Result<()> {
        // Create .github/workflows directory
        let workflows_dir = target_dir.join(".github/workflows");
        fs::create_dir_all(&workflows_dir).await?;
        
        // Generate GitHub Actions workflow
        let workflow_content = WorkflowTemplate::generate_release_workflow(context);
        fs::write(workflows_dir.join("release.yml"), workflow_content).await?;
        
        // Generate README.md
        let readme_content = TemplateGenerator::generate_readme(context);
        fs::write(target_dir.join("README.md"), readme_content).await?;
        
        // Generate .gitignore
        let gitignore_content = TemplateGenerator::generate_gitignore();
        fs::write(target_dir.join(".gitignore"), gitignore_content).await?;
        
        // Generate extension.yml manifest
        let manifest_content = TemplateGenerator::generate_extension_manifest(context);
        fs::write(target_dir.join("extension.yml"), manifest_content).await?;
        
        // Create LICENSE file
        let license_content = Self::generate_mit_license(&context.author);
        fs::write(target_dir.join("LICENSE"), license_content).await?;
        
        Ok(())
    }
    
    /// Initialize git repository
    async fn init_git_repository(target_dir: &Path) -> Result<()> {
        use std::process::Command;
        
        println!("🔧 Initializing git repository...");
        
        let status = Command::new("git")
            .arg("init")
            .current_dir(target_dir)
            .status()?;
        
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to initialize git repository"));
        }
        
        // Add all files
        let status = Command::new("git")
            .args(&["add", "."])
            .current_dir(target_dir)
            .status()?;
        
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to add files to git"));
        }
        
        // Create initial commit
        let status = Command::new("git")
            .args(&["commit", "-m", "Initial commit: PM extension template"])
            .current_dir(target_dir)
            .status()?;
        
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to create initial commit"));
        }
        
        println!("✅ Git repository initialized");
        Ok(())
    }
    
    /// Create GitHub repository (placeholder - would integrate with GitHub API)
    async fn create_github_repository(_target_dir: &Path, config: &ExtensionCreationConfig) -> Result<()> {
        // This would integrate with GitHub API to create repository
        // For now, just show instructions
        println!("📝 To create GitHub repository manually:");
        println!("   1. Go to https://github.com/new");
        println!("   2. Repository name: pm-ext-{}", config.name);
        println!("   3. Description: {}", config.description);
        println!("   4. Make it public");
        println!("   5. Don't initialize with README (already created)");
        println!("   6. After creation, run:");
        println!("      cd {}", config.target_directory.display());
        println!("      git remote add origin https://github.com/{}/pm-ext-{}.git", config.author, config.name);
        println!("      git push -u origin main");
        
        Ok(())
    }
    
    /// Validate extension name
    fn validate_extension_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow::anyhow!("Extension name cannot be empty"));
        }
        
        if name.len() > 50 {
            return Err(anyhow::anyhow!("Extension name too long (max 50 characters)"));
        }
        
        // Check for valid characters (kebab-case)
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(anyhow::anyhow!("Extension name must be kebab-case (letters, numbers, hyphens only)"));
        }
        
        if name.starts_with('-') || name.ends_with('-') {
            return Err(anyhow::anyhow!("Extension name cannot start or end with hyphen"));
        }
        
        if name.contains("--") {
            return Err(anyhow::anyhow!("Extension name cannot contain consecutive hyphens"));
        }
        
        Ok(())
    }
    
    /// Get default author from git config
    fn get_default_author() -> String {
        use std::process::Command;
        
        if let Ok(output) = Command::new("git")
            .args(&["config", "--global", "user.name"])
            .output()
        {
            if output.status.success() {
                if let Ok(name) = String::from_utf8(output.stdout) {
                    return name.trim().to_string();
                }
            }
        }
        
        "your-username".to_string()
    }
    
    /// Generate MIT license
    fn generate_mit_license(author: &str) -> String {
        let year = chrono::Utc::now().year();
        format!(
            r#"MIT License

Copyright (c) {} {}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#,
            year, author
        )
    }
    
    /// Print success message with next steps
    fn print_success_message(config: &ExtensionCreationConfig) {
        println!("\n🎉 Extension '{}' created successfully!", config.name);
        println!("📁 Location: {}", config.target_directory.display());
        println!("\n📋 Next steps:");
        println!("   cd {}", config.target_directory.display());
        
        match config.template_type {
            ExtensionTemplateType::Rust => {
                println!("   cargo build --release");
                println!("   cargo test");
                println!("   pm ext install {} --source ./target/release/pm-ext-{}", config.name, config.name);
            }
            ExtensionTemplateType::Bash | ExtensionTemplateType::Python => {
                println!("   pm ext install {} --source ./", config.name);
            }
        }
        
        println!("   pm {} --help", config.name);
        println!("\n🚀 Happy coding!");
    }
}