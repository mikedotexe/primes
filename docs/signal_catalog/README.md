# Signal Catalog

This directory contains the lightweight signal catalog:

```bash
cargo run --bin export_signal_catalog -- --out-dir docs/signal_catalog
scripts/signal_catalog.sh verify
cargo run --bin verify_signal_catalog -- --catalog docs/signal_catalog/signal_catalog.json
cargo run --bin verify_signal_catalog -- --catalog docs/signal_catalog/signal_catalog.json --deep --timeout-seconds 300
```

The catalog is a top-level index over the matched-control atlas, proof-carrying
witness atlas/manifests, connector signal atlas, and the connector width-6
stress artifact. It points to source artifacts and drift gates instead of
duplicating full report payloads.

The connector section also carries a compact digit-8 classifier-family summary:
three theorem-backed multi-modulus edge cells and zero remaining unclassified
exact separator rows in the tracked stress artifact. It also records the
outside-ladder replication screen over the next twelve twin-prime pairs: the
current verdict is partial collapse, so the finite classifiers remain bounded
stress-surface metadata rather than a connector law. The catalog also records
the split-only follow-up for trailing `00008` mod `29` / mod `31`; both rows
collapse on the next twelve twin-prime pairs, so this branch is not promoted to
a new theorem target. The branch-status picker then selects the next
non-collapsed stress family as trailing `00000006`, width `8`, digit `6`, with
status `needs-independent-replication`.

The catalog is a research-instrument index. It does not combine the underlying
surfaces into a density claim.

The default verifier is intentionally shallow: it checks that every catalog row
points at an existing artifact path and that every drift command is one of the
maintained known gates. When deeper local validation is wanted, pass `--deep`.
Deep mode runs every
catalog row's maintained drift gate with a per-row timeout and records
machine-readable `gate_results` in the optional `--json-out` report.
