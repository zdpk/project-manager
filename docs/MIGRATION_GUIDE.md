# PM Extension System Migration Guide

## Overview

PM has transitioned from built-in direnv and Git hooks functionality to a pure extension-based architecture. This migration creates a lightweight, fast core while maintaining powerful extensibility through the new extension system.

## What Changed

### ❌ Removed Built-in Features

#### Git Hooks Management
- **Removed**: `pm hooks` command and all subcommands
- **Removed**: Built-in hooks templates and management
- **Removed**: Hooks status in `pm status` output
- **Migration Path**: Install `pm-ext-hooks` extension

#### Direnv Integration  
- **Removed**: Direnv detection and status display
- **Removed**: `pm status` direnv information
- **Migration Path**: Install `pm-ext-direnv` extension

### ✅ Added Extension System

#### New Commands
```bash
pm ext install <name>        # Install extensions
pm ext uninstall <name>      # Remove extensions  
pm ext list                  # List installed extensions
pm ext info <name>           # Show extension details
pm ext update [name]         # Update extensions
pm ext search <query>        # Search extensions

# Extension usage (no prefix required)
pm <extension-name> <command>
```

## Migration Steps for Users

### 1. Check Current Usage

Before migration, check if you were using the removed features:

```bash
# Check if you have existing hooks
ls -la .git/hooks/

# Check if you have direnv files  
find . -name ".envrc" -type f

# Check if you have PM hooks templates
ls -la hooks/
```

### 2. Install Required Extensions

#### For Git Hooks Users
```bash
pm ext install hooks

# Migrate existing hooks
pm hooks sync              # Sync .githook/ to .git/hooks/
pm hooks status            # Check installation status
```

#### For Direnv Users  
```bash
pm ext install direnv

# Auto-activate direnv
pm direnv activate         # Enable auto-activation
pm direnv status           # Check direnv status
```

#### For 1Password Users
```bash
pm ext install 1password

# Generate .env files
pm 1password generate dev  # Generate .env.dev from .pw.yml
pm 1password sync          # Sync all environments
```

For complete documentation, see:
- [Extension System Overview](EXTENSION_SYSTEM.md)
- [Extension Development Guide](EXTENSION_DEVELOPMENT_GUIDE.md)
- [Extension Migration PRD](EXTENSION_MIGRATION_PRD.md)