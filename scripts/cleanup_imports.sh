#!/bin/bash

# Clean up duplicate imports and fix common issues
echo "🧹 Cleaning up duplicate imports and fixing issues..."
echo "===================================================="

EXAMPLES_DIR="/Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/examples"

# Fix 1: Remove duplicate is_prime imports
echo "Removing duplicate is_prime imports..."
for file in "$EXAMPLES_DIR"/*.rs; do
    if [[ -f "$file" ]]; then
        # Remove lines that contain "use prime_physics_engine::is_prime;" followed by other text
        sed -i '' '/^use prime_physics_engine::is_prime;[^$]/d' "$file"
        
        # Remove standalone duplicate imports
        # First pass - mark duplicates
        awk '!seen[$0]++' "$file" > "${file}.tmp" && mv "${file}.tmp" "$file"
        
        echo "  Cleaned: $(basename "$file")"
    fi
done

# Fix 2: Remove invalid imports like "use prime_physics_engine::is_prime;use std::..."
echo "Fixing malformed import lines..."
for file in "$EXAMPLES_DIR"/*.rs; do
    if [[ -f "$file" ]]; then
        # Split lines that have multiple use statements
        sed -i '' 's/use prime_physics_engine::is_prime;use /use prime_physics_engine::is_prime;\
use /g' "$file"
        
        # Remove lines that are just "use prime_physics_engine::is_prime;"
        sed -i '' '/^use prime_physics_engine::is_prime;$/d' "$file"
        
        echo "  Fixed: $(basename "$file")"
    fi
done

# Fix 3: Fix common_colors.rs - it shouldn't be an example
echo "Fixing common_colors.rs..."
if [[ -f "$EXAMPLES_DIR/common_colors.rs" ]]; then
    # This should be a library module, not an example
    echo "  common_colors.rs should be moved to src/ as a library module"
fi

echo "✅ Import cleanup complete!"
echo "Note: Some files may still need manual attention."