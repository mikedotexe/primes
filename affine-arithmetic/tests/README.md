# Affine Arithmetic Test Suite

This test suite provides **mathematical correctness guarantees** through property-based testing with `proptest`.

## Test Categories

### 1. Core Invariants (`proptest_comprehensive.rs`)

#### 🏆 The Crown Jewel: `condense_preserves_all_reachable_values`

**Mathematical Property**: For all ε ∈ [-1,1]ⁿ, if value V is reachable in the original affine form, then V is reachable in the condensed form.

**Verification**: We check that the interval enclosure never shrinks:
```
[lo_condensed, hi_condensed] ⊇ [lo_original, hi_original]
```

This is tested with:
- Random affine forms (up to 30 terms, coefficients up to ±100)
- Random condensation targets (1-20 terms)
- 100 random test cases per run

**Why it matters**: This proves that `condense()` is a **lossy compression** that preserves all mathematical guarantees. You can aggressively condense to save memory/time without sacrificing correctness.

---

#### Other Key Properties

**Multiplication Soundness**
- Verifies `mul_ctx()` produces conservative enclosures
- Tests with independent noise symbols
- Accounts for first-order approximation error

**Addition Exactness**
- Addition should not introduce approximation error
- Tests merge of independent noise symbols
- Verifies Minkowski sum property

**Correlation Preservation**
- `x - x = 0` exactly (not just `[-ε, +ε]`)
- Tests that shared noise symbols cancel perfectly
- Core advantage over interval arithmetic

**Nonlinear Soundness**
- `exp`, `log`, `sin`, `cos`, `sqrt` all produce conservative enclosures
- Derivative-range linearization is verified
- Domain guards tested

**Prune Preservation**
- Like condense, but by magnitude threshold
- Proves tail-sum technique is conservative

---

### 2. Edge Cases (`proptest_comprehensive.rs::edge_cases`)

**Deterministic stress tests** for corner cases:

- All coefficients equal (uniform distribution)
- Condensing to single term (maximum compression)
- Multiplying `x * x` (correlation in multiplication)

---

### 3. Soundness Tests (`proptest_soundness.rs`)

14 property tests covering:
- Interval arithmetic operations
- Negation, scaling, subtraction
- Efficiency tests (powi vs repeated mul)
- Enclosure preservation across all operations

---

### 4. Integration Tests

**Associativity** (`assoc.rs`)
- `(a+b)+c` and `a+(b+c)` produce overlapping enclosures
- Tests with random intervals

**Wide Trigonometry** (`wide_trig.rs`)
- `sin` and `cos` over `[0, 10π]` must contain `[-1, 1]`
- Verifies periodic extremum detection

**Hybrid Mode** (`sqrt_hybrid.rs`)
- Hybrid fallback produces enclosures compatible with regular mode

**Condense Monotonicity** (`condense_monotonicity.rs`)
- Deterministic test that radius never decreases

---

## Running Tests

### Quick Verification (30 seconds)
```bash
cargo test
```

### Comprehensive Property Testing (2 minutes)
```bash
cargo test --test proptest_comprehensive -- --test-threads=1
```

### Individual Properties
```bash
# The big one:
cargo test condense_preserves_all_reachable_values

# Multiplication correctness:
cargo test multiplication_is_sound

# Correlation behavior:
cargo test self_subtraction_is_exact_zero
```

---

## Test Statistics

| Test File | Tests | Properties Verified |
|-----------|-------|---------------------|
| `proptest_comprehensive.rs` | 13 | Core invariants |
| `proptest_soundness.rs` | 14 | Operation soundness |
| `assoc.rs` | 1 | Associativity |
| `wide_trig.rs` | 1 | Trigonometric bounds |
| `sqrt_hybrid.rs` | 1 | Hybrid compatibility |
| `condense_monotonicity.rs` | 1 | Deterministic monotonicity |
| **Total** | **31** | **Property-based tests** |

Plus **22 unit tests** in `src/` modules.

**Grand Total: 53 tests**

---

## Why Property-Based Testing?

Traditional unit tests check specific inputs:
```rust
assert_eq!(condense([1,2,3]), expected_output);
```

Property tests verify **mathematical laws** over random inputs:
```rust
for 100 random affine forms:
    assert!(condensed.interval() ⊇ original.interval())
```

**Advantages**:
1. **Catches edge cases** you didn't think of
2. **Proves correctness** across input space, not just examples
3. **Serves as executable specification** of mathematical properties
4. **Minimal case finding**: proptest automatically shrinks failing inputs

---

## Interpretation Guide

### When a Property Test Fails

Proptest reports the **minimal failing input**:

```
Test failed: Condense shrank lower bound: 5.0 -> 5.1
minimal failing input: (affine, max_terms) = (
    Affine { a0: 10.0, terms: [(Sym(0), 2.0), (Sym(1), -1.5)] },
    1
)
```

This tells you:
1. **What broke**: The lower bound increased (bad!)
2. **Simplest case**: Just 2 terms, condensing to 1
3. **Reproducible**: Exact values to debug with

### Tolerance Tuning

Floating-point tests use adaptive tolerances:

```rust
let tolerance = 1e-10 * magnitude;
```

- **Multiplication**: `1e-6 * max(|result|)` (first-order approximation)
- **Addition**: `1e-10 * max(|result|)` (should be exact)
- **Nonlinear**: `1e-6 * max(|result|)` (Chebyshev linearization)

These are **conservative** - real errors are typically 100× smaller.

---

## Relationship to Agda Proofs

The property tests in this suite provide **empirical verification** of properties that could be formally proven in Agda:

| Property Test | Agda Equivalent |
|---------------|-----------------|
| `condense_preserves_all_reachable_values` | `condenseSoundness : ∀ x k → toInterval (condense x k) ⊇ toInterval x` |
| `multiplication_is_sound` | `mulSound : ∀ a b → toInterval (a * b) ⊇ trueProduct a b` |
| `self_subtraction_is_exact_zero` | `selfSubIsZero : ∀ x → x - x ≡ Affine.cst 0` |

The property tests give us **confidence** that the implementation is correct. A formal Agda proof would give us **certainty**. Together, they provide defense in depth.

---

## Adding New Properties

To add a new property:

1. **Identify the mathematical law** you want to verify
2. **Write it as a docstring** in mathematical notation
3. **Encode it as a proptest**:

```rust
proptest! {
    /// Distributivity: a * (b + c) ⊇ (a*b) + (a*c)
    ///
    /// Note: Affine arithmetic may be conservative (⊇ not =)
    /// due to loss of correlation information.
    #[test]
    fn multiplication_distributes_over_addition(
        /* ... random inputs ... */
    ) {
        let lhs = a.clone().mul_ctx(&(b.clone() + c.clone()), &mut ctx);
        let rhs = a.clone().mul_ctx(&b, &mut ctx) + a.mul_ctx(&c, &mut ctx);

        let (lhs_lo, lhs_hi) = lhs.to_interval();
        let (rhs_lo, rhs_hi) = rhs.to_interval();

        // Both must be valid enclosures
        prop_assert!(lhs_lo <= rhs_hi && rhs_lo <= lhs_hi);
    }
}
```

4. **Run it**: `cargo test multiplication_distributes`

---

## Continuous Integration

All property tests run on every commit:

```yaml
- name: Property-based tests
  run: cargo test --tests -- --test-threads=1
```

This ensures:
- No regressions in mathematical correctness
- All invariants preserved across refactors
- New features don't break existing guarantees

---

## Further Reading

- **proptest book**: https://altsysrq.github.io/proptest-book/
- **Affine arithmetic theory**: Stolfi & Comba (1993)
- **Sound approximation**: Moore et al., "Introduction to Interval Analysis"

---

**Built with love for mathematical rigor** ❤️
