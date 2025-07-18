# Contributing to PM (Project Manager)

Thank you for your interest in contributing to PM! This guide will help you get started with contributing to both the core PM project and the extension ecosystem.

## Ways to Contribute

### 1. Core PM Development
- Bug fixes and performance improvements
- New core functionality
- Documentation improvements
- Testing and CI/CD enhancements

### 2. Extension Development
- Official extensions (hooks, direnv, 1password)
- Community extensions
- Extension templates and tools
- Extension documentation

### 3. Ecosystem Support
- Extension registry maintenance
- Documentation and tutorials
- Community support and Q&A
- Testing and quality assurance

## Getting Started

### Development Setup

```bash
# Clone the repository
git clone https://github.com/zdpk/project-manager.git
cd project-manager

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Build development version
make build-dev

# Test with development binary
./target/debug/_pm --help
```

### Development Commands

```bash
# Development mode (with extra features)
make run-dev -- <command>    # Run development binary
make build-dev               # Build development binary

# Production mode
make run-prod -- <command>   # Run production binary  
make build-prod              # Build production binary

# Testing and quality
make test                    # Run all tests
cargo fmt                    # Format code
cargo clippy                 # Lint code
```

## Contributing to Core PM

### Code Style
- Follow Rust conventions and `rustfmt` formatting
- Use meaningful variable and function names
- Add comprehensive error handling
- Write tests for new functionality
- Document public APIs with rustdoc

### Pull Request Process
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Format code (`cargo fmt`)
7. Lint code (`cargo clippy`)
8. Commit your changes (`git commit -m 'Add amazing feature'`)
9. Push to your branch (`git push origin feature/amazing-feature`)
10. Open a Pull Request

### Testing
- Write unit tests for new functions
- Add integration tests for new commands
- Test on multiple platforms when possible
- Include edge cases and error conditions

## Extension Development

### Quick Start

See the [Extension Development Guide](docs/EXTENSION_DEVELOPMENT_GUIDE.md) for comprehensive documentation.

#### 1. Create Extension Structure
```bash
mkdir pm-ext-myextension
cd pm-ext-myextension

# Create manifest
cat > manifest.yml << EOF
name: myextension
version: "1.0.0"
description: "My awesome PM extension"
author: "Your Name"
homepage: "https://github.com/username/pm-ext-myextension"
commands:
  - name: "start"
    help: "Start the service"
  - name: "stop"
    help: "Stop the service"
EOF
```

#### 2. Implement Binary (Any Language)

**Rust Example:**
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(hide = true)]
    PmInfo,
    Start,
    Stop,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::PmInfo => {
            println!(r#"{{"name": "myextension", "version": "1.0.0"}}"#);
        },
        Commands::Start => {
            println!("Starting service...");
        },
        Commands::Stop => {
            println!("Stopping service...");
        },
    }
}
```

**Shell Script Example:**
```bash
#!/bin/bash
case "$1" in
    "--pm-info")
        echo '{"name": "myextension", "version": "1.0.0"}'
        ;;
    "start")
        echo "Starting service..."
        ;;
    "stop")
        echo "Stopping service..."
        ;;
    *)
        echo "Usage: pm myextension <start|stop>"
        ;;
esac
```

#### 3. Test Locally
```bash
# Copy to extension directory
mkdir -p ~/.config/pm/extension/myextension
cp binary ~/.config/pm/extension/myextension/
cp manifest.yml ~/.config/pm/extension/myextension/
chmod +x ~/.config/pm/extension/myextension/binary

# Test with PM
pm ext list
pm ext info myextension
pm myextension start
```

### Extension Guidelines

#### Best Practices
- Keep extensions focused and lightweight
- Provide clear error messages
- Follow Unix philosophy (do one thing well)
- Use semantic versioning
- Include comprehensive help text
- Handle edge cases gracefully

#### Naming Conventions
- Repository: `pm-ext-{name}` (e.g., `pm-ext-docker`)
- Extension name: lowercase, no prefixes (e.g., `docker`, not `pm-docker`)
- Commands: descriptive, consistent with common tools
- Use established conventions where possible

#### Testing Extensions
```bash
# Manual testing
pm ext info myextension
pm myextension --help
pm myextension start
pm myextension stop

# Test error cases
pm myextension invalid-command
pm myextension start extra-args

# Test in different environments
PM_CURRENT_PROJECT='{}' pm myextension start
```

### Official Extension Repositories

Contribute to official extensions:
- **pm-ext-hooks**: [github.com/zdpk/pm-ext-hooks](https://github.com/zdpk/pm-ext-hooks)
- **pm-ext-direnv**: [github.com/zdpk/pm-ext-direnv](https://github.com/zdpk/pm-ext-direnv)
- **pm-ext-1password**: [github.com/zdpk/pm-ext-1password](https://github.com/zdpk/pm-ext-1password)

## Documentation

### Core Documentation
- Update README.md for new features
- Add to or update docs/ for complex features
- Include code examples and usage patterns
- Keep migration guides current

### Extension Documentation
- Each extension should have comprehensive README
- Include installation and usage instructions
- Provide configuration examples
- Document troubleshooting steps

## Community Guidelines

### Code of Conduct
- Be respectful and inclusive
- Focus on constructive feedback
- Help newcomers get started
- Celebrate contributions of all sizes

### Communication
- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: General questions, ideas
- **Pull Requests**: Code changes, documentation updates

### Quality Standards
- All code should be well-tested
- Documentation should be clear and complete
- Follow established patterns and conventions
- Consider backward compatibility

## Release Process

### Core PM Releases
1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create release PR
4. Tag release after merge
5. GitHub Actions builds and publishes binaries

### Extension Releases
1. Update version in manifest.yml
2. Create GitHub release with binary assets
3. Update extension registry (planned)
4. Announce in community channels

## Getting Help

- **Documentation**: Check docs/ directory first
- **GitHub Issues**: Search existing issues
- **Discussions**: Ask questions in GitHub Discussions
- **Discord**: Join our community Discord (link in README)

## Recognition

All contributors will be recognized in:
- GitHub contributors list
- Release notes for significant contributions
- Special mentions for major features or fixes

Thank you for helping make PM better! 🚀