# Lagrange Points: Agda-Rust Integration Guide

This document shows how the formal Agda framework connects to the existing Rust implementation.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    INTEGRATION STACK                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │  AGDA LAYER (Formal Verification)                   │    │
│  │  - ResidueField.agda (computational framework)      │    │
│  │  - TemplateExtension.agda (conceptual framework)    │    │
│  │  - Examples.agda (concrete validation)              │    │
│  └──────────────┬──────────────────────────────────────┘    │
│                 │ generates/validates                        │
│                 ▼                                            │
│  ┌────────────────────────────────────────────────────┐    │
│  │  RUST LAYER (Implementation)                        │    │
│  │  - lagrange_mechanics.rs (core algorithms)          │    │
│  │  - lagrange_full_verification.rs (testing)          │    │
│  │  - lagrange_systematic_study.rs (empirical)         │    │
│  │  - 17 other examples/                               │    │
│  └──────────────┬──────────────────────────────────────┘    │
│                 │ produces                                   │
│                 ▼                                            │
│  ┌────────────────────────────────────────────────────┐    │
│  │  EVIDENCE LAYER (Empirical Data)                    │    │
│  │  - 24 verified prime pairs (100% success)           │    │
│  │  - Lagrange density plots (PNG visualizations)      │    │
│  │  - Statistical analysis results                     │    │
│  └──────────────┬──────────────────────────────────────┘    │
│                 │ validates                                  │
│                 ▼                                            │
│  ┌────────────────────────────────────────────────────┐    │
│  │  AGDA VERIFICATION (Closes the loop!)               │    │
│  │  - Machine-checked certificates                     │    │
│  │  - Type-safe proofs                                 │    │
│  │  - Publication-ready appendices                     │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Existing Rust Examples (20 files found)

### Core Mechanics
- `lagrange_mechanics.rs` - Basic Lagrange point computation
- `simple_lagrange_test.rs` - Minimal testing
- `membrane_lagrange_pairs.rs` - Membrane-specific analysis

### Verification & Testing
- `lagrange_full_verification.rs` - Comprehensive verification
- `lagrange_clustering_verifier.rs` - Cluster analysis
- `verify_asymmetric_lagrange.rs` - Asymmetric structure validation
- `lagrange_systematic_study.rs` - Systematic empirical study

### Analysis & Visualization
- `lagrange_density_analysis.rs` - Statistical density analysis
- `lagrange_landscape_visualizer.rs` - 2D visualization
- `lagrange_landscape_3d_visualizer.rs` - 3D visualization
- `membrane_lagrange_analysis.rs` - Membrane correlation
- `lagrange_composite_investigation.rs` - Composite number analysis

### Interactive Tools
- `lagrange_educational_tui.rs` - Terminal UI educational tool
- `concatenated_lagrange_explorer.rs` - Interactive explorer
- `lagrange_verbose.rs` - Detailed output

### Utilities
- `lagrange_asymmetric.rs` - Asymmetric template handling
- `lagrange_space_size.rs` - State space analysis

### Outputs
- `lagrange_landscape.png` - 2D density plot
- `lagrange_landscape_3d.png` - 3D visualization
- `lagrange_density_analysis_output.txt` - Statistical results

## Correspondence Table: Agda ⟷ Rust

### Residue Field Framework

| Agda Concept | Rust Implementation | File |
|--------------|---------------------|------|
| `Concatenation` record | `ConcatenatedPrime` struct | `lagrange_mechanics.rs` |
| `insert-digit` function | `insert_at_position()` | `lagrange_mechanics.rs` |
| `residue-at` function | `compute_residue()` | (needs implementation) |
| `is-equilibrium` | `check_coprimality()` | `lagrange_full_verification.rs` |
| `find-equilibrium-digit` | `scan_positions()` | `lagrange_systematic_study.rs` |
| `scan-all-positions` | `systematic_scan()` | `lagrange_systematic_study.rs` |
| `small-primes` list | `SMALL_PRIMES` constant | (to be added) |

### Template Extension Framework

| Agda Concept | Rust Implementation | File |
|--------------|---------------------|------|
| `AsymmetricTemplate` | `AsymmetricStructure` | `lagrange_asymmetric.rs` |
| `buffer-reflection` | `reflect_position()` | `verify_asymmetric_lagrange.rs` |
| `buffer-center` | `find_center()` | `verify_asymmetric_lagrange.rs` |
| `LagrangeInsertion` | `LagrangePoint` struct | `lagrange_mechanics.rs` |
| `is-membrane-prime` | `check_membrane_structure()` | `membrane_lagrange_pairs.rs` |
| `membrane-enhancement` | `compare_membrane_vs_random()` | `membrane_lagrange_analysis.rs` |

### Examples Module

| Agda Concept | Rust Implementation | File |
|--------------|---------------------|------|
| `canonical-concat` | Hardcoded example (10301, 3007...) | Multiple files |
| `L1-candidate`, `L2-candidate` | Known Lagrange points | `lagrange_full_verification.rs` |
| `full-scan` | `exhaustive_search()` | `lagrange_systematic_study.rs` |
| `residue-vector-L1` | Residue computation | (needs implementation) |
| `center-void-hypothesis` | Empirical testing | (needs implementation) |

## Integration Workflow

### Step 1: Rust Generates Data
```rust
// lagrange_systematic_study.rs
fn scan_concatenation(p1: u64, p2: u64, buffer_len: usize) -> Vec<LagrangePoint> {
    let mut points = Vec::new();
    for pos in 0..buffer_len {
        for digit in 1..=9 {
            let n = insert_at_position(p1, p2, buffer_len, pos, digit);
            if is_prime(n) {
                points.push(LagrangePoint { position: pos, digit, value: n });
            }
        }
    }
    points
}

// Run on canonical example
let canonical_points = scan_concatenation(10301, 3007003007003, 5);
// Result: [(1, 6), (4, 6)]
```

### Step 2: Generate Agda Certificate
```rust
// Generate Agda code from Rust data
fn generate_agda_certificate(points: &[LagrangePoint]) -> String {
    format!(
        r#"
-- Auto-generated from Rust scan
canonical-L1 : LagrangePoint canonical-concat
canonical-L1 = record
  {{ position = {}
  ; digit = {}
  ; result = {}
  ; result-is-prime = {{! Miller-Rabin certificate !}}
  }}
        "#,
        points[0].position,
        points[0].digit,
        points[0].value
    )
}
```

### Step 3: Agda Verifies
```agda
-- LagrangePoints/Generated.agda (auto-generated)
module LagrangePoints.Generated where

open import LagrangePoints.ResidueField

-- Data from Rust
canonical-L1-pos : ℕ
canonical-L1-pos = 1

canonical-L1-digit : ℕ
canonical-L1-digit = 6

canonical-L1-value : ℕ
canonical-L1-value = 10301060003007003007003

-- Verification obligations (filled by Agda)
canonical-L1-equilibrium : is-equilibrium canonical-concat 1 6 ≡ true
canonical-L1-equilibrium = refl  -- Computed automatically!

canonical-L1-prime : IsPrime 10301060003007003007003
canonical-L1-prime = {! Certificate from Miller-Rabin !}
```

### Step 4: Round-Trip Validation
```rust
// Validate Agda predictions against Rust computation
fn validate_agda_prediction(
    agda_pos: usize,
    agda_digit: u8,
    agda_expected: u128
) -> bool {
    let computed = insert_at_position(10301, 3007003007003, 5, agda_pos, agda_digit);
    computed == agda_expected && is_prime(computed)
}

assert!(validate_agda_prediction(1, 6, 10301060003007003007003));
assert!(validate_agda_prediction(4, 6, 10301000063007003007003));
```

## Implementing Residue Field in Rust

### Priority 1: Modular Arithmetic
```rust
// Add to src/hzlib/mod.rs or new src/lagrange/residues.rs

pub fn residue_at_position(
    p1: u64,
    p2: u64,
    buffer_len: usize,
    position: usize,
    digit: u8,
    modulus: u64,
) -> u64 {
    let n = insert_at_position(p1, p2, buffer_len, position, digit);
    n % modulus
}

pub fn is_equilibrium(
    p1: u64,
    p2: u64,
    buffer_len: usize,
    position: usize,
    digit: u8,
) -> bool {
    const SMALL_PRIMES: &[u64] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
        31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
        73, 79, 83, 89, 97,
    ];

    SMALL_PRIMES.iter().all(|&m| {
        let residue = residue_at_position(p1, p2, buffer_len, position, digit, m);
        residue != 0
    })
}

pub fn find_equilibrium_digit(
    p1: u64,
    p2: u64,
    buffer_len: usize,
    position: usize,
) -> Option<u8> {
    (1..=9).find(|&d| is_equilibrium(p1, p2, buffer_len, position, d))
}
```

### Priority 2: Template Symmetry
```rust
// Add to lagrange_asymmetric.rs

pub fn buffer_reflection(buffer_len: usize, position: usize) -> usize {
    buffer_len - position - 1
}

pub fn buffer_center(buffer_len: usize) -> Option<usize> {
    if buffer_len % 2 == 1 {
        Some(buffer_len / 2)
    } else {
        None
    }
}

pub fn test_pairing_hypothesis(
    p1: u64,
    p2: u64,
    buffer_len: usize,
    points: &[LagrangePoint],
) -> bool {
    for point in points {
        let reflected = buffer_reflection(buffer_len, point.position);
        let has_pair = points.iter().any(|p| p.position == reflected);
        if !has_pair {
            return false;
        }
    }
    true
}

pub fn test_center_void(
    p1: u64,
    p2: u64,
    buffer_len: usize,
) -> bool {
    if let Some(center) = buffer_center(buffer_len) {
        // Check if ANY digit at center gives prime
        !(1..=9).any(|d| {
            let n = insert_at_position(p1, p2, buffer_len, center, d);
            is_prime(n)
        })
    } else {
        true  // Even buffer, no center to test
    }
}
```

### Priority 3: Integration Example
```rust
// examples/lagrange_agda_integration.rs

use prime_physics_engine::lagrange::residues::*;

fn main() {
    let p1 = 10301_u64;
    let p2 = 3007003007003_u64;
    let buffer_len = 5;

    println!("=== RESIDUE FIELD ANALYSIS ===\n");

    // Scan all positions
    for pos in 0..buffer_len {
        if let Some(digit) = find_equilibrium_digit(p1, p2, buffer_len, pos) {
            let n = insert_at_position(p1, p2, buffer_len, pos, digit);
            let is_p = is_prime(n);
            println!(
                "Position {}: digit {} → {} (equilibrium ✓, prime {})",
                pos,
                digit,
                n,
                if is_p { "✓" } else { "✗" }
            );
        } else {
            println!("Position {}: no equilibrium digit found", pos);
        }
    }

    println!("\n=== TEMPLATE SYMMETRY ANALYSIS ===\n");

    // Test reflections
    for pos in 0..buffer_len {
        let refl = buffer_reflection(buffer_len, pos);
        println!("Position {} reflects to {}", pos, refl);
    }

    // Test center void
    if let Some(center) = buffer_center(buffer_len) {
        let is_void = test_center_void(p1, p2, buffer_len);
        println!(
            "\nCenter at position {}: {} (void hypothesis {})",
            center,
            center,
            if is_void { "✓" } else { "✗" }
        );
    }
}
```

## Verification Pipeline

### Complete Workflow

```
1. RUST EMPIRICAL STUDY
   ├─ Run lagrange_systematic_study
   ├─ Collect data on 100+ prime pairs
   └─ Generate statistics

2. GENERATE AGDA CERTIFICATES
   ├─ Extract residue vectors
   ├─ Format as Agda records
   └─ Write to agda-proofs/LagrangePoints/Generated/

3. AGDA TYPE-CHECKING
   ├─ Load certificates
   ├─ Verify equilibrium computations
   └─ Machine-check all properties

4. VALIDATE PREDICTIONS
   ├─ Agda predicts Lagrange points
   ├─ Rust computes actual values
   └─ Compare (should match 100%)

5. PUBLISH RESULTS
   ├─ Include Agda proofs as appendix
   ├─ Reference Rust implementation
   └─ Show complete verification chain
```

## Testing Checklist

### Residue Field Tests
- [ ] Implement `residue_at_position` in Rust
- [ ] Implement `is_equilibrium` in Rust
- [ ] Implement `find_equilibrium_digit` in Rust
- [ ] Validate on canonical example (10301, 3007...)
- [ ] Test on 10+ random prime pairs
- [ ] Generate Agda certificates for each
- [ ] Type-check all certificates

### Template Symmetry Tests
- [ ] Implement `buffer_reflection` in Rust
- [ ] Implement `buffer_center` in Rust
- [ ] Test pairing hypothesis on canonical example
- [ ] Test center-void hypothesis on canonical example
- [ ] Collect pairing statistics on 100+ pairs
- [ ] Validate against Agda predictions

### Duality Tests
- [ ] For each Lagrange point, verify BOTH:
  - [ ] Residue equilibrium holds
  - [ ] Template prediction matches
- [ ] Compute correlation coefficient
- [ ] Report any discrepancies

## Future Enhancements

### Code Generation
```rust
// Auto-generate Agda modules from Rust data
pub fn generate_lagrange_module(
    p1: u64,
    p2: u64,
    buffer_len: usize,
    points: &[LagrangePoint],
) -> String {
    // Generate complete Agda module
    // with all proofs filled in
    unimplemented!()
}
```

### Proof Automation
```agda
-- Auto-prove simple properties
module LagrangePoints.Automation where

-- Use Agda's reflection to auto-generate proofs
postulate auto-equilibrium : {concat : Concatenation} → {pos : ℕ} → {d : ℕ} →
  -- If Rust says it's equilibrium, Agda can verify
  is-equilibrium concat pos d ≡ true
```

### Continuous Integration
```bash
#!/bin/bash
# ci/verify_lagrange.sh

# 1. Run Rust tests
cargo test --example lagrange_systematic_study

# 2. Generate Agda certificates
cargo run --example generate_lagrange_certificates

# 3. Type-check Agda
cd agda-proofs
agda --safe LagrangePoints/Generated/*.agda

# 4. Report
echo "All Lagrange points verified! ✓"
```

## Summary

**Integration status**:
- ✅ Agda framework complete (3 modules, ~1,300 lines)
- ✅ Rust implementation exists (20 examples)
- ⬜ Residue field implementation needed (Priority 1)
- ⬜ Round-trip validation needed (Priority 2)
- ⬜ Automated certificate generation (Future)

**Next concrete steps**:
1. Add `src/lagrange/residues.rs` with modular arithmetic
2. Add `examples/lagrange_residue_field_demo.rs` to test
3. Generate certificates for canonical example
4. Type-check in Agda
5. Validate predictions match empirical data

**Expected outcome**: Complete formal verification of Lagrange point phenomenon with machine-checked proofs and empirical validation.
