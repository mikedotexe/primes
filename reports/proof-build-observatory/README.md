# Proof-Build Observatory

This directory is the default local archive root for proof-build timing runs:

```bash
scripts/proof_build_observatory.sh timing --repeat 3
```

Each run writes raw timing JSONs plus a normalized local performance atlas under
`reports/proof-build-observatory/<utc-run-id>/`. These artifacts are
machine-specific and cache-aware; use them to choose engineering work such as
module splitting, generated-proof compression, or Lake target partitioning, not
as tracked benchmark claims.

The run directories are intentionally ignored by git. Keep only curated notes if
you need to preserve a conclusion from a local timing sweep.
