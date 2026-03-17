# Membrane Prime Toolkit -- Developer Reference

**Cargo package**: `primes`  
**Status snapshot**: see [`STATUS.md`](STATUS.md) for current verified counts and commands  
**Last verified**: March 2026

## What This Repository Does

This repository is a Rust crate plus research workspace for constructing and
analyzing structured prime candidates.

The verified core is:

- `MembraneConfig` and related membrane constructors
- `BitSieve` and `is_prime`
- `hzlib::*` for Hardy-Littlewood expectations, sieve helpers, and statistics
- `connector::ConcatenationSystem` for decimal connector experiments

The repository also contains optional metaphor and visualization modules such as
`PrimeUniverse`, gravity, Lagrange, and tidal analysis. Those modules are best
treated as visualization or pedagogical layers, not as the mathematical center
of the project.

## Current Framing

The measured membrane densities are real, but the current evidence base supports
an intentionally conservative interpretation:

- membrane constructions can produce much higher prime density than naive random
  integers of comparable size
- that lift is largely explained by coprimality filtering
- the repo's own membrane-vs-random-coprime comparison reports a structure boost
  of about `1.020 +/- 0.053`, not statistically distinguishable from `1.0`

That framing is documented in [`NOVELTY.md`](NOVELTY.md),
[`CLAIMS.md`](CLAIMS.md), and
[`collab/THEORETICAL_CLOSURE.md`](collab/THEORETICAL_CLOSURE.md).

## Representative Verified Findings

Measured examples (`n=1000`, Miller-Rabin, 20 rounds):

| Base | Boundary Digits | k | Prime Density |
|------|-----------------|---|---------------|
| 6    | (1, 5)          | (0, 0) | 33.0% |
| 30   | (11, 7)         | (0, 0) | 30.0% |
| 10   | (3, 7)          | (0, 0) | 18.5% |

Verified in current repo documents:

1. Boundary digits coprime to the base are essential.
2. Minimal padding `k=(0,0)` dominates for seed length `M >= 2`, with a
   documented base-10 `M=2` exception.
3. Compactness / diameter correlates strongly with observed density.
4. Connector asymmetry is real for the canonical pair `10301` and
   `3007003007003`, but generality is still open.
5. The membrane-specific structure effect beyond coprimality has not been shown
   to be statistically significant.

Open questions:

1. Why does `M=1` sometimes prefer `k>0` while larger `M` does not?
2. Does connector asymmetry generalize beyond the canonical pair?
3. Can the diameter-density relationship be derived theoretically?

## Core APIs

| API | Purpose |
|-----|---------|
| `MembraneConfig` | Symmetric membrane configuration and candidate construction |
| `BitSieve` | Segmented sieve for prime enumeration |
| `is_prime` | Primality testing for `BigUint` |
| `hzlib::*` | Hardy-Littlewood, sieves, number theory, effect sizes, regression |
| `connector::ConcatenationSystem` | Decimal concatenation and connector experiments |

```rust
use num_bigint::BigUint;
use primes::{
    connector::ConcatenationSystem,
    hzlib::{goldbach_coverage_from_lambda, hl_goldbach_lambda, sieve_spf, PairCount},
    is_prime, BitSieve, MembraneConfig,
};

let sieve = BitSieve::new(100);
assert_eq!(&sieve.primes()[..5], &[2, 3, 5, 7, 11]);

let cfg = MembraneConfig::new(10, 3, 7, 1, 1);
let n = cfg.construct_number(5).unwrap();
assert_eq!(n, BigUint::from(307050703u32));
assert!(is_prime(&n));

let spf = sieve_spf(10_000);
let lambda = hl_goldbach_lambda(1000, &spf, PairCount::Unordered);
let coverage = goldbach_coverage_from_lambda(lambda);
assert!(coverage > 0.0);

let sys = ConcatenationSystem::new(10301, 3007003007003);
assert!(sys.forward(6, 5).is_some());
```

## Repository Layout

### Core math and analysis

- `src/membrane`
- `src/prime_sieve.rs`
- `src/hzlib`
- `src/connector`
- `src/validation`

### Supporting analysis and profiling

- `src/fingerprint`
- `src/resonance_profiles.rs`
- `examples/`
- `tools/`

### Optional metaphor / visualization layer

- `src/gravity`
- `src/lagrange.rs`
- `src/tidal`
- `src/spacetime.rs`
- `src/tui`

## Hardy-Littlewood Conventions

- all logarithms are natural logs
- `PairCount` distinguishes ordered `(p, q)` from unordered `{p, q}` counting
- `singular_series_goldbach_multiplicative` returns `S2(n)` only
- `hl_goldbach_lambda` and `hl_goldbach_lambda_truncated` return full expected
  counts
- `goldbach_coverage_from_lambda` applies the Poisson coverage transform

## Build and Verification

```bash
cargo build --release
cargo test --lib
cargo clippy --lib -- -D warnings

cargo run --example prime_count_smoke_test
cargo run --example proper_membrane_generator
cargo run --example prime_verification_report
cargo run --example verify_prime_checker
```

Feature flags:

| Feature | Default | Purpose |
|---------|---------|---------|
| `wheel30` | yes | 30-wheel sieve compression |
| `visualization` | yes | TUI support via `ratatui` / `crossterm` |
| `dvfs-adaptive` | yes | Performance monitoring helpers |
| `metal` | no | Apple Metal kernels |
| `wasm` | no | WebAssembly bindings |
| `prime-harmonics` | no | Fourier analysis support |
| `phase4` | no | Experimental ARM AMX/SME backend |

TUI examples require a real terminal. In non-terminal environments, successful
compilation is the useful check.

## Formal Verification (Agda)

Current audited status:

- see [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) for the current
  clean-local / with-local-postulates / failing counts
- use the notes there for modules that are clean locally but still sit atop
  postulated foundations

Use [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) as the ground truth. Treat
older certification summaries as archival unless they explicitly defer to
`STATUS.md`.

## Key Documents

| Document | Use |
|----------|-----|
| [README.md](README.md) | Public-facing repo summary |
| [CLAIMS.md](CLAIMS.md) | Claim-to-evidence registry |
| [NOVELTY.md](NOVELTY.md) | Honest novelty assessment |
| [VERIFIED_FACTS_VS_SPECULATION.md](VERIFIED_FACTS_VS_SPECULATION.md) | Verified facts, falsified hypotheses, open questions |
| [EVIDENCE.md](EVIDENCE.md) | Detailed empirical tables and corrections |
| [GLOSSARY.md](GLOSSARY.md) | Terminology and notation |
| [examples/README.md](examples/README.md) | Curated example list |
| [tools/README.md](tools/README.md) | Standalone research tool reference |
| [collab/THEORETICAL_CLOSURE.md](collab/THEORETICAL_CLOSURE.md) | Coprimality interpretation and closure narrative |
| [agda-proofs/STATUS.md](agda-proofs/STATUS.md) | Current Agda compilation status |
