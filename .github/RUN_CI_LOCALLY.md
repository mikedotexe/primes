# Run CI Checks Before Pushing

## Automatic (Git Hook) ✨

**The hook is already set up!** It runs automatically before every `git push`.

When you push, it will:
1. Run all CI checks
2. Stop the push if anything fails
3. Let you fix issues before pushing

**To skip the check** (not recommended):
```bash
git push --no-verify
```

## Manual (Quick Script)

```bash
./scripts/quick-ci.sh
```

Runs in ~2-5 minutes and checks:
- ✅ Formatting (`cargo fmt`)
- ✅ Linting (`cargo clippy`)
- ✅ Build (`cargo build`)
- ✅ Tests (`cargo test`)

## Just Format + Lint (Super Fast)

```bash
cargo fmt && cargo clippy -- -D warnings
```

Takes ~30 seconds. Catches most issues.

## What Each Check Does

### Format Check
```bash
cargo fmt -- --check  # Check
cargo fmt             # Fix
```

Ensures code follows Rust style guidelines.

### Clippy
```bash
cargo clippy -- -D warnings
```

Catches common mistakes and bad patterns.

### Build
```bash
cargo build
```

Verifies code compiles.

### Tests
```bash
cargo test
```

Runs all tests to verify behavior.

## Workflow

**Before committing:**
```bash
cargo fmt  # Auto-format
```

**Before pushing:**
```bash
# Automatic via git hook, or manually:
./scripts/quick-ci.sh
```

**If you're in a hurry:**
```bash
# Just the essentials
cargo fmt && cargo clippy -- -D warnings
```

## Bypassing the Hook

Sometimes you need to push without checks (WIP branches, etc.):

```bash
git push --no-verify
```

**Note**: CI will still run on GitHub, so broken code will be caught.

## Testing WASM/Examples Locally

**WASM:**
```bash
cargo build --target wasm32-unknown-unknown --no-default-features --features wasm
```

**Examples:**
```bash
cargo check --example proper_membrane_generator
cargo check --example lagrange_verification
cargo check --example check_prime
```

## The Complete CI Simulation

Want to run EVERYTHING that CI runs?

```bash
# Test suite
cargo fmt -- --check
cargo clippy -- -D warnings
cargo build --verbose
cargo test --verbose

# WASM
cargo build --target wasm32-unknown-unknown --no-default-features --features wasm

# Examples
cargo check --example proper_membrane_generator
cargo check --example lagrange_verification
cargo check --example check_prime
cargo check --example prime_count_smoke_test
cargo check --example statistical_prime_generator
```

Or just use the pre-flight script:
```bash
./scripts/ci-preflight.sh  # More comprehensive
```

## Summary

**Fastest:** `cargo fmt && cargo clippy -- -D warnings` (~30s)
**Recommended:** `./scripts/quick-ci.sh` (~2-5 min)
**Automatic:** Git pre-push hook (already set up!)
**Complete:** `./scripts/ci-preflight.sh` (~5-10 min)

---

**The git hook is active** - CI checks run automatically before every push! 🎉
