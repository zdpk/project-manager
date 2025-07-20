#!/bin/bash

# TEST-HOOKS - Example Command
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

# Example command implementation
local message="${1:-Hello from PM extension!}"
print_success "$message"

if [ -n "$PM_CURRENT_PROJECT" ]; then
    print_info "Current PM project: $PM_CURRENT_PROJECT"
fi

if [ -n "$PM_CONFIG_PATH" ]; then
    print_info "PM config: $PM_CONFIG_PATH"
fi

echo "🎯 This is an example command for the test-hooks extension"
echo "🔧 You can modify this file to implement your example functionality"
