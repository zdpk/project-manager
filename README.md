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
- `pm ls` - List all managed projects
- `pm s <name>` - Switch to a project and open in editor

### Tag Management

- `pm tag add <project> <tags>` - Add tags to a project
- `pm tag rm <project> <tags>` - Remove tags from a project
- `pm tag ls` - List all tags and their usage counts
- `pm tag show [project]` - Show tags for a specific project

### Usage Examples

```bash
# Add a project with tags
pm add ~/workspace/frontend --tags "work,react,frontend"

# List projects with filtering (coming soon)
pm ls --tags work --recent 7d --limit 5

# Switch to project without opening editor
pm s backend-api --no-editor
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

MIT License