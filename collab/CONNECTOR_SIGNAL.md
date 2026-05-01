# Connector Signal Guide

**Updated**: April 2026
**Purpose**: arithmetic-first framing for the maintained connector/Lagrange lane

## Summary

The durable connector signal in this repository is not the gravity metaphor. It
is the arithmetic of fixed-prime concatenation together with narrow empirical
scans on bounded connector families.

Current framing:

- the canonical pair `10301` / `3007003007003` remains a real empirical object
  of study
- exact residue filters are direction-independent on the proved modulus lane
  (`mod 3` and `mod 9` in base 10 for the canonical pair, and the same family
  pattern for maintained non-canonical examples)
- in the matched width-5..7 zero-padded single-digit scan, the canonical pair
  currently yields `11` forward prime hits and `13` reverse prime hits after
  the exact residue filter
- the maintained density-aware rerun now also compares those hit counts against
  `sum 1 / ln(n)` and an exact small-prime conditioning layer; the canonical
  residual gap survives that correction, but it still does not generalize across
  the maintained comparison family
- any broader connector law remains open until it survives matched,
  same-budget comparison outside the canonical pair

## Standard Language

Use these arithmetic-first terms in preference to metaphor language:

- `ConnectorHit = (pair, width, position, digit, direction)`
- `ResidueAdmissible`: survives the exact small-modulus exclusion layer
- `ResonancePosition`: a width/position bucket with multiple working digits in
  the same matched scan
- `DirectionalAsymmetry`: forward/reverse difference after the residue layer is
  accounted for

Allowed historical alias:

- "Lagrange point" as repository shorthand for a productive insertion position

Avoid:

- presenting "Lagrange point" as a proved equilibrium law
- treating the simulation-oriented `src/lagrange.rs` module as mathematical
  evidence

## Canonical Source Case

The maintained canonical source case is the union of:

1. The Agda width-5 hits:
   - `(width 5, position 1, digit 6)`
   - `(width 5, position 4, digit 6)`
2. The maintained forward connector hits across widths `5..7`:
   - `(5, 4, 6)`
   - `(6, 1, 6)`
   - `(6, 4, 6)`
   - `(7, 3, 6)`

The width-5 shell cases should be treated as preserved source material. The
current Rust matched scan re-verifies the forward set above and keeps the extra
shell case visible without silently upgrading it into a stable claim.

Reproduce them with:

```bash
cargo run --example connector_utility_demo
cargo run --example connector_signal_report
cargo run --example connector_signal_report -- --json-out connector_signal.json --csv-out connector_signal_positions.csv
```

## Exact Layer vs Open Layer

### Exact layer

This is the current proved/arithmetic surface:

- fixed-width forward and reverse concatenation formulas
- reduction modulo `m` when `base ≡ 1 (mod m)`
- canonical decimal `mod 3` / `mod 9` exclusion classes
- reusable pair-residue profiles for maintained non-canonical examples

Primary files:

- `src/connector/mod.rs`
- `src/connector/analysis.rs`
- `lean-proofs/PrimeArithmetic/Connector/ConcatenationFilters.lean`
- `lean-proofs/PrimeArithmetic/Connector/ConcatenationFamilies.lean`

### Open layer

These remain open and should be framed as empirical or heuristic:

- whether the canonical forward/reverse gap extends beyond the canonical pair
- whether any residual asymmetry survives after size and small-prime baselines
  are matched across pair families
- whether any resonance pattern generalizes across same-budget scans
- whether a broader connector law survives matched comparison after the exact
  residue layer is removed

## Comparison Protocol

Use this protocol before promoting any broader connector-law claim:

1. Keep the family fixed: base 10, fixed pair, zero-padded single-digit scan.
2. Match the width/position/digit budget across pairs.
3. Record raw candidates, residue-admissible candidates, prime hits, post-filter
   prime rates, `sum 1 / ln(n)` expectations, small-prime corrected expected
   hits, residual ratios, and position-level multiplicity.
4. Export the JSON/CSV bundle when doing a serious rerun so the residual buckets
   can be inspected directly instead of summarized from memory.
5. Accept a broader-law candidate only if it survives comparison on maintained
   non-canonical pairs with the same sign and same qualitative pattern after
   the density-aware audit.
6. Otherwise downgrade it to an open heuristic.

The maintained first-pass comparison family is:

- canonical pair `10301 ∘ 3007003007003`
- zero-padded membrane pair `10301 ∘ 30305070305070303`
- twin pair `11 ∘ 13`
- Sophie Germain pair `23 ∘ 47`

## Claim Guardrails

Good claim wording:

- "The canonical pair shows a narrow empirical directional asymmetry in the
  matched width-5..7 zero-padded single-digit scan."
- "The canonical residual gap is worth tracking, but a general connector law
  does not survive the current density-aware comparison family."
- "The exact residue filters are direction-independent on the proved modulus
  lane."
- "The broader connector-law question remains open."

Bad claim wording:

- "There is a proved Lagrange law for prime pairs."
- "The connector mechanism is already general."
- "The gravity model explains the arithmetic effect."
