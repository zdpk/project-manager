# PM Extension: test-hooks

Git hooks management with new structure

## Installation

### Quick Install (Recommended)
```bash
curl -fsSL https://github.com/PM Team/pm-ext-test-hooks/releases/latest/download/install.sh | sh
```

### Manual Installation
Download the appropriate binary for your platform from [Releases](https://github.com/PM Team/pm-ext-test-hooks/releases):

- **darwin-aarch64**: [pm-ext-test-hooks-darwin-aarch64](https://github.com/PM Team/pm-ext-test-hooks/releases/latest/download/pm-ext-test-hooks-darwin-aarch64)
- **linux-x86_64**: [pm-ext-test-hooks-linux-x86_64](https://github.com/PM Team/pm-ext-test-hooks/releases/latest/download/pm-ext-test-hooks-linux-x86_64)

Then install via PM:
```bash
pm ext install test-hooks --source ./pm-ext-test-hooks-<platform>
```

## Usage

```bash
# Example usage
pm test-hooks example --message "Hello World"

# Get help
pm test-hooks --help
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
pm ext install test-hooks --source ./target/release/pm-ext-test-hooks
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
Generated with PM Extension Template on 2025-07-19
