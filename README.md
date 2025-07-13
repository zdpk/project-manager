# PM (Project Manager)

A fast, terminal-based project management CLI tool written in Rust. PM helps developers efficiently manage and switch between multiple projects with zero friction.

## Features

- **Interactive Setup**: Guided initialization with auto-detection and GitHub integration
- **Fast Project Switching**: Switch between projects in under 1 second with automatic editor launch
- **Smart Project Discovery**: Automatic detection of Git repositories and programming languages
- **Flexible Tagging System**: Organize projects with custom tags for easy filtering
- **GitHub Integration**: Clone and manage repositories directly from GitHub
- **Advanced Configuration**: Backup, templates, export/import, and validation
- **Machine-Specific Metadata**: Track project access and usage across different machines
- **Cross-Platform Editor Support**: Works with VS Code, Helix, Vim, Neovim, and more
- **Rich CLI Interface**: Colorful output with progress indicators and interactive prompts

## Installation

### Option 1: Quick Install Script (Recommended)

**macOS (Apple Silicon)**:
```bash
curl -fsSL https://github.com/zdpk/project-manager/releases/latest/download/install.sh | sh
```

### Option 2: Manual Download

Download the latest binary from the [Releases page](https://github.com/zdpk/project-manager/releases):

**macOS (Apple Silicon)**:
```bash
curl -L https://github.com/zdpk/project-manager/releases/latest/download/pm-aarch64-apple-darwin -o pm
chmod +x pm
sudo mv pm /usr/local/bin/
```

> **Note**: Currently only macOS Apple Silicon is supported.

### Option 3: Build from Source

#### Prerequisites

- Rust 1.70+ (for building from source)
- Git

```bash
# Clone the repository
git clone https://github.com/zdpk/project-manager.git
cd project-manager

# Build the project
cargo build --release

# Install to your system
cargo install --path .
```

## Quick Start

```bash
# Interactive initialization - sets up PM configuration
pm init

# Add a project
pm add ~/workspace/my-project --tags rust,backend

# Scan for existing repositories
pm scan

# Browse and clone GitHub repositories
pm browse

# List all projects
pm ls

# List projects with filters
pm ls --tags rust --recent 7d --limit 10

# Switch to a project (opens editor automatically)
pm s my-project

# Switch without opening editor
pm s my-project --no-editor
```

### Initial Setup Example

When you run `pm init`, you'll be guided through an interactive setup:

```bash
$ pm init
🚀 Initializing PM...

🔍 Detecting GitHub username from GitHub CLI...
✅ Detected GitHub username: your-username
> Use detected GitHub username 'your-username'? Yes
> Configuration directory: ~/.config/pm
  Where PM configuration files will be stored (press Enter for default)
> Projects root directory: ~/workspace  
  Where your projects will be stored (press Enter for default)
> Choose your preferred editor: hx (Helix)
> Automatically open editor when switching to projects? Yes
> Show git status in project listings? Yes

📂 Creating configuration directory: /Users/you/.config/pm
📁 Creating projects root directory: /Users/you/workspace

✅ PM initialized successfully
👤 GitHub username: your-username
📂 Config directory: /Users/you/.config/pm
📁 Projects root: /Users/you/workspace
⚙️ Config file: /Users/you/.config/pm/config.yml

🎯 Next steps:
  pm add <path>     # Add your first project
  pm scan           # Scan for existing repositories
  pm load <owner>/<repo> # Clone from GitHub
  pm browse         # Browse and select GitHub repositories

📖 Use 'pm --help' to see all available commands
```

> **Note**: PM automatically detects your GitHub username from GitHub CLI if you're authenticated. If detection fails, you can still enter it manually.

## Command Reference

### Project Management

```bash
# Add projects
pm add <path>                           # Add current or specified directory
pm add . --name "My Project" --tags web,frontend
pm add ~/code/api --description "REST API service"

# List projects
pm ls                                   # List all projects
pm ls --tags rust,backend              # Filter by tags (AND logic)
pm ls --tags-any frontend,web          # Filter by tags (OR logic)
pm ls --recent 7d                       # Show recent activity (7 days)
pm ls --detailed                        # Show detailed information

# Switch projects
pm switch <name>                        # Switch and open editor
pm s <name> --no-editor                # Switch without editor

# Scan for repositories
pm scan                                 # Scan default workspace
pm scan ~/Development                   # Scan specific directory
pm scan --show-all                      # Show all found repositories
```

### GitHub Integration

```bash
# Browse and select repositories interactively
pm browse                               # Browse your repositories
pm browse --username other-user         # Browse another user's repositories

# Clone and add repositories
pm load microsoft/vscode                # Clone to default location
pm load owner/repo --directory ~/custom # Clone to custom directory
```

### Tag Management

```bash
# Manage tags
pm tag add <project> <tags...>         # Add tags to project
pm tag remove <project> <tags...>      # Remove tags from project
pm tag list                            # List all available tags
pm tag show [project]                  # Show tags for project
```

### Configuration

```bash
# View and edit configuration
pm config show                         # Show current configuration
pm config edit                         # Edit in your preferred editor
pm config validate                     # Validate configuration file

# Get and set values
pm config get editor                    # Get specific value
pm config set editor code              # Set specific value
pm config list                         # List all available keys

# Backup and restore
pm config backup create [name]         # Create configuration backup
pm config backup restore <name>        # Restore from backup
pm config backup list                  # List available backups
pm config backup delete <name>         # Delete backup

# Templates
pm config template list                # List available templates
pm config template apply <name>        # Apply a template
pm config template save <name>         # Save current config as template

# Import/Export
pm config export --format yaml         # Export configuration
pm config import <file>                # Import configuration
pm config diff [backup]                # Show configuration differences
pm config history --limit 10           # Show configuration history
```

## Configuration

PM stores its configuration in a configurable location (default: `~/.config/pm/config.yml`). The configuration includes:

- **GitHub username**: For repository cloning and GitHub integration
- **Configuration path**: Where PM stores its configuration files (configurable during init)
- **Projects root directory**: Where your projects are located
- **Editor preference**: Your preferred code editor
- **Application settings**: Auto-open editor, show git status, etc.
- **Project data**: All managed projects and their metadata
- **Machine metadata**: Access tracking across different machines

### Example Configuration

```yaml
version: "1.0"
github_username: "your-username"
config_path: "/Users/you/.config/pm"
projects_root_dir: "/Users/you/workspace"
editor: "hx"
settings:
  auto_open_editor: false
  show_git_status: true
  recent_projects_limit: 10
projects: {}
machine_metadata: {}
```

### Configuration Setup

During `pm init`, you can customize:

- **Configuration Directory**: Where PM stores its files (default: `~/.config/pm`)
- **Projects Root Directory**: Where your projects will be stored (default: `~/workspace`)
- **Editor**: Your preferred code editor (VS Code, Helix, Vim, etc.)
- **Auto-open Editor**: Whether to automatically open editor when switching projects
- **Git Status Display**: Whether to show git status information in project listings

## Advanced Usage

### Filtering and Search

```bash
# Complex filtering
pm ls --tags rust,cli --recent 30d --limit 5

# Find projects by pattern
pm s my-proj    # Suggests similar project names if not found
```

### Workflow Integration

```bash
# Integration with other tools
pm s my-project && npm start
pm s api-service && docker-compose up -d

# Useful aliases
alias pml="pm ls"
alias pms="pm switch"
alias pma="pm add ."
```

### Multiple Machine Sync

PM automatically tracks which machine you last accessed each project on, making it easy to work across multiple development environments.

## Project Structure

```
project-manager/
├── src/
│   ├── commands/           # Command implementations
│   │   ├── config.rs      # Configuration management
│   │   ├── init.rs        # Interactive initialization
│   │   ├── project.rs     # Project operations
│   │   └── tag.rs         # Tag management
│   ├── config.rs          # Configuration types and loading
│   ├── display.rs         # Output formatting and colors
│   ├── validation.rs      # Input validation
│   ├── utils.rs           # Utility functions
│   └── main.rs            # CLI entry point
├── docs/                  # Comprehensive documentation
│   ├── ARCHITECTURE.md    # System architecture
│   ├── CLI_USAGE.md       # Detailed CLI guide
│   └── COMMANDS.md        # Command reference
├── schemas/               # JSON schemas for validation
├── script/                # Installation scripts
├── .github/workflows/     # CI/CD workflows
└── target/                # Build artifacts
```

## Development

### Running from Source

```bash
# Run in development mode
cargo run -- <command>

# Examples
cargo run -- init
cargo run -- ls --detailed
cargo run -- add . --tags rust,cli
```

### Testing

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Building

```bash
# Debug build
cargo build

# Optimized release build
cargo build --release

# Check code formatting
cargo fmt --all -- --check

# Run clippy lints
cargo clippy --all-targets --all-features -- -D warnings
```

## Troubleshooting

### Common Issues

**PM not initialized error**:
```bash
pm init  # Run the initialization wizard
```

**Project not found**:
```bash
pm ls    # List all projects to see available names
```

**Editor not opening**:
```bash
pm config set editor your-editor  # Set your preferred editor
pm s project --no-editor          # Skip editor launch
```

**Configuration issues**:
```bash
pm config validate  # Check configuration validity
pm config edit       # Manually edit configuration
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Check formatting (`cargo fmt --all -- --check`)
6. Run clippy (`cargo clippy --all-targets --all-features -- -D warnings`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## License

MIT License - see the [LICENSE](LICENSE) file for details.

## Version

Current version: 0.1.1

Built with ❤️ in Rust