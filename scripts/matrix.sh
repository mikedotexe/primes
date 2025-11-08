#!/bin/bash
# Feature Completeness Matrix for Prime Physics Engine
# Tests all major feature combinations to ensure compatibility

set -e

echo "🧪 Feature Completeness Matrix Test"
echo "===================================="
echo

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

PASSED=0
FAILED=0

test_feature_combo() {
    local name="$1"
    local features="$2"
    local target="${3:-}"
    
    printf "%-30s" "$name"
    
    if [ -n "$target" ]; then
        cargo_cmd="cargo build --target $target $features"
    else
        cargo_cmd="cargo build $features"
    fi
    
    if $cargo_cmd > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC}"
        ((FAILED++))
    fi
}

test_feature_tests() {
    local name="$1"
    local features="$2"
    
    printf "%-30s" "$name (tests)"
    
    if cargo test $features > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC}"
        ((FAILED++))
    fi
}

test_feature_docs() {
    local name="$1"
    local features="$2"
    
    printf "%-30s" "$name (docs)"
    
    if cargo test --doc $features > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC}"
        ((FAILED++))
    fi
}

echo "🔧 Build Matrix..."
echo "------------------"

# Default features
test_feature_combo "default" ""

# Individual features  
test_feature_combo "wheel30 only" "--no-default-features --features wheel30"
test_feature_combo "metal only" "--no-default-features --features metal"  
test_feature_combo "wasm only" "--no-default-features --features wasm"
test_feature_combo "amx only" "--no-default-features --features amx"
test_feature_combo "harmonics only" "--no-default-features --features prime-harmonics"

# Feature combinations
test_feature_combo "wheel30 + metal" "--no-default-features --features wheel30,metal"
test_feature_combo "wheel30 + amx" "--no-default-features --features wheel30,amx"
test_feature_combo "all features" "--all-features"

# Cross-compilation targets
if command -v rustup >/dev/null 2>&1; then
    echo
    echo "🌐 Cross-platform Matrix..."
    echo "----------------------------"
    
    # Ensure targets are installed
    rustup target add wasm32-unknown-unknown > /dev/null 2>&1 || true
    rustup target add x86_64-unknown-linux-gnu > /dev/null 2>&1 || true
    
    test_feature_combo "WASM target" "--no-default-features --features wasm" "wasm32-unknown-unknown"
    
    # Only test Linux target on macOS (cross-compilation)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        test_feature_combo "Linux target" "--no-default-features --features wheel30" "x86_64-unknown-linux-gnu"
    fi
fi

echo
echo "🧪 Test Matrix..."
echo "------------------"

# Test various feature combinations
test_feature_tests "default" ""
test_feature_tests "wheel30" "--no-default-features --features wheel30"
test_feature_tests "harmonics" "--no-default-features --features prime-harmonics"

echo
echo "📚 Documentation Matrix..."
echo "---------------------------"

# Doc tests for major features
test_feature_docs "default" ""
test_feature_docs "all features" "--all-features"

echo
echo "📊 Matrix Results"
echo "=================="

TOTAL=$((PASSED + FAILED))

echo "Passed: $PASSED/$TOTAL"
echo "Failed: $FAILED/$TOTAL"

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All matrix tests passed! ✅${NC}"
    echo "Feature compatibility is excellent."
    exit 0
else
    echo -e "${RED}❌ $FAILED tests failed!${NC}"
    echo "Feature incompatibilities detected."
    exit 1
fi