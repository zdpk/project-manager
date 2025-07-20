#!/bin/bash

# test-local - Test extension for local installation
# Author: testuser
# Generated with PM Extension Template

set -e

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

# Show help if requested
if [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage: pm test-local [COMMAND] [OPTIONS]"
    echo ""
    echo "Available Commands:"
    echo "  help       Show this help"
    echo ""
    echo "PM Environment Variables:"
    echo "  PM_CURRENT_PROJECT - Current project context"
    echo "  PM_CONFIG_PATH     - PM configuration path"
    echo "  PM_VERSION         - PM version"
    echo ""
    echo "Extension: test-local"
    echo "Description: Test extension for local installation"
    echo "Author: testuser"
    exit 0
fi

# Main extension functionality
print_success "test-local Extension - Local Installation Test"

if [ -n "$PM_CURRENT_PROJECT" ]; then
    print_info "Current PM project: $PM_CURRENT_PROJECT"
fi

if [ -n "$PM_CONFIG_PATH" ]; then
    print_info "PM config: $PM_CONFIG_PATH"
fi

# Handle command line arguments
message="${1:-Hello from locally installed PM extension!}"
if [ "$#" -gt 0 ]; then
    print_success "Message: $message"
else
    print_success "Default message: $message"
fi

echo "🎯 This is the test-local extension installed locally!"
echo "🔧 Local installation successful!"
echo "💡 Try: pm test-local help"