#!/bin/bash
# Pre-flight CI checks that mirror GitHub Actions validation
# Run this before pushing to catch issues early

set -e  # Exit on error

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

# 5. Build with all features (skip on Linux if metal is unavailable)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "⚙️  Building with all features (macOS)..."
    if cargo build --all-features --verbose 2>&1 | tail -10; then
        echo "   ✅ All-features build successful"
    else
        echo "   ❌ All-features build failed"
        exit 1
    fi
    echo ""
else
    echo "⚙️  Skipping all-features build (not macOS)"
    echo ""
fi

# 6. No default features
echo "🔧 Testing no default features..."
if cargo test --no-default-features --verbose 2>&1 | tail -10; then
    echo "   ✅ No-default-features tests passed"
else
    echo "   ❌ No-default-features tests failed"
    exit 1
fi
echo ""

# 7. WASM check (requires wasm32 target)
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

# 8. Documentation
echo "📚 Checking documentation..."
if RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items 2>&1 | tail -10; then
    echo "   ✅ Documentation OK"
else
    echo "   ❌ Documentation has warnings/errors"
    exit 1
fi
echo ""

# 9. Example compilation
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
