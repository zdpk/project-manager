# [FEATURE] Define standard error types for project manager

## Feature Description
Create a comprehensive error type system that provides clear, actionable error messages for all project manager operations.

## Use Case
- Replace generic anyhow errors with specific, typed errors
- Provide better debugging information for developers
- Enable more granular error handling in different contexts
- Improve user experience with clear error messages

## Acceptance Criteria
- [ ] Create `src/error.rs` module with custom Error enum
- [ ] Define specific error variants for:
  - [ ] Configuration file errors (missing, invalid format, schema validation)
  - [ ] Project scanning errors (permission denied, invalid paths)
  - [ ] Git repository errors (not a repo, remote issues)
  - [ ] File system errors (IO operations, directory creation)
  - [ ] Validation errors (invalid project names, duplicate entries)
- [ ] Implement Display and Debug traits for user-friendly messages
- [ ] Add error context with relevant details (file paths, line numbers)
- [ ] Update all modules to use the new error types
- [ ] Ensure errors include suggestions for resolution when possible

## Implementation Notes
- Use thiserror crate for ergonomic error handling
- Keep anyhow for Result chaining but use custom errors at boundaries
- Include file paths and context in error messages
- Follow Rust error handling best practices

## Related Issues
- Foundation for improved error handling across the codebase
- Enables better user experience in subsequent features