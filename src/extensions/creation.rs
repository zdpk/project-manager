use crate::extensions::{ExtensionManifest, ExtensionCommand};
use crate::ExtensionType;
use anyhow::{Context, Result};
use chrono::Datelike;
use std::path::PathBuf;
use tokio::fs;

/// Context for extension template generation
#[derive(Debug, Clone)]
pub struct TemplateContext {
    pub name: String,
    pub ext_type: ExtensionType,
    pub description: String,
    pub author: String,
    pub email: Option<String>,
    pub version: String,
    pub directory: PathBuf,
}

/// Create a new extension with interactive prompts for missing information
pub async fn create_extension(
    name: String,
    ext_type: Option<ExtensionType>,
    directory: Option<PathBuf>,
    description: Option<String>,
    author: Option<String>,
    non_interactive: bool,
) -> Result<()> {
    println!("🚀 Creating new PM extension...");
    println!();

    // Validate extension name
    validate_extension_name(&name)?;

    // Build template context with interactive prompts or defaults
    let context = if non_interactive {
        build_context_with_defaults(name, ext_type, directory, description, author).await?
    } else {
        build_context_interactive(name, ext_type, directory, description, author).await?
    };

    // Show configuration summary
    display_configuration_summary(&context);

    // Confirm creation (unless non-interactive)
    if !non_interactive && !confirm_creation()? {
        println!("❌ Extension creation cancelled.");
        return Ok(());
    }

    // Create extension directory structure
    create_extension_structure(&context).await?;

    // Display success message and next steps
    display_success_message(&context);

    Ok(())
}

/// Validate extension name format
fn validate_extension_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Extension name cannot be empty"));
    }

    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err(anyhow::anyhow!(
            "Extension name can only contain alphanumeric characters, hyphens, and underscores"
        ));
    }

    if name.starts_with('-') || name.ends_with('-') {
        return Err(anyhow::anyhow!("Extension name cannot start or end with a hyphen"));
    }

    Ok(())
}

/// Build template context with interactive prompts
async fn build_context_interactive(
    name: String,
    ext_type: Option<ExtensionType>,
    directory: Option<PathBuf>,
    description: Option<String>,
    author: Option<String>,
) -> Result<TemplateContext> {
    // Get extension type
    let ext_type = match ext_type {
        Some(t) => t,
        None => prompt_extension_type()?,
    };

    // Get description
    let description = match description {
        Some(d) => d,
        None => prompt_description(&name, ext_type)?,
    };

    // Get author
    let author = match author {
        Some(a) => a,
        None => prompt_author()?,
    };

    // Get email from git config
    let email = get_git_email().await;

    // Get target directory
    let directory = match directory {
        Some(d) => d,
        None => prompt_directory(&name)?,
    };

    Ok(TemplateContext {
        name,
        ext_type,
        description,
        author,
        email,
        version: "0.1.0".to_string(),
        directory,
    })
}

/// Build template context with defaults (non-interactive)
async fn build_context_with_defaults(
    name: String,
    ext_type: Option<ExtensionType>,
    directory: Option<PathBuf>,
    description: Option<String>,
    author: Option<String>,
) -> Result<TemplateContext> {
    let ext_type = ext_type.unwrap_or(ExtensionType::Bash);
    let description = description.unwrap_or_else(|| {
        format!("A {} extension for PM", format!("{:?}", ext_type).to_lowercase())
    });
    let author = author.unwrap_or_else(|| {
        get_git_name().unwrap_or_else(|| "Unknown".to_string())
    });
    let email = get_git_email().await;
    let directory = directory.unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_default().join(&name)
    });

    Ok(TemplateContext {
        name,
        ext_type,
        description,
        author,
        email,
        version: "0.1.0".to_string(),
        directory,
    })
}

/// Prompt for extension type
fn prompt_extension_type() -> Result<ExtensionType> {
    println!("Extension type:");
    println!("  1. bash   - Shell scripts (fast, simple)");
    println!("  2. python - Python scripts (versatile, rich ecosystem)");
    println!("  3. rust   - Rust binary (fast, safe, compiled)");
    print!("Choose type [1-3] (1): ");
    
    let input = read_user_input()?;
    let input = input.trim();
    
    match input {
        "" | "1" | "bash" => Ok(ExtensionType::Bash),
        "2" | "python" => Ok(ExtensionType::Python),
        "3" | "rust" => Ok(ExtensionType::Rust),
        _ => {
            println!("Invalid choice. Using bash as default.");
            Ok(ExtensionType::Bash)
        }
    }
}

/// Prompt for extension description
fn prompt_description(_name: &str, ext_type: ExtensionType) -> Result<String> {
    let default = format!("A {} extension for PM", format!("{:?}", ext_type).to_lowercase());
    print!("Description ({}): ", default);
    
    let input = read_user_input()?;
    let input = input.trim();
    
    if input.is_empty() {
        Ok(default)
    } else {
        Ok(input.to_string())
    }
}

/// Prompt for author name
fn prompt_author() -> Result<String> {
    let default = get_git_name().unwrap_or_else(|| "Unknown".to_string());
    print!("Author ({}): ", default);
    
    let input = read_user_input()?;
    let input = input.trim();
    
    if input.is_empty() {
        Ok(default)
    } else {
        Ok(input.to_string())
    }
}

/// Prompt for target directory
fn prompt_directory(name: &str) -> Result<PathBuf> {
    let default = std::env::current_dir()
        .unwrap_or_default()
        .join(name);
    
    print!("Target directory ({}): ", default.display());
    
    let input = read_user_input()?;
    let input = input.trim();
    
    if input.is_empty() {
        Ok(default)
    } else {
        Ok(PathBuf::from(input))
    }
}

/// Get git user name
fn get_git_name() -> Option<String> {
    std::process::Command::new("git")
        .args(&["config", "user.name"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Get git user email
async fn get_git_email() -> Option<String> {
    let output = tokio::process::Command::new("git")
        .args(&["config", "user.email"])
        .output()
        .await
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    } else {
        None
    }
}

/// Read user input from stdin
fn read_user_input() -> Result<String> {
    use std::io::{self, Write};
    
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

/// Display configuration summary
fn display_configuration_summary(context: &TemplateContext) {
    println!();
    println!("📋 Extension Configuration:");
    println!("  Name:        {}", context.name);
    println!("  Type:        {:?}", context.ext_type);
    println!("  Description: {}", context.description);
    println!("  Author:      {}", context.author);
    if let Some(email) = &context.email {
        println!("  Email:       {}", email);
    }
    println!("  Version:     {}", context.version);
    println!("  Directory:   {}", context.directory.display());
    println!();
}

/// Confirm extension creation
fn confirm_creation() -> Result<bool> {
    print!("✅ Create extension? [Y/n]: ");
    
    let input = read_user_input()?;
    let input = input.trim().to_lowercase();
    
    Ok(input.is_empty() || input == "y" || input == "yes")
}

/// Create the extension directory structure and files
async fn create_extension_structure(context: &TemplateContext) -> Result<()> {
    // Check if directory already exists
    if context.directory.exists() {
        return Err(anyhow::anyhow!(
            "Directory already exists: {}",
            context.directory.display()
        ));
    }

    // Create target directory
    fs::create_dir_all(&context.directory).await
        .with_context(|| format!("Failed to create directory: {}", context.directory.display()))?;

    // Generate extension manifest
    generate_manifest(context).await?;

    // Generate files based on extension type
    match context.ext_type {
        ExtensionType::Bash => generate_bash_extension(context).await?,
        ExtensionType::Python => generate_python_extension(context).await?,
        ExtensionType::Rust => generate_rust_extension(context).await?,
    }

    // Generate common files
    generate_readme(context).await?;
    generate_license(context).await?;

    Ok(())
}

/// Generate extension manifest (extension.yml)
async fn generate_manifest(context: &TemplateContext) -> Result<()> {
    let commands = match context.ext_type {
        ExtensionType::Bash => vec![
            ExtensionCommand {
                name: "example".to_string(),
                help: "Example command - replace with your extension's functionality".to_string(),
                aliases: None,
                args: None,
            },
        ],
        ExtensionType::Python => vec![
            ExtensionCommand {
                name: "run".to_string(),
                help: "Run the main functionality".to_string(),
                aliases: None,
                args: None,
            },
            ExtensionCommand {
                name: "help".to_string(),
                help: "Show help information".to_string(),
                aliases: Some(vec!["h".to_string()]),
                args: None,
            },
        ],
        ExtensionType::Rust => vec![
            ExtensionCommand {
                name: "run".to_string(),
                help: "Run the main functionality".to_string(),
                aliases: None,
                args: None,
            },
            ExtensionCommand {
                name: "version".to_string(),
                help: "Show version information".to_string(),
                aliases: Some(vec!["v".to_string()]),
                args: None,
            },
        ],
    };

    let manifest = ExtensionManifest {
        name: context.name.clone(),
        version: context.version.clone(),
        description: context.description.clone(),
        author: Some(context.author.clone()),
        homepage: None,
        pm_version: Some(">=0.1.0".to_string()),
        commands,
    };

    let manifest_path = context.directory.join("extension.yml");
    manifest.save_to_file(&manifest_path).await?;

    Ok(())
}

/// Generate bash extension files
async fn generate_bash_extension(context: &TemplateContext) -> Result<()> {
    let bash_dir = context.directory.join("bash");
    fs::create_dir_all(&bash_dir).await?;

    let example_script = generate_bash_example_script(context);
    let example_path = bash_dir.join("example.sh");
    fs::write(&example_path, example_script).await?;

    // Set executable permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&example_path).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&example_path, perms).await?;
    }

    Ok(())
}

/// Generate python extension files
async fn generate_python_extension(context: &TemplateContext) -> Result<()> {
    let python_dir = context.directory.join("python");
    fs::create_dir_all(&python_dir).await?;

    let main_script = generate_python_main_script(context);
    let main_path = python_dir.join("main.py");
    fs::write(&main_path, main_script).await?;

    let requirements = "# Add your Python dependencies here\n# requests>=2.25.0\n# click>=8.0.0\n";
    let requirements_path = context.directory.join("requirements.txt");
    fs::write(&requirements_path, requirements).await?;

    Ok(())
}

/// Generate rust extension files
async fn generate_rust_extension(context: &TemplateContext) -> Result<()> {
    let src_dir = context.directory.join("src");
    fs::create_dir_all(&src_dir).await?;

    let cargo_toml = generate_cargo_toml(context);
    let cargo_path = context.directory.join("Cargo.toml");
    fs::write(&cargo_path, cargo_toml).await?;

    let main_rs = generate_rust_main_script(context);
    let main_path = src_dir.join("main.rs");
    fs::write(&main_path, main_rs).await?;

    Ok(())
}

/// Generate README.md
async fn generate_readme(context: &TemplateContext) -> Result<()> {
    let readme_content = format!(r#"# {}

{}

## Installation

```bash
# Install locally for development
pm ext install . --local

# Test your extension
pm {} example
```

## Development

### Extension Type: {:?}

{}

## Commands

{}

## License

MIT License - see LICENSE file for details.
"#,
        context.name,
        context.description,
        context.name,
        context.ext_type,
        match context.ext_type {
            ExtensionType::Bash => "This is a Bash extension. Edit `bash/example.sh` to implement your functionality.",
            ExtensionType::Python => "This is a Python extension. Edit `python/main.py` to implement your functionality.\n\nInstall dependencies:\n```bash\npip install -r requirements.txt\n```",
            ExtensionType::Rust => "This is a Rust extension. Edit `src/main.rs` to implement your functionality.\n\nBuild the extension:\n```bash\ncargo build --release\n```",
        },
        match context.ext_type {
            ExtensionType::Bash => "- `example` - Example command",
            ExtensionType::Python => "- `run` - Run the main functionality\n- `help` - Show help information",
            ExtensionType::Rust => "- `run` - Run the main functionality\n- `version` - Show version information",
        }
    );

    let readme_path = context.directory.join("README.md");
    fs::write(&readme_path, readme_content).await?;

    Ok(())
}

/// Generate LICENSE file
async fn generate_license(context: &TemplateContext) -> Result<()> {
    let year = chrono::Utc::now().year();
    let license_content = format!(r#"MIT License

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
"#, year, context.author);

    let license_path = context.directory.join("LICENSE");
    fs::write(&license_path, license_content).await?;

    Ok(())
}

/// Generate bash example script
fn generate_bash_example_script(context: &TemplateContext) -> String {
    format!(r#"#!/bin/bash
# {} - Example Command
# {}

set -euo pipefail

# Get command name and arguments
COMMAND="$1"
shift

case "$COMMAND" in
    "example")
        echo "🎉 {} Extension - Example Command"
        echo "📋 Command: $COMMAND"
        echo "📦 Arguments: $*"
        echo "🔧 Extension is working correctly!"
        
        # Access PM environment variables
        echo ""
        echo "📍 PM Environment:"
        echo "  Config: $PM_CONFIG_PATH"
        echo "  Project: $PM_CURRENT_PROJECT"
        echo "  Version: $PM_VERSION"
        echo "  Extension Dir: $PM_EXTENSION_DIR"
        ;;
    "help"|*)
        echo "Usage: pm {} [command] [args...]"
        echo ""
        echo "Available Commands:"
        echo "  example    Example command - replace with your functionality"
        echo ""
        echo "Extension: {}"
        echo "Description: {}"
        ;;
esac
"#, context.name, context.description, context.name, context.name, context.name, context.description)
}

/// Generate python main script
fn generate_python_main_script(context: &TemplateContext) -> String {
    format!(r#"#!/usr/bin/env python3
"""
{} - A Python extension for PM
{}
"""

import sys
import os
import json

def main():
    """Main entry point for the extension."""
    if len(sys.argv) < 2:
        show_help()
        return
    
    command = sys.argv[1]
    args = sys.argv[2:]
    
    if command == "run":
        run_main_functionality(args)
    elif command == "help" or command == "h":
        show_help()
    else:
        print(f"Unknown command: {{command}}")
        show_help()
        sys.exit(1)

def run_main_functionality(args):
    """Run the main functionality of the extension."""
    print("🐍 {} Extension - Run Command")
    print(f"📋 Arguments: {{args}}")
    print("🔧 Python extension is working correctly!")
    
    # Access PM environment variables
    print()
    print("📍 PM Environment:")
    print(f"  Config: {{os.getenv('PM_CONFIG_PATH', 'Not set')}}")
    print(f"  Version: {{os.getenv('PM_VERSION', 'Not set')}}")
    print(f"  Extension Dir: {{os.getenv('PM_EXTENSION_DIR', 'Not set')}}")
    
    # Parse current project info
    project_json = os.getenv('PM_CURRENT_PROJECT', '{{}}')
    try:
        project = json.loads(project_json)
        if project:
            print(f"  Project: {{project.get('name', 'Unknown')}}")
            print(f"  Path: {{project.get('path', 'Unknown')}}")
    except json.JSONDecodeError:
        print(f"  Project: {{project_json}}")

def show_help():
    """Show help information."""
    print("Usage: pm {} [command] [args...]")
    print()
    print("Available Commands:")
    print("  run        Run the main functionality")
    print("  help, h    Show this help message")
    print()
    print("Extension: {}")
    print("Description: {}")

if __name__ == "__main__":
    main()
"#, context.name, context.description, context.name, context.name, context.name, context.description)
}

/// Generate Cargo.toml for Rust extension
fn generate_cargo_toml(context: &TemplateContext) -> String {
    format!(r#"[package]
name = "{}"
version = "{}"
edition = "2021"
authors = ["{}"]
description = "{}"

[dependencies]
clap = {{ version = "4.0", features = ["derive"] }}
anyhow = "1.0"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
"#, context.name, context.version, context.author, context.description)
}

/// Generate Rust main.rs
fn generate_rust_main_script(context: &TemplateContext) -> String {
    format!(r#"use clap::{{Arg, Command}};
use anyhow::Result;
use std::env;

fn main() -> Result<()> {{
    let matches = Command::new("{}")
        .version("{}")
        .author("{}")
        .about("{}")
        .subcommand(
            Command::new("run")
                .about("Run the main functionality")
        )
        .subcommand(
            Command::new("version")
                .about("Show version information")
                .alias("v")
        )
        .get_matches();

    match matches.subcommand() {{
        Some(("run", _)) => run_main_functionality(),
        Some(("version", _)) => show_version(),
        _ => show_help(),
    }}

    Ok(())
}}

fn run_main_functionality() {{
    println!("🦀 {} Extension - Run Command");
    println!("🔧 Rust extension is working correctly!");
    
    // Access PM environment variables
    println!();
    println!("📍 PM Environment:");
    println!("  Config: {{}}", env::var("PM_CONFIG_PATH").unwrap_or_else(|_| "Not set".to_string()));
    println!("  Version: {{}}", env::var("PM_VERSION").unwrap_or_else(|_| "Not set".to_string()));
    println!("  Extension Dir: {{}}", env::var("PM_EXTENSION_DIR").unwrap_or_else(|_| "Not set".to_string()));
    
    // Parse current project info
    let project_json = env::var("PM_CURRENT_PROJECT").unwrap_or_else(|_| "{{}}".to_string());
    if let Ok(project) = serde_json::from_str::<serde_json::Value>(&project_json) {{
        if let Some(name) = project.get("name") {{
            println!("  Project: {{}}", name.as_str().unwrap_or("Unknown"));
        }}
        if let Some(path) = project.get("path") {{
            println!("  Path: {{}}", path.as_str().unwrap_or("Unknown"));
        }}
    }}
}}

fn show_version() {{
    println!("{} v{{}}", env!("CARGO_PKG_VERSION"));
}}

fn show_help() {{
    println!("Usage: pm {} [command]");
    println!();
    println!("Available Commands:");
    println!("  run        Run the main functionality");
    println!("  version    Show version information");
    println!();
    println!("Extension: {}");
    println!("Description: {}");
}}
"#, context.name, context.version, context.author, context.description, context.name, context.name, context.name, context.name, context.description)
}

/// Display success message and next steps
fn display_success_message(context: &TemplateContext) {
    println!("✅ Extension '{}' created successfully!", context.name);
    println!();
    println!("📁 Created in: {}", context.directory.display());
    println!("📝 Files generated:");
    println!("  - extension.yml");
    
    match context.ext_type {
        ExtensionType::Bash => {
            println!("  - bash/example.sh");
        },
        ExtensionType::Python => {
            println!("  - python/main.py");
            println!("  - requirements.txt");
        },
        ExtensionType::Rust => {
            println!("  - Cargo.toml");
            println!("  - src/main.rs");
        },
    }
    
    println!("  - README.md");
    println!("  - LICENSE");
    println!();
    println!("🎯 Next steps:");
    println!("  1. cd {}", context.name);
    
    match context.ext_type {
        ExtensionType::Bash => {
            println!("  2. # Edit bash/example.sh to implement your functionality");
        },
        ExtensionType::Python => {
            println!("  2. pip install -r requirements.txt  # Install dependencies");
            println!("  3. # Edit python/main.py to implement your functionality");
        },
        ExtensionType::Rust => {
            println!("  2. cargo build --release           # Build your extension");
            println!("  3. # Edit src/main.rs to implement your functionality");
        },
    }
    
    println!("  {}. pm ext install . --local         # Install locally for testing", 
             match context.ext_type { ExtensionType::Rust => 4, _ => 3 });
    println!("  {}. pm {} run                     # Test your extension", 
             match context.ext_type { ExtensionType::Rust => 5, _ => 4 }, context.name);
}