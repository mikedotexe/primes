# Collaborator Reference

**Updated**: March 2026

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
