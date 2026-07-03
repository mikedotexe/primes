#!/bin/bash
# Pre-flight CI checks that mirror GitHub Actions validation
# Run this before pushing to catch issues early

set -euo pipefail

echo "🚀 Running pre-flight CI checks..."
echo ""

# 1. Format check
echo "📝 Checking formatting..."
if cargo fmt -- --check; then
    echo "   ✅ Formatting OK"
else
    echo "   ❌ Formatting issues found. Run: cargo fmt"
    exit 1
fi
echo ""

# 2. Clippy
echo "📎 Running clippy..."
if cargo clippy --all-targets -- -D warnings 2>&1 | tee /tmp/clippy-output.txt; then
    echo "   ✅ Clippy passed"
else
    echo "   ❌ Clippy found issues. Check output above."
    exit 1
fi
echo ""

# 3. Build
echo "🔨 Building..."
if cargo build --verbose 2>&1 | tail -10; then
    echo "   ✅ Build successful"
else
    echo "   ❌ Build failed"
    exit 1
fi
echo ""

# 4. Tests
echo "🧪 Running tests..."
if cargo test --verbose 2>&1 | tail -20; then
    echo "   ✅ Tests passed"
else
    echo "   ❌ Tests failed"
    exit 1
fi
echo ""

# 5. Lean build
echo "∑ Checking Lean package..."
if command -v lake >/dev/null 2>&1; then
    if (cd lean-proofs && lake build); then
        echo "   ✅ Lean build passed"
    else
        echo "   ❌ Lean build failed"
        exit 1
    fi
else
    echo "   ❌ Lean toolchain not found (`lake` missing)"
    echo "   Install Lean 4 / elan to run the local preflight fully."
    exit 1
fi
echo ""

# 6. Proof-catalog bridge
echo "🧾 Checking proof-catalog bridge..."
if SIGNAL_SPINE_RUN_ID=preflight-proof-catalog \
    SIGNAL_SPINE_OUT_DIR=/tmp/primes_preflight_proof_catalog \
    scripts/ci_proof_catalog.sh 2>&1 | tail -20; then
    echo "   ✅ Proof-catalog bridge passed"
else
    echo "   ❌ Proof-catalog bridge failed"
    exit 1
fi
echo ""

# 7. Proof-carrying witness certificate
echo "🧾 Checking proof-carrying witness certificate..."
if scripts/ci_witness_certificate.sh; then
    echo "   ✅ Proof-carrying witness certificate passed"
else
    echo "   ❌ Proof-carrying witness certificate failed"
    exit 1
fi
echo ""

# 8. Build with optional features. The Metal feature requires Apple's optional
# Metal toolchain component; keep local preflight useful when that component is
# not installed, while still exercising all non-Metal feature surfaces.
if [[ "$OSTYPE" == "darwin"* ]]; then
    if xcrun metal -help >/dev/null 2>&1; then
        echo "⚙️  Building with all features (macOS + Metal)..."
        feature_args=(--all-features)
    else
        echo "⚙️  Building with non-Metal features (Metal toolchain unavailable)..."
        feature_args=(
            --features
            "visualization,wheel30,dvfs-adaptive,full_precision,experimental,phase4,amx,rl-stats,prime-harmonics"
        )
    fi
    if cargo build "${feature_args[@]}" --verbose 2>&1 | tail -10; then
        echo "   ✅ Feature build successful"
    else
        echo "   ❌ Feature build failed"
        exit 1
    fi
    echo ""
else
    echo "⚙️  Building with non-Metal features (not macOS)..."
    if cargo build --features \
        "visualization,wheel30,dvfs-adaptive,full_precision,experimental,phase4,amx,rl-stats,prime-harmonics" \
        --verbose 2>&1 | tail -10; then
        echo "   ✅ Non-Metal feature build successful"
    else
        echo "   ❌ Non-Metal feature build failed"
        exit 1
    fi
    echo ""
fi

# 9. No default features
echo "🔧 Testing no default features..."
if cargo test --lib --no-default-features --verbose 2>&1 | tail -10; then
    echo "   ✅ No-default-features tests passed"
else
    echo "   ❌ No-default-features tests failed"
    exit 1
fi
echo ""

# 10. WASM check (requires wasm32 target)
if rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "🌐 Checking WASM build..."
    if cargo check --target wasm32-unknown-unknown --no-default-features --features wasm 2>&1 | tail -10; then
        echo "   ✅ WASM check passed"
    else
        echo "   ❌ WASM check failed"
        exit 1
    fi
    echo ""
else
    echo "⏭️  Skipping WASM (target not installed)"
    echo "   Install with: rustup target add wasm32-unknown-unknown"
    echo ""
fi

# 11. Documentation
echo "📚 Checking documentation..."
if RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items 2>&1 | tail -10; then
    echo "   ✅ Documentation OK"
else
    echo "   ❌ Documentation has warnings/errors"
    exit 1
fi
echo ""

# 12. Example compilation
echo "📋 Checking core examples..."
examples=(
    "proper_membrane_generator"
    "lagrange_verification"
    "check_prime"
    "prime_count_smoke_test"
    "statistical_prime_generator"
)

for example in "${examples[@]}"; do
    if cargo check --example "$example" > /dev/null 2>&1; then
        echo "   ✅ $example"
    else
        echo "   ❌ $example"
        cargo check --example "$example"
        exit 1
    fi
done
echo ""

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ All pre-flight checks passed!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Safe to:"
echo "  • git push"
echo "  • Create pull request"
echo "  • Trigger CI workflow: gh workflow run ci.yml"
echo ""
echo "Quick CI test with act (requires Docker):"
echo "  act -j test"
echo ""
