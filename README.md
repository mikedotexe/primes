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
- `connector::*` for fixed-prime decimal concatenation, residue profiles, and
  matched connector-hit scans with density-aware residual audits
- the **Prime Witness Engine** for seed-origin demos, affine residue funnels,
  large readable probable-prime witnesses, and verification transcripts
- the proof-carrying matched-control atlas in
  [`docs/atlas/`](docs/atlas/), linking maintained empirical panel identities
  to generated Lean lane names and exact local arithmetic facts
- `tools/prime_unified_cli.rs` and related CLIs for reproducible CSV workflows

## Current Checked Status

Canonical repo-level counts live in [`STATUS.md`](STATUS.md).

- library tests pass
- `cargo clippy --lib -- -D warnings` passes cleanly
- the curated top-level examples in [`examples/README.md`](examples/README.md) compile
- the Lean 4 package in [`lean-proofs/`](lean-proofs/) builds with `lake build`
- `scripts/ci_proof_catalog.sh` runs generated Lean/Agda catalog checks plus
  the matched-control atlas bridge

## Quick Start

```bash
cargo build --release
cargo test --lib
cargo clippy --lib -- -D warnings
scripts/ci_proof_catalog.sh

cargo run --example prime_count_smoke_test
cargo run --example proper_membrane_generator
cargo run --example prime_verification_report

cargo run --release --bin seed-to-witness
cargo run --release --bin seed-to-witness -- --seed 60
scripts/signal_spine.sh witness-engine
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

## Prime Witness Engine

The large-witness path has a maintained front door at
[`docs/PRIME_WITNESS_ENGINE.md`](docs/PRIME_WITNESS_ENGINE.md). Its core
workflow is:

```text
seed origin -> affine membrane lane -> residue funnel -> probable-prime witness -> verification transcript
```

The demo entrypoint is `seed-to-witness`: without `--seed`, it uses the current
epoch nanoseconds as the seed origin; with `--seed 60`, it returns the canonical
128-digit witness transcript for the default decimal `(3,7), k=(2,1)` lane. The
measurement entrypoint is `large_affine_witness_ladder_report`, which records
time-to-first witness, residue-funnel efficacy, backend scope, controls, and
semantic rarity for large visible witnesses.

The maintained proof-carrying witness bundle lives under
[`docs/witness/`](docs/witness/): seed-60, a 38-digit teaching witness, and a
timestamp-policy witness share one manifest and verifier path. These artifacts
certify construction and residue rows, not primality. The same directory now
includes a deterministic search-policy atlas summarizing lane identity,
seed-origin policy, rejection geometry, survivor counts, first-accepted
distance, and Lean replay theorem links across the maintained bundle. A broader
policy-matrix exporter can sweep deterministic lane/digit surfaces and emit
certificate candidates for future Lean promotion. The current matrix includes a
complete generated-Lean 64-digit tranche across the six maintained matrix
lanes plus the promoted decimal-readable, decimal-classic, decimal-breathing,
base6-compact, base12-compact, and base30-wheel 96-digit rows. Large
replay modules stay on compact aggregate proof links, and all rows keep the
probable-prime-not-proof-certified boundary explicit.

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
- the connector lane has exact direction-independent residue filters on the
  proved modulus surface, including canonical decimal `mod 3` / `mod 9`
  exclusions and reusable pair-residue profiles beyond the canonical pair
- the canonical pair `10301` / `3007003007003` shows a narrow empirical
  directional asymmetry in the maintained zero-padded single-digit scan
  (11 forward prime hits vs 13 reverse prime hits across widths 5..7 after the
  exact residue filter); the maintained density-aware rerun keeps a residual
  gap for that pair, but broader connector-law generality remains open

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
  certificate wrappers, generated-data window-certificate entrypoints and
  bundled proof objects, a runtime export path for Lean-shaped certificate
  artifacts with a tracked cross-base catalog through wheel-like bases `30` and
  `210`, affine template structure, coprimality/radical/unit
  residue theorems,
  `ZMod`/CRT/wheel-base structure, explicit finite certificate examples, exact
  odd-only segmented sieve arithmetic, exact wheel30 candidate agreement, and
  exact connector residue filters including a reusable family-level profile API
  and maintained non-canonical profile examples. The package now also includes
  a deliberately conservative Hardy-Littlewood shell fixing pair-count
  conventions, odd-prime local-factor support, and standard logarithmic /
  coverage notation without asserting a new density theorem. The sieve lane now
  also reaches the runtime layout level: segment capacity/span constants and
  wheel30 linear byte/bit indexing are both stated exactly in Lean, together
  with the runtime cross-off start/progression, the adjusted odd collection
  window, the shared byte/bit coordinates used by the runtime writer and reader
  paths, and the exact `1 << bit` / `((byte >> bit) & 1)` mask-readback
  semantics on those coordinates, plus the bounded single-byte array update
  shell for both the odd-only and wheel30 bit layouts, together with a generic
  bounded multi-mark family on disjoint byte slots and a cleaner aggregated
  same-byte mask layer for repeated writes landing in one byte, now unified by
  a grouped per-byte plan layer, a tiny shared coordinate bridge, and the first
  odd-only and wheel30 runtime mark-family shells on top of that layer.
  The bounded-`k` theorem lane now also includes an exact compactness layer,
  finite transfer-bucket combinatorics, direct residue-profile comparison for
  lane-to-lane audits, a universal conditional transfer-collapse criterion, and
  tracked generated counterexamples showing that the strongest direct-lane
  `M = 3` collapse theorem does not yet survive on the maintained full surface.
  The detailed theorem ledger and future queue live in
  [`lean-proofs/ROADMAP.md`](lean-proofs/ROADMAP.md) and
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
- [`collab/HARDENED_RESEARCH_PROGRAMS.md`](collab/HARDENED_RESEARCH_PROGRAMS.md):
  stable collaborator-facing language for the repo's live research programs
- [`docs/PRIME_WITNESS_ENGINE.md`](docs/PRIME_WITNESS_ENGINE.md): seed-origin
  demo, large-witness ladder, confirmation tiers, and non-claims for large
  readable affine witnesses
- [`collab/TRANSFER_COLLAPSE_THEOREM_PROGRAM.md`](collab/TRANSFER_COLLAPSE_THEOREM_PROGRAM.md):
  theorem-program boundary for direct bounded-`k` lane comparisons and their
  current counterexample-pinned limit
- [`collab/CONNECTOR_SIGNAL.md`](collab/CONNECTOR_SIGNAL.md): arithmetic-first
  connector framing, bounded claim wording, and comparison protocol
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
