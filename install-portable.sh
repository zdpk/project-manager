#!/bin/bash
set -e

# PM Portable Installation Script
# Creates a self-contained installation in current directory
# Usage: curl -sSf https://raw.githubusercontent.com/zdpk/project-manager/main/install-portable.sh | sh

REPO="zdpk/project-manager"
BINARY_NAME="pm"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Determine the correct binary name based on OS and architecture
case "$OS" in
    "darwin")
        if [ "$ARCH" = "arm64" ]; then
            TARGET="aarch64-apple-darwin"
        else
            echo "Error: Only Apple Silicon (ARM64) macOS is supported"
            exit 1
        fi
        ;;
    *)
        echo "Error: $OS is not currently supported"
        echo "PM currently only supports macOS Apple Silicon (M1/M2)"
        exit 1
        ;;
esac

BINARY_FILE="${BINARY_NAME}-${TARGET}"

echo "Installing PM portable version for $OS ($ARCH)..."
echo "Target: $TARGET"

# Get the latest release download URL
DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/$BINARY_FILE"

echo "Downloading from: $DOWNLOAD_URL"

# Download the binary to current directory
if command -v curl >/dev/null 2>&1; then
    curl -L "$DOWNLOAD_URL" -o "$BINARY_NAME"
elif command -v wget >/dev/null 2>&1; then
    wget "$DOWNLOAD_URL" -O "$BINARY_NAME"
else
    echo "Error: curl or wget is required"
    exit 1
fi

# Make it executable
chmod +x "$BINARY_NAME"

echo ""
echo "✅ PM portable installation complete!"
echo "📁 Binary saved as: ./$BINARY_NAME"
echo ""
echo "Usage:"
echo "  ./$BINARY_NAME --help     # Show help"
echo "  ./$BINARY_NAME init       # Initialize PM"
echo "  ./$BINARY_NAME add <path> # Add a project"
echo ""
echo "To use 'pm' globally, add this directory to your PATH or move the binary:"
echo "  export PATH=\"\$(pwd):\$PATH\"   # Add current dir to PATH"
echo "  mv $BINARY_NAME ~/.local/bin/   # Move to local bin"