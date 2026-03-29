# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-29

### Added
- Event sourcing crate structure for audit trail and event streaming support
- Policy engine crate structure for configuration and rule evaluation
- Language-agnostic hexagonal architecture specification and documentation
- Expanded PRD and FR with E9 workspace ergonomics user journey
- USER_JOURNEYS.md specification with comprehensive workflow documentation
- Docs-site scaffold and verification harness with @phenotype/docs integration

### Fixed
- Remove unused imports causing cargo warnings (#59)
- TDD test failures in domain layer (#50)
- Final cleanup for test compatibility

### Changed
- Integrate @phenotype/docs shared VitePress theme for unified documentation
- Migrate kitty-specs to docs/specs (AgilePlus format) (#61, #62)

### Chore
- Commit working changes from work-audit session 2026-03-28
- Finalize kitty-specs migration cleanup
- Add governance files (CODEOWNERS, CI workflow) (#52, #53)
- Fix cargo audit configuration
- Suppress dead_code warning on StoredEvent::event_type field
