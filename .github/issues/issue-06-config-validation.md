# [FEATURE] Implement JSON schema validation for config.json

## Feature Description
Add comprehensive JSON schema validation for the project configuration file to ensure data integrity and provide helpful error messages for invalid configurations.

## Use Case
- Validate configuration files before processing
- Provide clear error messages for invalid config entries
- Prevent runtime errors due to malformed configuration
- Guide users toward correct configuration format

## Acceptance Criteria
- [ ] Create JSON schema definition for config.json structure
- [ ] Implement schema validation using jsonschema crate
- [ ] Add validation for:
  - [ ] Required fields (projects array, default_directory)
  - [ ] Data types (strings, booleans, objects)
  - [ ] Format constraints (valid paths, project names)
  - [ ] Enum values (project types, tags)
- [ ] Generate helpful error messages with:
  - [ ] Field path that failed validation
  - [ ] Expected vs actual values
  - [ ] Suggestions for fixing the issue
- [ ] Add validation to config loading process
- [ ] Include unit tests for various invalid config scenarios

## Implementation Notes
- Extend existing jsonschema usage in the codebase
- Use the config.schema.json file as foundation
- Integrate validation into src/config.rs module
- Follow Rust error handling patterns with custom error types

## Related Issues
- Builds on #24 (error types definition)
- Improves robustness of configuration management