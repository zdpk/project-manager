# PM Direnv and Git Hooks Support

This document describes the direnv and Git hooks support features in PM (Project Manager).

## Features Overview

PM now supports:
- **Direnv integration**: Detection and status reporting of direnv configuration
- **Git hooks management**: Installation, management, and status reporting of Git hooks
- **Project status enhancement**: Extended status information including direnv and hooks

## Direnv Support

### Detection
PM automatically detects `.envrc` files in your projects and reports their status:

```bash
pm status
```

Output includes:
- **Direnv**: `active` or `inactive` (shows only if `.envrc` exists)

### JSON Output
```json
{
  "direnv": {
    "has_config": true,
    "is_active": false
  }
}
```

### Best Practices
- `.envrc` files are automatically ignored by Git (added to `.gitignore`)
- Use direnv for project-specific environment variables
- PM detects but doesn't manage direnv configuration

## Git Hooks Management

### Commands

#### Install Hooks
```bash
pm hooks install [--path <project_path>]
```
- Installs PM hooks templates as symlinks in `.git/hooks/`
- Creates symlinks to `hooks/` directory in project root

#### Uninstall Hooks
```bash
pm hooks uninstall [--path <project_path>]
```
- Removes PM-installed hooks
- Only removes hooks that are symlinks to PM templates

#### List Hooks Status
```bash
pm hooks list [--path <project_path>]
```
- Shows PM hooks status (installed/not installed)
- Lists active Git hooks
- Shows availability of PM hooks template

#### Initialize Hooks Template
```bash
pm hooks init [--path <project_path>]
```
- Creates `hooks/` directory with template hooks
- Includes pre-commit, commit-msg, and pre-push hooks
- Sets executable permissions

### Available Hooks

#### pre-commit
- Runs `cargo fmt --all --check`
- Runs `cargo clippy --all-targets --all-features -- -D warnings`
- Runs `cargo test --all`

#### commit-msg
- Validates commit message length (10-72 characters)
- Suggests conventional commit format
- Ensures non-empty messages

#### pre-push
- Checks for uncommitted changes
- Runs release build verification
- Checks documentation build

### Hook Templates

PM provides Rust-specific hook templates that can be customized:

```
hooks/
├── pre-commit       # Pre-commit quality checks
├── commit-msg       # Commit message validation
├── pre-push         # Pre-push verification
└── README.md        # Documentation
```

### Status Integration

The `pm status` command now includes hooks information:

```bash
pm status
```

Text output:
```
📋 Project: project-manager
🔧 PM Hooks: Fully installed
🪝 Active Hooks: pre-commit, commit-msg, pre-push
```

JSON output:
```json
{
  "hooks": {
    "active_hooks": ["pre-commit", "commit-msg", "pre-push"],
    "pm_hooks_status": "Fully installed"
  }
}
```

## Integration with PM Workflow

### Project Addition
When adding projects, PM will detect:
- Existing `.envrc` files
- Active Git hooks
- PM hooks template availability

### Project Status
Enhanced status reporting includes:
- Direnv configuration and activity
- Git hooks installation status
- Active hooks list

### Project Scanning
The scan command will identify:
- Projects with direnv configuration
- Projects with existing Git hooks
- Projects suitable for PM hooks installation

## Version Control

### What's Tracked
- `hooks/` directory and templates (✅ tracked)
- Hook installation scripts (✅ tracked)
- Documentation (✅ tracked)

### What's Ignored
- `.envrc` files (❌ not tracked - added to `.gitignore`)
- `.git/hooks/` actual hooks (❌ not tracked - Git design)

## Best Practices

### Direnv
1. Use `.envrc` for project-specific environment variables
2. Don't commit `.envrc` files (automatically ignored)
3. Document required environment variables in README

### Git Hooks
1. Keep hooks in version control (`hooks/` directory)
2. Install hooks after cloning (`pm hooks install`)
3. Customize hooks for project-specific needs
4. Use `git commit --no-verify` to bypass hooks when needed

### Team Workflow
1. Create hooks template: `pm hooks init`
2. Customize hooks in `hooks/` directory
3. Team members install: `pm hooks install`
4. Updates to hooks are version controlled

## Troubleshooting

### Direnv Not Active
- Install direnv: `brew install direnv` (macOS)
- Add to shell: `eval "$(direnv hook zsh)"` (or bash)
- Allow directory: `direnv allow`

### Hooks Not Working
- Check installation: `pm hooks list`
- Verify symlinks: `ls -la .git/hooks/`
- Reinstall: `pm hooks uninstall && pm hooks install`

### Hook Failures
- Check hook permissions: `chmod +x .git/hooks/*`
- Test hooks manually: `.git/hooks/pre-commit`
- Bypass temporarily: `git commit --no-verify`

## Examples

### Complete Setup
```bash
# Navigate to project
cd my-project

# Add to PM
pm add .

# Initialize hooks template
pm hooks init

# Install hooks
pm hooks install

# Check status
pm status
```

### Status Output Example
```
📋 Project: my-rust-project
🏷️  Tags: rust, cli
📁 Path: /Users/dev/projects/my-rust-project
🌿 Git: main (with changes)
🌍 Direnv: active
🔧 PM Hooks: Fully installed
🪝 Active Hooks: pre-commit, commit-msg, pre-push
```

This enhanced functionality makes PM a more comprehensive project management tool that integrates well with modern development workflows.