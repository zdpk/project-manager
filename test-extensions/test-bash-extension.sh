#!/bin/bash

# PM Extension Test Script for Bash template
# This simulates the extension creation process

echo "Testing Bash extension creation..."

# Expected inputs for the interactive wizard:
# Extension name: test-hooks
# Description: Git hooks management tool
# Author: testuser  
# Email: (empty)
# Template: Bash (Simple scripts)
# Platforms: All supported
# Directory: ./pm-ext-test-hooks
# Init git: yes
# Create GitHub repo: no

echo "test-hooks
Git hooks management tool
testuser

1
y
./pm-ext-test-hooks
y
n" | timeout 30s ../target/debug/pm extension new

echo "Bash extension creation test completed."