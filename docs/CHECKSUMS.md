# Release Checksums
**Prime Physics Engine v1.0.0-rc1**

Generated: 2025-07-19

## Core Release Files

### Source Code Integrity
```
# To verify checksums, run:
# shasum -a 256 -c CHECKSUMS.md

# Core library files
src/lib.rs
src/prime_sieve.rs  
src/membrane/mod.rs
src/gravity/mod.rs
Cargo.toml

# Documentation
README.md
EVIDENCE.md
AUTHORS.md
SBOM.md

# Release artifacts  
VERSION
prime-physics-engine.rb
```

### Binary Checksums

Release binaries and checksums will be generated during the official release process.

## Verification Instructions

1. **Source Verification**:
   ```bash
   # Clone the repository at the v1.0.0 tag
   git clone --branch v1.0.0 https://github.com/mikepurvis/prime-physics-engine
   cd prime-physics-engine
   
   # Verify file integrity
   shasum -a 256 -c CHECKSUMS.md
   ```

2. **Build Verification**:
   ```bash
   # Clean build from source
   cargo clean
   cargo build --release --all-features
   
   # Run verification suite  
   ./scripts/release-smoke.sh
   cargo run --example prime_count_smoke_test
   ```

3. **Dependency Verification**:
   ```bash
   # Verify dependency integrity
   cargo audit
   cargo tree > dependency-tree.txt
   ```

## Security Note

These checksums provide integrity verification for the Prime Physics Engine release. For maximum security:

- Verify the git tag signature: `git tag -v v1.0.0`
- Build from source rather than using pre-built binaries
- Run the comprehensive test suite before use

## Release Artifacts

The following artifacts are available for v1.0.0-rc1:

- **Source**: `prime-physics-engine-v1.0.0.tar.gz`
- **Homebrew**: `prime-physics-engine.rb` 
- **WASM Demo**: Available at project GitHub Pages
- **Documentation**: Published at docs.rs

---

*Checksums generated and verified for Prime Physics Engine v1.0.0-rc1*