# Agda Proof Workspace

This directory is the active Agda workspace for formalization experiments and
partial machine-checked proofs related to the repository.

## Canonical Status

[`STATUS.md`](STATUS.md) is the audited module-count surface in this tree. It
tracks the current `clean-local / with-local-postulates / failing` split and
calls out any current clean-local boundary cases explicitly.

Current maintained clean-local boundary cases: none known. Outside the
maintained clean spine, the certification lane is now also locally clean:
[`Theorems/Abstract/SymmetryFiniteReflect.agda`](Theorems/Abstract/SymmetryFiniteReflect.agda)
is clean-local, the standard even-base choice `mid = base / 2` has a
constructive observed fixed-point classifier, and the generic wrappers now
consume an explicit `ObservedFixedPointClassifier` contract instead of relying
on a hidden half-turn shell. Meanwhile,
[`Theorems/Abstract/BucketsAutoMatch.agda`](Theorems/Abstract/BucketsAutoMatch.agda)
and [`Theorems/Abstract/WindowCertificate.agda`](Theorems/Abstract/WindowCertificate.agda)
are now clean-local. In the concrete `countResid` path used there, the
support-count bridge is discharged constructively, the direct imported
involutive, roundtrip, and transport theorems are gone, and that concrete path
no longer depends on any imported auto-pairing theorem. The dynamic side is
also sharper now: `WindowCertificate` and the dual wrapper consume the smaller
`PointwiseSafe` contract, with `StableOrbital` derived internally.
`ConstrainedOrbitals.agda` now also exposes maintained smart constructors for
building `PointwiseSafe` incrementally:
`pointwiseSafeNil`, `pointwiseSafeCons`, `pointwiseSafeSingleton`, and
`pointwiseSafeFromAll`.

Do not infer proof coverage from filenames or older narrative docs alone.

## Active Entry Points

- [`STATUS.md`](STATUS.md): audited clean/postulated/failing lists
- [`RESIDUE_FOLD_README.md`](RESIDUE_FOLD_README.md): focused note on the
  residue-fold and CRT area
- [`Tests/TESTING_STRATEGY.md`](Tests/TESTING_STRATEGY.md): current executable
  proof-check posture
- [`Theorems/README.md`](Theorems/README.md): theorem-area orientation
- [`LagrangePoints/README.md`](LagrangePoints/README.md): cautious status note
  for the connector/Lagrange subtree
- [`SIGNAL_MAP.md`](SIGNAL_MAP.md): strongest Agda areas and next repair targets
- [`AGDA_RESOURCES.md`](AGDA_RESOURCES.md): external reference catalog
- [`FIX_IMPORTS.md`](FIX_IMPORTS.md): import-compatibility troubleshooting

## Historical Material

Older sprint plans, theory pitches, and session narratives that no longer meet
the active-doc standard now live under
[`../archive/agda-proofs/`](../archive/agda-proofs/).

## Expectations

- Many `.agda` files in this tree are exploratory or partially formalized.
- "Clean" in this subtree means no local postulates in that file; it does not
  automatically mean the full transitive dependency chain is postulate-free.
- [`STATUS.md`](STATUS.md) is where that boundary is tracked explicitly.
- Public repo claims should be taken from [`../CLAIMS.md`](../CLAIMS.md),
  [`../EVIDENCE.md`](../EVIDENCE.md), and [`STATUS.md`](STATUS.md), not from
  aspirational theorem names or archived notes.
- The extracted certification sketches under [`Examples/`](Examples/) are
  expository only; for the dynamic lane they should be read as
  `PointwiseSafe`-first notes, not as instructions to construct raw
  `StableOrbital` witnesses by hand.
- The active dynamic regression shell in
  [`Tests/InvariantTests.agda`](Tests/InvariantTests.agda) is intentionally
  two-sided: one helper-path `PointwiseSafe` witness and one helper-agnostic
  negative `InZone` counterexample.

## Local Usage

If Agda and the standard library are installed locally, a reasonable first
check is:

```bash
cd agda-proofs
./scripts/verify-clean-spine.sh
```

The helper resolves Agda in this order: `AGDA_BIN`, `agda` on `PATH`,
`/opt/homebrew/bin/agda`, `/usr/local/bin/agda`. If your local install lives
elsewhere, run it as:

```bash
cd agda-proofs
AGDA_BIN=/path/to/agda ./scripts/verify-clean-spine.sh
```

For targeted follow-up, use the clean/postulated module lists in [`STATUS.md`](STATUS.md)
rather than trying to type-check the entire tree at once.
