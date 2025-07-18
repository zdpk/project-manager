# PM Extension Migration PRD

## Overview

This document outlines the migration plan from built-in direnv and Git hooks functionality to a pure extension-based architecture. The goal is to keep PM core lightweight while providing powerful extensibility through the extension system.

## Migration Strategy

### Phase 1: Core Cleanup (Remove Built-in Features)

#### 1.1 Remove Built-in Hooks Functionality
- **Files to Remove/Modify:**
  - `src/commands/hooks.rs` - Delete entire file
  - `src/lib.rs` - Remove `Hooks` command and `HookAction` enum
  - `src/commands/mod.rs` - Remove hooks module export
  - `hooks/` directory - Remove from repository

- **Impact:** 
  - `pm hooks` command will no longer exist
  - Users will need to install `pm-ext-hooks` extension

#### 1.2 Remove Direnv Integration
- **Files to Modify:**
  - `src/utils.rs` - Remove `has_direnv_config()` and `is_direnv_active()` functions
  - `src/commands/status.rs` - Remove direnv status display logic

- **Impact:**
  - `pm status` will no longer show direnv information
  - Users will need to install `pm-ext-direnv` extension

#### 1.3 Clean Up Status Command
- **Files to Modify:**
  - `src/commands/status.rs` - Remove hooks and direnv status sections
  - Update both JSON and text output formats

- **Result:** 
  - Simplified status output focusing on core PM functionality
  - Extension-specific status will be handled by individual extensions

### Phase 2: Extension Development

#### 2.1 pm-ext-hooks Extension

**Repository:** `zdpk/pm-ext-hooks`

**Features:**
- **Enhanced Git Hooks Management**
  - `.githook/` directory support for version-controlled hooks
  - Automatic synchronization: `.githook/` → `.git/hooks/`
  - Template management and customization
  - Multi-project hook sharing
  - Hook dependency management

**Commands:**
```yaml
commands:
  - name: "install"
    help: "Install hooks from .githook directory"
    args: ["[hook-type]"]
  - name: "uninstall" 
    help: "Remove installed hooks"
    args: ["[hook-type]"]
  - name: "sync"
    help: "Synchronize .githook to .git/hooks"
  - name: "list"
    help: "List available and installed hooks"
  - name: "init"
    help: "Initialize .githook directory with templates"
  - name: "status"
    help: "Show hooks installation status"
  - name: "template"
    help: "Manage hook templates"
    args: ["<add|remove|list>", "[template-name]"]
```

**Directory Structure:**
```
project-root/
├── .githook/
│   ├── pre-commit           # Version controlled hooks
│   ├── commit-msg
│   ├── pre-push
│   └── templates/           # Reusable templates
│       ├── rust-quality
│       ├── node-lint
│       └── python-test
└── .git/hooks/              # Auto-generated from .githook
    ├── pre-commit -> ../.githook/pre-commit
    └── ...
```

**Advanced Features:**
- **Hook Chains:** Multiple hooks per Git event
- **Conditional Hooks:** Run hooks based on file types/patterns
- **Team Hooks:** Shared hook configurations
- **Hook Marketplace:** Community hook templates

#### 2.2 pm-ext-direnv Extension

**Repository:** `zdpk/pm-ext-direnv`

**Features:**
- **Auto-activation:** Automatically activate direnv when entering projects
- **Status Integration:** Show direnv status in project information
- **Environment Management:** Manage multiple environments per project
- **Template Generation:** Create .envrc templates for common setups

**Commands:**
```yaml
commands:
  - name: "activate"
    help: "Auto-activate direnv for current project"
  - name: "status"
    help: "Show direnv status and environment info"
  - name: "init"
    help: "Initialize .envrc with template"
    args: ["[template-name]"]
  - name: "allow"
    help: "Allow direnv for current directory"
  - name: "deny"
    help: "Deny direnv for current directory"
  - name: "reload"
    help: "Reload direnv environment"
  - name: "templates"
    help: "List available .envrc templates"
  - name: "edit"
    help: "Edit .envrc file safely"
```

**Templates:**
- **rust:** Rust development environment
- **node:** Node.js with version management
- **python:** Python virtual environment
- **go:** Go development setup
- **docker:** Docker-based development
- **custom:** User-defined templates

**Integration Features:**
- **PM Status:** Extends `pm status` with direnv information
- **Auto-detection:** Suggests direnv setup for projects
- **Environment Switching:** Seamless environment transitions
- **Security:** Safe .envrc editing and validation

#### 2.3 pm-ext-1password Extension

**Repository:** `zdpk/pm-ext-1password`

**Features:**
- **Configuration-driven:** `.pw.yml` files define environment mappings
- **Multi-environment:** Support dev, staging, prod environments
- **Secure Retrieval:** Direct integration with 1Password CLI
- **Template Generation:** Generate .env files from vault data

**Commands:**
```yaml
commands:
  - name: "generate"
    help: "Generate .env files from .pw.yml"
    args: ["[environment]"]
  - name: "init"
    help: "Initialize .pw.yml configuration"
  - name: "validate"
    help: "Validate .pw.yml and vault access"
  - name: "sync"
    help: "Sync all environments from 1Password"
  - name: "list"
    help: "List configured environments"
  - name: "status"
    help: "Show 1Password integration status"
  - name: "template"
    help: "Generate .pw.yml template"
    args: ["[template-type]"]
```

**Configuration Format (.pw.yml):**
```yaml
vault: "Development Secrets"
environments:
  dev:
    output: ".env.dev"
    items:
      - name: "Database Password"
        field: "password"
        env_var: "DB_PASSWORD"
      - name: "API Key"
        field: "credential"
        env_var: "API_KEY"
  staging:
    output: ".env.staging"
    vault: "Staging Secrets"  # Override vault
    items:
      - name: "Staging DB"
        field: "connection_string"
        env_var: "DATABASE_URL"
  prod:
    output: ".env.prod"
    vault: "Production Secrets"
    items:
      - name: "Prod API"
        field: "key"
        env_var: "PROD_API_KEY"
```

**Security Features:**
- **Vault Validation:** Verify vault access before operations
- **Secure Generation:** Temporary file handling for .env files
- **Audit Logging:** Track secret retrievals
- **Access Control:** Respect 1Password session management

### Phase 3: Migration Path for Users

#### 3.1 Automatic Migration Detection

**Detection Logic:**
```rust
// Check if user has existing hooks/direnv usage
async fn detect_migration_needed() -> Vec<MigrationSuggestion> {
    let mut suggestions = Vec::new();
    
    // Check for existing .githook directories
    if has_githook_directory() {
        suggestions.push(MigrationSuggestion::GitHooks);
    }
    
    // Check for .envrc files
    if has_envrc_files() {
        suggestions.push(MigrationSuggestion::Direnv);
    }
    
    // Check for .pw.yml files
    if has_pw_config() {
        suggestions.push(MigrationSuggestion::OnePassword);
    }
    
    suggestions
}
```

#### 3.2 Migration Assistance

**Command:** `pm migrate`
```bash
pm migrate                    # Detect and suggest migrations
pm migrate --install hooks   # Install and migrate hooks
pm migrate --install direnv  # Install and migrate direnv
pm migrate --install 1pass   # Install and migrate 1password
pm migrate --all             # Install all detected extensions
```

**Migration Process:**
1. **Detection:** Scan for existing configurations
2. **Suggestion:** Show what extensions are recommended
3. **Installation:** Install required extensions
4. **Migration:** Transfer existing configurations
5. **Validation:** Verify migration success
6. **Cleanup:** Optionally remove old configurations

#### 3.3 Documentation Updates

**Update Docs:**
- **README.md:** Update feature list to reflect extension-based architecture
- **MIGRATION.md:** Create migration guide for existing users
- **EXTENSIONS.md:** Update with new extension specifications
- **DIRENV_AND_HOOKS.md:** Replace with migration notice

### Phase 4: Extension Registry

#### 4.1 Official Extension Registry

**Repository:** `zdpk/pm-extensions`

**Structure:**
```
pm-extensions/
├── registry.yml             # Official extension registry
├── extensions/
│   ├── hooks/
│   │   ├── manifest.yml     # Extension metadata
│   │   ├── README.md        # Documentation
│   │   └── releases/        # Release information
│   ├── direnv/
│   └── 1password/
└── templates/               # Extension development templates
    ├── rust-extension/
    ├── python-extension/
    └── bash-extension/
```

**Registry Format:**
```yaml
extensions:
  hooks:
    name: "Git Hooks Management"
    description: "Enhanced Git hooks with .githook directory support"
    repository: "https://github.com/zdpk/pm-ext-hooks"
    category: "development"
    tags: ["git", "hooks", "quality"]
    maintainer: "zdpk"
    latest_version: "1.0.0"
    pm_compatibility: ">=0.1.0"
  
  direnv:
    name: "Direnv Integration"  
    description: "Auto-activation and management of direnv environments"
    repository: "https://github.com/zdpk/pm-ext-direnv"
    category: "environment"
    tags: ["direnv", "environment", "automation"]
    maintainer: "zdpk"
    latest_version: "1.0.0"
    pm_compatibility: ">=0.1.0"
```

#### 4.2 Installation Enhancement

**Enhanced Install Command:**
```bash
pm ext install hooks                    # Install from registry
pm ext install hooks --version 1.0.0   # Specific version
pm ext install hooks --source github:zdpk/pm-ext-hooks
pm ext install hooks --source https://releases.../hooks.tar.gz
pm ext install --all                    # Install all recommended
```

## Implementation Timeline

### Week 1: Core Cleanup
- [ ] Remove built-in hooks functionality
- [ ] Remove direnv integration  
- [ ] Clean up status command
- [ ] Update documentation

### Week 2: Extension Development
- [ ] Implement pm-ext-hooks
- [ ] Implement pm-ext-direnv
- [ ] Create extension templates

### Week 3: Migration Tools
- [ ] Implement migration detection
- [ ] Create migration assistant
- [ ] Add installation automation

### Week 4: Registry & Documentation
- [ ] Set up extension registry
- [ ] Create comprehensive documentation
- [ ] Implement pm-ext-1password

## Success Metrics

### Performance
- **Core PM Size:** <50% of current binary size
- **Startup Time:** <100ms for core commands
- **Extension Load:** <200ms additional per extension

### User Experience  
- **Migration Success:** >95% successful automated migrations
- **Extension Discovery:** Users find extensions within 2 commands
- **Documentation:** Complete coverage of migration path

### Ecosystem
- **Official Extensions:** 3 core extensions (hooks, direnv, 1password)
- **Community Extensions:** Enable 3rd party development
- **Registry:** Centralized extension discovery

## Risk Mitigation

### Backward Compatibility
- **Migration Period:** 3-month overlap with deprecation warnings
- **Documentation:** Clear migration guides
- **Support:** Migration assistance tooling

### User Adoption
- **Automatic Detection:** Suggest relevant extensions
- **Easy Installation:** One-command extension setup  
- **Progressive Enhancement:** Core functionality unaffected

### Development Overhead
- **Template System:** Standardized extension development
- **CI/CD:** Automated extension testing and releases
- **Documentation:** Generated from code where possible

## Post-Migration Benefits

### For Users
- **Faster Core:** Lighter PM binary with faster startup
- **Customizable:** Only install needed functionality
- **Extensible:** Easy to add custom workflows
- **Up-to-date:** Extensions update independently

### For Developers
- **Modular:** Clear separation of concerns
- **Testable:** Each extension tested independently  
- **Maintainable:** Smaller, focused codebases
- **Collaborative:** Community can contribute extensions

### For Ecosystem
- **Scalable:** Unlimited functionality through extensions
- **Innovative:** Community-driven feature development
- **Standards:** Consistent extension interfaces
- **Discovery:** Centralized extension registry