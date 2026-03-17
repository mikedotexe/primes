# Prime Physics Engine -- Developer Reference

**Status**: 174 library tests pass, clippy clean, 32 curated examples compile
**Last verified**: March 2026

## What This Crate Does

Symmetric "membrane" constructions produce numbers with significantly higher prime
density than random chance. This crate implements the generation method, a statistical
analysis framework (Hardy-Littlewood singular series, effect sizes, FDR correction),
and an optional physics-metaphor visualization layer.

### Core Concept: Membrane Structure

```
outer + (k1 zeros) + inner + (k2 zeros) + SEED + (k2 zeros) + inner + (k1 zeros) + outer

Example: Config (3,7) k=(2,1), seed 5
  3 00 7 0 5 0 7 00 3  -->  300705070003 (prime)

Example: Config (1,5) k=(0,0), seed 4 in base 6
  1 5 4 5 1  -->  15451 (base 6)  =  2551 (decimal, prime)
```

### Verified Empirical Results

Tested with Miller-Rabin (20 rounds), n=1000 samples per configuration, p < 0.001:

| Base | Boundary Digits | k | Prime Density | vs Random (~5%) |
|------|----------------|---|---------------|-----------------|
| 6    | (1, 5)         | (0, 0) | 33% | 6.6x |
| 30   | (11, 7)        | (0, 0) | 30% | 6.0x |
| 10   | (3, 7)         | (0, 0) | 18.5% | 3.7x |

### What Is Verified vs What Is Open

**Verified** (empirical, reproducible):
1. Membrane structures produce 3-7x higher prime density than random
2. Coprimality of boundary digits to the base is essential
3. Minimal padding (k=0,0) dominates for seed length M >= 2
4. Diameter-density law: compactness predicts density (Spearman rho > 0.77)
5. Membrane density advantage is largely explained by coprimality filtering
   (Euler + Mertens + PNT); structure boost is ~1.02x, not statistically significant

**Open** (unresolved):
1. Theoretical proof of the diameter-density law
2. Why M=1 prefers k>0 while all larger M prefer k=0
3. Directional asymmetry in prime connectors (tested on one pair only)

For rigorous fact/speculation separation, see
[VERIFIED_FACTS_VS_SPECULATION.md](VERIFIED_FACTS_VS_SPECULATION.md).

## Architecture

```
src/
  lib.rs                   # Crate root, re-exports, PrimeUniverse, is_prime
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
```

Two layers:
- **Math layer** (verified core): `membrane`, `prime_sieve`, `hzlib`, `connector`, `is_prime`
- **Simulation layer** (physics metaphor for visualization): `gravity`, `lagrange`,
  `tidal`, `spacetime`, `PrimeUniverse`

## Hardy-Littlewood API Reference

### Key Conventions

- All logarithms use base *e* (natural log), never base 10
- Distinguish ordered (p,q) vs unordered {p,q} pairs via `PairCount` enum
- Twin-prime constant: C2 = prod_{p>2} (1 - 1/(p-1)^2) ~ 0.6601618158468696

### Core Functions (`src/hzlib/hardy_littlewood.rs`)

| Function | Returns |
|----------|---------|
| `singular_series_goldbach_multiplicative(n, spf)` | S2(n) only (no kappa) |
| `hl_goldbach_lambda(n, spf, PairCount)` | Full expectation: kappa * S2(n) * n / ln^2(n) |
| `hl_goldbach_lambda_truncated(n, lo, spf, PairCount)` | Restricted: both primes >= lo |
| `goldbach_coverage_from_lambda(lambda)` | Poisson coverage: 1 - e^(-lambda) |

```rust
use prime_physics_engine::hzlib::*;

let spf = sieve_spf(10000);
let lambda = hl_goldbach_lambda(1000, &spf, PairCount::Unordered);
let coverage = goldbach_coverage_from_lambda(lambda);

// Restricted: both primes >= 100
let lambda_trunc = hl_goldbach_lambda_truncated(1000, 100, &spf, PairCount::Unordered);
```

### Statistical Tools (`src/hzlib/stats.rs`)

- **Hedges' g**: parametric effect size (bias-corrected)
- **Cliff's delta**: non-parametric effect size (rank-based)
- **linreg_with_ci**: linear regression with confidence intervals
- **Spearman rho**: monotonic correlation
- **Benjamini-Hochberg**: FDR correction for multiple comparisons

### Exact Denominators (`src/hzlib/density.rs`)

A number n can be prime only if gcd(n, rad(b)) = 1, where rad(b) is the product of
distinct prime factors of b. Example: rad(12) = 2*3 = 6 (not 12).

## Build and Test

```bash
cargo build --release                    # Standard build
cargo test --lib                         # 174 tests
cargo clippy --lib -- -D warnings        # Must pass clean

# WASM (excludes terminal UI)
cargo build --target wasm32-unknown-unknown --release --no-default-features --features wasm

# Metal GPU (macOS only, experimental)
cargo build --release --features metal
```

### Feature Flags

| Feature | Default | Purpose |
|---------|---------|---------|
| `wheel30` | yes | 30-wheel sieve compression |
| `visualization` | yes | Terminal UI (ratatui/crossterm) |
| `dvfs-adaptive` | yes | DVFS-aware performance monitoring |
| `metal` | no | Apple Metal GPU kernels (macOS, experimental) |
| `wasm` | no | WebAssembly / wasm-bindgen |
| `prime-harmonics` | no | Fourier analysis (num-complex) |
| `phase4` | no | ARM AMX/SME backend (experimental) |

### Quick Verification

```bash
cargo run --example prime_verification_report   # Verify documented claims
cargo run --example prime_count_smoke_test       # Sieve vs OEIS reference
cargo run --example verify_prime_checker         # Miller-Rabin validation
cargo run --example proper_membrane_generator    # Generate membrane primes
```

See [examples/README.md](examples/README.md) for the full list of 32 curated examples.

## Formal Verification (Agda)

32 of 80 Agda modules type-check (19 clean, 13 with postulates). The core
certification stack (SymmetryImpliesRepulsion through CertifiedResonanceParamDyn)
is fully operational. See [agda-proofs/STATUS.md](agda-proofs/STATUS.md) for
module-by-module status.

## Development Practices

### Pre-Commit Checklist

```bash
cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test --lib
```

### TUI Applications

TUI examples (membrane_lab_tui, prime_atom_tui, lagrange_tui_demo) require a real
terminal. They will fail with "Device not configured" in non-terminal contexts.
Successful compilation is sufficient verification for CI purposes.

## Key Documents

| Document | Purpose |
|----------|---------|
| [README.md](README.md) | Public-facing summary with quick start |
| [CLAIMS.md](CLAIMS.md) | Claim-evidence registry: every claim mapped to its verification |
| [VERIFIED_FACTS_VS_SPECULATION.md](VERIFIED_FACTS_VS_SPECULATION.md) | Rigorous claim classification with falsifiability criteria |
| [EVIDENCE.md](EVIDENCE.md) | Empirical data tables and external verification |
| [GLOSSARY.md](GLOSSARY.md) | Terminology definitions |
| [ROADMAP.md](ROADMAP.md) | Hardening roadmap and track status |
| [collab/THEORETICAL_CLOSURE.md](collab/THEORETICAL_CLOSURE.md) | Key finding: coprimality explains membrane density |
| [agda-proofs/STATUS.md](agda-proofs/STATUS.md) | Agda module compilation status |
