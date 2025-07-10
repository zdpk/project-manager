# Project Manager CLI - System Architecture

## Overview

A Python-based CLI tool for managing projects, tasks, and team workflows locally with optional sync capabilities.

## Architecture Principles

### Domain-Driven Design (DDD)
- **Project Domain**: Core business logic for project management
- **Task Domain**: Task lifecycle, dependencies, and tracking
- **User Domain**: User management and preferences
- **Reporting Domain**: Analytics and progress tracking

### CLI Design Patterns
- **Command Pattern**: Each CLI command is a separate handler
- **Repository Pattern**: Data access abstraction
- **Service Layer**: Business logic separation
- **Configuration Management**: Environment-specific settings

## System Components

```
project-manager/
├── src/
│   ├── project_manager/
│   │   ├── __init__.py
│   │   ├── cli/                 # CLI interface layer
│   │   │   ├── __init__.py
│   │   │   ├── main.py         # Entry point and CLI app
│   │   │   ├── commands/       # Command implementations
│   │   │   │   ├── project.py  # Project management commands
│   │   │   │   ├── task.py     # Task management commands
│   │   │   │   ├── team.py     # Team management commands
│   │   │   │   └── report.py   # Reporting commands
│   │   │   └── utils/          # CLI utilities
│   │   ├── domain/             # Business logic and models
│   │   │   ├── models/         # Data models
│   │   │   ├── services/       # Business services
│   │   │   └── repositories/   # Data access layer
│   │   ├── infrastructure/     # External dependencies
│   │   │   ├── database/       # Database setup and migrations
│   │   │   ├── config/         # Configuration management
│   │   │   └── storage/        # File and data storage
│   │   └── shared/             # Shared utilities
│   │       ├── exceptions.py
│   │       ├── constants.py
│   │       └── validators.py
├── tests/                      # Test suite
├── migrations/                 # Database migrations
└── config/                     # Configuration files
```

## Core Domains

### 1. Project Domain
**Entities:**
- Project: Main project entity with metadata
- Milestone: Project milestones and deadlines
- Template: Project templates for reuse

**Value Objects:**
- ProjectStatus (Active, Paused, Completed, Archived)
- Priority (Low, Medium, High, Critical)
- ProjectType (Personal, Team, Client)

### 2. Task Domain
**Entities:**
- Task: Individual work items
- Subtask: Hierarchical task breakdown
- Comment: Task discussions and updates

**Value Objects:**
- TaskStatus (Todo, InProgress, Review, Done)
- TimeEntry: Time tracking records
- Dependency: Task relationships

### 3. User Domain
**Entities:**
- User: System users and team members
- Team: User groups and permissions
- Role: User roles and capabilities

### 4. Reporting Domain
**Services:**
- ProgressReporter: Project and task progress
- TimeReporter: Time tracking analytics
- TeamReporter: Team performance metrics

## Data Layer

### Local Database (SQLite)
```sql
-- Core tables
projects (id, name, description, status, created_at, updated_at)
tasks (id, project_id, title, description, status, priority, due_date)
users (id, username, email, role, preferences)
teams (id, name, description, created_by)
time_entries (id, task_id, user_id, duration, date, description)

-- Relationship tables
project_members (project_id, user_id, role)
task_assignments (task_id, user_id)
task_dependencies (task_id, depends_on_task_id)
```

### Configuration Management
- **Global Config**: ~/.project-manager/config.yaml
- **Project Config**: ./.pm/config.yaml
- **Environment Variables**: PM_* prefixed variables

## CLI Interface Design

### Main Commands
```bash
pm project create "My Project"           # Create new project
pm project list                          # List all projects
pm project status                        # Show current project status

pm task add "Task description"           # Add new task
pm task list --status todo              # List tasks by status
pm task assign 123 @username            # Assign task to user
pm task complete 123                     # Mark task as complete

pm team add @username                    # Add team member
pm team list                             # List team members
pm team remove @username                 # Remove team member

pm report progress                       # Project progress report
pm report time --week                    # Weekly time report
pm report export --format json          # Export data
```

### Command Structure
- **Noun-Verb Pattern**: `pm <entity> <action> [options]`
- **Interactive Mode**: `pm shell` for continuous usage
- **Configuration**: `pm config set/get <key> <value>`
- **Help System**: `pm <command> --help` for detailed usage

## Integration Points

### Git Integration
- Automatic project detection from git repositories
- Commit message parsing for task updates
- Branch-based task workflow

### Export/Import
- JSON/CSV export for external tools
- Template sharing and import
- Backup and restore functionality

### Extensibility
- Plugin system for custom commands
- Custom report generators
- Webhook support for external integrations

## Security & Privacy

### Local-First Approach
- All data stored locally by default
- Optional cloud sync with encryption
- No sensitive data transmission without user consent

### Data Protection
- SQLite database encryption option
- Configuration file protection
- Audit logging for sensitive operations