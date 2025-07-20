use serde::{Deserialize, Serialize};

use super::platform::PlatformSelection;
use super::creation::ExtensionTemplateType;

/// Template context for generating extension files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateContext {
    pub name: String,
    pub description: String,
    pub author: String,
    pub email: Option<String>,
    pub repository_url: Option<String>,
    pub platforms: PlatformSelection,
    pub template_type: ExtensionTemplateType,
    pub created_at: String,
}

impl TemplateContext {
    pub fn new(
        name: String,
        description: String,
        author: String,
        platforms: PlatformSelection,
        template_type: ExtensionTemplateType,
    ) -> Self {
        Self {
            name,
            description,
            author,
            email: None,
            repository_url: None,
            platforms,
            template_type,
            created_at: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        }
    }
}

/// GitHub Actions workflow template generator
pub struct WorkflowTemplate;

impl WorkflowTemplate {
    /// Generate GitHub Actions release workflow
    pub fn generate_release_workflow(context: &TemplateContext) -> String {
        let matrix_entries = Self::generate_matrix_entries(&context.platforms);
        
        format!(
            r#"name: Release Extension

on:
  push:
    tags: ['v*']
  workflow_dispatch:

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    name: Build for ${{{{ matrix.target }}}}
    runs-on: ${{{{ matrix.os }}}}
    strategy:
      fail-fast: false
      matrix:
        include:
{matrix_entries}

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{{{ matrix.target }}}}

{cross_compilation_setup}

      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry
          key: ${{{{ runner.os }}}}-cargo-registry-${{{{ hashFiles('**/Cargo.lock') }}}}

      - name: Cache cargo index
        uses: actions/cache@v4
        with:
          path: ~/.cargo/git
          key: ${{{{ runner.os }}}}-cargo-index-${{{{ hashFiles('**/Cargo.lock') }}}}

      - name: Build binary
        run: cargo build --release --target ${{{{ matrix.target }}}}

      - name: Strip binary (Unix)
        if: matrix.os != 'windows-latest'
        run: strip target/${{{{ matrix.target }}}}/release/${{{{ matrix.binary_name }}}}

      - name: Upload binary
        uses: actions/upload-artifact@v4
        with:
          name: ${{{{ matrix.asset_name }}}}
          path: target/${{{{ matrix.target }}}}/release/${{{{ matrix.binary_name }}}}

  release:
    name: Create Release
    needs: build
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Download all artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts

      - name: Prepare release assets
        run: |
          mkdir -p release-assets
          find artifacts -name "*" -type f -exec cp {{}} release-assets/ \;
          ls -la release-assets/

      - name: Generate checksums
        run: |
          cd release-assets
          for file in *; do
            sha256sum "$file" > "$file.sha256"
          done

      - name: Create install script
        run: |
{install_script}

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: release-assets/*
          generate_release_notes: true
          draft: false
        env:
          GITHUB_TOKEN: ${{{{ secrets.GITHUB_TOKEN }}}}
"#,
            matrix_entries = matrix_entries,
            cross_compilation_setup = Self::generate_cross_compilation_setup(&context.platforms),
            install_script = Self::generate_install_script_step(&context.name)
        )
    }
    
    /// Generate matrix entries for GitHub Actions
    fn generate_matrix_entries(platforms: &PlatformSelection) -> String {
        let mut entries = Vec::new();
        
        for platform in &platforms.platforms {
            let binary_extension = if platform.os == super::platform::OperatingSystem::Windows {
                ".exe"
            } else {
                ""
            };
            
            let entry = format!(
                r#"          - target: {}
            os: {}
            binary_name: pm-ext-{{{{name}}}}{}
            asset_name: {}"#,
                platform.rust_target(),
                platform.github_runner_os(),
                binary_extension,
                platform.asset_name("{{name}}")
            );
            entries.push(entry);
        }
        
        entries.join("\n")
    }
    
    /// Generate cross-compilation setup steps
    fn generate_cross_compilation_setup(platforms: &PlatformSelection) -> String {
        let needs_linux_aarch64 = platforms.platforms.iter()
            .any(|p| matches!(
                (p.os.clone(), p.arch.clone()), 
                (super::platform::OperatingSystem::Linux, super::platform::Architecture::Aarch64)
            ));
        
        if needs_linux_aarch64 {
            r#"      - name: Install cross-compilation tools (Linux ARM64)
        if: matrix.target == 'aarch64-unknown-linux-gnu'
        run: |
          sudo apt-get update
          sudo apt-get install -y gcc-aarch64-linux-gnu
          echo "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc" >> $GITHUB_ENV
"#.to_string()
        } else {
            "".to_string()
        }
    }
    
    /// Generate install script creation step
    fn generate_install_script_step(extension_name: &str) -> String {
        format!(
            r#"          cat > release-assets/install.sh << 'EOF'
          #!/bin/bash
          
          set -e
          
          # Extension metadata
          EXTENSION_NAME="{}"
          GITHUB_REPO="${{{{ github.repository }}}}"
          
          # Colors for output
          RED='\033[0;31m'
          GREEN='\033[0;32m'
          YELLOW='\033[1;33m'
          BLUE='\033[0;34m'
          NC='\033[0m' # No Color
          
          # Print colored output
          print_info() {{ echo -e "${{BLUE}}ℹ️  $1${{NC}}"; }}
          print_success() {{ echo -e "${{GREEN}}✅ $1${{NC}}"; }}
          print_warning() {{ echo -e "${{YELLOW}}⚠️  $1${{NC}}"; }}
          print_error() {{ echo -e "${{RED}}❌ $1${{NC}}"; }}
          
          # Detect platform
          detect_platform() {{
              local os arch
              
              os=$(uname -s | tr '[:upper:]' '[:lower:]')
              arch=$(uname -m)
              
              case "$os" in
                  darwin)
                      case "$arch" in
                          arm64|aarch64) echo "darwin-aarch64" ;;
                          x86_64) 
                              print_error "Intel Macs are not supported for this extension"
                              print_info "Apple Silicon (M1/M2/M3) is required"
                              exit 1 
                              ;;
                          *)
                              print_error "Unsupported macOS architecture: $arch"
                              exit 1
                              ;;
                      esac
                      ;;
                  linux)
                      case "$arch" in
                          aarch64|arm64) echo "linux-aarch64" ;;
                          x86_64|amd64) echo "linux-x86_64" ;;
                          *)
                              print_error "Unsupported Linux architecture: $arch"
                              print_info "Supported: x86_64, aarch64"
                              exit 1
                              ;;
                      esac
                      ;;
                  mingw*|msys*|cygwin*)
                      case "$arch" in
                          x86_64|amd64) echo "windows-x86_64" ;;
                          aarch64|arm64) echo "windows-aarch64" ;;
                          *)
                              print_error "Unsupported Windows architecture: $arch"
                              print_info "Supported: x86_64, aarch64"
                              exit 1
                              ;;
                      esac
                      ;;
                  *)
                      print_error "Unsupported operating system: $os"
                      print_info "Supported: macOS (Apple Silicon), Linux, Windows"
                      exit 1
                      ;;
              esac
          }}
          
          # Get latest release info
          get_latest_release() {{
              curl -s "https://api.github.com/repos/$GITHUB_REPO/releases/latest" | \
                  grep -o '"tag_name": "[^"]*' | \
                  cut -d'"' -f4
          }}
          
          # Download and install
          install_extension() {{
              local platform version binary_name download_url
              
              platform=$(detect_platform)
              version=$(get_latest_release)
              
              if [ -z "$version" ]; then
                  print_error "Failed to get latest release version"
                  exit 1
              fi
              
              case "$platform" in
                  *windows*) binary_name="pm-ext-${{EXTENSION_NAME}}-${{platform}}.exe" ;;
                  *) binary_name="pm-ext-${{EXTENSION_NAME}}-${{platform}}" ;;
              esac
              
              download_url="https://github.com/$GITHUB_REPO/releases/download/$version/$binary_name"
              
              print_info "Detected platform: $platform"
              print_info "Latest version: $version"
              print_info "Downloading from: $download_url"
              
              # Download binary
              if ! curl -fsSL "$download_url" -o "/tmp/$binary_name"; then
                  print_error "Failed to download extension binary"
                  print_info "URL: $download_url"
                  print_info "Please check if this platform is supported for this extension"
                  exit 1
              fi
              
              # Make executable
              chmod +x "/tmp/$binary_name"
              
              # Install via PM
              if command -v pm >/dev/null 2>&1; then
                  print_info "Installing extension via PM..."
                  pm ext install "$EXTENSION_NAME" --source "/tmp/$binary_name"
                  print_success "Extension '$EXTENSION_NAME' installed successfully!"
                  print_info "Test with: pm $EXTENSION_NAME --help"
              else
                  print_warning "PM not found in PATH"
                  print_info "Manual installation: copy '/tmp/$binary_name' to your PM extensions directory"
                  print_info "Or add PM to your PATH and run this script again"
              fi
          }}
          
          # Main
          main() {{
              print_info "Installing PM extension: $EXTENSION_NAME"
              install_extension
          }}
          
          main "$@"
          EOF
          
          chmod +x release-assets/install.sh"#,
            extension_name
        )
    }
}

/// Extension template files generator
pub struct ExtensionTemplate;

impl ExtensionTemplate {
    /// Generate Cargo.toml template
    pub fn generate_cargo_toml(context: &TemplateContext) -> String {
        format!(
            r#"[package]
name = "pm-ext-{}"
version = "0.1.0"
edition = "2021"
description = "{}"
authors = ["{}{}"]
license = "MIT"
repository = "{}"

[[bin]]
name = "pm-ext-{}"
path = "src/main.rs"

[dependencies]
clap = {{ version = "4.5", features = ["derive"] }}
anyhow = "1.0"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
"#,
            context.name,
            context.description,
            context.author,
            context.email.as_ref().map(|e| format!(" <{}>", e)).unwrap_or_default(),
            context.repository_url.as_ref().unwrap_or(&format!("https://github.com/{}/pm-ext-{}", context.author, context.name)),
            context.name
        )
    }
    
    /// Generate main.rs template
    pub fn generate_main_rs(context: &TemplateContext) -> String {
        format!(
            r#"use anyhow::Result;
use clap::{{Parser, Subcommand}};

/// {} - {}
#[derive(Parser)]
#[command(name = "pm-ext-{}", about = "{}", version)]
struct Cli {{
    #[command(subcommand)]
    command: Option<Commands>,
}}

#[derive(Subcommand)]
enum Commands {{
    /// Example command - replace with your extension's functionality
    Example {{
        /// Example argument
        #[arg(short, long)]
        message: Option<String>,
    }},
}}

fn main() -> Result<()> {{
    let cli = Cli::parse();
    
    match cli.command {{
        Some(Commands::Example {{ message }}) => {{
            let msg = message.unwrap_or_else(|| "Hello from PM extension!".to_string());
            println!("🚀 {{}}: {{}}", env!("CARGO_PKG_NAME"), msg);
            
            // Get PM context from environment variables
            if let Ok(project) = std::env::var("PM_CURRENT_PROJECT") {{
                println!("📁 Current PM project: {{}}", project);
            }}
            
            if let Ok(config_path) = std::env::var("PM_CONFIG_PATH") {{
                println!("⚙️  PM config: {{}}", config_path);
            }}
        }}
        None => {{
            println!("🔧 {} v{{}}", env!("CARGO_PKG_VERSION"));
            println!("📖 Use --help to see available commands");
        }}
    }}
    
    Ok(())
}}
"#,
            context.name.to_uppercase(),
            context.description,
            context.name,
            context.description,
            context.name.to_uppercase()
        )
    }
    
    /// Generate README.md template
    pub fn generate_readme(context: &TemplateContext) -> String {
        match context.template_type {
            ExtensionTemplateType::Rust => Self::generate_rust_readme(context),
            ExtensionTemplateType::Bash => Self::generate_bash_readme(context),
            ExtensionTemplateType::Python => Self::generate_python_readme(context),
        }
    }
    
    /// Generate README for Rust extensions
    fn generate_rust_readme(context: &TemplateContext) -> String {
        let install_examples = Self::generate_install_examples(context);
        
        format!(
            r#"# PM Extension: {}

{}

## Installation

### Quick Install (Recommended)
```bash
curl -fsSL https://github.com/{}/pm-ext-{}/releases/latest/download/install.sh | sh
```

### Manual Installation
Download the appropriate binary for your platform from [Releases](https://github.com/{}/pm-ext-{}/releases):

{}

Then install via PM:
```bash
pm ext install {} --source ./pm-ext-{}-<platform>
```

## Usage

```bash
# Example usage
pm {} example --message "Hello World"

# Get help
pm {} --help
```

## Development

### Prerequisites
- Rust 1.70+
- PM CLI installed

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test
```

### Local Installation
```bash
# Build and install locally
cargo build --release
pm ext install {} --source ./target/release/pm-ext-{}
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

---
Generated with PM Extension Template on {}
"#,
            context.name,                    // PM Extension: {}
            context.description,             // {}
            context.author, context.name,    // github.com/{}/pm-ext-{}
            context.author, context.name,    // github.com/{}/pm-ext-{}
            install_examples,                // {}
            context.name, context.name,      // pm ext install {} --source ./pm-ext-{}-<platform>
            context.name,                    // pm {} example
            context.name,                    // pm {} --help
            context.name,                    // pm ext install {}
            context.name,                    // target/release/pm-ext-{}
            context.created_at              // Generated with PM Extension Template on {}
        )
    }
    
    /// Generate README for Bash extensions
    fn generate_bash_readme(context: &TemplateContext) -> String {
        format!(
            r#"# PM Extension: {}

{}

## Installation

### Local Installation
```bash
# Install from current directory
pm ext install {} --source ./
```

## Usage

```bash
# Main command
pm {}

# Get help
pm {} help
```

## Structure

This extension uses the new PM folder structure:
- `bash/example.sh` - Main extension implementation
- `extension.yml` - Extension manifest

## Development

### Prerequisites
- Bash 4.0+
- PM CLI installed

### Testing
```bash
# Test the extension
bash bash/example.sh

# Test help
bash bash/example.sh help
```

### Customization

Edit `bash/example.sh` to implement your extension's functionality. The script includes built-in help functionality and can handle multiple commands.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - see LICENSE file for details.

---
Generated with PM Extension Template on {}
"#,
            context.name,        // PM Extension: {}
            context.description, // {}
            context.name,        // pm ext install {} --source ./
            context.name,        // pm {}
            context.name,        // pm {} help
            context.created_at   // Generated with PM Extension Template on {}
        )
    }
    
    /// Generate README for Python extensions
    fn generate_python_readme(context: &TemplateContext) -> String {
        format!(
            r#"# PM Extension: {}

{}

## Installation

### Local Installation
```bash
# Install dependencies (if any)
pip install -r requirements.txt

# Install from current directory
pm ext install {} --source ./
```

## Usage

```bash
# Main command
pm {}

# Example command
pm {} example

# Get help
pm {} help
```

## Structure

This extension uses the new PM folder structure:
- `python/main.py` - Main command implementation
- `python/example.py` - Example command implementation
- `python/help.py` - Help command implementation
- `requirements.txt` - Python dependencies
- `extension.yml` - Extension manifest

## Development

### Prerequisites
- Python 3.7+
- PM CLI installed

### Virtual Environment (Recommended)
```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
```

### Testing
```bash
# Test main command
python python/main.py

# Test example command
python python/example.py

# Test help command
python python/help.py
```

### Customization

Edit the scripts in the `python/` directory to implement your extension's functionality:
- Modify `python/main.py` for the main command logic
- Update `python/example.py` for example functionality
- Customize `python/help.py` for help text
- Add dependencies to `requirements.txt` as needed

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - see LICENSE file for details.

---
Generated with PM Extension Template on {}
"#,
            context.name,        // PM Extension: {}
            context.description, // {}
            context.name,        // pm ext install {} --source ./
            context.name,        // pm {}
            context.name,        // pm {} example
            context.name,        // pm {} help
            context.created_at   // Generated with PM Extension Template on {}
        )
    }
    
    /// Generate platform-specific install examples
    fn generate_install_examples(context: &TemplateContext) -> String {
        let mut examples = Vec::new();
        
        for platform in &context.platforms.platforms {
            let asset_name = platform.asset_name(&context.name);
            let example = format!(
                "- **{}**: [{}](https://github.com/{}/pm-ext-{}/releases/latest/download/{})",
                platform,
                asset_name,
                context.author,
                context.name,
                asset_name
            );
            examples.push(example);
        }
        
        examples.join("\n")
    }
    
    /// Generate extension.yml manifest
    pub fn generate_extension_manifest(context: &TemplateContext) -> String {
        let (extension_type, commands) = match context.template_type {
            ExtensionTemplateType::Bash => {
                ("bash", r#"commands:
  - name: main
    help: Main functionality of the extension
    file: example.sh
  - name: help
    help: Show extension help
    file: example.sh"#)
            },
            ExtensionTemplateType::Python => {
                ("python", r#"commands:
  - name: main
    help: Main functionality of the extension
    file: main.py
  - name: example
    help: Example command demonstrating extension usage
    file: example.py
  - name: help
    help: Show extension help
    file: help.py"#)
            },
            ExtensionTemplateType::Rust => {
                ("binary", r#"commands:
  - name: main
    help: Main functionality of the extension
  - name: example
    help: Example command demonstrating extension usage
  - name: help
    help: Show extension help"#)
            }
        };
        
        format!(
            r#"name: {}
version: 0.1.0
description: {}
author: {}
homepage: {}
pm_version: ">=0.1.0"
type: {}
{}
"#,
            context.name,
            context.description,
            context.author,
            context.repository_url.as_ref().unwrap_or(&format!("https://github.com/{}/pm-ext-{}", context.author, context.name)),
            extension_type,
            commands
        )
    }
    
    /// Generate .gitignore
    pub fn generate_gitignore() -> String {
        r#"# Rust
/target/
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# PM
.pm/
"#.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extensions::platform::*;
    
    #[test]
    fn test_template_context_creation() {
        let platforms = PlatformSelection {
            platforms: vec![
                Platform::new(OperatingSystem::Darwin, Architecture::Aarch64),
                Platform::new(OperatingSystem::Linux, Architecture::X86_64),
            ],
        };
        
        let context = TemplateContext::new(
            "hooks".to_string(),
            "Git hooks management".to_string(),
            "testuser".to_string(),
            platforms,
            ExtensionTemplateType::Rust,
        );
        
        assert_eq!(context.name, "hooks");
        assert_eq!(context.description, "Git hooks management");
        assert_eq!(context.platforms.platforms.len(), 2);
    }
    
    #[test]
    fn test_cargo_toml_generation() {
        let platforms = PlatformSelection {
            platforms: vec![Platform::new(OperatingSystem::Linux, Architecture::X86_64)],
        };
        
        let context = TemplateContext::new(
            "test".to_string(),
            "Test extension".to_string(),
            "testuser".to_string(),
            platforms,
            ExtensionTemplateType::Rust,
        );
        
        let cargo_toml = ExtensionTemplate::generate_cargo_toml(&context);
        assert!(cargo_toml.contains("name = \"pm-ext-test\""));
        assert!(cargo_toml.contains("description = \"Test extension\""));
        assert!(cargo_toml.contains("authors = [\"testuser\"]"));
    }
}