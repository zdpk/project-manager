# CLI Usage Scenarios

This document provides comprehensive examples of all PM CLI interactions, covering both successful operations and error cases.

## Table of Contents
- [Initial Setup](#initial-setup)
- [Project Management](#project-management)
- [Configuration Management](#configuration-management)
- [Error Scenarios](#error-scenarios)
- [Advanced Workflows](#advanced-workflows)

---

## Initial Setup

### First Time Usage (No Config)

```bash
$ pm list
❌ PM not initialized: Configuration file not found

💡 Please initialize PM first:
   pm init
```

### Interactive Initialization

```bash
$ pm init
🚀 Initializing PM...

? Choose your initialization preference: 
  🔍 Auto-detect existing workspace and repositories
  🌐 Setup GitHub integration for cloning repositories
  🚀 Both auto-detection and GitHub integration
  ⚙️ Manual setup only

? GitHub username: myusername
? Projects root directory: (~/workspace) 
? Choose your preferred editor:
  code (Visual Studio Code)
> hx (Helix)
  nvim (Neovim)
  vim (Vim)
  nano (Nano)
  emacs (Emacs)
  Other (custom command)

? Automatically open editor when switching to projects? (Y/n) 
? Show git status in project listings? (Y/n) 

📁 Creating projects root directory: /Users/myusername/workspace

✅ PM initialized successfully!
👤 GitHub username: myusername
📁 Projects root: /Users/myusername/workspace
⚙️  Config file: /Users/myusername/.config/pm/config.yml

🎯 Next steps:
  pm add <path>     # Add your first project
  pm ls             # List projects
  pm s <name>       # Switch to project
```

### Custom Editor Setup

```bash
$ pm init
🚀 Initializing PM...

? Choose your initialization preference: ⚙️ Manual setup only
? GitHub username: developer
? Projects root directory: (~/workspace) ~/Development
? Choose your preferred editor: Other (custom command)
? Enter custom editor command: subl
? Automatically open editor when switching to projects? (Y/n) n
? Show git status in project listings? (Y/n) y

📁 Creating projects root directory: /Users/developer/Development

✅ PM initialized successfully!
```

---

## Project Management

### Adding Projects

#### Add Current Directory
```bash
$ pm add .
✅ Project 'my-awesome-project' added successfully!
🏷️  Tags: 
```

#### Add with Name and Tags
```bash
$ pm add ~/projects/web-app --name "My Web App" --tags frontend,react
✅ Project 'My Web App' added successfully!
🏷️  Tags: frontend, react
```

#### Add with Description
```bash
$ pm add ./api --description "RESTful API backend service"
✅ Project 'api' added successfully!
```

### Listing Projects

#### Basic List
```bash
$ pm ls
📋 Active Projects (3 found)
my-awesome-project                    [frontend, rust]     Git: 2시간 전         
My Web App                           [frontend, react]    Git: 1일 전           
api                                  []                   PM: 방금 전           
```

#### Detailed List
```bash
$ pm ls --detailed
📋 Active Projects (3 found)

my-awesome-project
  Tags: frontend, rust
  Path: /Users/developer/projects/my-awesome-project
  ID: 550e8400-e29b-41d4-a716-446655440000
  Created: 2024-12-07 10:30:15
  Updated: 2024-12-07 14:22:33
  Git Updated: 2024-12-07 14:22:33 (2시간 전)
  Access Count: 5

My Web App
  Tags: frontend, react
  Path: /Users/developer/projects/web-app
  Description: Modern React application
  ID: 550e8400-e29b-41d4-a716-446655440001
  Created: 2024-12-06 09:15:22
  Updated: 2024-12-07 08:45:11
  Git Updated: 2024-12-07 08:45:11 (1일 전)
  Last Accessed: 2024-12-07 11:30:45 (3시간 전)
  Access Count: 12
```

#### Filtered Lists
```bash
# By tags
$ pm ls --tags rust
📋 Active Projects (1 found)
my-awesome-project                    [frontend, rust]     Git: 2시간 전         

# Recent activity
$ pm ls --recent 1d
📋 Active Projects (2 found)
my-awesome-project                    [frontend, rust]     Git: 2시간 전         
My Web App                           [frontend, react]    Git: 1일 전           

# With limit
$ pm ls --limit 2
📋 Active Projects (2 found)
my-awesome-project                    [frontend, rust]     Git: 2시간 전         
My Web App                           [frontend, react]    Git: 1일 전           
```

### Switching Projects

#### Successful Switch
```bash
$ pm switch my-awesome-project
🔄 Switching to project: my-awesome-project
📊 Access count: 6 times
⏰ Last accessed: 3시간 전
📂 Working directory: /Users/developer/projects/my-awesome-project
🚀 Opening editor...
```

#### Switch without Editor
```bash
$ pm s My\ Web\ App --no-editor
🔄 Switching to project: My Web App
📊 Access count: 13 times
⏰ Last accessed: 1시간 전
📂 Working directory: /Users/developer/projects/web-app
✅ Project switched (editor not opened)
```

#### Project Not Found
```bash
$ pm switch nonexistent
❌ Project not found: nonexistent

💡 Did you mean one of these?
  - my-awesome-project
  - My Web App
  - api
```

### Repository Operations

#### Scanning for Projects
```bash
$ pm scan
🔍 Scanning for repositories in /Users/developer/workspace...
✅ Found 3 repositories
  - /Users/developer/workspace/project-a (added)
  - /Users/developer/workspace/project-b (added)
  - /Users/developer/workspace/old-project (skipped - already exists)
```

#### Loading from GitHub
```bash
$ pm load microsoft/vscode
🌐 Cloning microsoft/vscode...
📂 Cloning to: /Users/developer/workspace/vscode
✅ Repository cloned successfully
✅ Project 'vscode' added successfully!
```

---

## Configuration Management

### Viewing Configuration
```bash
$ pm config show
📋 PM Configuration

🔧 General Settings
  Version: 1.0.0
  GitHub Username: myusername
  Projects Root: /Users/myusername/workspace
  Editor: hx

⚙️  Application Settings
  Auto Open Editor: true
  Show Git Status: true
  Recent Projects Limit: 10

📊 Statistics
  Total Projects: 3
  Total Machines: 1
```

### Editing Configuration
```bash
$ pm config edit
🚀 Opening editor...
# Opens config file in configured editor
```

### Setting Individual Values
```bash
$ pm config set editor code
✅ Configuration updated: editor = code

$ pm config set settings.auto_open_editor false
✅ Configuration updated: settings.auto_open_editor = false
```

### Getting Values
```bash
$ pm config get editor
code

$ pm config get settings.show_git_status
true
```

---

## Tag Management

### Adding Tags
```bash
$ pm tag add my-awesome-project backend api
✅ Added tags to 'my-awesome-project': backend, api
🏷️  Current tags: frontend, rust, backend, api
```

### Removing Tags
```bash
$ pm tag remove my-awesome-project frontend
✅ Removed tags from 'my-awesome-project': frontend
🏷️  Current tags: rust, backend, api
```

### Listing All Tags
```bash
$ pm tag list
🏷️  Available Tags (5 total)
  backend     (2 projects)
  frontend    (1 project)
  api         (2 projects)
  react       (1 project)
  rust        (1 project)
```

### Showing Project Tags
```bash
$ pm tag show my-awesome-project
🏷️  Tags for 'my-awesome-project':
  rust, backend, api
```

---

## Error Scenarios

### PM Not Initialized
```bash
$ pm add .
❌ PM not initialized: Configuration file not found

💡 Please initialize PM first:
   pm init
```

### Already Initialized
```bash
$ pm init
✅ PM is already initialized
📁 Configuration file: /Users/myusername/.config/pm/config.yml

💡 To reinitialize, delete the config file first:
   rm /Users/myusername/.config/pm/config.yml
```

### Invalid Project Path
```bash
$ pm add /nonexistent/path
❌ Failed to add project: Invalid project path
   Path: /nonexistent/path

💡 Please check that the directory exists and is accessible
```

### Project Already Exists
```bash
$ pm add ~/workspace/existing-project
❌ Project already exists: existing-project
   Path: /Users/developer/workspace/existing-project

💡 Use 'pm ls' to see existing projects
```

### No Projects Found
```bash
$ pm ls
📋 No projects found

💡 Add your first project:
  - pm add <path>
  - pm scan
  - pm load owner/repo
```

### No Tag Matches
```bash
$ pm ls --tags nonexistent
📋 No projects match your filters

💡 Try:
  - No filters: pm ls
  - Longer time period: pm ls -r 30d
  - Different tags: pm ls --tags-any frontend,backend
```

---

## Advanced Workflows

### Multi-step Project Setup
```bash
# Initialize PM
$ pm init
🚀 Initializing PM...
# ... initialization flow ...

# Scan existing projects
$ pm scan ~/Development
🔍 Scanning for repositories in /Users/developer/Development...
✅ Found 5 repositories

# Load a specific repo
$ pm load facebook/react
🌐 Cloning facebook/react...
✅ Project 'react' added successfully!

# Add tags to categorize
$ pm tag add react frontend library opensource
✅ Added tags to 'react': frontend, library, opensource

# List organized projects
$ pm ls --tags frontend
📋 Active Projects (3 found)
web-app                              [frontend, react]     Git: 1일 전           
my-component-lib                     [frontend, vue]       Git: 3일 전           
react                               [frontend, library, opensource] Git: 방금 전
```

### Development Session Flow
```bash
# Start development session
$ pm s web-app
🔄 Switching to project: web-app
📊 Access count: 15 times
⏰ Last accessed: 6시간 전
📂 Working directory: /Users/developer/projects/web-app
🚀 Opening editor...

# After development, switch to another project
$ pm s api --no-editor
🔄 Switching to project: api
📊 Access count: 8 times
📂 Working directory: /Users/developer/projects/api
✅ Project switched (editor not opened)

# Check recent activity
$ pm ls --recent 1h
📋 Active Projects (2 found)
api                                  []                   PM: 방금 전    (접근: 방금 전)
web-app                              [frontend, react]    Git: 1일 전    (접근: 5분 전)
```

### Configuration Backup and Restore
```bash
# Create backup
$ pm config backup create daily-backup
✅ Configuration backup created: daily-backup

# Make changes
$ pm config set editor vim
✅ Configuration updated: editor = vim

# Restore if needed
$ pm config backup restore daily-backup
✅ Configuration restored from backup: daily-backup
```

### Project Organization
```bash
# Add descriptive projects
$ pm add ~/work/client-app --name "Client Dashboard" --description "React dashboard for client management" --tags work,react,dashboard

$ pm add ~/personal/blog --name "Personal Blog" --description "Static site generator blog" --tags personal,blog,gatsby

# Organize with comprehensive tagging
$ pm tag add "Client Dashboard" typescript material-ui
$ pm tag add "Personal Blog" markdown netlify

# Find projects by category
$ pm ls --tags work
$ pm ls --tags personal
$ pm ls --tags-any react,gatsby
```

---

## Tips and Best Practices

### Productive Aliases
```bash
# Common shortcuts
alias pml="pm ls"
alias pms="pm switch" 
alias pma="pm add ."
alias pmtl="pm tag list"
```

### Workflow Integration
```bash
# Integration with other tools
$ pm s my-project && npm start
$ pm s api && docker-compose up -d
```

### Regular Maintenance
```bash
# Weekly project cleanup
$ pm ls --detailed | grep "6개월 전"
$ pm config backup create weekly-$(date +%Y%m%d)
$ pm scan --show-all  # Check for new repositories
```

This documentation covers all major PM CLI interactions and can serve as both a user guide and reference for expected behavior.