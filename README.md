# Membrane Prime Toolkit

Cargo package `primes`: a Rust crate and research repository for building and
analyzing structured prime candidates.

This repository combines a small verified math core with a larger research
workspace around membrane constructions. The core pieces are membrane candidate
construction, segmented sieve/primality utilities, Hardy-Littlewood and
statistical tooling, connector experiments, and standalone research CLIs. The
repo also contains older physics-metaphor modules such as `PrimeUniverse`,
gravity, Lagrange, and tidal analysis; treat those as optional visualization or
pedagogical layers, not as the central mathematical claim.

## What This Repo Provides

- `MembraneConfig` for symmetric membrane candidate construction
- `BitSieve` and `is_prime` for prime enumeration and primality testing
- `hzlib::*` for Hardy-Littlewood expectations, sieve helpers, and statistics
- `connector::ConcatenationSystem` for decimal connector experiments
- `tools/prime_unified_cli.rs` and related tools for reproducible CSV analyses

## Current Checked Status

Canonical repo-level counts live in [`STATUS.md`](STATUS.md).

- library tests pass
- `cargo clippy --lib -- -D warnings` passes cleanly
- the curated top-level examples in [`examples/README.md`](examples/README.md) compile

## Quick Start

```bash
cargo build --release
cargo test --lib
cargo clippy --lib -- -D warnings

cargo run --example prime_count_smoke_test
cargo run --example proper_membrane_generator
cargo run --example prime_verification_report
```

## Core Rust API

```rust
use num_bigint::BigUint;
use primes::{
    connector::ConcatenationSystem,
    hzlib::{goldbach_coverage_from_lambda, hl_goldbach_lambda, sieve_spf, PairCount},
    is_prime, BitSieve, MembraneConfig,
};

let sieve = BitSieve::new(100);
assert_eq!(&sieve.primes()[..5], &[2, 3, 5, 7, 11]);

let config = MembraneConfig::new(10, 3, 7, 1, 1);
let n = config.construct_number(5).unwrap();
assert_eq!(n, BigUint::from(307050703u32));
assert!(is_prime(&n));

let spf = sieve_spf(10_000);
let lambda = hl_goldbach_lambda(1000, &spf, PairCount::Unordered);
let coverage = goldbach_coverage_from_lambda(lambda);
assert!(coverage > 0.0);

let sys = ConcatenationSystem::new(10301, 3007003007003);
assert!(sys.forward(6, 5).is_some());
```

## Membrane Construction at a Glance

A membrane candidate places a seed between symmetric boundary digits and zero
padding:

```text
outer + 0...0 + inner + 0...0 + seed + 0...0 + inner + 0...0 + outer
```

Example in base 10 with boundary digits `(3, 7)`, `k=(1,1)`, and seed `5`:

```text
3 0 7 0 5 0 7 0 3  ->  307050703
```

## Verified Findings and Limits

The repo reproduces several high-density membrane configurations, but the
current interpretation is intentionally conservative: the measured lift over
naive random integers appears to be explained largely by coprimality filtering,
not by a proven membrane-specific mechanism. The repo's own writeups report a
membrane-vs-random-coprime structure boost of about `1.020 +/- 0.053`
(`p > 0.05`), which is not statistically significant. See
[`NOVELTY.md`](NOVELTY.md) and
[`collab/THEORETICAL_CLOSURE.md`](collab/THEORETICAL_CLOSURE.md).

Measured prime densities for representative configurations (`n=1000`,
Miller-Rabin, 20 rounds):

| Base | Boundary Digits | k | Prime Density |
|------|-----------------|---|---------------|
| 6    | (1, 5)          | (0, 0) | 33.0% |
| 30   | (11, 7)         | (0, 0) | 30.0% |
| 10   | (3, 7)          | (0, 0) | 18.5% |

Verified in current repo documentation:

- boundary digits coprime to the base are essential for useful prime density
- minimal padding `k=(0,0)` dominates for seed length `M >= 2`, with a documented
  base-10 `M=2` exception
- compactness / diameter correlates strongly with observed density
- exact enumeration shows membrane families are broader than ordinary
  palindromes; non-palindromic subsets still contain dense prime candidates in
  the tested families
- exact same-budget scaffold controls do not show a consistent centered-gap
  advantage in the tested base-10 and base-6 families
- the connector asymmetry result is real for the canonical pair
  `10301` and `3007003007003`, but is not yet established as a general law

Open questions:

- why `M=1` sometimes prefers `k>0` while larger `M` does not
- whether any narrower family than the tested centered-gap scaffold shows extra
  signal after same-budget matching
- whether connector asymmetry extends beyond the canonical pair
- whether the diameter-density relationship can be derived theoretically

## Repo Structure

- **Core math layer**: [`src/membrane`](src/membrane), [`src/prime_sieve.rs`](src/prime_sieve.rs),
  [`src/hzlib`](src/hzlib), [`src/connector`](src/connector)
- **Optional metaphor / visualization layer**: [`src/gravity`](src/gravity),
  [`src/lagrange.rs`](src/lagrange.rs), [`src/tidal`](src/tidal),
  [`src/tui`](src/tui)
- **Research tools**: [`tools/prime_unified_cli.rs`](tools/prime_unified_cli.rs),
  [`tools/README.md`](tools/README.md), workspace tools under `tools/`
- **Formalization and archive**: [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md),
  [`historical/`](historical)

## Research Tools

For standalone dataset generation, the simplest entrypoint is
[`tools/prime_unified_cli.rs`](tools/prime_unified_cli.rs). It builds with
`rustc` and emits CSV outputs for CCRT and midpoint-density studies.

```bash
cd tools
rustc prime_unified_cli.rs -O -o prime_unified_local
./prime_unified_local --run=all --out-dir=./results --ccrt-max-base=100 --mdr-limit=10000000
```

See [`tools/README.md`](tools/README.md) for output schemas and parameter
details.

## Documentation and Claim Audit

If you want the evidence trail rather than the short README summary, start with:

- [`CLAIMS.md`](CLAIMS.md): claim-to-evidence registry with verification commands
- [`NOVELTY.md`](NOVELTY.md): honest assessment of what is and is not novel here
- [`VERIFIED_FACTS_VS_SPECULATION.md`](VERIFIED_FACTS_VS_SPECULATION.md):
  verified facts, falsified hypotheses, and open questions
- [`EVIDENCE.md`](EVIDENCE.md): corrected data tables and external validation links
- [`examples/README.md`](examples/README.md): curated example catalog
- [`tools/README.md`](tools/README.md): research CLI reference and CSV schemas
- [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md): current Agda status
  with clean-local vs postulated-foundation notes
- [`ROADMAP.md`](ROADMAP.md): hardening roadmap

## License

MIT. See [`LICENSE`](LICENSE).
