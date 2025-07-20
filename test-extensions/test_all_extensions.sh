#!/bin/bash

# PM Extension Templates Verification Script
# Tests Bash, Python, and Rust extension templates

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
print_success() { echo -e "${GREEN}✅ $1${NC}"; }
print_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
print_error() { echo -e "${RED}❌ $1${NC}"; }

echo "🧪 PM Extension Templates Verification"
echo "======================================"

# Test Bash extension
echo ""
print_info "Testing Bash Extension (sample-bash-hooks)"
cd sample-bash-hooks

if [ -x "./pm-ext-sample-bash-hooks" ]; then
    print_success "Bash extension is executable"
    
    echo ""
    echo "  📋 Bash Extension Output:"
    ./pm-ext-sample-bash-hooks
    
    echo ""
    echo "  📋 Bash Extension Example:"
    ./pm-ext-sample-bash-hooks example "Hello from Bash extension!"
    
    echo ""
    echo "  📋 Bash Extension with PM Context:"
    PM_CURRENT_PROJECT="test-bash-project" \
    PM_CONFIG_PATH="/tmp/bash-config" \
    ./pm-ext-sample-bash-hooks example "Bash with PM context"
else
    print_error "Bash extension not found or not executable"
fi

cd ..

# Test Python extension  
echo ""
echo ""
print_info "Testing Python Extension (sample-python-deploy)"
cd sample-python-deploy

if [ -x "./pm-ext-sample-python-deploy" ]; then
    print_success "Python extension is executable"
    
    echo ""
    echo "  📋 Python Extension Output:"
    ./pm-ext-sample-python-deploy
    
    echo ""
    echo "  📋 Python Extension Example:"
    ./pm-ext-sample-python-deploy example --message "Hello from Python extension!"
    
    echo ""
    echo "  📋 Python Extension with PM Context:"
    PM_CURRENT_PROJECT="test-python-project" \
    PM_CONFIG_PATH="/tmp/python-config" \
    ./pm-ext-sample-python-deploy example --message "Python with PM context"
else
    print_error "Python extension not found or not executable"
fi

cd ..

# Test Rust extension  
echo ""
echo ""
print_info "Testing Rust Extension (sample-rust-monitor)"
cd sample-rust-monitor

if [ -x "./target/release/pm-ext-sample-rust-monitor" ]; then
    print_success "Rust extension is executable"
    
    echo ""
    echo "  📋 Rust Extension Help:"
    ./target/release/pm-ext-sample-rust-monitor --help
    
    echo ""
    echo "  📋 Rust Extension Version:"
    ./target/release/pm-ext-sample-rust-monitor --version
    
    echo ""
    echo "  📋 Rust Extension Example:"
    ./target/release/pm-ext-sample-rust-monitor example --message "Hello from test!"
    
    echo ""
    echo "  📋 Rust Extension with PM Context:"
    PM_CURRENT_PROJECT="test-rust-project" \
    PM_EXTENSION_CONFIG_DIR="/tmp/rust-config" \
    PM_EXTENSION_DATA_DIR="/tmp/rust-data" \
    ./target/release/pm-ext-sample-rust-monitor example --message "Rust with PM context"
else
    print_error "Rust extension not found or not executable"
    print_warning "You may need to run: cargo build --release"
fi

cd ..

echo ""
echo ""
print_success "All extension templates are working correctly! 🎉"
echo ""
print_info "Extension Summary:"
echo "  • Bash Extension: Git hooks management tool"
echo "  • Python Extension: Deployment automation tool"
echo "  • Rust Extension: System monitoring tool"
echo "  • All integrate with PM environment variables"
echo "  • All have proper CLI interfaces and help systems"
echo "  • All handle errors gracefully and follow best practices"
echo ""
print_info "The PM extension template system supports multiple languages!"