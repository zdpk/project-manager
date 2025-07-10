# PM (Project Manager)

A fast, terminal-based project management CLI tool written in Rust. PM helps developers efficiently manage and switch between multiple projects with zero friction.

## Features

- **Fast project switching**: Switch between projects in under 1 second
- **Automatic project discovery**: Smart detection of Git repositories and programming languages
- **Flexible tagging system**: Organize projects with custom tags
- **Integration with Helix editor**: Seamless editor integration
- **Cross-platform support**: Works on macOS, Linux, and Windows

## Getting Started

### Prerequisites

- Rust 1.70+ (for building from source)
- Git
- Helix editor (recommended) or any text editor

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd project-manager

# Build the project
cargo build --release

# Install to your system
cargo install --path .
```

### Quick Start

```bash
# Initialize PM
pm init

# Add a project
pm add ~/workspace/my-project

# List projects
pm ls

# Switch to a project
pm s my-project
```

### Development

```bash
# Run in development mode
cargo run -- <command>

# Run tests
cargo test

# Build optimized release
cargo build --release
```

## Project Structure

```
project-manager/
├── src/                 # Source code
│   ├── main.rs         # Main entry point
│   ├── config.rs       # Configuration management
│   ├── utils.rs        # Utility functions
│   └── tag_commands.rs # Tag management
├── docs/               # Documentation
├── Cargo.toml         # Rust dependencies
├── .gitignore         # Git ignore patterns
└── README.md          # This file
```

## Commands

### Core Commands

- `pm init` - Initialize PM with your GitHub username and projects root directory
- `pm add <path>` - Add a new project to PM
- `pm ls` / `pm list` - List all managed projects with filtering options
- `pm s <name>` / `pm switch <name>` - Switch to a project and open in editor

### Tag Management

- `pm tag add <project> <tags>` - Add tags to a project
- `pm tag rm <project> <tags>` - Remove tags from a project
- `pm tag ls` - List all tags and their usage counts
- `pm tag show [project]` - Show tags for a specific project

### Filtering Options

The `pm ls` command supports powerful filtering:

- `-t, --tags <TAGS>` - Filter by tags (AND logic, all tags must match)
- `--tags-any <TAGS>` - Filter by tags (OR logic, any tag can match)  
- `-r, --recent <TIME>` - Show projects updated within time period
- `-l, --limit <N>` - Limit number of results
- `-d, --detailed` - Show detailed project information

### Time Period Format

Time periods support these units:
- `s` - seconds (e.g., `30s`)
- `m` - minutes (e.g., `15m`)
- `h` - hours (e.g., `2h`)
- `d` - days (e.g., `7d`)
- `w` - weeks (e.g., `2w`)
- `y` - years (e.g., `1y`)

### Usage Examples

```bash
# Add a project with tags
pm add ~/workspace/frontend --tags "work,react,frontend"

# List projects with work AND react tags
pm ls -t work,react

# List projects with frontend OR backend tags
pm ls --tags-any frontend,backend

# Show projects updated in last 7 days
pm ls -r 7d

# Show last 3 projects with detailed info
pm ls -l 3 -d

# Complex filtering: work projects from last 2 weeks, limit 5
pm ls -t work -r 2w -l 5

# Switch to project (both forms work)
pm s backend-api
pm switch backend-api --no-editor
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

MIT License