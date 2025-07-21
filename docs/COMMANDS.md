# PM (Project Manager) - Command Reference

This document provides a detailed reference for all commands available in the `pm` CLI tool.

## Global Options

*   `-h, --help`: Print help information
*   `-v, --version`: Print version information

## Commands

All commands now use a flat structure with intuitive aliases. No more nested subcommands like `pm project` or `pm github`.

### `pm init`

Initializes PM with interactive configuration setup.

**Usage:**

```bash
pm init
```

**Interactive Setup:**

PM will guide you through setting up:

1. **Configuration directory**: Where PM stores its configuration files (default: `~/.config/pm`)
2. **Git status display**: Whether to show git status information in project listings
3. **Shell integration**: Automatic setup for Fish, Zsh, or Bash shells for directory switching

**Shell Integration Setup:**
- Automatically detects your current shell (Fish, Zsh, Bash)
- Creates appropriate integration files for directory switching
- Handles conflicts with existing configurations safely
- Provides backup options for existing files

**Example Output:**

```bash
$ pm init
🚀 Initializing PM...

🔍 Detecting GitHub username from GitHub CLI...
✅ Detected GitHub username: your-username
> Use detected GitHub username 'your-username'? Yes
> Configuration directory: ~/.config/pm
  Where PM configuration files will be stored (press Enter for default)
> Show git status in project listings? Yes

📂 Creating configuration directory: /Users/you/.config/pm

✅ PM initialized successfully
👤 GitHub username: your-username
📂 Config directory: /Users/you/.config/pm
⚙️ Config file: /Users/you/.config/pm/config.yml

🎯 Next steps:
  pm add .                       # Add current directory with interactive tags
  pm add *                       # Add all subdirectories
  pm scan                        # Scan for existing repositories
  pm clone <owner>/<repo>        # Clone from GitHub
  pm clone                       # Browse and select GitHub repositories
```

**Fallback Example (GitHub CLI not authenticated):**

```bash
$ pm init
🚀 Initializing PM...

🔍 Detecting GitHub username from GitHub CLI...
⚠️  Could not detect GitHub username from GitHub CLI
💡 Make sure GitHub CLI is installed and you're authenticated with 'gh auth login'
> GitHub username: [manual input required]
```

**Behavior:**

*   If configuration already exists, warns the user and provides instructions to reinitialize
*   Creates configuration directory if it doesn't exist
*   Generates a YAML configuration file with user preferences
*   Provides clear next steps for getting started

### `pm add` (alias: `pm a`)

Adds projects to PM's management list with enhanced interactive features.

**Usage:**

```bash
pm add .                                        # Add current directory
pm add *                                        # Add all subdirectories in current folder  
pm add my-project                               # Create and add new project
pm add /path/to/project --name "Custom Name"   # Add with custom name
pm add . --description "My awesome project"    # Add with description
```

**Special Path Patterns:**

*   `.` - Current directory
*   `*` - All subdirectories in current directory
*   `<path>` - Specific path (relative to current dir or absolute)

**Options:**

*   `-n, --name <NAME>`: Specify a custom name for the project. If omitted, the directory name will be used.
*   `-d, --description <DESCRIPTION>`: A brief description of the project.

**Interactive Tag Selection:**

For single operations, PM provides a flexible tag input interface:

```
🏷️  Tags: _____ 

Type tag name to search/create, space for multiple, Enter to confirm
```

**Two-Step Tag Selection Interface:**

PM now uses a clean two-step approach that eliminates interface confusion:

**Step 1 - Action Selection (Always First):**
- **Create Project without tags**: Quick project creation
- **Add tags to this project**: Select from existing tags with filtering
- **Create new tag and add**: Create new tags, optionally add existing ones

**Step 2 - Conditional Tag Selection:**
- **Smart filtering**: Type to filter existing tags in real-time
- **Usage statistics**: See project counts for existing tags  
- **Multiple selection**: Use Space key to select/deselect tags
- **Flexible workflow**: Mix new and existing tags as needed

**Example Workflows:**

**Workflow A - No Tags (Fastest):**
```bash
$ pm add ./quick-script

? What would you like to do?
  > Create Project [quick-script] (without tags)
    Add tags to this project
    Create new tag and add to project

✅ Successfully added project 'quick-script'
   Path: /Users/you/projects/quick-script
```

**Workflow B - Select Existing Tags:**
```bash
$ pm add ./web-dashboard

? What would you like to do?
    Create Project [web-dashboard] (without tags)
  > Add tags to this project
    Create new tag and add to project

🏷️ Select tags for this project (type to filter):
  [ ] frontend (12 projects)
  [ ] react (8 projects)
  [ ] dashboard (3 projects)
  [ ] typescript (6 projects)

# Type "react" to filter:
🏷️ Select tags for this project (type to filter): react
  [x] react (8 projects)

✅ Successfully added project 'web-dashboard' with tags: react
   Path: /Users/you/projects/web-dashboard
```

**Workflow C - Create New + Existing Tags:**
```bash
$ pm add ./ml-project

? What would you like to do?
    Create Project [ml-project] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: machine-learning
? Add another new tag? Yes

✨ Create new tag: pytorch
? Add another new tag? No

? Add existing tags as well? Yes

🏷️ Select tags for this project (type to filter):
  [x] python (15 projects)
  [x] research (4 projects)

✅ Successfully added project 'ml-project' with tags: machine-learning, pytorch, python, research
   Path: /Users/you/projects/ml-project
```

**Directory Creation:**
For non-existent paths, PM will:
1. **Confirm creation**: Ask permission to create missing directories
2. **Proceed with tagging**: Continue with interactive tag selection

**Batch Operations:**
For `pm add *`, the process is streamlined:
- Creates/validates all subdirectories
- Skips interactive tagging for efficiency
- Provides comprehensive summary of results

**Interactive Features:**
*   **Tag Selection**: For single operations, interactive tag selection with existing tags + ability to create new ones
*   **Directory Creation**: Prompts to create directories that don't exist
*   **Duplicate Handling**: Skips already registered projects

**Behavior:**

*   Resolves paths relative to current working directory
*   Automatically detects Git repositories and stores last commit time
*   For single operations: full interactive experience with tag selection
*   For batch operations (`*`): streamlined processing with summary
*   Intelligent duplicate detection and handling

### `pm list` (alias: `pm ls`)

Lists all projects currently managed by PM.

**Usage:**

```bash
pm list                                         # List all projects
pm ls --tags rust,backend                      # Filter by tags (AND logic) 
pm ls --tags-any frontend,web                  # Filter by tags (OR logic)
pm ls --recent 7d                               # Show recent activity (7 days)
pm ls --detailed                                # Show detailed information
```

**Options:**

*   `-t, --tags <TAGS>`: Filter by tags (comma-separated, all tags must match)
*   `--tags-any <TAGS>`: Filter by tags (comma-separated, any tag can match)  
*   `-r, --recent <TIME>`: Show only projects updated within time period (e.g., 7d, 2w, 1m, 1y)
*   `-l, --limit <NUMBER>`: Limit the number of results
*   `-d, --detailed`: Show detailed information

**Behavior:**

*   Lists projects sorted by `git_updated_at` (if available), then `updated_at`, then `created_at`
*   Asynchronously updates `git_updated_at` for projects if it's missing or older than 1 hour
*   Displays comprehensive project information in columnar format:
    - **NAME**: Project name
    - **PATH**: Full directory path
    - **GIT**: Git repository status (📁 = Git repo, ❌ = not Git repo)
    - **TAGS**: Project tags in bracket format
    - **TIME**: Last activity time in human-readable format

**Example Output:**
```
📋 Active Projects (3 found)

NAME                 PATH                                     GIT   TAGS            TIME           
project-manager      /Users/you/projects/project-manager     📁    [rust,cli]      2 hours ago
web-app             /Users/you/projects/web-app              📁    [frontend]      1 day ago
my-script           /Users/you/scripts/my-script             ❌    [python]        1 week ago
```

### `pm switch` (alias: `pm sw`)

Switches to a specified project's directory with automatic shell integration.

**Usage:**

```bash
pm switch my-project                            # Switch to project directory
pm sw my-project                               # Switch using alias
```

**Arguments:**

*   `<NAME>`: Project name to switch to

**Behavior:**

*   Changes the current working directory to the project's path
*   Records project access for usage tracking
*   Provides suggestions for similar project names if not found
*   Automatically sets up shell integration for Fish, Zsh, and Bash shells
*   With shell integration, changes your shell's current directory (not just PM's)

**Shell Integration:**

Shell integration is automatically set up during `pm init` for all supported shells:

```bash
pm init
🚀 Initializing PM...
📂 Configuration directory: ~/.config/pm
🐚 Show git status in project listings? › Yes  
🔧 Setup Zsh shell integration for directory switching? › Yes
   Detected shell: Zsh
🐚 Zsh shell integration installed successfully
   Function file: ~/.config/pm/pm.zsh
   Added to: ~/.zshrc
✅ PM initialized successfully!
```

**Supported Shells:**
- **Fish**: `~/.config/fish/functions/pm.fish` (native autoloading)
- **Zsh**: `~/.config/pm/pm.zsh` + `.zshrc` sourcing  
- **Bash**: `~/.config/pm/pm.bash` + `.bashrc` sourcing

**Integration Features:**
- **Automatic detection**: No manual setup required during init
- **Conflict handling**: Backup options for existing functions/files
- **Easy removal**: Delete the integration file to disable
- **Environment variable support**: Use `_PM_BINARY` to specify custom PM binary path

**Environment Variable Configuration:**

The shell integration supports the `_PM_BINARY` environment variable for specifying a custom PM binary path. This is particularly useful for:
- **Development/Testing**: Using locally built binaries instead of system-installed versions
- **Custom installations**: Pointing to PM binaries in non-standard locations
- **Version switching**: Testing different PM versions

```bash
# Use development binary for testing
export _PM_BINARY="/path/to/project-manager/target/debug/pm"
pm sw my-project  # Uses development binary

# Unset to use system binary
unset _PM_BINARY
pm sw my-project  # Uses system binary from PATH
```

Once integrated, `pm sw` will change your shell's current directory and display:
```bash
pm sw my-project
📁 Changed directory to: /path/to/my-project
```

### `pm status`

Shows information about the current project for prompt integration. This command is designed to work with shell prompts like Starship to display project context.

**Usage:**

```bash
pm status                                      # Full project information
pm status --quiet                             # Compact output for prompts
pm status --format json                       # JSON format output
pm status --format json --quiet               # Minimal JSON for parsing
```

**Options:**

* `--format <FORMAT>`: Output format (`text` or `json`, default: `text`)
* `-q, --quiet`: Quiet mode for prompt integration (minimal output)

**Behavior:**

* Detects if current directory is a PM-managed project
* Shows project name, tags, Git status, and metadata
* Supports parent directory detection (works in subdirectories)
* Returns appropriate exit codes for conditional display in prompts

**Output Examples:**

**Text format (default):**
```bash
$ pm status
📋 Project: project-manager
🏷️  Tags: rust, cli, tools
📁 Path: /Users/user/github/project-manager
🌿 Git: feat/enhanced-add-command (with changes)
📊 Access count: 15
🕒 Last accessed: 2025-07-15 10:30:00
```

**Text quiet format:**
```bash
$ pm status --quiet
project-manager (rust, cli, tools) [feat/enhanced-add-command*]
```

**JSON format:**
```json
{
  "project": {
    "name": "project-manager",
    "tags": ["rust", "cli", "tools"],
    "path": "/Users/user/github/project-manager",
    "description": "CLI project manager",
    "language": "Rust"
  },
  "git": {
    "is_repository": true,
    "branch": "feat/enhanced-add-command",
    "has_changes": true,
    "remote_url": "https://github.com/user/project-manager.git"
  },
  "metadata": {
    "access_count": 15,
    "last_accessed": "2025-07-15T10:30:00Z"
  }
}
```

**JSON quiet format:**
```json
{"name":"project-manager","tags":"rust,cli,tools","git_branch":"feat/enhanced-add-command","git_changes":true}
```

**Not in a project:**
```bash
$ pm status
Current directory is not a PM-managed project
💡 Use 'pm add .' to add this directory as a project

$ pm status --quiet
# (exits with code 1 for conditional display)
```

**Starship Integration:**

The `status` command is designed for integration with Starship prompt. Add this to your `~/.config/starship.toml`:

```toml
[custom.pm]
command = "pm status --format json --quiet"
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
```

**Starship Integration:**

The `status` command is designed for integration with Starship prompt. Here are some example configurations to add to your `~/.config/starship.toml`:

**Basic Configuration:**
```toml
[custom.pm]
command = '''pm status --format json --quiet | jq -r "
  if .git_branch != \"\" then
    if .git_changes then .name + \" [\" + .git_branch + \"*]\"
    else .name + \" [\" + .git_branch + \"]\"
    end
  else .name
  end
" 2>/dev/null || echo ""'''
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
description = "Show PM project with git status"
```

**Minimal Configuration:**
```toml
[custom.pm]
command = 'pm status --format json --quiet | jq -r ".name" 2>/dev/null || echo ""'
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
description = "Show PM project name"
```

**Simple Configuration (without jq):**
```toml
[custom.pm]
command = 'pm status --quiet'
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
description = "Show PM project information"
```

For more detailed Starship integration examples, see [STARSHIP_INTEGRATION.md](STARSHIP_INTEGRATION.md).

### `pm remove` (alias: `pm rm`)

Removes projects from PM's management list with interactive confirmation and smart matching.

**Usage:**

```bash
pm rm                                           # Interactive project selection
pm rm my-project                                # Remove project by name
pm rm my-project -y                             # Remove without confirmation
```

**Arguments:**

*   `<PROJECT>`: Project name (optional for interactive mode)

**Options:**

*   `-y, --yes`: Skip confirmation prompt

**Interactive Features:**

*   **Project Selection**: When no project name is provided, shows a filterable list of all projects
*   **Duplicate Resolution**: When multiple projects have the same name, shows detailed selection with paths and access statistics
*   **Smart Suggestions**: Suggests similar project names when exact match is not found
*   **Confirmation Prompt**: Shows comprehensive project details before removal

**Behavior:**

*   Matches projects by exact name only (no path matching)
*   Handles duplicate project names with interactive selection
*   Shows project details including path, tags, description, and access statistics
*   Removes project from configuration and cleans up all machine metadata
*   Provides confirmation prompt unless `-y` flag is used

**Example Interactive Flow:**

```bash
$ pm rm api
🔍 Multiple projects found with the same name:
? Select which project to remove:
  > 1. api - /Users/me/work/api [backend, rust] (accessed 15 times)
    2. api - /Users/me/personal/api [frontend, react] (accessed 3 times)

🗑️ About to remove project:
   Name: api
   Path: /Users/me/work/api
   Tags: backend, rust
   Accessed: 15 times
   Last used: 2 hours ago
   Created: 2024-01-15 14:30

? Are you sure you want to remove this project? Yes
✅ Project 'api' removed successfully
```

### `pm tag` (alias: `pm t`)

Manages tags associated with your projects.

#### `pm tag add <PROJECT_NAME> <TAGS>...`

Adds one or more tags to a specified project.

**Usage:**

```bash
pm tag add my-project frontend react
```

**Behavior:**

*   Adds the specified tags to the project's tag list. Duplicate tags are ignored.
*   Updates the project's `updated_at` timestamp.

#### `pm tag remove <PROJECT_NAME> <TAGS>...` (alias: `pm tag rm`)

Removes one or more tags from a specified project.

**Usage:**

```bash
pm tag remove my-project old-tag
pm tag rm my-project another-old-tag
```

**Behavior:**

*   Removes the specified tags from the project's tag list.
*   Updates the project's `updated_at` timestamp.

#### `pm tag ls`

Lists all unique tags used across all managed projects, along with their usage counts.

**Usage:**

```bash
pm tag ls
```

**Behavior:**

*   Iterates through all projects and collects all unique tags.
*   Displays each tag and the number of projects it's applied to, sorted by usage count (descending).

#### `pm tag show [PROJECT_NAME]`

Shows the tags associated with a specific project.

**Usage:**

```bash
pm tag show my-project
pm tag show # If run inside a project directory
```

**Behavior:**

*   If `PROJECT_NAME` is provided, it shows tags for that project.
*   If `PROJECT_NAME` is omitted, it attempts to find a project associated with the current working directory and displays its tags.

### `pm clone` (alias: `pm cl`)

Clone repositories from GitHub with interactive browse or direct clone functionality.

**Usage:**

```bash
pm clone                                        # Interactive browse your repositories
pm clone microsoft/vscode                      # Clone specific repository  
pm clone owner/repo --directory ~/custom       # Clone to custom directory
```

**Arguments:**

*   `[REPO]`: Repository in `owner/repo` format (optional for interactive browse)

**Options:**

*   `-d, --directory <DIRECTORY>`: Target directory (defaults to `<current_dir>/<owner>/<repo>`)

**Behavior:**

**Interactive Mode (no arguments):**
*   Requires GitHub CLI authentication (`gh auth login`)
*   Displays all your repositories (public and private)
*   Provides multi-select interface with repository details
*   Shows privacy status (🔒 private, 🌐 public) and fork status (🍴)
*   Displays programming language and description
*   Clones selected repositories with progress bars
*   Adds cloned repositories to PM management with 'github' tag

**Direct Clone Mode (with repository argument):**
*   Requires GitHub CLI authentication (`gh auth login`)
*   Clones the specified repository from GitHub
*   Creates parent directories if needed
*   Adds cloned project to PM management
*   Assigns 'github' tag automatically

### `pm scan` (alias: `pm sc`)

Scan directories for existing Git repositories and add them to PM.

**Usage:**

```bash
pm scan                           # Scan current directory
pm scan ~/Development             # Scan specific directory
pm scan --show-all               # Show all found repositories without selection
```

**Options:**

*   `-d, --directory <DIRECTORY>`: Directory to scan (defaults to current directory)
*   `--show-all`: Show all repositories found, don't prompt for selection

**Behavior:**

*   Recursively scans directories (max depth: 3)
*   Identifies Git repositories and project roots
*   Filters out already managed projects
*   Provides multi-select interface for adding new projects
*   Assigns 'scanned' tag to added projects
*   Preserves Git remote URLs as descriptions

### `pm config` (alias: `pm cf`)

Manage PM configuration with comprehensive options for customization.

**Usage:**

```bash
pm config                              # Show current configuration (default)
pm config show                         # Show current configuration
pm config edit                         # Edit in your preferred editor
pm config validate                     # Validate configuration file
pm config get settings.show_git_status # Get specific value
pm config set settings.show_git_status true # Set specific value
```

**Subcommands:**

*   `show`: Display current configuration
*   `edit`: Open configuration file in editor
*   `validate`: Check configuration validity
*   `get <key>`: Get specific configuration value
*   `set <key> <value>`: Set configuration value
*   `list`: List all available configuration keys
*   `backup`: Backup and restore operations
*   `template`: Template operations
*   `export`: Export configuration
*   `import`: Import configuration

### `pm ext` (alias: `pm extension`)

Manage PM extensions for extending functionality.

**Usage:**

```bash
pm ext list                          # List installed extensions
pm ext install <name>                # Install extension from registry
pm ext install . --local             # Install from current directory
pm ext uninstall <name>              # Remove extension
pm ext info <name>                   # Show extension information
pm ext search <query>                # Search for extensions (planned)
pm ext update [name]                 # Update extensions (planned)
```

**Subcommands:**

#### `pm ext list` (alias: `pm ext ls`)

List all installed extensions with their information.

```bash
pm ext list
📦 Installed extensions:

  a                 /Users/user/.config/pm/extension/a
    Version: 0.1.0
    Description: A bash extension for PM
    Commands: example
```

#### `pm ext install <name>`

Install an extension from various sources.

**Options:**
* `--local`: Install from local directory (supports `.` for current directory)
* `--source <url>`: Install from specific URL or path
* `--version <version>`: Install specific version (planned)
* `--registry <registry>`: Install from specific registry (planned)
* `--force`: Force reinstallation if already installed

**Examples:**
```bash
pm ext install hooks                 # Install from registry (planned)
pm ext install . --local            # Install from current directory
pm ext install ../my-ext --local    # Install from relative path
pm ext install /path/to/ext --local # Install from absolute path
```

#### `pm ext uninstall <name>` (aliases: `remove`, `rm`)

Remove an installed extension with confirmation.

```bash
pm ext uninstall hooks
🗑️  Removing extension: hooks v1.0.0
📁 Directory: /Users/user/.config/pm/extension/hooks
⚠️  This will permanently remove extension 'hooks' and all its files
Continue? [y/N]: y
✅ Extension 'hooks' has been removed successfully
```

#### `pm ext info <name>`

Show detailed information about an installed extension.

```bash
pm ext info a
📦 Extension: a
Version: 0.1.0
Description: A bash extension for PM
Author: zdpk
Location: /Users/user/.config/pm/extension/a

Commands:
  example    Example command - replace with your functionality
```

### `pm ext registry` (Planned)

Manage extension registries for remote installation.

**Usage:**

```bash
pm ext registry list                 # List configured registries
pm ext registry add <name> <url>     # Add new registry
pm ext registry remove <name>        # Remove registry
pm ext registry default <name>       # Set default registry
pm ext registry ping [name]          # Test registry connectivity
```

The remote extension system is fully implemented and ready for use once the registry server infrastructure is available.

### `pm run` (alias: `pm r`)

Execute installed extensions with built-in discovery and help features.

**Usage:**

```bash
pm run <extension> [args...]         # Run extension with arguments
pm r <extension> [args...]           # Using alias 'r'
pmr <extension> [args...]            # Using shell alias (fastest)

# Discovery and help
pm run ls                            # List all installed extensions
pm run help                          # Show extension usage help
pm run <extension> help              # Show help for specific extension
```

**Arguments:**

* `<EXTENSION>`: Extension name to execute
* `[ARGS]...`: Arguments to pass to the extension

**Special Commands:**

* `ls`, `list`: Display all installed extensions with their commands
* `help`, `-h`, `--help`: Show extension usage and available extensions

**Behavior:**

* Resolves extension aliases to actual names automatically
* Provides environment variables to extensions (PM_CONFIG_PATH, PM_CURRENT_PROJECT, etc.)
* Offers multiple execution methods for different use cases:
  - `pm run`: Explicit, verbose form
  - `pm r`: Quick alias
  - `pmr`: Shell alias for fastest execution

**Shell Alias Integration:**

The `pmr` shell alias is automatically set up during `pm init`:
- **Development**: `_pmr` (uses `_pm` binary)
- **Production**: `pmr` (uses `pm` binary)

**Extension Discovery Output:**

```bash
$ pm run ls
📦 Installed Extensions:
  a (v0.1.0)    A bash extension for PM
    Commands:
      example    Example command - replace with your functionality

Use 'pm run <extension> <command>' to execute extension commands
```

**Extension Help Output:**

```bash
$ pm run help
PM Run - Execute installed extensions

Usage: pm run <EXTENSION> [ARGS...]
       pm run ls           # List available extensions
       pm run help         # Show this help

Available Extensions:
  a (v0.1.0)    A bash extension for PM
    └─ example    Example command - replace with your functionality

Examples:
  pm run a example      # Run 'example' command from extension 'a'
  _pmr a example        # Same as above (shell alias)

Options:
  -h, --help, help      Print this help
  ls, list              List available extensions
```

## Command Aliases Quick Reference

| Command | Alias | Description |
|---------|-------|-------------|
| `pm add` | `pm a` | Add projects with interactive tag selection |
| `pm list` | `pm ls` | List managed projects |
| `pm switch` | `pm sw` | Switch to project directory |
| `pm status` | - | Show current project status (for prompt integration) |
| `pm remove` | `pm rm` | Remove projects from PM |
| `pm clone` | `pm cl` | Clone GitHub repositories |
| `pm scan` | `pm sc` | Scan for existing repositories |
| `pm tag` | `pm t` | Manage project tags |
| `pm config` | `pm cf` | Configuration management |
| `pm run` | `pm r` | Execute extensions with discovery |
| `pm ext` | `pm extension` | Manage extensions (install, remove, list) |

## Interactive Features

### Enhanced Tag Selection

PM's tag selection system provides:
- **Real-time search and filtering**
- **Smart suggestions based on usage patterns**
- **Seamless new tag creation**
- **Multi-select capabilities**
- **Fuzzy matching for flexible input**

### Batch Operations

When using pattern matching (`pm add *`), PM optimizes the experience:
- **Streamlined processing** for multiple directories
- **Progress indicators** for long operations
- **Comprehensive summaries** showing added vs skipped
- **Intelligent error handling**

### Git Integration

PM automatically:
- **Detects Git repositories** and tracks commit times
- **Displays Git repository status** with visual indicators
- **Preserves remote URLs** as project descriptions
- **Updates activity tracking** based on Git history

## Best Practices

### Tag Management
- Use lowercase, descriptive tags: `rust`, `frontend`, `microservice`
- Maintain consistency across similar projects
- Leverage usage counts to identify popular patterns
- Mix project-type tags (`library`, `cli`) with domain tags (`work`, `personal`)

### Project Organization
- Use `pm add .` for interactive single project setup
- Use `pm add *` for quick batch imports
- Regularly scan development directories with `pm scan`
- Utilize filtering options in `pm list` for project discovery

### Workflow Integration
- Use aliases (`pm ls`, `pm sw`) for faster command execution
- Set up meaningful project descriptions for better organization
- Regularly backup configuration with `pm config backup`
