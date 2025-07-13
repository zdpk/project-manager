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

1. **GitHub username**: Automatically detected from GitHub CLI, with manual fallback
2. **Configuration directory**: Where PM stores its configuration files (default: `~/.config/pm`)
3. **Editor preference**: Your preferred code editor (VS Code, Helix, Vim, etc.)
4. **Auto-open editor**: Whether to automatically open editor when switching projects
5. **Git status display**: Whether to show git status information in project listings

**GitHub Username Detection:**
- PM automatically detects your GitHub username using `gh api user`
- Requires GitHub CLI (`gh`) to be installed and authenticated
- Falls back to manual input if detection fails
- Provides confirmation prompt for detected username

**Example Output:**

```bash
$ pm init
🚀 Initializing PM...

🔍 Detecting GitHub username from GitHub CLI...
✅ Detected GitHub username: your-username
> Use detected GitHub username 'your-username'? Yes
> Configuration directory: ~/.config/pm
  Where PM configuration files will be stored (press Enter for default)
> Choose your preferred editor: hx (Helix)
> Automatically open editor when switching to projects? Yes
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

For single operations, PM provides an intelligent tag selection interface:

```
🏷️  Select tags for project: my-awesome-project

> _____ (Type to search or create new tag)
[ ] rust (15 projects)
[ ] frontend (12 projects) 
[ ] backend (18 projects)
[ ] api (8 projects)
[ ] microservice (5 projects)

✨ Navigation: ↑↓ move • Space select • Enter confirm • Type search
```

**Tag Selection Features:**
- **Real-time Filtering**: Type to filter existing tags instantly
- **Smart Ranking**: Tags sorted by relevance and usage frequency
- **New Tag Creation**: Type new names to create custom tags
- **Usage Statistics**: See how many projects use each tag
- **Fuzzy Matching**: Find tags with partial or misspelled input
- **Multi-select**: Choose multiple tags with space, confirm with enter

**Example Tag Selection Workflow:**

```bash
$ pm add ./my-rust-api

🏷️  Select tags for project: my-rust-api

> backend_____ (User types "backend")
[x] backend (18 projects)
[ ] ✨ Create new tag: "backend" 

User presses space to select backend, then types "api":
> api_____
[x] backend (18 projects)  
[x] api (8 projects)

User presses enter to confirm:
✅ Added project 'my-rust-api' with tags: backend, api
   Path: /Users/you/projects/my-rust-api
```

**Directory Creation:**
For non-existent paths, PM will:
1. **Confirm creation**: Ask permission to create missing directories
2. **Git initialization**: Offer to initialize as Git repository
3. **Proceed with tagging**: Continue with interactive tag selection

**Batch Operations:**
For `pm add *`, the process is streamlined:
- Creates/validates all subdirectories
- Skips interactive tagging for efficiency
- Provides comprehensive summary of results

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
*   Displays project name, path, tags, and last update time
*   Shows both name and path to distinguish projects with same names

### `pm switch` (alias: `pm sw`)

Switches to a specified project's directory and optionally opens an editor.

**Usage:**

```bash
pm switch my-project                            # Switch and open editor
pm sw my-project --no-editor                   # Switch without opening editor
```

**Arguments:**

*   `<NAME>`: Project name to switch to

**Options:**

*   `--no-editor`: Prevents PM from opening the configured editor

**Behavior:**

*   Changes the current working directory to the project's path
*   Records project access for usage tracking
*   Opens configured editor (respects `config.editor`, `EDITOR` env var, or defaults to `hx`)
*   Provides suggestions for similar project names if not found

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
pm config show                         # Show current configuration
pm config edit                         # Edit in your preferred editor
pm config validate                     # Validate configuration file
pm config get editor                   # Get specific value
pm config set editor hx                # Set specific value
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

## Command Aliases Quick Reference

| Command | Alias | Description |
|---------|-------|-------------|
| `pm add` | `pm a` | Add projects with interactive tag selection |
| `pm list` | `pm ls` | List managed projects |
| `pm switch` | `pm sw` | Switch to project directory |
| `pm clone` | `pm cl` | Clone GitHub repositories |
| `pm scan` | `pm sc` | Scan for existing repositories |
| `pm tag` | `pm t` | Manage project tags |
| `pm config` | `pm cf` | Configuration management |

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
- **Offers Git initialization** for new directories
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
- Configure auto-open editor for seamless project switching
- Use aliases (`pm ls`, `pm sw`) for faster command execution
- Set up meaningful project descriptions for better organization
- Regularly backup configuration with `pm config backup`