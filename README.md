# Membrane Prime Toolkit

Cargo package `primes` is a Rust library, formalization workspace, and analysis
repository for symmetric digit templates (repo term: membranes) and associated
prime-distribution computations.

The active mathematical surface consists of template construction,
sieve/primality routines, Hardy-Littlewood heuristics, statistical analysis,
connector concatenation utilities, and Agda/Lean formalization. Legacy
metaphor-oriented modules such as `PrimeUniverse`, gravity, Lagrange, and tidal
analysis remain available as optional visualization or compatibility layers;
they are not the primary mathematical interface.

## What This Repo Provides

- `MembraneConfig` for symmetric digit-template construction
- `BitSieve` and `is_prime` for prime enumeration and primality testing
- `hzlib::*` for asymptotic heuristics, residue-class analysis, and statistics
- `connector::ConcatenationSystem` for fixed-prime decimal concatenation
- `tools/prime_unified_cli.rs` and related CLIs for reproducible CSV workflows

## Current Checked Status

Canonical repo-level counts live in [`STATUS.md`](STATUS.md).

- library tests pass
- `cargo clippy --lib -- -D warnings` passes cleanly
- the curated top-level examples in [`examples/README.md`](examples/README.md) compile
- the Lean 4 package in [`lean-proofs/`](lean-proofs/) builds with `lake build`

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

## Symmetric Digit Template

Throughout the active docs, a symmetric digit template (repo alias:
`membrane`) means a candidate of the form

```text
outer + 0...0 + inner + 0...0 + seed + 0...0 + inner + 0...0 + outer
```

Example in base 10 with boundary digits `(3, 7)`, `k=(1,1)`, and seed `5`:

```text
3 0 7 0 5 0 7 0 3  ->  307050703
```

## Verified Statements

Selected symmetric digit-template families exhibit substantially higher measured
prime density than naive random integer baselines. The current interpretation is
deliberately conservative: the strongest matched control indicates that most of
the observed lift is explained by coprimality filtering and candidate
magnitude, not by a proved template-specific mechanism. The repo's
template-vs-random-coprime comparison reports a structure ratio of about
`1.020 +/- 0.053` (`p > 0.05`), which is not statistically distinguishable from
`1.0`. See
[`NOVELTY.md`](NOVELTY.md) and
[`collab/THEORETICAL_CLOSURE.md`](collab/THEORETICAL_CLOSURE.md).

Representative measured densities (`n = 1000`, Miller-Rabin, 20 rounds):

| Base | Boundary Digits | k | Prime Density |
|------|-----------------|---|---------------|
| 6    | (1, 5)          | (0, 0) | 33.0% |
| 30   | (11, 7)         | (0, 0) | 30.0% |
| 10   | (3, 7)          | (0, 0) | 18.5% |

Verified in the current audited docs:

- boundary digits coprime to the base are a necessary admissibility condition
  for useful prime density
- minimal padding `k=(0,0)` dominates for seed length `M >= 2`, with a documented
  base-10 `M=2` exception
- compactness / diameter correlates strongly with observed density
- exact enumeration shows the symmetric digit-template family is broader than
  the ordinary palindrome subset; non-palindromic subsets still contain dense
  prime candidates in the tested families
- exact same-budget scaffold controls do not show a consistent centered-gap
  advantage in the tested base-10 and base-6 families
- the connector asymmetry result is real for the canonical pair
  `10301` and `3007003007003`, but is not yet established as a general law

## Open Questions

- why `M=1` sometimes prefers `k>0` while larger `M` does not
- whether any narrower family than the tested centered-gap scaffold shows extra
  signal after same-budget matching
- whether connector asymmetry extends beyond the canonical pair
- whether the diameter-density relationship can be derived theoretically

## Repository Structure

- **Mathematical core**: [`src/membrane`](src/membrane), [`src/prime_sieve.rs`](src/prime_sieve.rs),
  [`src/hzlib`](src/hzlib), [`src/connector`](src/connector)
- **Optional metaphor / visualization layer**: [`src/gravity`](src/gravity),
  [`src/lagrange.rs`](src/lagrange.rs), [`src/tidal`](src/tidal),
  [`src/tui`](src/tui)
- **Analysis tooling**: [`tools/prime_unified_cli.rs`](tools/prime_unified_cli.rs),
  [`tools/README.md`](tools/README.md), workspace tools under `tools/`
- **Formalization and archive**: [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md),
  [`lean-proofs/README.md`](lean-proofs/README.md), [`historical/`](historical)

## Formalization Workspaces

- **Agda**: [`agda-proofs/`](agda-proofs/) remains the broader active proof
  workspace, with an audited clean-local spine and a larger shell layer.
- **Lean 4**: [`lean-proofs/`](lean-proofs/) is the in-repo mathlib-backed
  package. Its current proved surface is symmetry-first and exact-arithmetic
  first: midpoint obstruction and certified witnesses, modular reflection and
  certificate wrappers, affine template structure, coprimality/radical/unit
  residue theorems, `ZMod`/CRT/wheel-base structure, explicit finite
  certificate examples, and exact connector residue filters including a
  reusable family-level profile API. The detailed theorem ledger and future
  queue live in [`lean-proofs/ROADMAP.md`](lean-proofs/ROADMAP.md) and
  [`lean-proofs/THEOREM_INDEX.md`](lean-proofs/THEOREM_INDEX.md).

## Analysis Tools

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

## Documentation Map

For the full claim/evidence trail rather than the short README summary, start
with:

- [`CLAIMS.md`](CLAIMS.md): claim-to-evidence registry with verification commands
- [`NOVELTY.md`](NOVELTY.md): contribution classification and non-claims
- [`VERIFIED_FACTS_VS_SPECULATION.md`](VERIFIED_FACTS_VS_SPECULATION.md):
  verified facts, falsified hypotheses, and open questions
- [`EVIDENCE.md`](EVIDENCE.md): corrected data tables and external validation links
- [`examples/README.md`](examples/README.md): curated example catalog
- [`tools/README.md`](tools/README.md): research CLI reference and CSV schemas
- [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md): current Agda status
  with clean-local vs postulated-foundation notes
- [`lean-proofs/README.md`](lean-proofs/README.md): Lean 4 package and local
  build commands
- [`lean-proofs/ROADMAP.md`](lean-proofs/ROADMAP.md): Lean formalization ledger
- [`lean-proofs/THEOREM_INDEX.md`](lean-proofs/THEOREM_INDEX.md): current map
  from repo prose to Lean theorem families
- [`ROADMAP.md`](ROADMAP.md): hardening roadmap

## License

MIT. See [`LICENSE`](LICENSE).
