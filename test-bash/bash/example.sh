#!/bin/bash

# test-bash - Test Bash extension
# Author: testuser

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

# Get the command name from the arguments
COMMAND="$1"
shift || true

case "$COMMAND" in
    "run")
        print_success "test-bash Extension - Run Command"
        print_info "Executing Bash extension functionality..."
        
        if [ -n "$PM_CURRENT_PROJECT" ]; then
            print_info "Current PM project: $PM_CURRENT_PROJECT"
        fi
        
        if [ -n "$PM_CONFIG_PATH" ]; then
            print_info "PM config: $PM_CONFIG_PATH"
        fi
        
        echo "🔧 Bash extension is running successfully!"
        echo "📁 Working with shell scripts and environment variables"
        echo "⚡ Fast execution with native bash performance"
        ;;
        
    "status")
        print_success "test-bash Extension - Status Command"
        print_info "Extension Type: Bash"
        print_info "Version: 1.0.0"
        print_info "Status: Active and ready"
        
        # Check bash version
        bash_version=$(bash --version | head -n1)
        print_info "Bash: $bash_version"
        
        # Check if in PM context
        if [ -n "$PM_CURRENT_PROJECT" ]; then
            print_success "Running in PM project context"
        else
            print_warning "Not running in PM project context"
        fi
        ;;
        
    "version")
        print_success "test-bash Extension v1.0.0"
        print_info "Type: Bash scripting extension"
        print_info "Author: testuser"
        print_info "Homepage: https://github.com/testuser/test-bash"
        ;;
        
    *)
        echo "Usage: pm test-bash [COMMAND]"
        echo ""
        echo "Available Commands:"
        echo "  run        Run the bash extension functionality"
        echo "  status     Show bash extension status"
        echo "  version    Show extension version"
        echo ""
        echo "PM Environment Variables:"
        echo "  PM_CURRENT_PROJECT - Current project context"
        echo "  PM_CONFIG_PATH     - PM configuration path"
        echo "  PM_VERSION         - PM version"
        echo ""
        echo "Extension: test-bash"
        echo "Description: Test Bash extension for local installation testing"
        ;;
esac