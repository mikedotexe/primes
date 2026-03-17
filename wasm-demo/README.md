# WebAssembly Demo

This directory preserves an older browser demo for the repository.

## Current Status

- The static assets under `www/` are still useful for inspection.
- The Rust crate in this directory is not part of the active workspace hardening
  path.
- A March 2026 check showed that `cargo check` from `wasm-demo/` currently fails
  because the package is not integrated cleanly with the root workspace.

Treat this directory as an experimental prototype, not a current supported build
target.

## What Is Here

- `src/`: older WASM bindings and demo logic
- `www/`: static demo assets
- `build.sh`: historical helper script

## Local Inspection

If you only want to inspect the existing static demo assets:

```bash
cd wasm-demo/www
python3 -m http.server 8080
```

Then open `http://localhost:8080`.

## If You Need Current Claims

Use the active repo docs instead:

- [`../README.md`](../README.md)
- [`../CLAIMS.md`](../CLAIMS.md)
- [`../EVIDENCE.md`](../EVIDENCE.md)

The older detailed demo writeup is preserved at
[`../archive/wasm-demo/README_legacy.md`](../archive/wasm-demo/README_legacy.md).
