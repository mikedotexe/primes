# Collaborator Reference

**Updated**: April 2026

This directory contains collaborator-facing synthesis notes. Treat the root
documents and audited status files as the primary source of truth. Use this
folder for technical summaries, theorem-planning notes, and exploratory
follow-up that has not yet been promoted into the canonical claim surface.

## Start Here

### Best current interpretation

[`THEORETICAL_CLOSURE.md`](./THEORETICAL_CLOSURE.md)

This is the collaborator summary of the current best interpretation: prime
density gains in the symmetric digit-template family are best explained by
coprimality filtering and ordinary prime-density effects, not by a demonstrated
template-specific bonus.

### Stable research-language anchor

[`HARDENED_RESEARCH_PROGRAMS.md`](./HARDENED_RESEARCH_PROGRAMS.md)

This note keeps the repo's five strongest live research programs in stable
dual-register language: sober maintained wording plus a constructive steelman
for collaborator use, together with a practical signal ladder showing which
lanes currently have the most leverage.

### Exact hinge explanation boundary

[`HINGE_DISCRIMINATOR.md`](./HINGE_DISCRIMINATOR.md)

This note freezes the exact target of the hinge-discriminator work: what counts
as a real finite discriminator, which tautological shortcuts are banned from
the search, and how the species labels should stay downstream of the exact rule
layer.

### Atom-family depth ladder

[`HINGE_ATOM_FAMILIES.md`](./HINGE_ATOM_FAMILIES.md)

This note classifies the deterministic hinge atom families by explanatory
depth, theorem proximity, and bridge-vs-diagnostic status so the overlap /
boundary language stays grounded while the geometry and carry-through language
remain useful but properly bounded.

### Robustness matrix boundary

[`HINGE_ROBUSTNESS.md`](./HINGE_ROBUSTNESS.md)

This note records which parts of the hinge atom-family ladder actually survive
data-surface and threshold-vocabulary perturbations, with family-ladder
stability treated as the main success criterion rather than exact rule-string
immutability.

### Transfer-collapse theorem boundary

[`TRANSFER_COLLAPSE_THEOREM_PROGRAM.md`](./TRANSFER_COLLAPSE_THEOREM_PROGRAM.md)

This note freezes the new direct lane-comparison theorem program: the exact
three-rung ladder, the universal conditional criterion, the matched `2p` and
wheel-class wrappers, and the current negative boundary showing that the full
maintained `M = 3` surface does not yet support a positive public theorem.

### Affine hinge classifier boundary

[`AFFINE_HINGE_CLASSIFIER.md`](./AFFINE_HINGE_CLASSIFIER.md)

This note freezes the boundary for the affine hinge atlas: local shift /
gradient / zero-seed comparison as classifier exploration, Lean as the primary
theorem engine, and Agda as a concept mirror rather than a parity-forcing proof
target in the first tranche.

### Affine period-lock species boundary

[`AFFINE_PERIOD_LOCK_SPECIES.md`](./AFFINE_PERIOD_LOCK_SPECIES.md)

This note records the next affine decomposition layer: period lock as the exact
answer to where gradient agreement can occur, locked shift residuals as the
exact answer to what survives inside that regime, and the current maintained
reading that the meaningful `M = 2` winners are low-order while the base-22
higher-order story survives only as a direct-lane side pocket.

### Affine core visual intuition

[`AFFINE_CORE_VISUAL_INTUITION.md`](./AFFINE_CORE_VISUAL_INTUITION.md)

This note freezes the human-facing explanation of the affine core:
fixed templates become `N(s)=A+G*s`, residue filters are exact local affine
constraints, the residue torus visualizes multiplicative-order period lock,
and prime witnesses remain construction examples rather than density proofs.

### Prime generation external comparison

[`PRIME_GENERATION_EXTERNAL_COMPARISON.md`](./PRIME_GENERATION_EXTERNAL_COMPARISON.md)

This note anchors the fast-generation comparison frame against primesieve,
GMP, OpenSSL, CUDASieve, and GIMPS-style special-form systems, with the main
boundary that our maintained path searches structured affine lanes rather than
general intervals or cryptographic random primes.

### Strongest repo-wide evidence docs

| Document | Purpose |
|----------|---------|
| [../CLAIMS.md](../CLAIMS.md) | Claim-to-evidence registry |
| [../NOVELTY.md](../NOVELTY.md) | Honest novelty assessment |
| [../VERIFIED_FACTS_VS_SPECULATION.md](../VERIFIED_FACTS_VS_SPECULATION.md) | Audited fact/speculation split |
| [../EVIDENCE.md](../EVIDENCE.md) | Corrected empirical tables |
| [../agda-proofs/STATUS.md](../agda-proofs/STATUS.md) | Current Agda compilation status |

## Other Notes in This Folder

Read these as working notes unless another audited document has already
promoted the relevant claim:

- [`openprover/README.md`](./openprover/README.md)
- [`PRIMORIAL_MEMBRANE_OPTIMIZATION_GUIDE.md`](./PRIMORIAL_MEMBRANE_OPTIMIZATION_GUIDE.md)
- [`EXPLORATION_SYNTHESIS.md`](./EXPLORATION_SYNTHESIS.md)
- [`PERIOD6_RESONANCE_DISCOVERY.md`](./PERIOD6_RESONANCE_DISCOVERY.md)

## Quick Verification

```bash
cargo test --lib
cargo run --example prime_count_smoke_test
cargo run --example proper_membrane_generator
cargo run --example prime_verification_report
```

## Current Working Assumptions

1. Symmetric digit templates (repo alias: membranes) can achieve high measured
   prime density in selected configurations.
2. Coprimality is the dominant explanatory factor currently supported by the
   repo's evidence.
3. Claims about connector asymmetry, Lagrange behavior, and resonance effects
   should be treated as narrower than the main density interpretation.
4. The Agda work is partial but much broader than older summaries suggest:
   `agda-proofs/STATUS.md` currently reports 40 clean-local modules, 41 modules
   with local postulates, and 0 failing modules.
