#!/bin/bash
# a - Example Command
# A bash extension for PM

set -euo pipefail

# Get command name and arguments
COMMAND="$1"
shift

case "$COMMAND" in
    "example")
        echo "🎉 a Extension - Example Command"
        echo "📋 Command: $COMMAND"
        echo "📦 Arguments: $*"
        echo "🔧 Extension is working correctly!"
        
        # Access PM environment variables
        echo ""
        echo "📍 PM Environment:"
        echo "  Config: $PM_CONFIG_PATH"
        echo "  Project: $PM_CURRENT_PROJECT"
        echo "  Version: $PM_VERSION"
        echo "  Extension Dir: $PM_EXTENSION_DIR"
        ;;
    "help"|*)
        echo "Usage: pm a [command] [args...]"
        echo ""
        echo "Available Commands:"
        echo "  example    Example command - replace with your functionality"
        echo ""
        echo "Extension: a"
        echo "Description: A bash extension for PM"
        ;;
esac
