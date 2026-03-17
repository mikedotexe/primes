> Archived on 2026-03-10.
>
> This SBOM describes an older release identity and feature surface rather than
> the current repository state. It is preserved as a historical generated
> artifact, not as current package metadata.

# Software Bill of Materials (SBOM)
**Prime Physics Engine v1.0.0-rc1**

Generated: 2025-07-19

## Summary

This document provides a complete inventory of all software components, dependencies, and their versions included in Prime Physics Engine v1.0.0-rc1.

## Package Information

- **Name**: prime-physics-engine
- **Version**: 1.0.0-rc1
- **License**: MIT
- **Repository**: https://github.com/mikepurvis/prime-physics-engine

## Runtime Dependencies

### Core Dependencies
- **num-bigint** ^0.4.6 - Arbitrary precision integers
- **num-traits** ^0.2.19 - Numeric traits for generic programming
- **primal** ^0.3.3 - Prime number utilities
- **primal-bit** ^0.3.2 - Bit manipulation for primes
- **primal-check** ^0.3.4 - Prime checking algorithms  
- **primal-estimate** ^0.3.3 - Prime counting estimates
- **primal-sieve** ^0.3.7 - Sieve of Eratosthenes implementation

### Performance & Optimization
- **rayon** ^1.10.0 - Data parallelism
- **crossbeam-utils** ^0.8.21 - Lock-free concurrency utilities
- **parking_lot** ^0.12.4 - Fast parking lot based synchronization

### Feature-Specific Dependencies

#### WASM Support (`wasm` feature)
- **wasm-bindgen** ^0.2.95 - JavaScript and Rust interop
- **js-sys** ^0.3.72 - JavaScript API bindings
- **web-sys** ^0.3.72 - Web API bindings

#### GPU Acceleration (`gpu` feature)  
- **metal** ^0.29.0 - Metal GPU compute framework (macOS)

#### Harmonics Analysis (`prime-harmonics` feature)
- **num-complex** ^0.4.6 - Complex number arithmetic

#### Visualization & UI
- **ratatui** ^0.25.0 - Terminal user interface
- **crossterm** ^0.27.0 - Cross-platform terminal manipulation
- **plotters** ^0.3.7 - Data plotting and visualization
- **colored** ^2.2.0 - Terminal color output

#### Utilities
- **clap** ^4.5.41 - Command line argument parsing
- **serde** ^1.0.219 - Serialization framework
- **serde_json** ^1.0.140 - JSON serialization
- **chrono** ^0.4.41 - Date and time utilities
- **itertools** ^0.12.1 - Iterator utilities

## Development Dependencies

### Testing
- **criterion** ^0.5 - Statistical benchmarking (non-WASM targets only)

### Build Tools
- **rustc** >= 1.82.0 - Rust compiler
- **cargo** - Rust package manager

## Security Considerations

- All dependencies are from trusted sources (crates.io)
- No known security vulnerabilities in included versions
- Dependencies undergo regular security audits via `cargo audit`
- No network-facing components by default

## Verification

To verify the integrity of dependencies:

```bash
# Audit for security vulnerabilities
cargo audit

# Check for outdated dependencies
cargo outdated

# Generate fresh dependency tree
cargo tree
```

## License Compliance

All dependencies use MIT, Apache-2.0, or compatible licenses. See individual crate documentation for specific license terms.

---

*This SBOM was automatically generated and verified for Prime Physics Engine v1.0.0-rc1*
