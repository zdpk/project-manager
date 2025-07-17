# PM Project Git Hooks

This directory contains Git hooks templates for PM projects to ensure code quality and consistency.

## Available Hooks

### pre-commit
- Runs before each commit
- Checks code formatting with `cargo fmt`
- Runs linting with `cargo clippy`
- Runs tests with `cargo test`

### commit-msg
- Validates commit message format
- Ensures minimum/maximum length requirements
- Suggests conventional commit format

### pre-push
- Runs before pushing to remote
- Checks for uncommitted changes
- Runs release build verification
- Checks documentation build

## Installation

To install these hooks in your project:

```bash
# Make hooks executable
chmod +x hooks/*

# Install hooks (creates symlinks)
ln -sf ../../hooks/pre-commit .git/hooks/pre-commit
ln -sf ../../hooks/commit-msg .git/hooks/commit-msg
ln -sf ../../hooks/pre-push .git/hooks/pre-push
```

Or use the PM command (when implemented):
```bash
pm hooks install
```

## Customization

You can customize these hooks by:
1. Modifying the templates in the `hooks/` directory
2. Creating project-specific hooks
3. Disabling specific checks by commenting out sections

## Best Practices

1. **Keep hooks fast**: Avoid long-running operations
2. **Make them fail-safe**: Exit on first error
3. **Provide clear feedback**: Show what's being checked
4. **Allow bypassing**: Use `git commit --no-verify` if needed
5. **Version control**: Keep hooks in the repository for team sharing