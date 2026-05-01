# Signal Spine Reports

Raw signal-spine runs under this directory are generated artifacts. They are
useful for local inspection, but they are not the source of truth and should not
be committed wholesale.

## Policy

- `reports/signal-spine/<run-id>/` raw bundles are ignored by git.
- Curated snapshots live under `reports/signal-spine/curated/`.
- A curated snapshot keeps only the top-level summary and a compact manifest.
- Raw stdout logs, CSVs, JSON report bundles, PNGs, and intermediate artifacts
  remain reproducible from the manifest and are intentionally left untracked.

## Reproduce

Run the full local spine:

```bash
SIGNAL_SPINE_RUN_ID=post-commit-full \
scripts/signal_spine.sh --all
```

Run the bounded CI-style smoke:

```bash
SIGNAL_SPINE_RUN_ID=ci-smoke-local \
SIGNAL_SPINE_OUT_DIR=/tmp/primes_signal_spine_smoke \
scripts/signal_spine.sh core affine fast-generation base57-codec
```

Promote the latest local run to the curated snapshot:

```bash
scripts/curate_signal_spine_snapshot.py \
  --run-dir reports/signal-spine/post-commit-full
```

## Promotion Rule

Promote a snapshot when it documents a meaningful checkpoint in the research
stack: a post-commit full-spine run, a release candidate, or a report bundle
whose summary will help future collaborators understand the current signal
surface.
