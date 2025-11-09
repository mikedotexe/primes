#!/bin/bash
# Quick CI check - runs exactly what GitHub Actions runs

set -e

echo "Running CI checks locally..."
echo ""

# 1. Format
echo "▶ cargo fmt -- --check"
cargo fmt -- --check
echo "✓ Format OK"
echo ""

# 2. Clippy
echo "▶ cargo clippy -- -D warnings"
cargo clippy -- -D warnings
echo "✓ Clippy OK"
echo ""

# 3. Build
echo "▶ cargo build"
cargo build --quiet
echo "✓ Build OK"
echo ""

# 4. Tests
echo "▶ cargo test"
cargo test --quiet
echo "✓ Tests OK"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ All CI checks passed!"
echo "Safe to push."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
