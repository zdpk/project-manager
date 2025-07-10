# PM (Project Manager) - Command Reference

This document provides a detailed reference for all commands available in the `pm` CLI tool.

## Global Options

*   `--help`: Print help information
*   `--version`: Print version information

## Commands

### `pm init`

Initializes the `pm` tool by setting up the configuration file (`~/.config/pm/config.json`).

**Usage:**

```bash
pm init
```

**Behavior:**

*   If `config.json` already exists, it will warn the user and exit.
*   If `config.json` does not exist, it will prompt the user for:
    *   Your GitHub username.
    *   The absolute path to your projects root directory (e.g., `~/workspace`). This path will be used to resolve relative paths when adding projects.
*   Creates the `config.json` file with the provided information.

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

