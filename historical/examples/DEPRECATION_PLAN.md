# Examples Deprecation Plan

## Summary
Out of 95 examples, we recommend:
- **15 to be deprecated** (broken, duplicates, pure cosmetics)
- **4 to be prioritized** for repair (core educational value)  
- **20 need API auditing** (potentially outdated)
- **56 can remain** as specialized tools

## Phase 1: Immediate Cleanup (DELETE)

### Broken Examples (5 files)
These have major compilation errors and aren't worth fixing:
- `massive_prime_finder.rs` - 45+ unclosed delimiters
- `breathing_pattern_analyzer.rs` - unclosed delimiters  
- `prime_discovery_dashboard.rs` - missing imports, syntax errors
- `membrane_fourier_analysis.rs` - missing complex number imports
- `membrane_lab_tui.rs` - missing struct definitions

### Pure Visualization Examples (6 files)
These are eye candy with minimal educational value:
- `ascii_animation_generator.rs` - generates ASCII "animations" to text files
- `grand_unified_visualization.rs` - elaborate ASCII art documentation
- `findings_visualization.rs` - beautiful ASCII art but no computation
- `ascii_diagram_generator.rs` - pure ASCII art generation
- `membrane_visualization.rs` - complex TUI but mostly visual
- `visual_membrane_explorer.rs` - another visual-only example

### Duplicate Variations (4 files)
Keep only the best version of each concept:
- DELETE: `educational_explorer_colored.rs` (keep `educational_explorer.rs`)
- DELETE: `membrane_lab_tui_enhanced.rs` and `membrane_lab_tui_ultimate.rs` (keep the consolidated version)
- DELETE: `comprehensive_claim_validator.rs` (keep `comprehensive_claim_verifier.rs`)

**Total to DELETE: 15 files**

## Phase 2: Core Educational Examples (PRIORITIZE)

These 4 examples should be the gold standard:
1. `basic_membrane.rs` - Simple, clear introduction ✅ ALREADY FIXED
2. `base_comparison.rs` - Shows practical comparison
3. `find_large_primes.rs` - Demonstrates real use case  
4. `educational_explorer.rs` - Good interactive introduction ✅ ALREADY FIXED

## Phase 3: Audit Specialized Examples (20 files)

### GPU/Metal Examples (need API verification)
- `gpu_benchmark.rs`
- `gpu_power_demonstration.rs`
- `metal_membrane_advanced.rs`
- `metal_membrane_sieve.rs`
- `metal_performance_projections.rs`

### Test Examples (may reference deprecated interfaces)
- `test_hardened_core.rs`
- `test_membrane_lab.rs`
- `test_hardened_lagrange.rs`
- `test_triangular_lagrange.rs`

### Research Tools (verify they still work)
- `lagrange_clustering_verifier.rs`
- `breathing_claim_verifier.rs`
- `exclusive_configuration_finder.rs`
- `statistical_sampling_demo.rs`
- `deterministic_prime_predictor.rs`
- `seed_exclusivity_explorer.rs`
- `membrane_resonance_networks.rs`

## Phase 4: Archive (move to separate folder)

### Working Examples (keep as specialized tools)
These work but are advanced/specialized:
- `affine_transform_verifier.rs` ✅ WORKING
- `common_colors.rs` ✅ WORKING 
- `membrane_lab_ascii_demo.rs` ✅ WORKING
- `ultimate_tui_demo.rs` ✅ WORKING
- And 52 others that need individual assessment

## Execution Commands

```bash
# Phase 1: Delete broken and duplicate examples
rm examples/massive_prime_finder.rs
rm examples/breathing_pattern_analyzer.rs
rm examples/prime_discovery_dashboard.rs
rm examples/membrane_fourier_analysis.rs
rm examples/membrane_lab_tui.rs

rm examples/ascii_animation_generator.rs
rm examples/grand_unified_visualization.rs
rm examples/findings_visualization.rs
rm examples/ascii_diagram_generator.rs
rm examples/membrane_visualization.rs
rm examples/visual_membrane_explorer.rs

rm examples/educational_explorer_colored.rs
rm examples/membrane_lab_tui_enhanced.rs
rm examples/membrane_lab_tui_ultimate.rs
rm examples/comprehensive_claim_validator.rs

# Phase 2: Focus repair on core examples
# (these are the priorities for fixing)

# Phase 3: Create specialized folders
mkdir -p examples/archived
mkdir -p examples/research
mkdir -p examples/gpu
mkdir -p examples/visualization
```

## Expected Results
- **Compilation success rate** should jump from 10% to 40-50%
- **Focused documentation** on core concepts
- **Cleaner examples directory** with clear purpose
- **Easier maintenance** with fewer duplicates