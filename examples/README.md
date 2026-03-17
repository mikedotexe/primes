# Examples

All curated top-level examples compile and are maintained. Run any example with:

```bash
cargo run --example <name>
# or for performance-sensitive examples:
cargo run --release --example <name>
```

## Quick Start (5 minutes)

```bash
cargo run --example prime_count_smoke_test      # Validate sieve against OEIS
cargo run --example proper_membrane_generator   # Generate membrane primes
cargo run --example lagrange_full_verification  # See concatenated prime Lagrange points
cargo run --example statistical_prime_generator # Statistical prime generation
cargo run --example prime_verification_report   # Verify all documented claims
```

## Verification and Core Tools

| Example | Description |
|---------|-------------|
| `check_prime` | Simple CLI prime checker (reads from stdin) |
| `prime_count_smoke_test` | Deterministic prime-counting tests against OEIS A000720 |
| `prime_verification_report` | Verification report for all documented membrane primes |
| `verify_prime_checker` | Validates the Miller-Rabin checker against known primes and composites |

## Membrane Generation

| Example | Description |
|---------|-------------|
| `comprehensive_base_analysis` | Systematic membrane config testing across multiple bases |
| `membrane_palindrome_probe` | Exact structure probe: palindrome overlap and zero-layout symmetry |
| `membrane_scaffold_probe` | Exact centered-scaffold vs same-budget control probe |
| `membrane_showcase` | Demonstration of membrane prime patterns across different bases |
| `membrane_vs_random` | Compares membrane-structured numbers against random coprime numbers |
| `membrane_vs_random_fast` | Fast version of membrane-vs-random using base 30 |
| `proper_membrane_generator` | Deterministic membrane generator using seeds (not random search) |
| `solution_space_explorer` | Systematic parameter space mapping (base, M, k_outer, k_inner) |
| `statistical_prime_factory` | Production-ready prime generator using verified membrane patterns |
| `statistical_prime_generator` | Statistical prime generator using empirically-derived patterns |
| `statistical_sampling_demo` | Demonstrates proper statistical sampling of membrane configurations |

## Lagrange Points and Connectors

| Example | Description |
|---------|-------------|
| `connector_utility_demo` | Demonstrates the connector concatenation API |
| `lagrange_clustering_verifier` | Verifies prime clustering around Lagrange points between prime pairs |
| `lagrange_full_verification` | Verifies entire concatenated strings for primality |
| `lagrange_mechanics` | Explores Lagrange point mechanics with position/digit analysis |
| `lagrange_verification` | Tests Lagrange point insertions between concatenated primes |
| `scan_connectors` | CLI tool to discover prime connectors between two primes |

## Hardy-Littlewood and Statistical Analysis

| Example | Description |
|---------|-------------|
| `babylonian_prime_orthogonality` | Demonstrates orthogonality of human-convenient vs prime-harmonic metrics |
| `empirical_verification_pipeline` | Joins sample/model CSVs and runs verification pipeline |
| `hardy_littlewood_validation` | Computes HL singular series and compares with empirical observations |
| `harmonic_lagrange_explorer` | Polynomial fitting for harmonic Lagrange lineout data |
| `harmonic_overtones_explorer` | Overtone spectrum computation from sample/model data |
| `orthogonality_verification` | Tests independence of spectral regularity and phase lock density |
| `prime_gap_analysis` | Prime gap distributions in coordinate constellations |
| `symmetry_breaking_explorer` | Ridge/trough analysis of symmetry-breaking patterns |

## Interactive TUI Applications

These require a terminal (will show "Device not configured" if run without one).

| Example | Description |
|---------|-------------|
| `lagrange_tui_demo` | Research-grade TUI for exploring prime connectors |
| `membrane_lab_tui` | Interactive membrane laboratory with real-time parameter tuning |
| `prime_atom_tui` | Visualizes membrane primes as atomic structures |

## Special-Purpose Tools

| Example | Description |
|---------|-------------|
| `belphegor_scanner` | Palindromic prime scanner inspired by Numberphile |
| `sandwich_prime_finder` | Finds primes with 1[zeros]meatball[zeros]1 structure |

## Historical Examples

174 exploration scripts, hypothesis tests, and one-off investigations have been
moved to `historical/examples/`. This includes the former `examples/verified/`
(25 files, 24 broken) and `examples/experimental/` (7 files, 4 broken)
subdirectories, whose compiling members were duplicates of top-level examples.
Historical examples are preserved for reference but are not maintained.
