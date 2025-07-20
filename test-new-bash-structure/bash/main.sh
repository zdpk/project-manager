#!/bin/bash

# TEST-HOOKS - Git hooks management with new structure
# Author: PM Team
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

# Main entry point for the extension
print_success "test-hooks Extension - Main Command"

if [ -n "$PM_CURRENT_PROJECT" ]; then
    print_info "Current PM project: $PM_CURRENT_PROJECT"
fi

if [ -n "$PM_CONFIG_PATH" ]; then
    print_info "PM config: $PM_CONFIG_PATH"
fi

echo "🔧 This is the main command for the test-hooks extension"
echo "📝 Replace this with your extension's main functionality"
