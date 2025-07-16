# Configuration Path Structure

This document clarifies the correct configuration path structure for PM (Project Manager) in both production and development modes.

## Unified Directory Approach

PM uses a **unified directory approach** where both production and development configurations are stored in the same directory but with different filenames.

### Configuration File Locations

```
~/.config/pm/
├── config.yml          # Production configuration
├── config-dev.yml       # Development configuration  
├── config.schema.json   # JSON schema for validation
├── pm.zsh              # Zsh shell integration
├── pm.bash             # Bash shell integration
└── pm.fish             # Fish shell integration (symlinked to ~/.config/fish/functions/)
```

### Environment-Based Selection

The configuration file selection is controlled by the `PM_DEV_MODE` environment variable:

- **Production mode**: Uses `~/.config/pm/config.yml`
  - Default behavior when `PM_DEV_MODE` is not set
  - Used by the `pm` binary

- **Development mode**: Uses `~/.config/pm/config-dev.yml`  
  - Activated when `PM_DEV_MODE` environment variable is set
  - Used by the `_pm` development binary

## Binary-Specific Behavior

### Production Binary (`pm`)

```bash
# Uses production config automatically
pm init                    # Creates ~/.config/pm/config.yml
pm add ~/my-project        # Stores in production config
pm list                    # Reads from production config
```

### Development Binary (`_pm`)

```bash
# Uses development config automatically
_pm init                   # Creates ~/.config/pm/config-dev.yml
_pm add ~/test-project     # Stores in development config  
_pm list                   # Reads from development config
```

## Implementation Details

The configuration path logic is implemented in `src/config.rs`:

```rust
pub fn get_config_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Failed to find home directory")?;
    let config_dir = home_dir.join(CONFIG_DIR_NAME);
    
    // Use same directory but different filename for development mode
    let pm_dir = config_dir.join(CONFIG_SUBDIR_NAME);
    
    let config_filename = if std::env::var("PM_DEV_MODE").is_ok() {
        "config-dev.yml"
    } else {
        CONFIG_FILENAME  // "config.yml"
    };
    
    Ok(pm_dir.join(config_filename))
}
```

Where:
- `CONFIG_DIR_NAME = ".config"`
- `CONFIG_SUBDIR_NAME = "pm"`
- `CONFIG_FILENAME = "config.yml"`

## Benefits of Unified Directory

1. **Simplified Management**: All PM files in one location
2. **Easy Backup**: Single directory to backup/restore
3. **Clear Separation**: Different files prevent config conflicts
4. **Consistent Organization**: Same structure regardless of mode
5. **Shell Integration**: All shell files in same directory

## Migration from Previous Versions

If you have configurations in separate directories from earlier versions:

```bash
# Old structure (incorrect)
~/.config/pm/config.yml      # Production
~/.config/pm-dev/config.yml  # Development (wrong location)

# New structure (correct)  
~/.config/pm/config.yml      # Production
~/.config/pm/config-dev.yml  # Development (same directory)
```

To migrate:
```bash
# Move development config to correct location
mv ~/.config/pm-dev/config.yml ~/.config/pm/config-dev.yml

# Remove old directory
rmdir ~/.config/pm-dev
```

## Verification

You can verify the correct configuration paths:

```bash
# Check production config path
pm config show | grep config_path

# Check development config path (if using dev binary)
_pm config show | grep config_path

# List all PM configuration files
ls -la ~/.config/pm/
```

## Common Mistakes to Avoid

❌ **Don't manually create separate directories**:
```bash
mkdir ~/.config/pm-dev  # Wrong approach
```

✅ **Let PM handle config file creation**:
```bash
pm init     # Creates production config
_pm init    # Creates development config (same directory)
```

❌ **Don't mix config files between modes**:
```bash
PM_DEV_MODE=1 pm init   # Wrong: forces production binary to use dev config
```

✅ **Use correct binary for each mode**:
```bash
pm init      # Production mode
_pm init     # Development mode
```

## Build System Integration

The build system enforces this separation through:

1. **Cargo.toml configuration**:
   ```toml
   [[bin]]
   name = "_pm"
   required-features = ["dev"]
   ```

2. **Makefile commands**:
   ```bash
   make run-prod    # Uses pm binary → config.yml
   make run-dev     # Uses _pm binary → config-dev.yml
   ```

3. **Runtime safety checks**: Both binaries validate their feature configuration at startup

This ensures the correct configuration file is always used with the appropriate binary.