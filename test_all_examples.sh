#!/bin/bash

# Test all examples for compilation
echo "🔍 Testing all examples for compilation..."
echo "=========================================="

EXAMPLES_DIR="/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/examples"
FAILED_EXAMPLES=()
PASSED_EXAMPLES=()

# Get all .rs files in examples directory
for example_file in "$EXAMPLES_DIR"/*.rs; do
    if [[ -f "$example_file" ]]; then
        # Extract example name (without .rs extension)
        example_name=$(basename "$example_file" .rs)
        
        echo -n "Testing $example_name... "
        
        # Try to compile the example
        cd /Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine
        if cargo check --example "$example_name" 2>/dev/null >/dev/null; then
            echo "✅ PASS"
            PASSED_EXAMPLES+=("$example_name")
        else
            echo "❌ FAIL"
            FAILED_EXAMPLES+=("$example_name")
        fi
    fi
done

echo ""
echo "=========================================="
echo "📊 SUMMARY"
echo "=========================================="
echo "Total examples: $((${#PASSED_EXAMPLES[@]} + ${#FAILED_EXAMPLES[@]}))"
echo "Passed: ${#PASSED_EXAMPLES[@]}"
echo "Failed: ${#FAILED_EXAMPLES[@]}"

if [[ ${#FAILED_EXAMPLES[@]} -gt 0 ]]; then
    echo ""
    echo "❌ Failed examples:"
    for example in "${FAILED_EXAMPLES[@]}"; do
        echo "  - $example"
    done
    
    echo ""
    echo "🔧 Getting detailed error for first failed example..."
    cargo check --example "${FAILED_EXAMPLES[0]}"
fi

echo ""
echo "✅ Passed examples:"
for example in "${PASSED_EXAMPLES[@]}"; do
    echo "  - $example"
done