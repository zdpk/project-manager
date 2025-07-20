#!/bin/bash

# PM Extension Test Script for Python template
# This simulates the extension creation process for Python

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

echo "🐍 Testing Python extension creation via PM CLI..."

# Clean up any existing test extension
if [ -d "./pm-ext-test-deploy" ]; then
    print_info "Cleaning up existing test extension"
    rm -rf "./pm-ext-test-deploy"
fi

# Expected inputs for the interactive wizard:
# Extension name: test-deploy
# Description: Deployment automation tool
# Author: testuser  
# Email: (empty)
# Template: Python (CLI applications) - option 2
# Platforms: All supported - y
# Directory: ./pm-ext-test-deploy
# Init git: yes
# Create GitHub repo: no

print_info "Running PM extension creation wizard with Python template"
echo "test-deploy
Deployment automation tool
testuser

2
y
./pm-ext-test-deploy
y
n" | timeout 30s ../target/debug/pm extension new

print_success "Python extension creation test completed."

# If the extension was created, test it
if [ -d "./pm-ext-test-deploy" ]; then
    print_success "Extension directory created successfully"
    
    cd "./pm-ext-test-deploy"
    
    # Check if the script exists and is executable
    if [ -x "./pm-ext-test-deploy" ]; then
        print_success "Extension script is executable"
        
        # Test basic functionality
        print_info "Testing extension commands:"
        
        echo ""
        echo "  📋 Help command:"
        if ./pm-ext-test-deploy --help > /dev/null 2>&1; then
            print_success "Help command works"
        else
            print_error "Help command failed"
        fi
        
        echo ""
        echo "  📋 Default behavior:"
        if ./pm-ext-test-deploy > /dev/null 2>&1; then
            print_success "Default command works"
        else
            print_error "Default command failed"
        fi
        
        echo ""
        echo "  📋 Example command:"
        if ./pm-ext-test-deploy example > /dev/null 2>&1; then
            print_success "Example command works"
        else
            print_error "Example command failed"
        fi
        
        echo ""
        echo "  📋 Example with custom message:"
        if ./pm-ext-test-deploy example --message "Test message" > /dev/null 2>&1; then
            print_success "Example with message works"
        else
            print_error "Example with message failed"
        fi
        
        # Test with PM environment variables
        echo ""
        echo "  📋 With PM environment variables:"
        if PM_CURRENT_PROJECT="test-project" \
           PM_CONFIG_PATH="/tmp/pm-config" \
           PM_VERSION="1.0.0" \
           ./pm-ext-test-deploy example --message "Testing with PM context" > /dev/null 2>&1; then
            print_success "PM context test works"
        else
            print_error "PM context test failed"
        fi
        
        # Test invalid command (should fail gracefully)
        echo ""
        echo "  📋 Invalid command handling:"
        if ./pm-ext-test-deploy invalid-command > /dev/null 2>&1; then
            print_error "Invalid command should have failed"
        else
            print_success "Invalid command handled correctly"
        fi
        
    else
        print_error "Extension script not found or not executable"
    fi
    
    cd ..
    
    # Show final summary
    echo ""
    print_info "Generated extension files:"
    find "./pm-ext-test-deploy" -type f -exec basename {} \; | sort | sed 's/^/  • /'
    
else
    print_error "Extension directory not created"
fi

echo ""
print_info "Python extension interactive test completed"