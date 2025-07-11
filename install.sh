#!/bin/bash
set -e

# PM Installation Script
# Usage: curl -sSf https://raw.githubusercontent.com/zdpk/project-manager/main/install.sh | sh

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
    "linux")
        echo "Error: Linux is not currently supported"
        echo "PM currently only supports macOS Apple Silicon (M1/M2)"
        exit 1
        ;;
    *)
        echo "Error: Unsupported OS: $OS"
        exit 1
        ;;
esac

BINARY_FILE="${BINARY_NAME}-${TARGET}"

echo "Installing PM for $OS ($ARCH)..."
echo "Target: $TARGET"

# Get the latest release download URL
DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/$BINARY_FILE"

# Create temporary directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

echo "Downloading from: $DOWNLOAD_URL"

# Download the binary
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

# Install to /usr/local/bin (with sudo if needed)
INSTALL_DIR="/usr/local/bin"
if [ -w "$INSTALL_DIR" ]; then
    mv "$BINARY_NAME" "$INSTALL_DIR/"
else
    echo "Installing to $INSTALL_DIR (requires sudo)..."
    sudo mv "$BINARY_NAME" "$INSTALL_DIR/"
fi

# Cleanup
cd /
rm -rf "$TMP_DIR"

echo "✅ PM successfully installed!"
echo "Run 'pm --help' to get started."
echo ""
echo "Next steps:"
echo "  pm init    # Initialize PM with your settings"
echo "  pm add <path>  # Add your first project"
echo "  pm ls      # List projects"