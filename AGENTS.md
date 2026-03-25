# Prime Physics Engine -- Developer Reference

**Status**: 174 library tests pass, clippy clean, 34 curated examples compile
**Last verified**: March 2026

## What This Crate Does

This crate implements symmetric digit-template construction (repo alias:
"membrane"), prime search utilities, analytic heuristics, and optional
metaphor-based visualization APIs.

The current empirical record shows that selected template families achieve
substantially higher prime density than naive random baselines. The current
conservative interpretation is narrower: most of the observed lift is explained
by coprimality filtering and candidate magnitude, while any additional
template-specific effect remains unproved.

### Core Template

```
outer + (k1 zeros) + inner + (k2 zeros) + SEED + (k2 zeros) + inner + (k1 zeros) + outer

Example: Config (3,7) k=(2,1), seed 5
  3 00 7 0 5 0 7 00 3  -->  300705070003 (prime)

Example: Config (1,5) k=(0,0), seed 4 in base 6
  1 5 4 5 1  -->  15451 (base 6)  =  2551 (decimal, prime)
```

### Representative Empirical Results

Measured with Miller-Rabin (20 rounds), `n = 1000` samples per configuration:

| Base | Boundary Digits | k | Prime Density | vs Naive Random (~5%) |
|------|----------------|---|---------------|-----------------|
| 6    | (1, 5)         | (0, 0) | 33% | 6.6x |
| 30   | (11, 7)        | (0, 0) | 30% | 6.0x |
| 10   | (3, 7)         | (0, 0) | 18.5% | 3.7x |

### Verified vs Open

**Verified** (empirical, reproducible):
1. Selected symmetric digit-template families produce `3x-7x` higher prime
   density than naive random baselines.
2. Boundary digits coprime to the base are required for useful prime density.
3. Minimal padding `k=(0,0)` dominates for seed length `M >= 2`.
4. Diameter/compactness correlates strongly with observed density
   (`Spearman rho > 0.77`).
5. The observed density lift is largely explained by coprimality filtering
   together with ordinary prime-density effects; the best matched
   template-specific ratio is about `1.02`, not statistically significant.

**Open** (unresolved):
1. A theoretical derivation of the diameter-density relationship.
2. The `M=1` exception, where some bases prefer `k>0` while larger `M` do not.
3. Directional asymmetry for prime connectors beyond the canonical pair.

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
- **Mathematical core**: `membrane`, `prime_sieve`, `hzlib`, `connector`, `is_prime`
- **Optional visualization / legacy metaphor layer**: `gravity`, `lagrange`,
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
use primes::hzlib::*;

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

A number `n` can be prime only if `gcd(n, rad(b)) = 1`, where `rad(b)` is the
product of the distinct prime factors of `b`. Example: `rad(12) = 2*3 = 6`
(not `12`).

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

See [examples/README.md](examples/README.md) for the full list of 34 curated examples.

## Formal Verification (Agda)

81 of 81 Agda modules type-check individually (40 clean-local, 41 with local
postulates, 0 failing). The core certification stack
(SymmetryImpliesRepulsion through CertifiedResonanceParamDyn) is fully
operational. See [agda-proofs/STATUS.md](agda-proofs/STATUS.md) for
module-by-module status and boundary notes.

## Formal Verification (Lean 4)

The repo now also includes a mathlib-backed Lean package under
[lean-proofs/](lean-proofs/). Its current proved surface is symmetry-first: the
initial lane contains the abstract midpoint-obstruction theorem, a concrete
base-6 witness, and a conservative exact arithmetic layer for coprimality,
radicals, unit residues, and wheel-like bases. Broader density formalization
remains roadmap material in
[lean-proofs/ROADMAP.md](lean-proofs/ROADMAP.md).

Verify the Lean package locally with:

```bash
cd lean-proofs
lake build
```

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
