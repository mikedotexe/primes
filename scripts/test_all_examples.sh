#!/bin/bash

# Test which examples from prime-physics-engine compile and run

set -u

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$REPO_ROOT"

echo "Testing prime-physics-engine examples..."
echo "========================================"
echo

SUCCESS_COUNT=0
FAIL_COUNT=0
TIMEOUT_COUNT=0

# Array to store results
declare -a SUCCESSES
declare -a FAILURES
declare -a TIMEOUTS

# Test each example
for example_file in examples/*.rs; do
    if [ -f "$example_file" ]; then
        example_name=$(basename "$example_file" .rs)

        echo -n "Testing $example_name... "

        # Try to build the example
        if timeout 10s cargo build --example "$example_name" 2>/dev/null >/dev/null; then
            # Try to run it briefly (some are interactive)
            if timeout 2s cargo run --example "$example_name" 2>/dev/null >/dev/null; then
                echo "✅ SUCCESS"
                SUCCESSES+=("$example_name")
                ((SUCCESS_COUNT++))
            else
                exit_code=$?
                if [ $exit_code -eq 124 ]; then
                    echo "⏱️  TIMEOUT (likely interactive/TUI)"
                    TIMEOUTS+=("$example_name")
                    ((TIMEOUT_COUNT++))
                else
                    echo "❌ RUNTIME FAIL"
                    FAILURES+=("$example_name")
                    ((FAIL_COUNT++))
                fi
            fi
        else
            echo "❌ BUILD FAIL"
            FAILURES+=("$example_name")
            ((FAIL_COUNT++))
        fi
    fi
done

echo
echo "========================================"
echo "SUMMARY"
echo "========================================"
echo "✅ Successful: $SUCCESS_COUNT"
echo "⏱️  Timeout (interactive): $TIMEOUT_COUNT"
echo "❌ Failed: $FAIL_COUNT"
echo "Total: $((SUCCESS_COUNT + TIMEOUT_COUNT + FAIL_COUNT))"
echo

if [ ${#SUCCESSES[@]} -gt 0 ]; then
    echo "Working examples:"
    for ex in "${SUCCESSES[@]}"; do
        echo "  ✅ cargo run --example $ex"
    done
    echo
fi

if [ ${#TIMEOUTS[@]} -gt 0 ]; then
    echo "Interactive/TUI examples (need terminal):"
    for ex in "${TIMEOUTS[@]}"; do
        echo "  ⏱️  cargo run --example $ex"
    done
    echo
fi

if [ ${#FAILURES[@]} -gt 0 ]; then
    echo "Failed examples:"
    for ex in "${FAILURES[@]}"; do
        echo "  ❌ $ex"
    done
fi
