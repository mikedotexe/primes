# Verification Spine

This note freezes the repo's first explicit verification spine so maintained
surfaces stop depending on source-of-truth-by-convention.

Primary references:
- [examples/README.md](/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/examples/README.md)
- [lean-proofs/README.md](/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/lean-proofs/README.md)
- [agda-proofs/README.md](/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/agda-proofs/README.md)
- [tools/verification_spine.toml](/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/tools/verification_spine.toml)

## Conventions

- Tracked formal generated outputs must live under their proof trees and be
  registered in `tools/verification_spine.toml`.
- Maintained report and visual bundles must emit into an explicit bundle
  directory such as `/tmp/primes_*` and must write `artifact_manifest.json`.
- `artifact_manifest.json` is the reproducibility sidecar for maintained
  bundles. It records:
  - `artifact_id`
  - `generator_cmd`
  - `args`
  - `upstream_inputs`
  - `expected_outputs`
- New root-level checked-in PNG/CSV/TXT analysis outputs are not allowed unless
  they are promoted into maintained status, spine-registered, and have an owner
  document.
- Historical and exploratory lanes are exempt until they are promoted into
  maintained status.

## Enforcement

- `cargo run --bin verify_verification_spine -- check`
  validates the registered spine surfaces.
- `cargo run --bin verify_verification_spine -- regenerate`
  regenerates tracked formal-generated surfaces only.
- Wrapper scripts:
  - `scripts/lean_generated_catalog.sh`
  - `scripts/lean_bounded_k_catalog.sh`
  - `scripts/agda_generated_catalog.sh`
  - `scripts/verification_spine.sh`

## Boundary

- Formal generated surfaces are hard-gated by diff-style verification.
- Maintained report and visual bundles are validated by manifest plus required
  outputs, not by byte-for-byte image diffing.
- Prime-threshold visual/report bundles are maintained surfaces once they emit
  manifests; historical image dumps are not retroactively covered.
