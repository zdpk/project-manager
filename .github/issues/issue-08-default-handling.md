# [FEATURE] Improve default value handling for configuration

## Feature Description
Enhance the configuration system to intelligently handle missing fields and provide smart defaults based on user environment and common usage patterns.

## Use Case
- Gracefully handle incomplete or minimal configuration files
- Reduce initial configuration burden for new users
- Provide environment-aware defaults (OS, shell, common directories)
- Enable progressive configuration enhancement

## Acceptance Criteria
- [ ] Implement smart default value system:
  - [ ] OS-aware default directories (~/Projects, ~/Code, ~/Development)
  - [ ] Shell-aware command preferences
  - [ ] Common git hosting service patterns
- [ ] Add configuration auto-discovery:
  - [ ] Detect existing project structures in common locations
  - [ ] Suggest project types based on file patterns
  - [ ] Auto-configure git integration if available
- [ ] Create configuration bootstrapping:
  - [ ] Generate minimal config with intelligent defaults
  - [ ] Provide interactive configuration wizard
  - [ ] Allow incremental configuration addition
- [ ] Add validation for default value consistency
- [ ] Include comprehensive tests for default generation logic

## Implementation Notes
- Extend src/config.rs with default value providers
- Use dirs crate for OS-aware directory detection
- Implement strategy pattern for different default providers
- Integrate with existing validation system from #06

## Related Issues
- Builds on #06 (config validation) and #07 (migration)
- Improves initial user experience from pm init command