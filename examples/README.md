# Prime Physics Engine Examples

## Directory Structure

This directory contains **94 total examples** organized as follows:

### Main Directory - Research & Verification Tools (63 examples)

**Core working examples** including:
- Prime verification and testing tools
- Membrane construction and generation
- Lagrange point discovery and analysis
- Base comparison and optimization
- Statistical analysis tools
- Interactive TUI applications

**Usage:**
```bash
# Run any example from main directory
cargo run --example <name>

# With specific features
cargo run --example <name> --features "visualization wheel30"
```

**Recommended Starting Point:** See `../RESEARCHER_QUICKSTART.md` for the 5-command tour

### `verified/` - Legacy Production Examples (25 examples)

**Older working examples** from earlier development phases:
- Basic membrane demonstrations
- Performance benchmarking tools
- GPU readiness testing
- Educational explorers

**Note:** Most actively maintained examples have moved to the main directory.

### `experimental/` - Advanced Research Tools (6 examples)

**Hardy-Littlewood framework** and advanced statistical analysis:
- Goldbach conjecture analysis
- Phase 2 density analysis
- Lagrange clustering verification
- Prime density midpoint studies

**Status**: Cutting-edge research tools, may require specific configurations.

## Quick Start

### The 5-Command Tour (15 minutes)

For a comprehensive introduction, follow the **RESEARCHER_QUICKSTART.md** guide. Here's the essential sequence:

```bash
# 1. Validate installation (30 seconds)
cargo run --example prime_count_smoke_test

# 2. Generate membrane primes (1 minute)
cargo run --example proper_membrane_generator

# 3. Witness Lagrange discovery (2 minutes)
cargo run --example lagrange_full_verification

# 4. See base-6 champion (1 minute)
cargo run --example statistical_prime_generator

# 5. Verify all claims (1 minute)
cargo run --example prime_verification_report
```

**Expected result:** You'll see verified 33% prime density and Lagrange point equilibrium.

### Additional Research Tools

```bash
# Interactive membrane construction (requires terminal)
cargo run --example membrane_lab_tui
cargo run --example lagrange_educational_tui
cargo run --example prime_atom_tui

# Base-specific investigations
cargo run --example base6_investigation
cargo run --example comprehensive_base_analysis

# Lagrange point mechanics
cargo run --example lagrange_mechanics
cargo run --example concatenated_lagrange_explorer

# Statistical and verification tools
cargo run --example check_prime  # Interactive: check any number
cargo run --example membrane_showcase

# Hardy-Littlewood framework (advanced)
cargo run --example experimental/goldbach_hl_analysis -- --min-base 60 --max-base 80
cargo run --example experimental/hz_phase2_density -- --bases 6,30,10

# Legacy examples (in verified/)
cargo run --example verified/educational_explorer
cargo run --example verified/sieve_benchmark
cargo run --example verified/harmonic_viewer
```

## Example Categories

**By Purpose:**
- **Verification**: `prime_count_smoke_test`, `prime_verification_report`, `check_prime`
- **Membrane Generation**: `proper_membrane_generator`, `statistical_prime_generator`, `membrane_showcase`
- **Lagrange Points**: `lagrange_full_verification`, `lagrange_mechanics`, `concatenated_lagrange_explorer`
- **Base Analysis**: `base6_investigation`, `comprehensive_base_analysis`
- **Interactive TUI**: `membrane_lab_tui`, `lagrange_educational_tui`, `prime_atom_tui`
- **Statistical**: `experimental/goldbach_hl_analysis`, `experimental/hz_phase2_density`

**By Experience Level:**
- **Beginner**: Start with the 5-command tour in `RESEARCHER_QUICKSTART.md`
- **Intermediate**: Explore base analysis and interactive tools
- **Advanced**: Hardy-Littlewood framework and custom research

## Current Status

- **Main directory**: 63 examples, all verified working
- **Verified directory**: 25 legacy examples, production-ready
- **Experimental directory**: 6 advanced research tools
- **Total**: 94 examples across all categories

All examples compile successfully. The core mathematical functionality is production-ready and thoroughly tested.
EOF < /dev/null