# PM (Project Manager)

A fast, terminal-based project management CLI tool written in Rust. PM helps developers efficiently manage and switch between multiple projects with zero friction.

## Features

- **Fast project switching**: Switch between projects in under 1 second
- **Automatic project discovery**: Smart detection of Git repositories and programming languages
- **Flexible tagging system**: Organize projects with custom tags
- **Integration with Helix editor**: Seamless editor integration
- **Cross-platform support**: Works on macOS, Linux, and Windows

## Installation

### Option 1: Download Pre-built Binaries (Recommended)

Download the latest binary for your platform from the [Releases page](https://github.com/zdpk/project-manager/releases):

**Linux (x86_64)**:
```bash
curl -L https://github.com/zdpk/project-manager/releases/latest/download/pm-x86_64-unknown-linux-gnu -o pm
chmod +x pm
sudo mv pm /usr/local/bin/
```

**Linux (ARM64 - AWS Graviton, Google ARM, etc.)**:
```bash
curl -L https://github.com/zdpk/project-manager/releases/latest/download/pm-aarch64-unknown-linux-gnu -o pm
chmod +x pm
sudo mv pm /usr/local/bin/
```

**macOS (Intel)**:
```bash
curl -L https://github.com/zdpk/project-manager/releases/latest/download/pm-x86_64-apple-darwin -o pm
chmod +x pm
sudo mv pm /usr/local/bin/
```

**macOS (Apple Silicon M1/M2)**:
```bash
curl -L https://github.com/zdpk/project-manager/releases/latest/download/pm-aarch64-apple-darwin -o pm
chmod +x pm
sudo mv pm /usr/local/bin/
```

**Windows**:
Download `pm-x86_64-pc-windows-gnu.exe` from the releases page and add it to your PATH.

### Option 2: Build from Source

#### Prerequisites

- Rust 1.70+ (for building from source)
- Git
- Helix editor (recommended) or any text editor

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
├── tests/              # Test files
├── docs/               # Documentation
├── .github/            # GitHub workflows
├── .gitignore         # Git ignore patterns
└── README.md          # This file
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## License

MIT License