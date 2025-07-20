# PM Extension: quick-test

Quick test extension

## Installation

### Quick Install (Recommended)
```bash
curl -fsSL https://github.com/Test/pm-ext-quick-test/releases/latest/download/install.sh | sh
```

### Manual Installation
Download the appropriate binary for your platform from [Releases](https://github.com/Test/pm-ext-quick-test/releases):

- **linux-x86_64**: [pm-ext-quick-test-linux-x86_64](https://github.com/Test/pm-ext-quick-test/releases/latest/download/pm-ext-quick-test-linux-x86_64)

Then install via PM:
```bash
pm ext install quick-test --source ./pm-ext-quick-test-<platform>
```

## Usage

```bash
# Example usage
pm quick-test example --message "Hello World"

# Get help
pm quick-test --help
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
pm ext install quick-test --source ./target/release/pm-ext-quick-test
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
