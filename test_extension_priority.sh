#!/bin/bash

# Test script for extension priority logic
set -e

echo "🧪 Testing Extension Priority Logic"
echo "=================================="

# Change to the project directory
cd /Users/x/github/zdpk/project-manager

echo "📍 Current directory: $(pwd)"

# Build the project
echo "🔨 Building project..."
if cargo build; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

# Check if the binary exists
if [ -f target/debug/_pm ]; then
    echo "✅ Binary found: target/debug/_pm"
else
    echo "❌ Binary not found"
    exit 1
fi

# Test if extension 'a' is installed locally or install it
echo "📦 Checking extension 'a'..."
if [ -d a/ ]; then
    echo "✅ Extension 'a' directory found"
    
    # Install the extension locally
    echo "🔧 Installing extension 'a' locally..."
    target/debug/_pm ext install --local a/
else
    echo "❌ Extension 'a' directory not found"
    exit 1
fi

# Source shell integration
echo "🐚 Setting up shell integration..."
if [ -f /Users/x/.config/_pm/_pm.sh ]; then
    source /Users/x/.config/_pm/_pm.sh
    echo "✅ Shell integration loaded"
else
    echo "⚠️  Shell integration not found, using direct binary"
    # Create alias for testing
    alias _pm='/Users/x/github/zdpk/project-manager/target/debug/_pm'
fi

echo ""
echo "🧪 Running Tests"
echo "==============="

# Test 1: List extensions
echo "1️⃣  Testing: _pm ext list"
_pm ext list

# Test 2: Test extension command execution
echo ""
echo "2️⃣  Testing: _pm a example (should execute extension)"
_pm a example

# Test 3: Test built-in add command still works
echo ""
echo "3️⃣  Testing: _pm add --help (should show built-in help)"
_pm add --help

echo ""
echo "✅ All tests completed successfully!"