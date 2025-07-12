# [FEATURE] Add progress bars for long-running operations

## Feature Description
Implement visual progress indicators to provide real-time feedback during time-consuming operations like project scanning, batch operations, and file processing.

## Use Case
- Show scan progress when processing large directory trees
- Provide feedback during batch project operations
- Indicate completion percentage for multi-step processes
- Reduce user anxiety during long-running commands

## Acceptance Criteria
- [ ] Implement progress bar system using indicatif crate:
  - [ ] Determinate progress bars with percentage completion
  - [ ] Indeterminate spinners for unknown duration tasks
  - [ ] Multi-bar displays for concurrent operations
  - [ ] Customizable progress bar styles and themes
- [ ] Add progress tracking for key operations:
  - [ ] Directory scanning with file count estimation
  - [ ] Batch project processing with item counts
  - [ ] Configuration file operations
  - [ ] Cache building and maintenance
- [ ] Create progress reporting framework:
  - [ ] Abstract progress traits for different operation types
  - [ ] Hierarchical progress for nested operations
  - [ ] Progress event system for custom handlers
- [ ] Add configuration options:
  - [ ] Enable/disable progress bars
  - [ ] Customize update frequency and styling
  - [ ] Terminal width-aware formatting

## Implementation Notes
- Extend existing indicatif dependency usage
- Create src/ui/progress.rs module
- Integrate with async operations from #09 (parallel scanning)
- Ensure progress bars work well with colored output from #14
- Add progress reporting to existing commands gradually

## Related Issues
- Enhances user experience for #09 (parallel scanning)
- Complements visual improvements from #14 (colorful output)