#!/bin/bash

# Demo script to showcase Rust extension creation and testing
echo "🦀 PM Rust Extension Template Demo"
echo "=================================="
echo ""
echo "This demo will:"
echo "1. Generate a sample Rust extension called 'sample-rust-monitor'"
echo "2. Show the generated file structure"
echo "3. Demonstrate compilation and execution"
echo "4. Test PM integration features"
echo ""

read -p "Press Enter to start the demo..."

echo ""
echo "🚀 Starting Rust extension generation..."
echo ""

# Run the extension creation
cargo run --bin create_sample_rust_extension

echo ""
echo "📁 Generated file structure:"
echo ""
find ./sample-rust-monitor -type f | head -20 | sort

echo ""
echo "🔍 Let's examine the key files:"
echo ""

echo "📄 Cargo.toml:"
echo "=============="
head -20 ./sample-rust-monitor/Cargo.toml

echo ""
echo "🦀 Main Rust source (first 30 lines):"
echo "====================================="
head -30 ./sample-rust-monitor/src/main.rs

echo ""
echo "🔧 Building the Rust project..."
echo ""
cd ./sample-rust-monitor
cargo build --release

echo ""
echo "✨ Testing the compiled binary:"
echo ""

echo "📋 Help output:"
echo "--------------"
./target/release/pm-ext-sample-rust-monitor --help

echo ""
echo "📊 Version information:"
echo "---------------------"
./target/release/pm-ext-sample-rust-monitor --version

echo ""
echo "🖥️  Example command demo:"
echo "========================="
./target/release/pm-ext-sample-rust-monitor example --message "Hello from Rust extension!"

cd ..

echo ""
echo "🎯 Demo complete!"
echo ""
echo "The Rust extension template provides:"
echo "• ✅ Complete Cargo.toml with proper dependencies"
echo "• ✅ Well-structured main.rs with clap argument parsing"
echo "• ✅ PM environment variable integration"
echo "• ✅ Cross-platform compilation support"
echo "• ✅ Professional README and documentation"
echo "• ✅ Proper error handling and logging"
echo ""
echo "📂 Extension files are in: ./sample-rust-monitor/"
echo "🦀 Compiled binary is at: ./sample-rust-monitor/target/release/pm-ext-sample-rust-monitor"