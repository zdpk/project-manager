# PM (Project Manager) - Command Reference

This document provides a detailed reference for all commands available in the `pm` CLI tool.

## Global Options

*   `--help`: Print help information
*   `--version`: Print version information

## Commands

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
3. **Projects root directory**: Where your projects will be stored (default: `~/workspace`)
4. **Editor preference**: Your preferred code editor (VS Code, Helix, Vim, etc.)
5. **Auto-open editor**: Whether to automatically open editor when switching projects
6. **Git status display**: Whether to show git status information in project listings

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
> Projects root directory: ~/workspace  
  Where your projects will be stored (press Enter for default)
> Choose your preferred editor: hx (Helix)
> Automatically open editor when switching to projects? Yes
> Show git status in project listings? Yes

📂 Creating configuration directory: /Users/you/.config/pm
📁 Creating projects root directory: /Users/you/workspace

✅ PM initialized successfully
👤 GitHub username: your-username
📂 Config directory: /Users/you/.config/pm
📁 Projects root: /Users/you/workspace
⚙️ Config file: /Users/you/.config/pm/config.yml

🎯 Next steps:
  pm add <path>     # Add your first project
  pm scan           # Scan for existing repositories
  pm load <owner>/<repo> # Clone from GitHub
  pm browse         # Browse and select GitHub repositories
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
*   Creates configuration and projects directories if they don't exist
*   Generates a YAML configuration file with user preferences
*   Provides clear next steps for getting started

### `pm project` (alias: `pm p`)

Manages your projects (add, list, switch).

#### `pm project add <PATH>`

Adds a new project to `pm`'s management list.

**Usage:**

```bash
pm project add ~/path/to/my-project
pm p add my-relative-project # Resolves relative to projects_root_dir from pm init
```

**Options:**

*   `-n, --name <NAME>`: Specify a custom name for the project. If omitted, the directory name will be used.
*   `-t, --tags <TAGS>...`: Comma-separated list of tags to associate with the project (e.g., `frontend,react,work`).
*   `-d, --description <DESCRIPTION>`: A brief description of the project.

**Behavior:**

*   Resolves the provided `<PATH>` to an absolute path. If `<PATH>` is relative, it's resolved against the `projects_root_dir` configured during `pm init`.
*   Automatically detects if the path is a Git repository and stores the last commit time (`git_updated_at`).
*   Adds the project metadata to `config.json`.

#### `pm project ls`

Lists all projects currently managed by `pm`.

**Usage:**

```bash
pm project ls
pm p ls
```

**Behavior:**

*   Lists projects sorted by `git_updated_at` (if available), then `updated_at`, then `created_at`.
*   Asynchronously updates `git_updated_at` for projects if it's missing or older than 1 hour.
*   Displays project name, tags, and last update time (either Git commit time or PM update time).

#### `pm project s <NAME>`

Switches to a specified project's directory and optionally opens an editor.

**Usage:**

```bash
pm project s my-project
pm p s another-project
```

**Options:**

*   `--no-editor`: Prevents `pm` from opening the default editor (Helix).

**Behavior:**

*   Changes the current working directory to the project's path.
*   If `--no-editor` is not specified, it attempts to open the `hx` (Helix) editor in the project directory.

### `pm tag`

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

## GitHub Integration

### `pm browse`

Browse and select repositories from GitHub with interactive multi-select interface.

**Usage:**

```bash
pm browse                        # Browse your repositories  
pm browse --username other-user  # Browse another user's repositories
```

**Options:**

*   `-u, --username <USERNAME>`: GitHub username to browse (defaults to configured username)

**Behavior:**

*   Connects to GitHub API (uses GitHub CLI authentication if available)
*   Displays all repositories (public and private if authenticated)
*   Provides multi-select interface with repository details
*   Shows privacy status (🔒 private, 🌐 public) and fork status (🍴)
*   Displays programming language and description
*   Clones selected repositories with progress bars
*   Adds cloned repositories to PM management

### `pm load <REPO>`

Clone a specific repository from GitHub.

**Usage:**

```bash
pm load microsoft/vscode                # Clone to default location
pm load owner/repo --directory ~/custom # Clone to custom directory
```

**Arguments:**

*   `<REPO>`: Repository in `owner/repo` format

**Options:**

*   `-d, --directory <DIRECTORY>`: Target directory (defaults to `<projects_root>/<owner>/<repo>`)

**Behavior:**

*   Clones repository from GitHub
*   Creates parent directories if needed
*   Adds cloned project to PM management
*   Assigns 'github' tag automatically

### `pm scan`

Scan directories for existing Git repositories and add them to PM.

**Usage:**

```bash
pm scan                    # Scan default workspace (~/workspace)
pm scan ~/Development      # Scan specific directory
pm scan --show-all         # Show all found repositories without selection
```

**Options:**

*   `-d, --directory <DIRECTORY>`: Directory to scan (defaults to ~/workspace)
*   `--show-all`: Show all repositories found, don't prompt for selection

**Behavior:**

*   Recursively scans directories (max depth: 3)
*   Identifies Git repositories and project roots
*   Filters out already managed projects
*   Provides multi-select interface for adding new projects
*   Assigns 'scanned' tag to added projects
*   Preserves Git remote URLs as descriptions

