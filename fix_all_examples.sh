#!/bin/bash

# Comprehensive fix script for all examples

echo "🔧 Fixing all examples systematically..."

EXAMPLES_DIR="examples"
FIXED_COUNT=0
FAILED_COUNT=0

# Function to fix common issues in a file
fix_file() {
    local file="$1"
    local fixed=false
    
    echo "Fixing $file..."
    
    # Fix 1: Add missing std::env import if needed
    if grep -q "env::args" "$file" && ! grep -q "use std::env" "$file"; then
        sed -i '' '1i\
use std::env;' "$file"
        fixed=true
    fi
    
    # Fix 2: Add missing std::io import if needed
    if grep -q "io::" "$file" && ! grep -q "use std::io" "$file"; then
        sed -i '' '1i\
use std::io;' "$file"
        fixed=true
    fi
    
    # Fix 3: Add missing std::str::FromStr import if needed
    if grep -q "from_str" "$file" && ! grep -q "use std::str::FromStr" "$file"; then
        sed -i '' '1i\
use std::str::FromStr;' "$file"
        fixed=true
    fi
    
    # Fix 4: Fix membrane imports (convert old style to new style)
    if grep -q "use prime_physics_engine::membrane::" "$file"; then
        # Check if it needs MembraneBuilder
        if grep -q "MembraneBuilder" "$file" && ! grep -q "MembraneBuilder" "$file" | grep -q "use"; then
            sed -i '' 's/use prime_physics_engine::membrane::MembraneConfig;/use prime_physics_engine::membrane::{MembraneConfig, MembraneBuilder};/' "$file"
            fixed=true
        fi
    fi
    
    # Fix 5: Fix simple missing closing braces for functions that end with expressions
    # This is a simple heuristic - look for function definitions followed by expressions without closing braces
    if grep -E "^[[:space:]]*&?[a-zA-Z_][a-zA-Z0-9_]*[[:space:]]*$" "$file" | tail -1 | grep -q "^[[:space:]]*&"; then
        echo "}" >> "$file"
        fixed=true
    fi
    
    if $fixed; then
        echo "  ✅ Fixed $file"
        return 0
    else
        echo "  ⚠️  No automatic fixes applied to $file"
        return 1
    fi
}

# Process all rust files in examples
for file in "$EXAMPLES_DIR"/*.rs; do
    if [ -f "$file" ]; then
        if fix_file "$file"; then
            ((FIXED_COUNT++))
        else
            ((FAILED_COUNT++))
        fi
    fi
done

echo ""
echo "📊 Summary:"
echo "  Files with fixes applied: $FIXED_COUNT"
echo "  Files needing manual attention: $FAILED_COUNT"
echo ""
echo "Testing compilation of all examples..."
echo "=========================================="

# Now test compilation
PASS_COUNT=0
FAIL_COUNT=0

for file in "$EXAMPLES_DIR"/*.rs; do
    if [ -f "$file" ]; then
        example_name=$(basename "$file" .rs)
        printf "Testing %-30s ... " "$example_name"
        
        if cargo check --example "$example_name" 2>/dev/null >/dev/null; then
            echo "✅ PASS"
            ((PASS_COUNT++))
        else
            echo "❌ FAIL"
            ((FAIL_COUNT++))
        fi
    fi
done

echo ""
echo "📊 Final Results:"
echo "  Passing: $PASS_COUNT"
echo "  Failing: $FAIL_COUNT"
echo "  Total: $((PASS_COUNT + FAIL_COUNT))"
echo "  Success rate: $(( PASS_COUNT * 100 / (PASS_COUNT + FAIL_COUNT) ))%"