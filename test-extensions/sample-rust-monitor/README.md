# PM Extension: sample-rust-monitor

System monitoring tool built in Rust for PM demonstration

## Installation

### Quick Install (Recommended)
```bash
curl -fsSL https://github.com/PM Team/pm-ext-sample-rust-monitor/releases/latest/download/install.sh | sh
```

### Manual Installation
Download the appropriate binary for your platform from [Releases](https://github.com/PM Team/pm-ext-sample-rust-monitor/releases):

- **darwin-aarch64**: [pm-ext-sample-rust-monitor-darwin-aarch64](https://github.com/PM Team/pm-ext-sample-rust-monitor/releases/latest/download/pm-ext-sample-rust-monitor-darwin-aarch64)
- **linux-x86_64**: [pm-ext-sample-rust-monitor-linux-x86_64](https://github.com/PM Team/pm-ext-sample-rust-monitor/releases/latest/download/pm-ext-sample-rust-monitor-linux-x86_64)
- **linux-aarch64**: [pm-ext-sample-rust-monitor-linux-aarch64](https://github.com/PM Team/pm-ext-sample-rust-monitor/releases/latest/download/pm-ext-sample-rust-monitor-linux-aarch64)
- **windows-x86_64**: [pm-ext-sample-rust-monitor-windows-x86_64.exe](https://github.com/PM Team/pm-ext-sample-rust-monitor/releases/latest/download/pm-ext-sample-rust-monitor-windows-x86_64.exe)

Then install via PM:
```bash
pm ext install sample-rust-monitor --source ./pm-ext-sample-rust-monitor-<platform>
```

## Usage

```bash
# Example usage
pm sample-rust-monitor example --message "Hello World"

# Get help
pm sample-rust-monitor --help
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
pm ext install sample-rust-monitor --source ./target/release/pm-ext-sample-rust-monitor
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
