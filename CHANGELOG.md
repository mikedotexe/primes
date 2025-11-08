# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-07-19

### Added
- **General Availability Release** - Production-ready status achieved
- Complete release documentation (RELEASE_NOTES_v1.0.0.md)
- Metal safety documentation (METAL_SAFETY.md)
- Full CI/CD pipeline for automated releases
- Enhanced build configuration with proper feature flags

### Changed
- Version bumped from 1.0.0-rc1 to 1.0.0 (GA)
- Documentation updated to reflect production status
- Build system refined for cross-platform compatibility

### Fixed
- All critical issues from RC1 addressed
- Metal GPU compilation properly documented
- WASM build process fully automated

## [1.0.0-rc1] - 2025-01-19

### Added
- Core membrane prime generation with 33% success rate (Base-6)
- Comprehensive test suite (59 tests passing)
- 19 working examples organized in `examples/verified/`
- WASM build support (criterion dependency fixed)
- Educational explorer for newcomers
- Performance monitoring with cycle-accurate timing
- Cross-base pattern validation
- Tidal field analysis and organization zones
- Resonance profile discovery system

### Changed
- Moved from aspirational GPU claims to honest "experimental" status
- Updated all documentation to reflect verified capabilities
- Organized 93 examples into verified/experimental directories
- Improved error handling with bounds checking

### Fixed
- Metal GPU build properly feature-gated
- WASM build issues with criterion dependency
- Zero warnings in library build
- BitMapBackend compatibility for WASM targets
- Thread import issues in performance module

### Security
- Added comprehensive bounds checking in integration
- Safe median calculation with bounds validation
- Panic prevention in critical paths

### Known Issues
- Metal GPU requires manual shader compilation
- Some plotters features unavailable in WASM context
- 74 examples in experimental/ need syntax restoration

[1.0.0]: https://github.com/mikepurvis/prime-physics-engine/releases/tag/v1.0.0
[1.0.0-rc1]: https://github.com/mikepurvis/prime-physics-engine/releases/tag/v1.0.0-rc1