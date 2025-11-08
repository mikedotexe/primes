# Prime Physics Engine - Fix Priority List 🔧

## Executive Summary

74 examples are currently failing. We can fix 80%+ with systematic approach:
1. **Syntax fixes**: 30 examples (40%) - Simple delimiter/brace fixes
2. **Unicode cleanup**: 19 examples (25%) - Replace → with ->
3. **Import updates**: 15 examples (20%) - Update module paths
4. **Complex fixes**: 10 examples (15%) - Require deeper changes

## Quick Wins (1-2 hours to fix all)

### Batch 1: Unicode Character Replacement
Replace all Unicode arrows and symbols with ASCII equivalents.

```bash
# Fix Unicode arrows
sed -i '' 's/→/->/g' examples/*.rs
sed -i '' 's/↓/|/g' examples/*.rs
sed -i '' 's/←/<-/g' examples/*.rs
sed -i '' 's/∞/infinity/g' examples/*.rs
```

**Examples fixed by this**:
- atomic_membrane_explorer.rs
- distribution_visualizer.rs
- field_navigation_explorer.rs
- force_field_mapper.rs
- generalization_tester.rs
- interactive_pattern_explorer.rs
- multi_atom_simulation.rs
- tidal_resonance_explorer.rs
- tui_exploration_advanced.rs

### Batch 2: Simple Syntax Fixes
Fix unclosed delimiters and missing braces.

**High-value targets** (core functionality):
1. `base_metric_explorer.rs` - Missing closing braces
2. `breathing_claim_verifier.rs` - Unclosed delimiter
3. `membrane_construction_demo.rs` - Syntax error
4. `prime_generation_showcase.rs` - Missing delimiter
5. `validation_suite.rs` - Brace mismatch

**Command to identify**:
```bash
for f in examples/*.rs; do
    echo "Checking $f..."
    rustc --edition 2021 --crate-type bin "$f" 2>&1 | grep -E "unclosed delimiter|missing"
done
```

## Medium Priority (2-4 hours)

### Batch 3: Import Path Updates
Update module paths after refactoring.

**Common patterns to fix**:
```rust
// Old:
use prime_physics_engine::visualization::*;
// New:
#[cfg(feature = "visualization")]
use prime_physics_engine::visualization::*;

// Old:
use crate::gpu::*;
// New:
#[cfg(feature = "metal")]
use crate::gpu::*;
```

**Examples needing import fixes**:
- All GPU/Metal examples
- Phase4 examples
- Some visualization examples

### Batch 4: Feature Gate Fixes
Add proper feature gates for optional dependencies.

```rust
// Add to examples that use visualization:
#[cfg(not(feature = "visualization"))]
fn main() {
    println!("This example requires the 'visualization' feature");
}

#[cfg(feature = "visualization")]
fn main() {
    // Actual example code
}
```

## Low Priority (Keep in Legacy)

### Already in Legacy Directory
These are intentionally preserved as historical artifacts:
- breathing_pattern_analyzer.rs
- massive_prime_finder.rs
- membrane_fourier_analysis.rs
- All duplicate TUI examples

### Platform-Specific Examples
Keep but document requirements:
- GPU/Metal examples (macOS only)
- Phase4 examples (future ARM features)
- WASM examples (need wasm-pack)

## Automated Fix Script

```bash
#!/bin/bash
# fix_examples_systematic.sh

echo "Phase 1: Unicode cleanup..."
find examples -name "*.rs" -exec sed -i '' 's/→/->/g' {} \;
find examples -name "*.rs" -exec sed -i '' 's/↓/|/g' {} \;
find examples -name "*.rs" -exec sed -i '' 's/←/<-/g' {} \;

echo "Phase 2: Format all examples..."
find examples -name "*.rs" -exec rustfmt {} \;

echo "Phase 3: Test compilation..."
SUCCESS=0
FAILED=0

for example in examples/*.rs; do
    if rustc --edition 2021 --crate-type bin "$example" -o /dev/null 2>/dev/null; then
        echo "✓ $example"
        ((SUCCESS++))
    else
        echo "✗ $example"
        ((FAILED++))
    fi
done

echo "Results: $SUCCESS working, $FAILED failing"
```

## High-Value Fix Targets

These 10 examples would provide the most value if fixed:

1. **membrane_construction_demo.rs** - Core feature demonstration
2. **prime_generation_showcase.rs** - Main showcase of capabilities  
3. **validation_suite.rs** - Testing infrastructure
4. **performance_comparison.rs** - Benchmarking tool
5. **interactive_prime_explorer.rs** - User interaction
6. **base_metric_explorer.rs** - Physics exploration
7. **lagrange_hunt_explorer.rs** - Lagrange point finder
8. **tidal_organization_demo.rs** - Tidal physics demo
9. **chaos_visualization.rs** - Chaos dynamics
10. **resonance_discovery.rs** - Resonance patterns

## Expected Outcomes

After systematic fixes:
- **Quick wins**: 49 examples fixed (66% success rate)
- **Medium effort**: +15 examples (86% success rate)
- **Full effort**: +10 examples (95%+ success rate)

## Next Steps

1. **Run Unicode cleanup** (5 minutes)
2. **Fix top 10 syntax errors** (30 minutes)
3. **Update import paths** (1 hour)
4. **Test and document** (30 minutes)
5. **Update CI to test examples** (30 minutes)

Total effort: ~3 hours to achieve 85%+ success rate!