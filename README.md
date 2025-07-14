# PM (Project Manager)

A fast, terminal-based project management CLI tool written in Rust. PM helps developers efficiently manage and switch between multiple projects with zero friction.

## Features

- **Interactive Setup**: Guided initialization with auto-detection and GitHub integration
- **Fast Project Switching**: Switch between projects in under 1 second
- **Smart Project Discovery**: Automatic detection of Git repositories and programming languages
- **Intuitive Tag Selection**: Two-step interface with smart filtering and flexible tag creation
- **GitHub Integration**: Clone and manage repositories directly from GitHub
- **Advanced Configuration**: Backup, templates, export/import, and validation
- **Machine-Specific Metadata**: Track project access and usage across different machines
- **Fast Directory Switching**: Instant navigation to project directories
- **Shell Integration**: Automatic shell setup for Fish, Bash, and Zsh with directory changing
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

# Add a project (with intuitive two-step tag selection)
pm add ~/my-projects/awesome-app

# Add current directory with enhanced experience  
pm add .

# Add all subdirectories in current folder
pm add *

# Scan for existing repositories
pm scan

# Clone GitHub repositories (interactive browse)
pm clone

# Clone specific repository
pm clone owner/repo

# List all projects
pm list

# List projects with filters (aliases work too)
pm ls --tags rust --recent 7d --limit 10

# Switch to a project directory
pm switch my-project

# Using alias
pm sw my-project
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
> Show git status in project listings? Yes

📂 Creating configuration directory: /Users/you/.config/pm

✅ PM initialized successfully
📂 Config directory: /Users/you/.config/pm
⚙️ Config file: /Users/you/.config/pm/config.yml

🎯 Next steps:
  pm add .                       # Add current directory with interactive tags
  pm add *                       # Add all subdirectories
  pm scan                        # Scan for existing repositories
  pm clone <owner>/<repo>        # Clone specific repository
  pm clone                       # Browse and select repositories

📖 Use 'pm --help' to see all available commands
```

> **Note**: PM automatically detects your GitHub username from GitHub CLI if you're authenticated. If detection fails, you can still enter it manually.

## Command Reference

All commands support aliases shown in parentheses. Use `pm --help` for complete usage information.

### Project Management

```bash
# Add projects with intuitive two-step tag selection (alias: a)
pm add <path>                                   # Add specific directory
pm add . --name "My Project"                   # Add current directory with custom name
pm add ~/code/api --description "REST API"     # Add with description
pm add *                                        # Add all subdirectories (batch mode)
pm add my-project                               # Create and add new project

# List projects (alias: ls)
pm list                                         # List all projects
pm ls --tags rust,backend                      # Filter by tags (AND logic) 
pm ls --tags-any frontend,web                  # Filter by tags (OR logic)
pm ls --recent 7d                               # Show recent activity (7 days)
pm ls --detailed                                # Show detailed information

# Switch projects (alias: sw)
pm switch <name>                                # Switch to project directory
pm sw <name>                                    # Switch to project (alias)

# Remove projects (alias: rm)
pm rm                                           # Interactive project selection
pm rm <name>                                    # Remove project by name
pm rm <name> -y                                 # Remove without confirmation
```

### GitHub Integration

```bash
# Clone repositories (alias: cl)
pm clone                                        # Interactive browse your repositories
pm clone microsoft/vscode                      # Clone specific repository
pm clone owner/repo --directory ~/custom       # Clone to custom directory

# Scan for repositories (alias: sc)
pm scan                                         # Scan current directory
pm scan ~/Development                           # Scan specific directory
pm scan --show-all                             # Show all found repositories
```

### Two-Step Tag Selection Interface

When adding projects, PM provides an intuitive two-step workflow:

**Step 1 - Choose Your Action:**
```
? What would you like to do?
  > Create Project [project-name] (without tags)
    Add tags to this project
    Create new tag and add to project
```

**Step 2 - Smart Tag Selection:**
```
🏷️ Select tags for this project (type to filter):
  [ ] frontend (8 projects)
  [ ] react (5 projects)
  [ ] typescript (6 projects)
```

**Key Improvements:**
- **🎯 Clear separation**: Actions vs. tag selection
- **⚡ No confusion**: Simple select for actions, checkboxes for tags
- **🔍 Real-time filtering**: Type to find tags instantly
- **✨ Flexible workflow**: Create new, select existing, or skip tags
- **📊 Usage insights**: See project counts for each tag

**Example Workflow:**

```bash
$ pm add ./react-dashboard

? What would you like to do?
  > Add tags to this project

🏷️ Select tags for this project (type to filter): react
  [x] react (5 projects)

✅ Successfully added project 'react-dashboard' with tags: react
```

For more detailed examples and workflows, see [TAG_SELECTION_GUIDE.md](docs/TAG_SELECTION_GUIDE.md).


### Tag Management

```bash
# Manage tags (alias: t)
pm tag add <project> <tags...>                 # Add tags to project
pm tag remove <project> <tags...>              # Remove tags from project
pm tag list                                     # List all available tags
pm tag show [project]                          # Show tags for project
```

### Configuration

```bash
# View and edit configuration (alias: cf)
pm config                              # Show current configuration (default)
pm config show                         # Show current configuration  
pm config edit                         # Edit configuration file
pm config validate                     # Validate configuration file

# Get and set values
pm config get settings.show_git_status # Get specific value
pm config set settings.show_git_status true # Set specific value
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

- **Configuration path**: Where PM stores its configuration files (configurable during init)
- **Application settings**: Show git status, recent projects limit, etc.
- **Project data**: All managed projects and their metadata
- **Machine metadata**: Access tracking across different machines

### Example Configuration

```yaml
version: "1.0"
config_path: "/Users/you/.config/pm"
settings:
  show_git_status: true
  recent_projects_limit: 10
projects: {}
machine_metadata: {}
```

### Configuration Setup

During `pm init`, you can customize:

- **Configuration Directory**: Where PM stores its files (default: `~/.config/pm`)
- **Git Status Display**: Whether to show git status information in project listings

## Advanced Usage

### Filtering and Search

```bash
# Complex filtering
pm ls --tags rust,cli --recent 30d --limit 5

# Find projects by pattern
pm s my-proj    # Suggests similar project names if not found
```

### Shell Integration

PM provides automatic shell integration that allows `pm sw` to actually change your shell's current directory (not just the PM process directory).

#### Fish Shell (Automatic)
When you run `pm sw` in Fish shell, PM automatically detects Fish and sets up integration:

```bash
# First time using pm sw in Fish
pm sw my-project
# PM detects Fish and offers to setup integration
? Setup Fish shell integration for directory switching? › Yes
🐠 Fish shell integration installed successfully
   Function file: ~/.config/fish/functions/pm.fish
   Usage: pm sw <project> will now change your shell directory
📁 Changed directory to: /path/to/my-project
```

**Fish Integration Features:**
- ✅ **Zero manual setup** - automatic detection and installation
- ✅ **Native autoloading** - uses Fish's function system
- ✅ **Conflict handling** - backup/remove options for existing functions
- ✅ **Easy removal** - `rm ~/.config/fish/functions/pm.fish`

#### Bash/Zsh (Coming Soon)
```bash
# Manual setup for now
pm() { if [[ "$1" == "sw" ]]; then eval "$(command pm "$@")"; else command pm "$@"; fi; }
```

### Workflow Integration

```bash
# Integration with other tools (with shell integration)
pm sw my-project && npm start
pm sw api-service && docker-compose up -d

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

**GitHub connectivity issues**:
```bash
gh auth status                    # Check GitHub CLI authentication
pm clone                          # Try interactive repository browsing
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