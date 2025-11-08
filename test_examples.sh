#!/bin/bash
# Batch test all examples for compilation

WORKING=()
BROKEN=()

echo "Testing examples in main examples/ directory..."
echo "================================================"

cd "$(dirname "$0")"

for file in examples/*.rs; do
    name=$(basename "$file" .rs)
    echo -n "Testing $name... "

    if cargo build --example "$name" 2>&1 | grep -q "Finished"; then
        echo "✓"
        WORKING+=("$name")
    else
        echo "✗"
        BROKEN+=("$name")
    fi
done

echo ""
echo "================================================"
echo "SUMMARY:"
echo "  Working: ${#WORKING[@]}/$(( ${#WORKING[@]} + ${#BROKEN[@]} ))"
echo "  Broken:  ${#BROKEN[@]}/$(( ${#WORKING[@]} + ${#BROKEN[@]} ))"
echo ""

if [ ${#WORKING[@]} -gt 0 ]; then
    echo "Working examples:"
    printf '  %s\n' "${WORKING[@]}"
    echo ""
fi

if [ ${#BROKEN[@]} -gt 0 ]; then
    echo "Broken examples:"
    printf '  %s\n' "${BROKEN[@]}"
fi
