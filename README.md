# Prime Physics Engine

Membrane-based prime number generation with Hardy-Littlewood statistical analysis.

## What This Is

Symmetric "membrane" constructions produce numbers with significantly higher prime
density than random chance. This crate implements the generation method, a
statistical analysis framework (Hardy-Littlewood singular series, effect sizes,
FDR correction), and an optional physics-metaphor visualization layer.

```
Membrane structure: outer + zeros + inner + zeros + SEED + zeros + inner + zeros + outer

Example: Config (1,5) k=(0,0), seed 4 in base 6
         1 5 4 5 1  =  15451 (base 6)  =  2551 (decimal)  -- prime
```

### Verified Results

Tested with Miller-Rabin (20 rounds), n=1000 samples per configuration, p < 0.001:

| Base | Boundary Digits | k | Prime Density | vs Random (~5%) |
|------|----------------|---|---------------|-----------------|
| 6    | (1, 5)         | (0, 0) | 33% | 6.6x |
| 30   | (11, 7)        | (0, 0) | 30% | 6.0x |
| 10   | (3, 7)         | (0, 0) | 18.5% | 3.7x |

Key empirical findings (all verified across multiple bases):
- **Coprimality required**: Boundary digits must be coprime to the base
- **Minimal padding wins**: k=(0,0) dominates for seed length M >= 2
- **Diameter-density law**: Compactness (1/total_digits) predicts density (Spearman rho > 0.77)
- **Base 10 M=2 is a uniquely isolated exception** where k=1 outperforms k=0

For the full evidence base including falsified hypotheses, see
[VERIFIED_FACTS_VS_SPECULATION.md](VERIFIED_FACTS_VS_SPECULATION.md).

## Quick Start

```bash
# Build
cargo build --release

# Run tests
cargo test --lib    # 174 tests

# Try it
cargo run --example proper_membrane_generator    # Generate membrane primes
cargo run --example prime_count_smoke_test        # Validate sieve against OEIS
cargo run --example check_prime                   # Simple prime checker
cargo run --example statistical_prime_generator   # Statistical prime generation
```

### Interactive Tools (require a terminal)

```bash
cargo run --example membrane_lab_tui       # Interactive membrane laboratory
cargo run --example prime_atom_tui         # Visualize membrane primes
cargo run --example lagrange_tui_demo      # Explore prime connectors
```

### Verification

```bash
cargo run --example prime_verification_report      # Verify documented claims
cargo run --example lagrange_full_verification     # Verify Lagrange point primes
cargo run --example lagrange_clustering_verifier   # Verify clustering patterns
cargo run --example verify_prime_checker           # Validate Miller-Rabin
```

See [examples/README.md](examples/README.md) for the full list of 32 curated
examples organized by category.

## Research Analysis Tools

Standalone CLI for reproducible prime density analysis:

```bash
cd tools
rustc prime_unified_cli.rs -O -o prime_unified
./prime_unified --out-dir=./results --ccrt-max-base=100 --mdr-limit=10000000
cat ./results/SUMMARY.txt
```

This produces CSV datasets for Complementary CRT patterns (Goldbach pair
coverage by base factorization) and Midpoint Density Ratios (PNT deviation
measurements). See [tools/README.md](tools/README.md) for parameter reference
and CSV schemas.

## Architecture

```
src/
  lib.rs                   # Crate root -- re-exports, PrimeUniverse, is_prime
  membrane/                # Symmetric membrane construction and builder
  prime_sieve/             # BitSieve (Eratosthenes) with optional wheel30
  hzlib/                   # Hardy-Littlewood framework, stats, sieves, density
  connector/               # Prime concatenation and connector analysis
  gravity/                 # Physics metaphor: N-body gravitational model
  lagrange.rs              # Lagrange point analysis (metaphor layer)
  tidal/                   # Tidal field analysis (metaphor layer)
  chaos/                   # Chaos/stability indicators
  spacetime.rs             # Base metrics and phase space (metaphor layer)
  harmonics.rs             # Fourier analysis (requires prime-harmonics feature)
  bin/                     # CLI binaries (5 membrane-prime variants)
  wasm/                    # WebAssembly bindings (requires wasm feature)

examples/                  # 32 curated examples (see examples/README.md)
tools/                     # Standalone research CLIs
agda-proofs/               # Formal verification framework (partial -- see STATUS.md)
historical/                # Relocated exploration scripts and session docs
collab/                    # Shared artifacts for collaborators
```

The crate has two layers:
- **Math layer** (the verified core): `membrane`, `prime_sieve`, `hzlib`,
  `connector`, `is_prime`
- **Simulation layer** (physics metaphor for visualization): `gravity`, `lagrange`,
  `tidal`, `spacetime`, `PrimeUniverse`

## Features

| Feature | Default | Purpose |
|---------|---------|---------|
| `wheel30` | yes | 30-wheel sieve compression |
| `visualization` | yes | Terminal UI (ratatui/crossterm) |
| `dvfs-adaptive` | yes | DVFS-aware performance monitoring |
| `metal` | no | Apple Metal GPU kernels (macOS only, experimental) |
| `wasm` | no | WebAssembly / wasm-bindgen |
| `prime-harmonics` | no | Fourier analysis (num-complex) |
| `phase4` | no | ARM AMX/SME backend (experimental) |

```bash
# Standard build
cargo build --release

# WASM (excludes terminal UI)
cargo build --target wasm32-unknown-unknown --release --no-default-features --features wasm

# Metal GPU (macOS only)
cargo build --release --features metal
```

## Formal Verification (Agda)

The `agda-proofs/` directory contains a formal verification framework targeting
coordinate constellation properties. Current status:

- **32 of 80 modules type-check** (Agda 2.8.0, stdlib v2.3): 19 clean, 13 with postulates
- The core certification stack (9 modules) is fully operational after repair of
  SymmetryFromList and BucketsAutoMatch
- Modules with postulates assume axioms rather than proving them

See [agda-proofs/STATUS.md](agda-proofs/STATUS.md) for the complete compilation
status of each module.

## Documentation

| Document | Purpose |
|----------|---------|
| [NOVELTY.md](NOVELTY.md) | Honest assessment: what this project actually contributes |
| [CLAUDE.md](CLAUDE.md) | Detailed research context, membrane theory, HL framework reference |
| [CLAIMS.md](CLAIMS.md) | Every claim mapped to its evidence and verification command |
| [VERIFIED_FACTS_VS_SPECULATION.md](VERIFIED_FACTS_VS_SPECULATION.md) | Rigorous fact/speculation separation with falsifiability criteria |
| [EVIDENCE.md](EVIDENCE.md) | Empirical data tables and external verification URLs |
| [examples/README.md](examples/README.md) | Curated example list with descriptions |
| [tools/README.md](tools/README.md) | Research CLI parameter reference and CSV schemas |
| [GLOSSARY.md](GLOSSARY.md) | Terminology definitions |
| [ROADMAP.md](ROADMAP.md) | Hardening roadmap and track status |

Parent directory documentation:
- [../CLAUDE.md](../CLAUDE.md) -- Comprehensive executive summary
- [../EVIDENCE.md](../EVIDENCE.md) -- Full evidence base

## License

MIT License -- see LICENSE file for details.
