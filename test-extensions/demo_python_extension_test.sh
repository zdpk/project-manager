#!/bin/bash

# Comprehensive Python Extension Test Script
# Tests the complete PM Python extension creation and functionality

set -e

echo "🐍 Testing PM Python Extension Creation and Functionality"
echo "========================================================="

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

# Function to run tests and track results
run_test() {
    local test_name="$1"
    local command="$2"
    local expected_status="${3:-0}"
    
    print_info "Running test: $test_name"
    
    if eval "$command"; then
        if [ "$expected_status" -eq 0 ]; then
            print_success "Test passed: $test_name"
            return 0
        else
            print_error "Test failed: $test_name (expected failure but command succeeded)"
            return 1
        fi
    else
        if [ "$expected_status" -ne 0 ]; then
            print_success "Test passed: $test_name (expected failure)"
            return 0
        else
            print_error "Test failed: $test_name"
            return 1
        fi
    fi
}

# Test counters
TESTS_RUN=0
TESTS_PASSED=0

# Change to project root
cd "$(dirname "$0")/.."

print_info "Starting Python extension tests from: $(pwd)"

# Test 1: Create sample Python extension
echo ""
print_info "Test 1: Creating sample Python extension"
TESTS_RUN=$((TESTS_RUN + 1))
if cargo run --bin create_sample_python_extension > /dev/null 2>&1; then
    print_success "Sample Python extension created successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Failed to create sample Python extension"
fi

# Move to test-extensions directory for remaining tests
cd test-extensions

# Verify extension directory exists
if [ ! -d "../sample-python-deploy" ]; then
    print_error "Sample Python extension directory not found"
    exit 1
fi

# Test 2: Verify all required files exist
echo ""
print_info "Test 2: Verifying generated files"
REQUIRED_FILES=(
    "../sample-python-deploy/pm-ext-sample-python-deploy"
    "../sample-python-deploy/extension.yml"
    "../sample-python-deploy/README.md"
    "../sample-python-deploy/LICENSE"
    "../sample-python-deploy/requirements.txt"
)

for file in "${REQUIRED_FILES[@]}"; do
    TESTS_RUN=$((TESTS_RUN + 1))
    if [ -f "$file" ]; then
        print_success "File exists: $(basename "$file")"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        print_error "Missing file: $(basename "$file")"
    fi
done

# Test 3: Verify script is executable
echo ""
print_info "Test 3: Checking script permissions"
TESTS_RUN=$((TESTS_RUN + 1))
if [ -x "../sample-python-deploy/pm-ext-sample-python-deploy" ]; then
    print_success "Python script is executable"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Python script is not executable"
fi

# Test 4: Python syntax validation
echo ""
print_info "Test 4: Validating Python syntax"
TESTS_RUN=$((TESTS_RUN + 1))
if python3 -m py_compile "../sample-python-deploy/pm-ext-sample-python-deploy" 2>/dev/null; then
    print_success "Python syntax is valid"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Python syntax validation failed"
fi

# Test 5: Extension functionality tests
echo ""
print_info "Test 5: Testing extension functionality"

cd ../sample-python-deploy

# Test help command
TESTS_RUN=$((TESTS_RUN + 1))
if run_test "Help command" "./pm-ext-sample-python-deploy --help > /dev/null"; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

# Test default behavior
TESTS_RUN=$((TESTS_RUN + 1))
if run_test "Default behavior" "./pm-ext-sample-python-deploy > /dev/null"; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

# Test example command
TESTS_RUN=$((TESTS_RUN + 1))
if run_test "Example command" "./pm-ext-sample-python-deploy example > /dev/null"; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

# Test example command with message
TESTS_RUN=$((TESTS_RUN + 1))
if run_test "Example with message" "./pm-ext-sample-python-deploy example --message 'Test message' > /dev/null"; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

# Test with PM environment variables
TESTS_RUN=$((TESTS_RUN + 1))
if run_test "PM environment context" "PM_CURRENT_PROJECT='test-proj' PM_CONFIG_PATH='/tmp/config' ./pm-ext-sample-python-deploy example > /dev/null"; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

# Test invalid command (should fail)
TESTS_RUN=$((TESTS_RUN + 1))
if run_test "Invalid command handling" "./pm-ext-sample-python-deploy invalid-command > /dev/null 2>&1" 1; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

# Test 6: Extension manifest validation
echo ""
print_info "Test 6: Validating extension manifest"
cd ../test-extensions

TESTS_RUN=$((TESTS_RUN + 1))
# Simple YAML validation without PyYAML dependency
if [ -f "../sample-python-deploy/extension.yml" ] && \
   grep -q "name: sample-python-deploy" "../sample-python-deploy/extension.yml" && \
   grep -q "version:" "../sample-python-deploy/extension.yml" && \
   grep -q "description:" "../sample-python-deploy/extension.yml" && \
   grep -q "author:" "../sample-python-deploy/extension.yml" && \
   grep -q "pm:" "../sample-python-deploy/extension.yml"; then
    print_success "Extension manifest is valid"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Extension manifest validation failed"
fi

# Test 7: README file content check
echo ""
print_info "Test 7: Checking README content"
TESTS_RUN=$((TESTS_RUN + 1))
if grep -q "sample-python-deploy" "../sample-python-deploy/README.md" && \
   grep -q "Deployment automation tool" "../sample-python-deploy/README.md"; then
    print_success "README contains expected content"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "README content validation failed"
fi

# Final results
echo ""
echo "========================================"
echo "🧪 Test Results Summary"
echo "========================================"
echo "Tests run: $TESTS_RUN"
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $((TESTS_RUN - TESTS_PASSED))"

if [ $TESTS_PASSED -eq $TESTS_RUN ]; then
    print_success "All tests passed! 🎉"
    echo ""
    print_info "The Python extension template is working correctly"
    print_info "Generated extension location: ../sample-python-deploy"
    print_info "Extension script: ../sample-python-deploy/pm-ext-sample-python-deploy"
    echo ""
    print_info "To test the extension manually:"
    echo "  cd ../sample-python-deploy"
    echo "  ./pm-ext-sample-python-deploy --help"
    echo "  ./pm-ext-sample-python-deploy example"
    echo "  ./pm-ext-sample-python-deploy example --message 'Custom message'"
    exit 0
else
    print_error "Some tests failed"
    echo ""
    print_warning "Please check the failed tests and fix any issues"
    exit 1
fi