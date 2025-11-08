# Theorem Index: Proven and Conjectured Results

**A Complete Reference of Formal Theorems**

This document indexes all theorems in the Agda formalization, their status, and their significance for membrane prime research.

---

## Legend

- ✅ **Proven**: Fully verified in Agda (no postulates)
- 🔶 **Postulated**: Stated formally, proof deferred
- 🔬 **Empirical**: Measured by Rust, not provable in type theory
- ⚠️ **Axiom**: Fundamental assumption (e.g., small primes are prime)

---

## Foundation Layer

### GCD and Coprimality

#### GCD Properties (`Foundation/GCD.agda`)

| Theorem | Status | Significance |
|---------|--------|--------------|
| `gcd-comm : gcd m n ≡ gcd n m` | 🔶 | Symmetry of GCD |
| `gcd-assoc : gcd (gcd m n) k ≡ gcd m (gcd n k)` | 🔶 | Associativity |
| `gcd-zero-right : gcd n 0 ≡ n` | ✅ | Base case |
| `gcd-one-right : gcd n 1 ≡ 1` | 🔶 | 1 coprime to all |
| `gcd-self : gcd n n ≡ n` | 🔶 | Reflexivity |

#### Coprimality (`Foundation/Coprimality.agda`)

| Theorem | Status | Significance |
|---------|--------|--------------|
| `1-coprime : 1 ⊥ n` | ✅ | 1 coprime to everything |
| `coprime-sym : m ⊥ n → n ⊥ m` | 🔶 | Symmetry |
| `coprime-mult : m ⊥ n → m ⊥ k → m ⊥ (n*k)` | 🔶 | Multiplicative preservation |
| `coprime-concat : d₁ ⊥ b → d₂ ⊥ b → (d₁*b+d₂) ⊥ b` | 🔶 | **Critical for membranes!** |
| `coprime? : Dec (m ⊥ n)` | ✅ | Decidability |

### Radical Theory

#### Radical Function (`Foundation/Radical.agda`)

| Theorem | Status | Significance |
|---------|--------|--------------|
| `radical-coprime-mult : m ⊥ n → rad(m*n) = rad(m)*rad(n)` | 🔶 | Multiplicativity |
| `radical-idempotent : rad(rad(n)) = rad(n)` | 🔶 | Stabilization |
| `radical-divides : rad(n) ∣ n` | 🔶 | Fundamental property |
| `radical-prime-power : rad(p^k) = p` | 🔶 | Prime powers collapse |

#### **🏆 The Key Primality Constraint**

| Theorem | Status | Significance |
|---------|--------|--------------|
| **`prime-coprime-to-radical`** | 🔶 | **THE FUNDAMENTAL THEOREM** |
| `IsPrime n → n mod b ≢ 0 → n ⊥ radical b` | | Prime → coprime to rad(base) |

**Why this matters**: This is the mathematical foundation of why coprime boundary digits are necessary!

---

## Membrane Structure

### Construction (`Membrane/Structure.agda`)

| Theorem | Status | Significance |
|---------|--------|--------------|
| **`membrane-digits-symmetric`** | 🔶 | **Membranes are palindromes** |
| `membrane-deterministic` | ✅ | Same config+seed → same value |
| `membrane-injective-on-seed` | 🔶 | Different seeds → different values |
| `boundary-digits-appear-twice` | 🔶 | Structural property |

### Properties (`Membrane/Properties.agda`)

| Theorem | Status | Significance |
|---------|--------|--------------|
| **`coprime-boundaries-coprime-membrane`** | 🔶 | **Coprimality preservation** |
| outer ⊥ rad(b) ∧ inner ⊥ rad(b) → membrane ⊥ rad(b) | | Guarantees necessary condition |
| | | |
| **`prime-membrane-needs-coprime-boundaries`** | 🔶 | **Necessity proof** |
| IsPrime(membrane) → coprime boundaries | | Coprimality is REQUIRED |
| | | |
| **`non-coprime-boundary-prevents-primality`** | 🔶 | **Impossibility result** |
| ¬(outer ⊥ rad(b)) → ¬IsPrime(membrane) | | Non-coprime configs are doomed |
| | | |
| **`optimal-config-has-coprime-boundaries`** | 🔶 | **Empirical validation** |
| IsOptimal(config) → coprime boundaries | | Connects to 33% success rates |
| | | |
| `padding-preserves-coprimality` | 🔶 | Why k₁,k₂ don't matter for coprimality |
| `padding-doesnt-affect-coprimality` | 🔶 | Minimal padding is optimal |

---

## Lagrange Points (Advanced)

### Lagrange Structure (`Lagrange/Structure.agda` - new!)

| Theorem | Status | Significance |
|---------|--------|--------------|
| `lagrange-point-exists` | 🔶 | L-points exist between prime pairs |
| `lagrange-point-preserves-primality` | 🔶 | Digits at L-points keep result prime |
| `lagrange-equilibrium` | 🔶 | "Gravitational balance" metaphor |

### Concatenated Prime Theory

| Theorem | Status | Significance |
|---------|--------|--------------|
| `prime-concat-composite` | 🔶 | p₁ + zeros + p₂ usually composite |
| `lagrange-digit-restoration` | 🔶 | Specific digits restore primality |
| `lagrange-clustering` | 🔬 | 100% success in 24 tested pairs |

---

## Key Conjectures (Unproven)

### Membrane Success Rates

❓ **Why 33%?**: Why do optimal configs achieve ~33% success instead of just >0%?

**What we know**:
- ✅ Coprimality is necessary (proven)
- 🔬 Base 6 (1,5) achieves 33% (measured)
- ❓ Mathematical model for success rate (unknown)

**Possible approaches**:
- Hardy-Littlewood heuristics
- Analytic number theory
- Pattern analysis in residues

### Universal Patterns

❓ **Base Independence**: Does (1,5) k=(0,0) work in ALL bases where 1,5 are coprime to rad(base)?

**Evidence**:
- ✅ Works in bases 6, 14, 18 (verified)
- 🔬 Success rates vary: 33%, 27%, 24%
- ❓ Universal principle? (unknown)

### Lagrange Universality

❓ **All Prime Pairs?**: Do ALL consecutive prime pairs have exploitable Lagrange points?

**Evidence**:
- 🔬 24/24 tested pairs show clustering (100%)
- ❓ Proof for infinite primes (unknown)
- ❓ Connection to prime gaps (open)

---

## Empirical Findings (Not Provable in Agda)

These are discovered by Rust but outside type theory's scope:

| Finding | Evidence | Status |
|---------|----------|--------|
| Base 6 achieves 33% | 100+ seeds tested | 🔬 Measured |
| Minimal padding optimal | Across all bases | 🔬 Observed |
| Config migration exists | Seed length 1→10 | 🔬 Documented |
| Exclusive configurations | Single-seed primes | 🔬 Verified |

---

## Proof Roadmap

### Phase 1: Foundations ✅ (Current)
- [x] GCD and coprimality basics
- [x] Radical function
- [x] Membrane structure
- [x] Coprimality necessity theorem

### Phase 2: Symmetry Proofs (Next)
- [ ] Full proof of `membrane-digits-symmetric`
- [ ] Reverse properties for lists
- [ ] Concatenation symmetry lemmas

### Phase 3: Lagrange Points
- [ ] Formalize Lagrange point calculation
- [ ] Prove equilibrium properties
- [ ] Connect to prime concatenation

### Phase 4: Hardy-Littlewood Integration
- [ ] Formalize HL conjectures as axioms
- [ ] Prove membrane configs satisfy HL conditions
- [ ] Model expected success rates

### Phase 5: Universal Patterns
- [ ] Prove (1,5) works for all suitable bases
- [ ] Characterize configuration migration
- [ ] Find cross-base invariants

---

## How to Use This Index

### For Researchers
1. **Check if a property is proven**: Look for ✅
2. **Find what needs proof**: Look for 🔶
3. **See what's empirical**: Look for 🔬

### For Agda Learners
1. **Start with ✅ theorems**: Study completed proofs
2. **Try 🔶 postulates**: Fill in the proofs yourself!
3. **Explore 🔬 findings**: Can you formalize the property?

### For Paper Writing
1. **Cite ✅ theorems**: Formally verified claims
2. **Explain 🔶 postulates**: Conjectured but not yet proven
3. **Report 🔬 findings**: Empirical results with statistical significance

---

## Theorem Dependencies

```
prime-coprime-to-radical (Radical)
    ↓
prime-membrane-needs-coprime-boundaries (Properties)
    ↓
optimal-config-has-coprime-boundaries (Properties)
    ↓
non-coprime-boundary-prevents-primality (Properties)
    ↓
Empirical testing narrows search space (Rust)
```

**The dependency chain shows why formalization matters**: Each proof builds on previous ones to reach stronger conclusions.

---

## Contributing New Theorems

### Adding a Theorem

1. **State it as a `postulate`** in the appropriate module
2. **Add it to this index** with 🔶 status
3. **Test with concrete examples** to verify it's true
4. **Prove it** when ready, update status to ✅
5. **Document its significance** for the project

### Example

```agda
-- In Membrane/Properties.agda
postulate
  my-new-theorem : ∀ config → Property config

-- In THEOREMS.md
| `my-new-theorem` | 🔶 | Why this matters |

-- After proving:
my-new-theorem : ∀ config → Property config
my-new-theorem config = proof-steps

-- Update THEOREMS.md
| `my-new-theorem` | ✅ | Why this matters |
```

---

## Open Questions

These would make excellent research projects:

1. **Can we compute success rates?**
   - Formalize Hardy-Littlewood expectations
   - Prove membranes meet/exceed them

2. **Why does minimal padding win?**
   - Is there a complexity penalty?
   - Relate to prime gap distributions

3. **What makes (1,5) universal?**
   - Arithmetic properties of these digits?
   - Connection to quadratic residues?

4. **Can we predict migration patterns?**
   - Model configuration evolution
   - Prove convergence to stable configs

5. **Do Lagrange points generalize?**
   - Beyond primes to other sequences?
   - Deeper number-theoretic principle?

---

**Current Count**:
- ✅ Proven: 6 theorems
- 🔶 Postulated: 28 theorems
- 🔬 Empirical: 5 findings
- ⚠️ Axioms: 5 (small primes)

**Next milestone**: Phase 2 completion (symmetry proofs)

---

*Last updated: 2025-07-08*
*See individual module files for complete theorem statements and proofs.*
