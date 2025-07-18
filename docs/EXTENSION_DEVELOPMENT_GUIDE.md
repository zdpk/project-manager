# PM Extension Development Guide

## Overview

This guide helps developers create extensions for PM (Project Manager). Extensions are standalone executables that integrate seamlessly with PM's CLI interface.

## Quick Start

### 1. Extension Structure

Every extension consists of two files in `~/.config/pm/extension/{extension-name}/`:

```
~/.config/pm/extension/my-extension/
├── binary                # Executable file
└── manifest.yml         # Extension metadata
```

### 2. Create Manifest

Create `manifest.yml` with your extension metadata:

```yaml
name: my-extension
version: "1.0.0"
description: "My awesome PM extension"
author: "Your Name"
homepage: "https://github.com/username/pm-ext-my-extension"
pm_version: ">=0.1.0"
commands:
  - name: "start"
    help: "Start the service"
  - name: "stop"
    help: "Stop the service"
    aliases: ["halt"]
  - name: "status"
    help: "Show service status"
    args: ["[service-name]"]
```

### 3. Create Binary

Your binary must handle these cases:

```bash
#!/bin/bash

case "$1" in
    "--pm-info")
        # Return JSON metadata (optional - PM reads from manifest.yml)
        echo '{"name": "my-extension", "version": "1.0.0"}'
        ;;
    "start")
        echo "Starting service..."
        # Your implementation here
        ;;
    "stop"|"halt")
        echo "Stopping service..."
        # Your implementation here
        ;;
    "status")
        echo "Service status: running"
        # Your implementation here
        ;;
    *)
        echo "Usage: pm my-extension <start|stop|status>"
        echo "Commands:"
        echo "  start    Start the service"
        echo "  stop     Stop the service (alias: halt)"
        echo "  status   Show service status"
        ;;
esac
```

### 4. Make Executable

```bash
chmod +x ~/.config/pm/extension/my-extension/binary
```

### 5. Test Your Extension

```bash
pm ext list                    # Should show your extension
pm ext info my-extension       # Show extension details
pm my-extension start          # Execute your command
```

## Development Guidelines

### Binary Implementation

#### Languages
Extensions can be written in any language that produces an executable:

- **Shell Script** (bash, zsh) - Simple and fast
- **Rust** - Performance and safety
- **Go** - Fast compilation and single binary
- **Python** - Quick prototyping (use shebang)
- **Node.js** - JavaScript ecosystem (use shebang)

#### Example: Rust Extension

```rust
use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser)]
#[command(name = "docker-ext")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show extension metadata
    #[command(name = "--pm-info", hide = true)]
    PmInfo,
    /// List containers
    Ps,
    /// Show container logs
    Logs { container: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::PmInfo => {
            let info = serde_json::json!({
                "name": "docker",
                "version": "1.0.0",
                "description": "Docker container management"
            });
            println!("{}", info);
        },
        Commands::Ps => {
            // Get PM context
            let project_info = env::var("PM_CURRENT_PROJECT")?;
            let project: serde_json::Value = serde_json::from_str(&project_info)?;
            
            println!("Containers in project: {}", project["name"]);
            // Your docker ps implementation
        },
        Commands::Logs { container } => {
            println!("Logs for container: {}", container);
            // Your docker logs implementation
        },
    }
    
    Ok(())
}
```

#### Example: Python Extension

```python
#!/usr/bin/env python3

import sys
import json
import os

def main():
    if len(sys.argv) < 2:
        show_help()
        return
    
    command = sys.argv[1]
    
    if command == "--pm-info":
        info = {
            "name": "k8s",
            "version": "1.0.0",
            "description": "Kubernetes management extension"
        }
        print(json.dumps(info))
    elif command == "pods":
        list_pods()
    elif command == "logs":
        if len(sys.argv) > 2:
            show_logs(sys.argv[2])
        else:
            print("Error: pod name required")
            sys.exit(1)
    else:
        show_help()

def list_pods():
    # Get PM context
    project_info = os.getenv("PM_CURRENT_PROJECT", "{}")
    project = json.loads(project_info)
    
    print(f"Pods in project: {project.get('name', 'unknown')}")
    # Your kubectl implementation

def show_logs(pod_name):
    print(f"Logs for pod: {pod_name}")
    # Your kubectl logs implementation

def show_help():
    print("Kubernetes Extension")
    print("Commands:")
    print("  pods     List pods")
    print("  logs     Show pod logs")

if __name__ == "__main__":
    main()
```

### PM Context API

Your extension receives context through environment variables:

#### Available Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `PM_CONFIG_PATH` | Path to PM config file | `/Users/john/.config/pm/config.yml` |
| `PM_CURRENT_PROJECT` | Current project info (JSON) | `{"name": "my-app", "path": "/path/to/project"}` |
| `PM_VERSION` | PM version | `0.1.1` |
| `PM_EXTENSION_DIR` | Your extension directory | `/Users/john/.config/pm/extension/my-ext` |

#### Example: Using Context

```bash
#!/bin/bash

# Get current project information
PROJECT_JSON="$PM_CURRENT_PROJECT"
PROJECT_NAME=$(echo "$PROJECT_JSON" | jq -r '.name // "unknown"')
PROJECT_PATH=$(echo "$PROJECT_JSON" | jq -r '.path // "."')

echo "Current project: $PROJECT_NAME"
echo "Project path: $PROJECT_PATH"

# Check if project is a Git repository
if [ -d "$PROJECT_PATH/.git" ]; then
    echo "Git repository: Yes"
else
    echo "Git repository: No"
fi
```

### Manifest Specification

#### Required Fields

```yaml
name: extension-name        # Must match directory name
version: "1.0.0"           # Semantic version
description: "Brief description"
commands:                  # At least one command required
  - name: "command-name"
    help: "Command description"
```

#### Optional Fields

```yaml
author: "Your Name"
homepage: "https://github.com/user/repo"
pm_version: ">=0.1.0"      # PM version requirement
commands:
  - name: "command"
    help: "Description"
    aliases: ["alt1", "alt2"]  # Alternative names
    args: ["<arg1>", "[arg2]"] # Argument descriptions
```

#### Version Requirements

Specify PM version compatibility:

```yaml
pm_version: ">=0.1.0"      # Minimum version
pm_version: "^0.1.0"       # Compatible with 0.1.x
pm_version: "~0.1.5"       # Compatible with 0.1.5-0.1.x
pm_version: "0.1.0"        # Exact version
```

### Command Naming

- Use lowercase letters, numbers, hyphens
- Avoid spaces and special characters
- Be descriptive but concise
- Consider common abbreviations

Good examples:
- `start`, `stop`, `restart`
- `list`, `ls`
- `create`, `delete`, `update`
- `logs`, `status`, `info`

### Error Handling

#### Exit Codes

- `0` - Success
- `1` - General error
- `2` - Invalid arguments
- `64` - Usage error (BSD standard)

#### Error Messages

```bash
# Good error messages
echo "Error: Container 'web' not found" >&2
echo "Error: Missing required argument: <container-name>" >&2

# Include helpful suggestions
echo "Error: Unknown command 'stat'" >&2
echo "Did you mean 'status'?" >&2
```

### Testing Your Extension

#### Manual Testing

```bash
# Test installation detection
pm ext list

# Test extension info
pm ext info my-extension

# Test commands
pm my-extension --help
pm my-extension start
pm my-extension status

# Test error cases
pm my-extension invalid-command
pm my-extension start extra-arg
```

#### Automated Testing

Create a test script:

```bash
#!/bin/bash
# test-extension.sh

set -e

echo "Testing my-extension..."

# Test help
pm my-extension --help > /dev/null || {
    echo "FAIL: Help command failed"
    exit 1
}

# Test valid command
pm my-extension status > /dev/null || {
    echo "FAIL: Status command failed"
    exit 1
}

# Test invalid command (should fail)
if pm my-extension invalid-cmd 2>/dev/null; then
    echo "FAIL: Invalid command should have failed"
    exit 1
fi

echo "All tests passed!"
```

## Distribution

### Directory Distribution

For simple distribution, create a tarball:

```bash
cd ~/.config/pm/extension/
tar -czf my-extension.tar.gz my-extension/
```

Users can install by extracting to their extension directory.

### GitHub Releases

1. Create repository: `pm-ext-my-extension`
2. Add your source code
3. Create releases with binary assets
4. Users install with: `pm ext install my-extension --source github:user/pm-ext-my-extension`

### Package Structure

```
my-extension.tar.gz
├── binary              # Executable
├── manifest.yml        # Metadata
├── README.md          # Usage documentation
└── LICENSE            # License file (optional)
```

## Best Practices

### Performance

- Keep binaries small and fast
- Avoid heavy dependencies for simple tasks
- Use efficient languages for performance-critical extensions
- Cache data when appropriate

### User Experience

- Provide clear help messages
- Use consistent command naming
- Include usage examples
- Handle errors gracefully
- Respect user's terminal settings (colors, width)

### Security

- Validate all inputs
- Don't expose sensitive information
- Use secure defaults
- Respect file permissions
- Don't modify files outside your scope

### Compatibility

- Test on multiple platforms (Linux, macOS, Windows)
- Handle different shell environments
- Use portable commands and tools
- Document any external dependencies

## Examples

See these official extensions for reference:

- [pm-ext-docker](https://github.com/zdpk/pm-ext-docker) - Docker container management
- [pm-ext-k8s](https://github.com/zdpk/pm-ext-k8s) - Kubernetes integration
- [pm-ext-1password](https://github.com/zdpk/pm-ext-1password) - 1Password integration

## Troubleshooting

### Extension Not Found

```bash
# Check if extension is properly installed
pm ext list

# Check directory structure
ls -la ~/.config/pm/extension/my-extension/

# Verify binary is executable
ls -l ~/.config/pm/extension/my-extension/binary
```

### Permission Denied

```bash
# Make binary executable
chmod +x ~/.config/pm/extension/my-extension/binary
```

### Invalid Manifest

```bash
# Validate YAML syntax
pm ext info my-extension
```

### Environment Variables Not Set

Check if PM is passing context correctly:

```bash
#!/bin/bash
echo "PM_VERSION: $PM_VERSION"
echo "PM_CONFIG_PATH: $PM_CONFIG_PATH"
echo "PM_CURRENT_PROJECT: $PM_CURRENT_PROJECT"
echo "PM_EXTENSION_DIR: $PM_EXTENSION_DIR"
```

## Contributing

We welcome community extensions! Consider:

1. Adding your extension to the official registry
2. Sharing on GitHub with `pm-extension` topic
3. Contributing to the core PM project
4. Writing documentation and tutorials

## Support

- GitHub Issues: [PM Issues](https://github.com/zdpk/project-manager/issues)
- Discussions: [PM Discussions](https://github.com/zdpk/project-manager/discussions)
- Extension Registry: [PM Extensions](https://github.com/zdpk/pm-extensions)