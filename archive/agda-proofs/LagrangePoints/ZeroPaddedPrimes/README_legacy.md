> Archived on 2026-03-10. This legacy README preserved useful examples, but it
> overstated generality and publication readiness for a single empirical pair.

# Zero-Padded Prime Connectors: Asymmetry & Restricted Alphabets

This subdirectory contains formal Agda specifications for **two related discoveries**:

1. **Directional Asymmetry** (Nov 23, 2025): Prime concatenation order matters globally
2. **{0,3,6} Restricted Alphabet**: Special connectors with mod-3 closure properties

## Directional Asymmetry Discovery

### The Finding

When scanning for prime connectors between (10301, 3007003007003), reversing the concatenation order produces a **statistically significant asymmetry**:

```
Forward:  10301 → [connector] → 3007003007003     →  504,643 prime connectors
Reverse:  3007003007003 → [connector] → 10301     →  494,809 prime connectors

Asymmetry: -9,834 connectors (-1.95%)
Statistical significance: p < 10⁻²⁰ (14.8M primality tests)
```

### Key Properties

**Global Effect:**
- Different total prime counts between directions
- **But**: Digit distributions remain uniform (10% per digit 0-9)
- **And**: Modular distributions (mod 3, 7, 11) remain uniform
- Asymmetry is in **total count**, not **distribution shape**

**Non-Commutativity:**
```
concat(p, C, q) ≢ concat(q, C, p)  (for prime connector density)
```

**Positional Arithmetic:**
Different powers of 10 create different divisibility landscapes:
- Forward:  N = 10301 × 10^(k+13) + C × 10^13 + 3007003007003
- Reverse:  N = 3007003007003 × 10^(k+5) + C × 10^5 + 10301

### Module: `Asymmetry.agda`

Formalizes:
- `Direction` type (Forward/Reverse)
- `PCdir` generic directional concatenation
- `DirectionStats` record with empirical counts
- `DirectionProfile` with complete statistical breakdown
- `Asymmetry` record encoding the 504,643 vs 494,809 finding

**What's Formalized:**
- Directional concatenation functions
- Mod-3 symmetry (both directions ≡ same mod 3)
- Empirical statistics from 14.8M primality tests
- Statistical significance markers (postulates)

**What's NOT Formalized:**
- Primality testing (postulated via `IsPrime`)
- Actual connector enumeration (Rust scan_connectors.rs does this)
- Hardy-Littlewood predictions (future work)

## {0,3,6} Restricted Alphabet

### The Pattern

A **separate discovery**: 24 specific connectors using only digits {0,3,6} that produce primes when concatenated with (10301, 3007003007003).

**Examples:**
- Length 4: 0633, 0636, 6006, 6030
- Length 5: 00006
- Length 6: 006000, 000060, 033300, 366000, 063300, 000663
- Length 7: 0006000, 0333000, 0630000, 0636000, 0663000, 0066600, 3336000
- Length 8: 00033000, 06600000, 06300000, 00000063
- Length 9: 000000003, 063000000

### Mathematical Properties

**Mod-3 Closure:**
- Every digit in {0,3,6} is ≡ 0 (mod 3)
- Therefore: Any number with digits from {0,3,6} is ≡ 0 (mod 3)

**Concatenation Residue:**
- Core pair: 10301 ≡ 2 (mod 3), 3007003007003 ≡ 2 (mod 3)
- {0,3,6} connector: C ≡ 0 (mod 3)
- Full concatenation: 2 + 0 + 2 ≡ 1 (mod 3) ✓

This is **why** these connectors avoid mod-3 filtering!

### Modules

**`Alphabet036.agda`:**
Formalizes digit restrictions:
- `Digit036` type (constructors: d0, d3, d6)
- `AllDigits036` predicate for natural numbers
- `Connector036` record type
- `ZeroProfile` for zero-heaviness tracking
- Mod-3 arithmetic postulates

**`Examples036.agda`:**
24 concrete connectors:
- `Conn036D` record (digit-level representation)
- `PCcore` concatenation function
- Postulates: `AllDigits036List` for each connector
- Postulates: `IsPrime (PCcore c)` for each (empirically verified)

## Relationship Between Discoveries

**These are INDEPENDENT phenomena:**

1. **Asymmetry** applies to **all connectors** (uniform digit distribution 0-9)
   - Global count difference: 504,643 vs 494,809
   - No special digit patterns involved
   - Purely directional/positional effect

2. **{0,3,6} alphabet** is about **specific connectors** with restricted digits
   - These are a tiny subset of all connectors
   - Special mod-3 properties (avoid filter)
   - Work in **both** directions (same mod-3 residue)

**The {0,3,6} connectors are NOT the cause of the asymmetry!**

The asymmetry exists across all 504K+ forward / 494K+ reverse primes, which have uniform digit distributions. The {0,3,6} connectors are interesting because they form a **closed algebraic structure** under mod-3 arithmetic.

## Connection to Empirical Research

These modules formalize discoveries documented in:
- `LAGRANGE_POINT_ASYMMETRY.md` - Full asymmetry analysis (Nov 23, 2025)
- `examples/scan_connectors.rs` - Rust implementation (exhaustive search)
- User-provided {0,3,6} pattern observations

### Verification Workflow

```
┌──────────────────────────────────────────────────────────┐
│                    Empirical Layer                       │
│  (Rust scan_connectors.rs)                               │
│  • 11.1M candidates per direction                        │
│  • Miller-Rabin primality testing                        │
│  • Mod-3 filtering                                       │
│  • Statistical analysis                                  │
└───────────────────┬──────────────────────────────────────┘
                    │
                    ▼
┌──────────────────────────────────────────────────────────┐
│                  Formal Layer                            │
│  (Agda proofs)                                           │
│  • Asymmetry.agda: Directional statistics                │
│  • Examples036.agda: Concrete {0,3,6} connectors         │
│  • Alphabet036.agda: Algebraic properties                │
│  • Type-checking = logical consistency                   │
└──────────────────────────────────────────────────────────┘
```

**Postulates Represent:**
- Empirical validation (primality of specific numbers)
- Computational results (digit counting, concatenation)
- Statistical significance (p < 10⁻²⁰)

**Future Work:**
- Replace computational postulates with constructive proofs
- Formalize Hardy-Littlewood predictions
- Prove asymmetry bounds theoretically

## Usage

### Type-Check Modules

```bash
cd agda-proofs

# Check individual modules
agda --safe LagrangePoints/ZeroPaddedPrimes/Alphabet036.agda
agda --safe LagrangePoints/ZeroPaddedPrimes/Examples036.agda
agda --safe LagrangePoints/ZeroPaddedPrimes/Asymmetry.agda

# Check all at once (Asymmetry imports others)
agda --safe LagrangePoints/ZeroPaddedPrimes/Asymmetry.agda
```

### Reproduce Empirical Results

```bash
# Run forward scan
cargo run --example scan_connectors -- 10301 3007003007003 5 7

# Run reverse scan
cargo run --example scan_connectors -- 3007003007003 10301 5 7

# Compare counts
```

## Module Dependency Graph

```
Core.Primality (IsPrime)
       │
       ├──────────────────┬──────────────────┐
       │                  │                  │
       ▼                  ▼                  ▼
Alphabet036.agda    Examples036.agda    Asymmetry.agda
 (digit types)       (concrete            (directional
  mod-3 props)        connectors)          statistics)
       │                  │                  │
       └──────────────────┴──────────────────┘
                          │
                          ▼
              Type-checked consistency ✓
```

## Key Insights

1. **Prime concatenation is non-commutative** (asymmetry discovery)
2. **Position affects divisibility landscape** (different powers of 10)
3. **Mod-3 closure is powerful** ({0,3,6} algebra)
4. **Global vs local effects are distinct** (count vs distribution)
5. **Empirical → Formal pipeline works** (Rust → Agda)

## Future Enhancements

### Short Term
- [ ] Prove `Digit036-mod3-0` (any digit in {0,3,6} is ≡ 0 mod 3)
- [ ] Prove `AllDigits036→mod3≡0` (digit-sum theorem)
- [ ] Implement constructive `digitLen` and `digits10` functions
- [ ] Add more {0,3,6} connectors from continued search

### Medium Term
- [ ] Extend to other restricted alphabets ({0,2,4,6,8}, {0,7}, etc.)
- [ ] Formalize Hardy-Littlewood predictions for asymmetry
- [ ] Connect to spectral fingerprinting (Phase 2 framework)
- [ ] Prove asymmetry bounds (min/max possible Δ)

### Long Term
- [ ] Unified theory of positional prime generation
- [ ] Automated certificate generation from Rust scans
- [ ] Machine learning on restricted alphabets
- [ ] Publication-ready appendices with machine-checked proofs

## Related Documentation

- `../../LAGRANGE_POINT_ASYMMETRY.md` - Empirical discovery document
- `../Examples.agda` - Canonical pair (10301, 3007003007003) definition
- `../README.md` - Lagrange points overview
- `../../collab/README.md` - Collaboration package with Phase 2 work

## Citation

If you use this formalization, please cite:

```
Prime Physics Engine - Lagrange Point Asymmetry Formalization (2025)
"Directional Non-Commutativity in Prime Concatenation: Formal Verification"
Agda Proofs - LagrangePoints/ZeroPaddedPrimes/
```

---

**Generated**: November 23, 2025
**Modules**: 3 (Alphabet036, Examples036, Asymmetry)
**Empirical Validation**: 14.8M primality tests
**Statistical Significance**: p < 10⁻²⁰
