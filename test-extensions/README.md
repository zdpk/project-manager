# PM Extension Testing Framework

This directory contains comprehensive tests and demonstrations for the PM extension system, focusing on Bash, Python, and Rust extension template generation.

## Overview

The test suite validates that the `ExtensionCreator` can properly generate functional Bash, Python, and Rust extensions that integrate seamlessly with the PM ecosystem.

## Test Files

### Core Test Suite

- **`tests/test_bash_extension_creation.rs`** - Main integration test suite with 4 comprehensive tests:
  - `test_bash_extension_creation` - Complete end-to-end validation
  - `test_bash_script_syntax_validation` - Bash syntax checking
  - `test_extension_creation_error_handling` - Error condition testing
  - `test_extension_with_different_platforms` - Multi-platform support

### Demonstration Scripts

#### Bash Extension Testing
- **`demo_bash_extension_test.sh`** - Interactive demo showing Bash test capabilities
- **`create_sample_bash_extension.rs`** - Binary to create sample Bash extensions
- **`test-bash-extension.sh`** - PM CLI wizard test for Bash extensions
- **`sample-bash-hooks/`** - Example generated Bash extension

#### Python Extension Testing
- **`demo_python_extension_test.sh`** - Comprehensive Python extension test suite
- **`create_sample_python_extension.rs`** - Binary to create sample Python extensions
- **`test-python-extension.sh`** - PM CLI wizard test for Python extensions
- **`sample-python-deploy/`** - Example generated Python extension

#### Rust Extension Testing
- **`demo_rust_extension_test.sh`** - Comprehensive Rust extension test suite
- **`create_sample_rust_extension.rs`** - Binary to create sample Rust extensions
- **`test-rust-extension.sh`** - Complete Rust extension validation
- **`sample-rust-monitor/`** - Example generated Rust extension

## Test Coverage

### 1. Extension Creation (`test_bash_extension_creation`)

✅ **File Generation**
- Main Bash script (`pm-ext-{name}`)
- README.md with installation instructions
- extension.yml manifest
- LICENSE file (MIT)
- .gitignore with appropriate patterns
- GitHub Actions workflow (`.github/workflows/release.yml`)

✅ **Script Content Validation**
- Proper bash shebang (`#!/bin/bash`)
- Error handling (`set -e`)
- Color output functions (print_info, print_success, etc.)
- PM environment integration (PM_CURRENT_PROJECT, PM_CONFIG_PATH)
- Command parsing and help system
- Author and description metadata

✅ **File Properties**
- Script is executable (Unix permissions 755)
- Proper file structure and organization

### 2. Syntax Validation (`test_bash_script_syntax_validation`)

✅ **Bash Syntax**
- Valid bash syntax verification using `bash -n`
- Proper function definitions
- Safe variable handling (`${1:-}`, `"$@"`)
- Error handling best practices

### 3. Error Handling (`test_extension_creation_error_handling`)

✅ **Failure Cases**
- Directory already exists
- Invalid configurations
- Proper error messages

### 4. Multi-Platform Support (`test_extension_with_different_platforms`)

✅ **Platform Coverage**
- macOS Apple Silicon (aarch64-apple-darwin)
- Linux x86_64 (x86_64-unknown-linux-gnu)
- Linux ARM64 (aarch64-unknown-linux-gnu)
- Windows x86_64 (x86_64-pc-windows-msvc)
- Windows ARM64 (aarch64-pc-windows-msvc)

✅ **GitHub Actions Workflow**
- Cross-platform build matrix
- Proper target configurations
- Asset generation and release automation

## Running Tests

### Quick Test Run
```bash
# Run all Bash extension tests
cargo test --test test_bash_extension_creation

# Run specific test
cargo test test_bash_extension_creation
```

### Interactive Demos
```bash
# Run comprehensive test suites
./test-extensions/demo_bash_extension_test.sh
./test-extensions/demo_python_extension_test.sh
./test-extensions/demo_rust_extension_test.sh

# Test PM CLI wizard interactions
./test-extensions/test-bash-extension.sh
./test-extensions/test-python-extension.sh
./test-extensions/test-rust-extension.sh

# Test all extension types together
./test-extensions/test_all_extensions.sh
```

### Create Sample Extensions
```bash
# Generate sample extensions for examination
cargo run --bin create_sample_bash_extension
cargo run --bin create_sample_python_extension
cargo run --bin create_sample_rust_extension

# Examine generated Bash extension
ls -la test-extensions/sample-bash-hooks/
cat test-extensions/sample-bash-hooks/pm-ext-sample-bash-hooks

# Examine generated Python extension  
ls -la test-extensions/sample-python-deploy/
cat test-extensions/sample-python-deploy/pm-ext-sample-python-deploy

# Examine generated Rust extension
ls -la test-extensions/sample-rust-monitor/
cat test-extensions/sample-rust-monitor/src/main.rs
cd test-extensions/sample-rust-monitor && cargo run -- --help
```

## Generated Extension Features

### Bash Extension (`sample-bash-hooks`)
- **Command System**: Extensible bash command parser with help
- **PM Integration**: Access to PM environment variables
- **Error Handling**: Robust error handling and colored output
- **Cross-Platform**: Works on Unix-like systems (Darwin, Linux)

### Python Extension (`sample-python-deploy`)
- **CLI Framework**: argparse-based command-line interface
- **Type Safety**: Python type hints and proper structure
- **PM Integration**: Access to PM environment variables via os.getenv
- **Error Handling**: Graceful error handling with colored output
- **Cross-Platform**: Works on Darwin, Linux, and Windows

### Rust Extension (`sample-rust-monitor`)
- **CLI Framework**: clap-based command-line interface with derive macros
- **Type Safety**: Full Rust type safety and memory safety
- **PM Integration**: Access to PM environment variables via std::env
- **Error Handling**: anyhow-based error handling with proper Result types
- **Performance**: Compiled binary with optimal performance
- **Cross-Platform**: Native compilation for Darwin, Linux, and Windows

### GitHub Actions CI/CD
- **Multi-Platform Builds**: Automated builds for all supported platforms
- **Release Automation**: Automatic binary releases with checksums
- **Install Script**: Generated installation script for easy deployment

### Documentation
- **README**: Comprehensive installation and usage instructions
- **Manifest**: Structured metadata for PM integration
- **License**: MIT license with proper attribution

## Integration with PM

Generated extensions seamlessly integrate with PM:

```bash
# Install generated Bash extension
pm ext install sample-bash-hooks --source ./pm-ext-sample-bash-hooks
pm sample-bash-hooks --help
pm sample-bash-hooks example

# Install generated Python extension
pm ext install sample-python-deploy --source ./pm-ext-sample-python-deploy
pm sample-python-deploy --help
pm sample-python-deploy example --message "Deploying with PM!"

# Install generated Rust extension
pm ext install sample-rust-monitor --source ./target/release/pm-ext-sample-rust-monitor
pm sample-rust-monitor --help
pm sample-rust-monitor example --message "Monitoring with PM!"
```

## Validation Checklist

- ✅ Script has proper shebang and is executable
- ✅ Contains PM environment variable integration
- ✅ Includes error handling and colored output
- ✅ Has command parsing with help system
- ✅ README contains installation instructions
- ✅ GitHub Actions supports multi-platform builds
- ✅ Extension manifest has correct metadata
- ✅ All files follow PM extension standards

## Test Architecture

The tests use:
- **Temporary directories** for isolated testing
- **Real ExtensionCreator** for authentic validation
- **Platform-specific logic** for cross-platform support
- **Comprehensive assertions** for thorough validation

This ensures that generated extensions work correctly in real-world scenarios and maintain compatibility with the PM ecosystem.