# GitHub CI Testing Guide

This guide covers different approaches to testing GitHub Actions workflows before pushing to the repository.

## Table of Contents

1. [Quick Validation](#quick-validation)
2. [Manual Workflow Triggering](#manual-workflow-triggering)
3. [Local Testing with Act](#local-testing-with-act)
4. [Pre-Flight Checks](#pre-flight-checks)

---

## Quick Validation

### Validate YAML Syntax

Before pushing, ensure your workflow YAML is valid:

```bash
# Install yamllint (if not already installed)
# macOS: brew install yamllint
# Ubuntu: apt-get install yamllint

# Validate all workflows
yamllint .github/workflows/*.yml

# Or use a simple YAML parser
python3 -c "import yaml, sys; yaml.safe_load(open(sys.argv[1]))" .github/workflows/ci.yml
```

### Check Workflow with gh CLI

```bash
# List all workflows
gh workflow list

# View a specific workflow
gh workflow view ci.yml
gh workflow view security.yml

# Check workflow status
gh run list --workflow=ci.yml --limit 5
```

---

## Manual Workflow Triggering

All our main workflows now support `workflow_dispatch`, allowing manual triggering from the GitHub UI or CLI.

### Using GitHub UI

1. Go to **Actions** tab on GitHub
2. Select the workflow (e.g., "CI")
3. Click **Run workflow** dropdown
4. Select branch and click **Run workflow**

### Using gh CLI

```bash
# Trigger CI workflow manually
gh workflow run ci.yml

# Trigger on a specific branch
gh workflow run ci.yml --ref feature-branch

# Trigger security audit
gh workflow run security.yml

# Monitor the run
gh run watch

# View recent runs
gh run list --workflow=ci.yml
```

**Note**: This triggers the workflow on GitHub's servers, not locally. It's useful for testing without creating a PR.

---

## Local Testing with Act

[Act](https://github.com/nektos/act) runs GitHub Actions locally using Docker, providing the closest simulation to actual GitHub runners.

### Installation

```bash
# macOS
brew install act

# Linux
curl -s https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Or download from releases
# https://github.com/nektos/act/releases
```

### Basic Usage

```bash
# List all workflows and jobs
act -l

# Run the default event (push)
act

# Run a specific workflow
act -W .github/workflows/ci.yml

# Run a specific job
act -j test

# Run pull_request event
act pull_request

# Run with specific platform (matches GitHub runners)
act -P ubuntu-latest=catthehacker/ubuntu:act-latest

# Dry run (show what would run without executing)
act -n

# Run with verbose output
act -v
```

### Recommended Act Usage for This Repo

```bash
# Test the main CI workflow (test job only, faster)
act -j test

# Test all CI jobs on ubuntu
act -j test -j wasm -j docs -j examples -P ubuntu-latest=catthehacker/ubuntu:act-latest

# Test security workflow
act -W .github/workflows/security.yml

# Test without pulling new Docker images (faster if images exist)
act -j test --reuse
```

### Act Limitations

- **macOS/Windows runners**: Act primarily supports ubuntu-latest. macOS and Windows jobs require separate testing
- **Secrets**: Need to be provided via `.secrets` file or `-s` flag
- **External services**: GitHub-specific features may not work identically
- **Caching**: GitHub Actions caching behaves differently locally

**Best Practice**: Use `act` for quick local validation, then test on GitHub with `workflow_dispatch` for full verification.

---

## Pre-Flight Checks

Before pushing or creating a PR, run these local checks that mirror CI validation:

### Complete Pre-Push Checklist

```bash
#!/bin/bash
# Save as scripts/ci-preflight.sh

set -e  # Exit on error

echo "Running pre-flight CI checks..."

# 1. Format check
echo "✓ Checking formatting..."
cargo fmt -- --check

# 2. Clippy
echo "✓ Running clippy..."
cargo clippy --all-targets -- -D warnings

# 3. Build
echo "✓ Building..."
cargo build --verbose

# 4. Tests
echo "✓ Running tests..."
cargo test --verbose

# 5. Build with all features (skip on Linux if metal is unavailable)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "✓ Building with all features..."
    cargo build --all-features --verbose
fi

# 6. No default features
echo "✓ Testing no default features..."
cargo test --no-default-features --verbose

# 7. WASM check (requires wasm32 target)
if rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "✓ Checking WASM build..."
    cargo check --target wasm32-unknown-unknown --no-default-features --features wasm
fi

# 8. Documentation
echo "✓ Checking documentation..."
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items

# 9. Example compilation
echo "✓ Checking core examples..."
for example in proper_membrane_generator lagrange_verification check_prime prime_count_smoke_test; do
    cargo check --example "$example"
done

echo ""
echo "✅ All pre-flight checks passed!"
echo "Safe to push or create PR."
```

### Quick Version (Fast Checks Only)

```bash
# Quick pre-commit hook
cargo fmt -- --check && cargo clippy --lib -- -D warnings && cargo test --lib
```

### Make the Script Executable

```bash
chmod +x scripts/ci-preflight.sh

# Run it
./scripts/ci-preflight.sh
```

---

## Comparison Matrix

| Method | Speed | Accuracy | Use Case |
|--------|-------|----------|----------|
| **YAML validation** | ⚡ Instant | Syntax only | Catch YAML errors |
| **Pre-flight script** | ⚡⚡ Fast (2-5 min) | High | Catch build/test failures |
| **Act (local)** | ⚡⚡ Moderate (5-10 min) | Very High | Test workflow logic |
| **workflow_dispatch** | ⚡⚡⚡ Slow (10-20 min) | Perfect | Full GitHub environment |

---

## Recommended Workflow

1. **Before commit**: Run `cargo fmt && cargo clippy`
2. **Before push**: Run `./scripts/ci-preflight.sh`
3. **For workflow changes**: Use `act -n` (dry-run) to validate
4. **For complex changes**: Use `gh workflow run ci.yml` on a test branch
5. **For PRs**: Let GitHub Actions run the full suite

---

## Troubleshooting

### Act Issues

**Problem**: "Error: Cannot connect to the Docker daemon"
```bash
# Ensure Docker is running
docker ps
```

**Problem**: "Error: failed to get git ref"
```bash
# Run from repository root
cd /path/to/prime-physics-engine
act
```

**Problem**: Act runs are slow
```bash
# Use smaller images and enable reuse
act -j test -P ubuntu-latest=catthehacker/ubuntu:act-latest --reuse
```

### gh CLI Issues

**Problem**: "workflow not found"
```bash
# List available workflows
gh workflow list

# Use exact filename
gh workflow run ci.yml  # not "CI" or "ci"
```

**Problem**: Permission denied
```bash
# Re-authenticate
gh auth login

# Check status
gh auth status
```

---

## Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Act Documentation](https://github.com/nektos/act)
- [gh CLI Manual](https://cli.github.com/manual/)
- [yamllint Documentation](https://yamllint.readthedocs.io/)
