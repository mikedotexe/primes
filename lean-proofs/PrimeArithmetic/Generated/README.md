# Generated Lean Artifacts

This directory is the landing zone for runtime-exported Lean artifacts that
instantiate the maintained window-certificate shell.

Layout:

- `Examples/`: tracked sample artifacts that are kept in the repo and checked
  as part of the Lean package
- `Runtime/`: default output location for ad hoc exported artifacts generated
  from the Rust runtime side

Current tracked examples:

- `Examples/WindowP3Base6Span5.lean`
- `Examples/WindowP5Base10Span5.lean`
- `Examples/WindowP5Base12Span17.lean`
- `Examples/WindowP11Base30Span5.lean`
- `Examples/WindowP101Base30Span29.lean`
- `Examples/WindowP163Base30Span35.lean`
- `Examples/WindowP41Base210Span5.lean`

Catalog workflow:

- `./scripts/lean_generated_catalog.sh verify`: regenerate each tracked example
  in place, compare it to a backup, and restore the tracked file on success
- `./scripts/lean_generated_catalog.sh regenerate`: rewrite the tracked example
  files in place from the exporter

The canonical exporter is the Rust binary:

```bash
cargo run --bin export_window_certificate -- \
  --p 5 \
  --base 10 \
  --window-span 5 \
  --exclude-radius 1 \
  --out lean-proofs/PrimeArithmetic/Generated/Examples/WindowP5Base10Span5.lean
```

This writes a Lean module under `PrimeArithmetic/Generated/...` that:

- defines a concrete `GeneratedWindowPayload`
- derives support counts automatically
- supplies balanced-count, fixed-point-exclusion, and pointwise-safety evidence
- rebuilds a `GeneratedDualEvidence` bundle and the corresponding dual
  certificate

Check the resulting artifact locally with:

```bash
cd lean-proofs
lake env lean PrimeArithmetic/Generated/Examples/WindowP5Base10Span5.lean
```

Verify the whole tracked catalog from the repo root with:

```bash
./scripts/lean_generated_catalog.sh verify
```
