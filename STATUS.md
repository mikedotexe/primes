# Repository Status

**Updated**: March 2026  
**Purpose**: canonical snapshot of repo-level verification status

## Current Verified Checks

| Check | Current result | Verification |
|------|----------------|--------------|
| Library tests | 174 passed | `cargo test --lib` |
| Clippy | passes cleanly | `cargo clippy --lib -- -D warnings` |
| Top-level examples | 34 compile | `for f in examples/*.rs; do cargo build --example "$(basename "$f" .rs)"; done` |
| Agda status | 40 clean-local, 41 with local postulates, 0 failing | [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) |

## Notes

- This file is the canonical source for mutable repo-level counts used across
  active docs.
- Agda counts here summarize the file-local categories from
  [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md); use that file for boundary
  notes about clean-local modules that sit atop postulated foundations when any
  exist.
- Public claim wording still lives in [`CLAIMS.md`](CLAIMS.md),
  [`README.md`](README.md), and [`EVIDENCE.md`](EVIDENCE.md).
