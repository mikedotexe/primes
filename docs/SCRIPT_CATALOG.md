# Prime Physics Engine - Script & Example Catalog 📋

## Overview

Comprehensive catalog of all scripts and examples with their current status, issues, and fix recommendations.

## Summary Statistics

- **Total Examples**: 93 (19 working, 74 failing)
- **Shell Scripts**: 15+ (status varies)
- **Python Scripts**: 13+ (status varies)
- **Success Rate**: 20.4% for examples

## Working Examples ✅ (19)

These examples compile and run successfully:

```
affine_transform_verifier.rs
base_comparison.rs
basic_membrane.rs
common_colors.rs
comprehensive_benchmark.rs
debug_base_primes.rs
debug_million.rs
debug_sieve.rs
educational_explorer_simple.rs
educational_explorer.rs
find_large_primes.rs
gpu_readiness_benchmark.rs
holistic_optimization_demo.rs
membrane_lab_ascii_demo.rs
prime_sieve_benchmark.rs
sieve_benchmark.rs
test_updated_configs.rs
tui_demo_output.rs
ultimate_tui_demo.rs
```

## Failing Examples ❌ (74)

### Category 1: Syntax Errors (Most Common)

**Issue**: Unclosed delimiters, missing braces
```
Examples affected: ~40% of failures
Fix: Run rustfmt and fix syntax errors
```

**Specific Examples**:
- `base_metric_explorer.rs` - Unclosed delimiter at line 228
- `breathing_claim_verifier.rs` - Unclosed delimiter at line 317
- `interactive_pattern_explorer.rs` - Syntax error
- `lagrange_hunt_explorer.rs` - Multiple syntax errors

### Category 2: Unicode Issues

**Issue**: Unicode characters (→, ↓, ∞) not properly escaped
```
Examples affected: ~25% of failures
Fix: Replace with ASCII or use raw strings
```

**Specific Examples**:
- `atomic_membrane_explorer.rs` - Unicode arrows
- `distribution_visualizer.rs` - Unicode symbols
- `tui_exploration_advanced.rs` - Multiple Unicode issues

### Category 3: Missing Dependencies

**Issue**: References to non-existent modules/functions
```
Examples affected: ~20% of failures
Fix: Update imports or implement missing functionality
```

**Specific Examples**:
- GPU/Metal examples (feature-gated)
- Phase4 examples (require AMX/SME)
- Some visualization examples

### Category 4: Logic/Runtime Errors

**Issue**: Compiles but fails at runtime
```
Examples affected: ~15% of failures
Fix: Debug and fix logic issues
```

## Shell Scripts Status

### Working Scripts ✅
```bash
/build-wasm.sh              # WASM build
/scripts/verify_optimizations.sh  # Optimization verification
```

### Need Testing ❓
```bash
/test_tui.sh                # TUI testing
/snapshot_tui.sh            # TUI snapshots
/test_all_examples.sh       # Example testing (runs but shows failures)
/fix_examples.sh            # Example fixes
/cleanup_imports.sh         # Import cleanup
/fix_all_examples.sh        # Batch fixes
/wasm-demo/build.sh         # WASM demo build
```

### Heritage Scripts (Untested)
```bash
/heritage/scripts/run_examples.sh
/heritage/scripts/run_working_examples.sh
/heritage/scripts/benchmark.sh
/heritage/scripts/build_metal.sh
/heritage/scripts/performance_tracker.sh
/heritage/scripts/compare_modes.sh
/heritage/scripts/collect_primes.sh
```

## Python Scripts Status

### Working Scripts ✅
```python
/test_atomic_primes.py      # Atomic prime testing
/check_coprimality.py       # Coprimality checking
```

### Need Testing ❓
```python
/comprehensive_atomic_search.py
/test_non_coprime.py
/capture_tui.py
```

### Heritage Scripts (Partially Working)
```python
/heritage/scripts/analyze_primes.py    # Needs arguments
/heritage/scripts/explore_configs.py
/heritage/scripts/metal_demo.py
/heritage/scripts/pattern_consistency.py
/heritage/scripts/superlinear_analysis.py
/heritage/scripts/verify_prime.py
/heritage/scripts/visualize_affine.py
/heritage/scripts/visualize_membrane_data.py
```

## Common Failure Patterns

### 1. Syntax Errors (40% of failures)
```rust
// Common pattern: Unclosed delimiter
println!("Example: {}", 
    some_long_expression
    .chain()
    .of()
    .methods()
// Missing closing parenthesis
```

### 2. Unicode Issues (25% of failures)
```rust
// Problem:
println!("→ Result: {}", value);

// Fix:
println!("-> Result: {}", value);
// Or:
println!(r"→ Result: {}", value);
```

### 3. Missing Imports (20% of failures)
```rust
// Problem:
use crate::some_module::SomeType;  // Module doesn't exist

// Fix:
use crate::actual_module::ActualType;
```

### 4. Feature Gate Issues (10% of failures)
```rust
// Problem:
#[cfg(feature = "gpu")]  // Feature not defined

// Fix:
#[cfg(feature = "metal")]  // Use correct feature
```

## Fix Priority Matrix

### High Priority (Core Functionality)
1. `membrane_construction_demo.rs` - Core feature demo
2. `prime_generation_showcase.rs` - Main showcase
3. `performance_comparison.rs` - Benchmarking
4. `validation_suite.rs` - Testing infrastructure

### Medium Priority (Educational)
1. `interactive_*` examples - User interaction
2. `educational_*` examples - Learning tools
3. `visualization_*` examples - Visual demos

### Low Priority (Experimental/Legacy)
1. GPU/Metal examples (platform-specific)
2. Phase4 examples (future features)
3. Legacy examples (intentionally broken)

## Automated Fix Script

```bash
#!/bin/bash
# fix_failing_examples.sh

# 1. Run rustfmt on all examples
find examples -name "*.rs" -exec rustfmt {} \;

# 2. Replace common Unicode characters
find examples -name "*.rs" -exec sed -i '' 's/→/->/g' {} \;
find examples -name "*.rs" -exec sed -i '' 's/↓/|/g' {} \;
find examples -name "*.rs" -exec sed -i '' 's/∞/inf/g' {} \;

# 3. Test each example
for example in examples/*.rs; do
    echo "Testing $example..."
    if ! rustc --edition 2021 --crate-type bin "$example" 2>/dev/null; then
        echo "  FAILED: $example"
    fi
done
```

## Recommendations

### Immediate Actions
1. **Run syntax fixer**: Use rustfmt on all examples
2. **Unicode cleanup**: Replace or escape Unicode characters
3. **Update imports**: Fix module paths for refactored code
4. **Document status**: Mark examples as working/broken/legacy

### Long-term Improvements
1. **CI Integration**: Add example compilation to CI
2. **Example categories**: Separate educational/production/experimental
3. **Dependency management**: Use feature flags consistently
4. **Documentation**: Add README for each example category

### Example Template
```rust
//! Example: [Name]
//! 
//! Status: ✅ Working | ❌ Broken | 🔧 In Progress
//! Category: Educational | Production | Experimental
//! Dependencies: None | [List features]
//! 
//! Description: What this example demonstrates

use prime_physics_engine::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example code here
    Ok(())
}
```

## Next Steps

1. **Triage**: Identify which examples are worth fixing
2. **Batch fixes**: Apply automated fixes where possible
3. **Manual fixes**: Address complex issues individually
4. **Testing**: Verify all fixes work correctly
5. **Documentation**: Update example documentation

The 20% success rate can be improved to 80%+ with systematic fixes!