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
- the connector atlas now carries a bounded residual follow-up for the selected
  twin-prime target `(11,13)`: width `6` has the largest forward/reverse
  observed-to-corrected ratio gap in that target, while the exact `mod 3` /
  `mod 9` residue-survivor layer remains direction-neutral
- zooming inside width `6`, the strongest single position/digit row is
  connector `500000`, but it is reverse-only while the aggregate width-level
  gap is forward-positive; the current maintained reading is therefore
  diffuse/mixed-sign and better suited to replication or stress testing than
  immediate theorem growth
- the width-6 stress artifact keeps that selected `(11,13)` gap sign-stable
  across small-prime correction bounds `5..31`, but the small twin-prime ladder
  is sign-changing and non-monotone: `(3,5)` has the largest median absolute
  ratio gap, `(11,13)` ranks second, and the selected target still preserves the
  diffuse/mixed-sign microscope warning
- zooming into the ladder peak `(3,5)`, the aligned top connector is `900000`
  across the correction sweep; the exact `mod 3` / `mod 9` residue-null layer
  still cannot distinguish direction, and the current reading is an aligned but
  diffuse feature candidate that needs replication before theorem growth
- the v2 peak matched-control screen makes the exact boundary sharper:
  `900000` is pair-specific under nearby twin controls; only `(3,5)` among the
  six same-connector twin-pair controls is residue-admissible in both directions
  and aligned with the peak gap, so the new signal is exact admissibility
  geometry rather than a directional survivor-count advantage
- Lean now proves the corresponding exact null explanation: for any twin-prime
  pair `(p, p + 2)` with `3 < p`, decimal connectors divisible by `3` force
  both forward and reverse concatenations to be divisible by `3`; `(3,5)` is
  the exceptional small twin pair outside that theorem's `3 < p` hypothesis
- the width-6 stress JSON now exposes this as structured classifier metadata:
  `(3,5)` connector-multiple-of-`3` rows are `exceptional-small-twin-pair-mod3-lane`,
  while nearby `p > 3` twin-pair controls are
  `theorem-blocked-twin-pair-above-three-connector-multiple-of-3`
- after filtering out theorem-blocked rows, the stress picker selects the next
  non-blocked candidate as the adjacent-width `(3,5)` connector `9000000` at
  width `7`, position `0`, digit `9`
- the adjacent-width follow-up compares `90000`, `900000`, and `9000000`
  under the same exact residue guardrail: width `5` is neutral, widths `6` and
  `7` are aligned reverse-only rows, and width `7` has the strongest
  observed/corrected ratio gap
- the width-extension probe keeps `(3,5)`, position `0`, and digit `9` fixed
  while extending to widths `5..=9`: reverse-only appears at widths `6` and
  `7`, but widths `5`, `8`, and `9` are neutral, so the current signal is
  partial persistence rather than a stable all-width lane
- the leading-digit heatmap then compares digits `1..=9` at widths `6` and
  `7`: width `6` is reverse-only for digits `6`, `8`, and `9`, while width `7`
  is reverse-only only for digit `9`; because `800000` is not divisible by `3`,
  this rejects the simplest leading-multiple-of-`3` explanation, and the
  current ranking favors `width-6-row-phenomenon` before
  `digit-9-persistence-phenomenon`
- the width-6/width-7 position comparison then scans digits `6`, `8`, and `9`
  across positions: width `6` has reverse-only rows at position `0` plus
  far-edge `000006`, and width `7` has reverse-only digit-`9` rows at position
  `0` plus far-edge `0000009`; the current comparison decision is
  `width6-and-width7-both-spread-across-positions-under-exact-residue-guardrail`
- the edge-position probe compares leading and trailing edges across widths
  `5..=9` for digits `6`, `8`, and `9`: reverse-only cells appear on both
  edges, with leading hits at widths `6` and `7`, trailing hits at widths `6`,
  `7`, and `8`, and neutral rows at widths `5` and `9`; the current reading is
  `leading-and-trailing-edge-spread`, not an edge-localized mechanism
- the edge pair-replication probe repeats that scan across nearby twin-prime
  controls after excluding theorem-blocked mod-`3` rows from the ranking:
  trailing width-`8` connector `00000006` is only a singleton non-blocked
  reverse-only row, while the replicated non-blocked edge cells are digit-`8`
  lanes `800000` and `00008`
- the digit-`8` edge zoom widens the scan to the first twelve twin-prime pairs
  and adjacent widths `5..=7`: both anchor cells still replicate (`00008` for
  four non-blocked pairs and `800000` for three), and neighboring trailing
  width-`6` connector `000008` also replicates for three; this makes digit-`8`
  edge behavior the next empirical target, not a mechanism claim
- the digit-`8` residue-class profiler compares reverse-only vs non-hit rows by
  `p mod q` for small primes: all three replicated digit-`8` edge cells have
  exact separators in the bounded ladder, with current smallest candidates
  `800000` mod `17` residues `[0, 3]`, `00008` mod `19` residues
  `[6, 10, 11, 12]`, and `000008` mod `17` residues `[1, 13, 16]`
- the Lean Hardy-Littlewood shell certifies only the monotone conversion from
  supplied expected-hit `λ` values to Poisson-style coverage; it does not turn a
  connector residual into a density mechanism
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
cargo run --bin export_connector_signal_atlas -- --out-dir docs/connector
scripts/connector_signal_atlas.sh verify
```

## Exact Layer vs Open Layer

### Exact layer

This is the current proved/arithmetic surface:

- fixed-width forward and reverse concatenation formulas
- reduction modulo `m` when `base ≡ 1 (mod m)`
- canonical decimal `mod 3` / `mod 9` exclusion classes
- reusable pair-residue profiles for maintained non-canonical examples
- monotone Poisson-style coverage transforms for supplied Hardy-Littlewood
  expected-hit baselines

Primary files:

- `src/connector/mod.rs`
- `src/connector/analysis.rs`
- `lean-proofs/PrimeArithmetic/Connector/ConcatenationFilters.lean`
- `lean-proofs/PrimeArithmetic/Connector/ConcatenationFamilies.lean`
- `lean-proofs/PrimeArithmetic/Analysis/HardyLittlewoodShell.lean`

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
