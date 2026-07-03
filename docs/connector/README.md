# Connector Signal Atlas

This directory contains the deterministic connector signal atlas:

```bash
cargo run --bin export_connector_signal_atlas -- --out-dir docs/connector
scripts/connector_signal_atlas.sh verify
```

The atlas indexes maintained same-budget connector scans, exact residue-filter
Lean links, and the conservative Hardy-Littlewood coverage-transform guardrail.
It is an empirical/proof-catalog surface, not a connector law and not evidence
of a new prime-density mechanism.

The same connector drift gate also verifies the tracked width-6 stress artifact
and checks any finite classifier theorem links named by its JSON rows.
It also regenerates `connector_replication_null_atlas.json`, a standalone
branch-accounting surface that records which source/fresh/profile/separator
chains split or collapsed before theorem promotion.

The residual follow-up block includes a tiny width-contrast micro-atlas for the
currently selected `(11,13)` target. It compares the selected width-6 empirical
residual contrast against the theorem-backed residue-survivor null layer, and
records the next target as a bounded follow-up experiment rather than a new
mechanism claim. The nested position/digit picker currently classifies that
width-6 contrast as diffuse and mixed-sign, so the maintained next step is
replication/stress testing before adding a new connector theorem.

The width-6 stress runner makes that next step reproducible:

```bash
cargo run --bin export_connector_width6_stress -- --out-dir docs/connector
```

It sweeps small-prime correction bounds across a small twin-prime ladder. The
current tracked stress artifact keeps the `(11,13)` width-6 gap sign-stable
across the correction sweep, but `(11,13)` is not the ladder peak: `(3,5)` has
the largest median absolute gap, signs change across the ladder, and the
position/digit microscope for `(11,13)` remains diffuse/mixed-sign rather than
a single aligned exact-feature candidate. The nested peak follow-up now zooms
into `(3,5)` directly: connector `900000` is aligned with the peak direction
across all correction bounds, while the exact `mod 3` / `mod 9` residue-null
layer still cannot distinguish direction. Because the signal remains diffuse
rather than concentrated in a single exact row, the maintained decision is
replication/feature scanning before theorem growth. The v2 matched-control
screen broadens that check across nearby twin-pair controls, adjacent widths,
and leading-digit/position variants. Its current decision is
`peak-alignment-is-pair-specific-under-nearby-twin-controls`: among six
same-connector twin-pair controls, only `(3,5)` / `900000` is residue-admissible
in both directions and aligned with the peak gap. That is a real exact
admissibility clue, but not a directional prime-density mechanism. The Lean
connector examples now prove the null explanation: for every twin-prime pair
`(p, p + 2)` with `3 < p`, decimal connectors divisible by `3` make both
forward and reverse concatenations divisible by `3`. The stress JSON now carries
that explanation as structured row metadata: `(3,5)` rows are labeled
`exceptional-small-twin-pair-mod3-lane`, and nearby `p > 3` twin-pair controls
with connector multiples of `3` are labeled theorem-blocked. The same screen
also picks the next non-blocked follow-up automatically after filtering out
theorem-blocked rows: the current selected candidate is the adjacent-width
`(3,5)` connector `9000000` at width `7`, position `0`, digit `9`. The nested
adjacent-width follow-up compares `90000`, `900000`, and `9000000` directly:
width `5` is neutral, widths `6` and `7` are aligned reverse-only rows, and
width `7` is the strongest observed/corrected ratio-gap row under the exact
residue guardrail. The width-extension probe keeps `(3,5)`, position `0`, and
digit `9` fixed while extending to widths `5..=9`: reverse-only appears at
widths `6` and `7`, while widths `5`, `8`, and `9` are neutral under the same
guardrail. The maintained reading is therefore partial persistence, not a
stable all-width lane. A follow-on leading-digit heatmap compares digits
`1..=9` at widths `6` and `7`: width `6` is reverse-only for digits `6`, `8`,
and `9`, while width `7` is reverse-only only for digit `9`. Since `800000` is
not a connector multiple of `3`, this bounded probe rejects the simplest
leading-multiple-of-3 explanation. The tracked ranking currently favors
`width-6-row-phenomenon`, followed by `digit-9-persistence-phenomenon`; sparse
prime-hit noise remains visible because six digits have no reverse-only hit at
either width. The position/digit probes then scan digits `6`, `8`, and `9`
across width `6` positions `0..5` and width `7` positions `0..6`. Width `6`
has reverse-only rows at position `0` plus the far-edge `000006`; width `7`
has reverse-only digit-`9` rows at position `0` plus the far-edge `0000009`.
The side-by-side comparison therefore favors
`width6-and-width7-both-spread-across-positions-under-exact-residue-guardrail`
over a position-0-localized exact-feature candidate. The edge-position probe
then compares only leading and trailing edges across widths `5..=9` for digits
`6`, `8`, and `9`: reverse-only cells appear on both edges, with leading hits
at widths `6` and `7`, trailing hits at widths `6`, `7`, and `8`, and neutral
rows at widths `5` and `9`. The maintained interpretation is therefore
`leading-and-trailing-edge-spread` under the exact residue guardrail, not an
edge-localized mechanism. The follow-on pair-replication probe repeats that
edge scan across the nearby twin-prime ladder after excluding theorem-blocked
mod-3 rows from the ranking. The width-8 trailing connector `00000006` is a
singleton non-blocked reverse-only row, not replicated evidence. The replicated
non-blocked edge cells are digit-8 lanes instead: leading width `6` connector
`800000` and trailing width `5` connector `00008`, each reverse-only for two
nearby twin-pair controls. The digit-8 edge zoom widens that check to the first
twelve twin-prime pairs and adjacent widths `5..=7`. Both anchor cells continue
to replicate: `00008` is reverse-only for four non-blocked pairs and `800000`
for three. A neighboring trailing width-6 cell, `000008`, also replicates for
three pairs, while leading width `7` is neutral. This keeps digit-8 edge
behavior alive as an empirical follow-up target, not as a mechanism claim. The
residue-class profiler then compares reverse-only vs non-hit pairs by `p mod q`
for small primes `q`: every replicated digit-8 edge cell has at least one exact
small-prime separator in the bounded ladder. The smallest current candidates
are mod `17` for `800000` with reverse-only residues `[0, 3]`, mod `19` for
`00008` with residues `[6, 10, 11, 12]`, and mod `17` for `000008` with
residues `[1, 13, 16]`. These are theorem-candidate filters, not yet connector
laws. The v8 stress artifact also tests the three theorem-backed digit-8
classifier cells on the next twelve twin-prime pairs. That outside-ladder
replication result is partial collapse: all three cells collapse at least
somewhere outside the original bounded ladder, while the trailing width-5 cell
splits cleanly at two moduli. The split-only follow-up then tests those two
trailing `00008` mod `29` / mod `31` rows on the next twelve twin-prime pairs;
both collapse with no reverse-only hits. That is useful falsification signal,
not a general connector rule. The branch-status picker now stops that digit-8
branch and selects the next non-collapsed stress target as a singleton requiring
independent replication: trailing edge width `8`, digit `6`, connector
`00000006`.

The `verify` gate also generates a temporary Lean import-check surface from
`proof_links[*].lean_module` and runs it through Lake, so proof-link module
renames fail the connector atlas gate instead of becoming stale JSON strings.

The human console report remains:

```bash
cargo run --example connector_signal_report
```
