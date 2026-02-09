# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Retry helper APIs: `flush_with_retry(max_attempts)` and
  `close_with_retry(max_attempts)`
- Packaging exclusions for local/dev-only files

### Changed
- Clarified 0.x API stability expectations in README
- Documented retry-layer interaction in README
- Updated docs to describe blocking HTTP support clearly

### Fixed
- Per-attempt timeout now respects remaining total timeout budget
- HTTP failures now include better status/body context
- Dataset path segment is percent-encoded for safe endpoint construction
- `Event::add` now errors on non-object payloads instead of silently succeeding
- Removed unused direct dependency on `parking_lot`

## [0.1.0] - 2026-02-09

### Added
- Initial project structure
- Core client implementation
- Basic documentation
- CI/CD pipeline with quality gates
- Minimal, security-focused Honeycomb.io client
- Pure crates.io dependencies (no git deps)
- Modern reqwest 0.11+ blocking HTTP support (feature-gated)
- Exponential backoff retry logic
- Event batching for efficiency
- Type-safe configuration
