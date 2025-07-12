# [FEATURE] Implement parallel directory scanning for better performance

## Feature Description
Add multi-threaded directory scanning to significantly improve performance when processing large numbers of projects or deep directory structures.

## Use Case
- Speed up project discovery in large codebases
- Improve responsiveness when scanning multiple directories
- Better utilize modern multi-core systems
- Reduce wait time for users with many projects

## Acceptance Criteria
- [ ] Implement parallel directory traversal using:
  - [ ] Tokio async runtime for I/O operations
  - [ ] Rayon for CPU-intensive processing
  - [ ] Channel-based communication between threads
- [ ] Add configurable concurrency limits:
  - [ ] Maximum number of parallel scan operations
  - [ ] Depth limits for recursive scanning
  - [ ] Timeout controls for individual scan operations
- [ ] Implement work-stealing scheduler for balanced load distribution
- [ ] Add progress reporting for long-running scan operations
- [ ] Ensure thread safety for shared data structures
- [ ] Include performance benchmarks and tests
- [ ] Add graceful error handling for individual scan failures

## Implementation Notes
- Use existing tokio dependency for async operations
- Integrate with walkdir crate for directory traversal
- Create src/scanner/parallel.rs module
- Maintain backward compatibility with synchronous scanning
- Add configuration options for tuning performance

## Related Issues
- Enhances the pm scan command performance
- Foundation for #10 (scan result caching)