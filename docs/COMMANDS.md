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
3. **Git status display**: Whether to show git status information in project listings

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

Switches to a specified project's directory.

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

## Command Aliases Quick Reference

| Command | Alias | Description |
|---------|-------|-------------|
| `pm add` | `pm a` | Add projects with interactive tag selection |
| `pm list` | `pm ls` | List managed projects |
| `pm switch` | `pm sw` | Switch to project directory |
| `pm remove` | `pm rm` | Remove projects from PM |
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
