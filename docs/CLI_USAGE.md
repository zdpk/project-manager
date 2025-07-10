# Project Manager CLI - Usage Guide

## Installation

```bash
# Using uv (recommended)
uv sync
uv run pm --help

# For development
uv sync --dev
```

## Basic Commands

### Project Management

```bash
# Create a new project
pm project create "My New Project" --description "Project description"

# List all projects
pm project list
pm project list --status active

# Show project details
pm project show 1
pm project show --name "My New Project"

# Update project
pm project update 1 --status completed
pm project update 1 --name "Updated Name" --description "New description"

# Delete project
pm project delete 1
```

### Task Management

```bash
# Create tasks
pm task create "Implement user authentication" --project 1
pm task create "Write tests" --project 1 --priority high --due "2024-01-15"

# List tasks
pm task list                    # All tasks
pm task list --project 1       # Tasks in specific project
pm task list --status todo     # Tasks by status
pm task list --assigned-to me  # My tasks

# Update tasks
pm task update 123 --status in_progress
pm task update 123 --assign @username
pm task update 123 --priority critical --due "2024-01-10"

# Complete tasks
pm task complete 123
pm task block 123 --reason "Waiting for API"

# Task dependencies
pm task depend 123 --on 124    # Task 123 depends on 124
pm task unblock 123
```

### Team Management

```bash
# Create team
pm team create "Development Team" --description "Core dev team"

# Add/remove members
pm team add-member 1 @username --role member
pm team remove-member 1 @username

# List teams and members
pm team list
pm team show 1
```

### Time Tracking

```bash
# Log time
pm time log 123 --hours 2.5 --description "Implemented feature X"
pm time log 123 --hours 1 --date "2024-01-10"

# Start/stop time tracking
pm time start 123
pm time stop

# View time reports
pm time report --week
pm time report --project 1 --month
pm time report --user @username
```

### Reporting

```bash
# Project progress
pm report progress --project 1
pm report progress --all

# Time reports
pm report time --week
pm report time --project 1 --month "2024-01"

# Team performance
pm report team --team 1
pm report user @username

# Export data
pm report export --format json --output report.json
pm report export --format csv --project 1
```

### Configuration

```bash
# View configuration
pm config list
pm config get user.name

# Set configuration
pm config set user.name "John Doe"
pm config set user.email "john@example.com"
pm config set default.project 1

# Initialize project
pm init                         # Initialize in current directory
pm init --template python      # Use project template
```

## Command Patterns

### Global Options
```bash
pm --verbose task list          # Verbose output
pm --quiet project create       # Minimal output
pm --config custom.yaml task list  # Custom config file
pm --no-color report progress   # Disable colored output
```

### Filtering and Sorting
```bash
# Filter by status
pm task list --status todo,in_progress

# Filter by priority
pm task list --priority high,critical

# Filter by date range
pm task list --due-after "2024-01-01" --due-before "2024-01-31"

# Sort results
pm task list --sort priority    # Sort by priority
pm task list --sort due_date    # Sort by due date
pm project list --sort name     # Sort by name
```

### Output Formats
```bash
# Table format (default)
pm task list

# JSON output
pm task list --format json

# CSV output  
pm task list --format csv

# Minimal output
pm task list --format minimal
```

## Interactive Mode

```bash
# Enter interactive shell
pm shell

# In shell mode:
> project list
> task create "New task" --project 1
> exit
```

## Configuration Files

### Global Config: `~/.project-manager/config.yaml`
```yaml
user:
  name: "John Doe"
  email: "john@example.com"
  timezone: "UTC"

defaults:
  project_type: "team"
  task_priority: "medium"
  time_format: "hours"

display:
  theme: "dark"
  show_colors: true
  table_style: "grid"

integrations:
  git:
    auto_detect: true
    commit_message_format: "[{task_id}] {title}"
```

### Project Config: `./.pm/config.yaml`
```yaml
project:
  id: 1
  name: "My Project"
  
team_members:
  - username: "alice"
    role: "manager"
  - username: "bob"
    role: "member"

workflows:
  task_statuses:
    - "todo"
    - "in_progress" 
    - "review"
    - "done"
```

## Environment Variables

```bash
export PM_CONFIG_FILE="/path/to/config.yaml"
export PM_DATABASE_URL="sqlite:///custom/path/pm.db"
export PM_LOG_LEVEL="DEBUG"
export PM_THEME="dark"
export PM_NO_COLOR="1"          # Disable colors
```

## Examples

### Daily Workflow
```bash
# Morning standup
pm task list --assigned-to me --status in_progress
pm report progress --project current

# Start working
pm time start 123
pm task update 123 --status in_progress

# End of day
pm time stop
pm task list --assigned-to me --completed-today
```

### Project Setup
```bash
# Initialize new project
pm project create "Website Redesign" --type team
pm team create "Design Team"
pm team add-member 1 @designer --role member
pm team add-member 1 @developer --role member

# Create initial tasks
pm task create "Design wireframes" --project 1 --assign @designer --priority high
pm task create "Setup development environment" --project 1 --assign @developer
pm task depend 2 --on 1  # Dev environment depends on wireframes
```

### Reporting and Analytics
```bash
# Weekly team report
pm report team --week --format json > weekly_report.json

# Project burn-down
pm report progress --project 1 --chart

# Individual productivity
pm report user @username --month --include-time
```