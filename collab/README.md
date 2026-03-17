# Collaborator Reference

**Updated**: March 2026

This directory contains collaborator-facing synthesis notes. Treat the root
documents and audited status files as the primary source of truth, and use this
folder for interpretation, working summaries, and exploratory follow-up.

## Start Here

### Best current interpretation

[`THEORETICAL_CLOSURE.md`](./THEORETICAL_CLOSURE.md)

This is the collaborator summary of the repo's current best interpretation:
membrane density gains are most plausibly explained by coprimality filtering and
ordinary prime-density effects, not by a demonstrated membrane-specific bonus.

### Strongest repo-wide evidence docs

| Document | Purpose |
|----------|---------|
| [../CLAIMS.md](../CLAIMS.md) | Claim-to-evidence registry |
| [../NOVELTY.md](../NOVELTY.md) | Honest novelty assessment |
| [../VERIFIED_FACTS_VS_SPECULATION.md](../VERIFIED_FACTS_VS_SPECULATION.md) | Audited fact/speculation split |
| [../EVIDENCE.md](../EVIDENCE.md) | Corrected empirical tables |
| [../agda-proofs/STATUS.md](../agda-proofs/STATUS.md) | Current Agda compilation status |

## Other Notes in This Folder

Read these as exploratory or heuristic unless another audited document has
already promoted the claim:

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

1. Membrane constructions can achieve high prime density in selected measured
   configurations.
2. Coprimality is the dominant explanatory factor currently supported by the
   repo's evidence.
3. Claims about connector asymmetry, Lagrange behavior, and resonance effects
   should be treated as narrower than the membrane-density story.
4. The Agda work is partial: 20 clean modules and 12 postulated modules
   currently type-check.
