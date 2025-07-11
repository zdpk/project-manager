#!/bin/bash
set -e

# PM Installation Script
# Usage: curl -fsSL https://github.com/zdpk/project-manager/releases/latest/download/install.sh | sh

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

# Try local install first, fall back to system install
LOCAL_INSTALL_DIR="$HOME/.local/bin"
SYSTEM_INSTALL_DIR="/usr/local/bin"

# Create local bin directory if it doesn't exist
mkdir -p "$LOCAL_INSTALL_DIR"

# Check if local bin is in PATH
if echo "$PATH" | grep -q "$LOCAL_INSTALL_DIR"; then
    echo "Installing to $LOCAL_INSTALL_DIR..."
    mv "$BINARY_NAME" "$LOCAL_INSTALL_DIR/"
    INSTALLED_TO="$LOCAL_INSTALL_DIR"
else
    # Check if we can write to system directory
    if [ -w "$SYSTEM_INSTALL_DIR" ]; then
        echo "Installing to $SYSTEM_INSTALL_DIR..."
        mv "$BINARY_NAME" "$SYSTEM_INSTALL_DIR/"
        INSTALLED_TO="$SYSTEM_INSTALL_DIR"
    else
        # Try local install and add to PATH
        echo "Installing to $LOCAL_INSTALL_DIR..."
        mv "$BINARY_NAME" "$LOCAL_INSTALL_DIR/"
        INSTALLED_TO="$LOCAL_INSTALL_DIR"
        
        # Add to PATH in shell profiles
        add_to_path() {
            local shell_profile="$1"
            if [ -f "$shell_profile" ]; then
                if ! grep -q "$LOCAL_INSTALL_DIR" "$shell_profile"; then
                    echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$shell_profile"
                    echo "Added $LOCAL_INSTALL_DIR to PATH in $shell_profile"
                fi
            fi
        }
        
        add_to_path "$HOME/.bashrc"
        add_to_path "$HOME/.zshrc"
        add_to_path "$HOME/.profile"
        
        echo ""
        echo "⚠️  Please restart your terminal or run:"
        echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi
fi

# Cleanup
cd /
rm -rf "$TMP_DIR"

echo ""
echo "✅ PM successfully installed to $INSTALLED_TO!"
echo "Run 'pm --help' to get started."
echo ""
echo "Next steps:"
echo "  pm init    # Initialize PM with your settings"
echo "  pm add <path>  # Add your first project"
echo "  pm ls      # List projects"