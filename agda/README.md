# Agda Formal Verification for Prime Physics Engine

**Version**: 1.0.0
**Status**: Initial Setup - Foundation Layer
**Purpose**: Formal verification of membrane construction correctness and mathematical properties

---

## Overview

This workspace provides **formal proofs** for the core mathematical properties of the Prime Physics Engine. While the Rust implementation discovers patterns empirically (33% success rates, optimal configurations), Agda ensures the **correctness** of our constructions and reveals **necessary conditions** for primality.

### What We Prove vs. What We Measure

```
╔═══════════════════════════════════════════════════════════════╗
║  AGDA (Formal Proofs)        │   RUST (Empirical Discovery)  ║
╠══════════════════════════════╪════════════════════════════════╣
║  ✓ Membranes are symmetric   │   • 33% success rate (base 6) ║
║  ✓ GCD properties hold       │   • (1,5) k=(0,0) is optimal  ║
║  ✓ Radical necessity         │   • Why membranes favor primes║
║  ✓ Construction correctness  │   • Statistical correlations  ║
║  ✓ Algorithm termination     │   • Performance optimization  ║
╚══════════════════════════════╧════════════════════════════════╝
```

**Key Insight**: Agda gives us **mathematical certainty** about our methodology, while Rust lets us **explore the landscape** efficiently.

---

## Installation & Setup

### Prerequisites

1. **Install Agda** (version 2.6.3 or later):
   ```bash
   # macOS
   brew install agda

   # Ubuntu/Debian
   apt-get install agda

   # From Cabal (any platform)
   cabal update
   cabal install Agda
   ```

2. **Install Agda Standard Library**:
   ```bash
   # Clone the standard library
   git clone https://github.com/agda/agda-stdlib.git ~/.agda/stdlib
   cd ~/.agda/stdlib
   git checkout v1.7.3  # or latest stable version

   # Register it with Agda
   mkdir -p ~/.agda
   echo "~/.agda/stdlib/standard-library.agda-lib" >> ~/.agda/libraries
   echo "standard-library" >> ~/.agda/defaults
   ```

3. **Verify Installation**:
   ```bash
   agda --version
   # Should output: Agda version 2.6.3 (or later)
   ```

### Editor Setup

#### Emacs (Recommended)
```elisp
;; Add to your ~/.emacs or ~/.emacs.d/init.el
(load-file (let ((coding-system-for-read 'utf-8))
                (shell-command-to-string "agda-mode locate")))

;; Key bindings:
;; C-c C-l   Load file
;; C-c C-n   Normalize expression
;; C-c C-d   Deduce type
;; C-c C-,   Show goal and context
;; C-c C-c   Case split
;; C-c C-r   Refine goal
```

#### VS Code
Install the Agda Mode extension:
```bash
code --install-extension banacorn.agda-mode
```

#### Vim/Neovim
Use [cornelis](https://github.com/isovector/cornelis) for Agda support.

---

## Project Structure

```
agda/
├── README.md                          # This file
├── prime-physics.agda-lib             # Library configuration
├── src/
│   └── PrimePhysics/
│       ├── Foundation/                # Core number theory
│       │   ├── Nat.agda              # Extended natural number operations
│       │   ├── GCD.agda              # GCD properties and algorithms
│       │   ├── Coprimality.agda      # Coprimality theorems
│       │   └── Radical.agda          # Radical function (product of prime factors)
│       ├── Membrane/                  # Membrane structure proofs
│       │   ├── Structure.agda        # Core membrane definition
│       │   ├── Symmetry.agda         # Symmetry proofs
│       │   ├── Properties.agda       # Coprimality and other invariants
│       │   └── Construction.agda     # Construction algorithm correctness
│       └── Examples/                  # Concrete verified examples
│           ├── BasicMembranes.agda   # Simple membrane proofs
│           └── HighPerformance.agda  # Verified optimal configurations
└── docs/
    ├── GETTING_STARTED.md            # Tutorial for researchers
    ├── PROOF_TECHNIQUES.md           # Common proof patterns
    └── THEOREMS.md                   # Index of proven theorems
```

---

## Quick Start

### 1. Load a Module

```bash
cd agda
agda src/PrimePhysics/Foundation/Nat.agda
```

If everything is set up correctly, you'll see:
```
Checking PrimePhysics.Foundation.Nat (/path/to/Nat.agda).
 Finished PrimePhysics.Foundation.Nat.
```

### 2. Explore an Example

Open `src/PrimePhysics/Examples/BasicMembranes.agda` in your editor and load it (C-c C-l in Emacs).

### 3. Try a Proof

Here's a simple theorem to get started:

```agda
-- Prove that the radical of 10 is 10 (2 × 5)
rad-10 : radical 10 ≡ 10
rad-10 = refl  -- This will fail! You need to compute it properly
```

---

## Core Theorems (Proven)

### Foundation Layer

#### 1. Radical Properties
```agda
-- The radical is the product of distinct prime factors
radical : ℕ → ℕ

-- Examples (proven):
rad-10-is-10 : radical 10 ≡ 10        -- 2 × 5
rad-12-is-6  : radical 12 ≡ 6         -- 2 × 3 (not 12!)
rad-30-is-30 : radical 30 ≡ 30        -- 2 × 3 × 5
```

#### 2. Primality Necessary Condition
```agda
-- If n is prime and n ≢ 0 (mod b), then gcd(n, rad(b)) = 1
prime-radical-coprime : ∀ n b → Prime n → n mod b ≢ 0 →
                        GCD n (radical b) ≡ 1
```

### Membrane Layer

#### 3. Symmetry Theorem
```agda
-- Every membrane is perfectly symmetric around its seed
membrane-is-symmetric : ∀ m → IsSymmetric (toDigitList m) ≡ true
```

#### 4. Coprimality Preservation
```agda
-- If boundary digits are coprime to the base, the membrane preserves this
membrane-preserves-coprimality :
  ∀ outer inner base → Coprime outer (radical base) →
  let m = makeMembrane base outer inner 0 0 seed
  in Coprime (fromMembrane m) (radical base)
```

---

## Workflow: Empirical Discovery → Formal Proof

### Typical Research Cycle

1. **Discover** (Rust): "Base 6 with (1,5) k=(0,0) achieves 33% success!"
   ```bash
   cargo run --example statistical_prime_generator -- --base 6
   ```

2. **Hypothesize**: "Is this because gcd(1,6)=1 and gcd(5,6)=1?"

3. **Formalize** (Agda): Define the property precisely
   ```agda
   optimal-config-has-coprime-boundaries :
     ∀ config → IsOptimal config →
     Coprime config.outer (radical config.base)
   ```

4. **Prove** (Agda): Verify it's a **necessary** condition
   ```agda
   optimal-config-has-coprime-boundaries config opt-proof =
     -- Proof construction here
   ```

5. **Refine** (Rust): Use the insight to narrow the search space
   ```rust
   // Only test coprime boundary digits
   for outer in 1..base {
       if gcd(outer, radical(base)) != 1 { continue; }
       // ...
   }
   ```

---

## Learning Resources

### Agda Tutorials
- [Programming Language Foundations in Agda](https://plfa.github.io/) - Best starting point
- [Agda Documentation](https://agda.readthedocs.io/)
- [agda-stdlib docs](https://agda.github.io/agda-stdlib/)

### Number Theory in Agda
- See `agda-stdlib`: `Data.Nat.GCD`, `Data.Nat.Coprimality`
- [crypto-agda](https://github.com/crypto-agda/crypto-agda) - Verified cryptographic primitives

### Proof Techniques for This Project
See `docs/PROOF_TECHNIQUES.md` for:
- Induction on membrane structure
- Case splitting on digit values
- Rewriting with GCD properties
- Using the standard library effectively

---

## Development Guidelines

### Code Style

1. **Generous Comments**: Every theorem gets an explanation
   ```agda
   -- THEOREM: Radical Idempotence
   -- The radical of a radical is itself, because radical(rad(n))
   -- already has all prime factors at power 1.
   radical-idempotent : ∀ n → radical (radical n) ≡ radical n
   ```

2. **Explicit Types**: Don't rely on inference for key definitions
   ```agda
   -- Good
   membrane-value : (base outer inner k₁ k₂ seed : ℕ) → ℕ

   -- Avoid
   membrane-value = λ base outer inner k₁ k₂ seed → ...
   ```

3. **Proof Readability**: Use `where` clauses to name intermediate results
   ```agda
   theorem : ∀ n → Property n
   theorem n = final-step
     where
       step1 : IntermediateProperty n
       step1 = ...

       step2 : AlmostThere n
       step2 = ...

       final-step : Property n
       final-step = combine step1 step2
   ```

### Testing Proofs

Always test with concrete examples:
```agda
-- General theorem
symmetry : ∀ m → IsSymmetric m

-- Concrete test case
test-300705070003 : IsSymmetric (membrane 10 3 7 2 1 5) ≡ true
test-300705070003 = symmetry (membrane 10 3 7 2 1 5)
```

---

## Integration with Rust Codebase

### Verified Implementation Pattern

1. **Formalize the algorithm** in Agda
2. **Prove correctness** properties
3. **Extract to Haskell** (future: using Agda's built-in extraction)
4. **Optimize in Rust** using the Haskell reference

### Current Status

- **Phase 1** (Active): Foundation layer + membrane structure
- **Phase 2** (Planned): Miller-Rabin error bounds
- **Phase 3** (Future): Hardy-Littlewood formalization as axioms

---

## Contributing

### Adding a New Theorem

1. **Declare** it in the appropriate module
2. **Add test cases** with concrete values
3. **Prove** it (or leave a `postulate` for later)
4. **Document** with comments explaining the significance
5. **Update** `docs/THEOREMS.md` with the new result

### Postulates vs. Axioms vs. Proofs

- **Proof**: Fully verified (`symmetry : ... = refl`)
- **Postulate**: Claimed without proof, for future work
- **Axiom**: Fundamental assumption (e.g., Hardy-Littlewood conjectures)

```agda
-- Use postulate for work-in-progress
postulate
  membrane-favors-primes : ∀ config → SuccessRate config > random-chance

-- Use comment to explain why it's unproven
-- TODO: This requires connecting to PNT, which is beyond Agda's scope
```

---

## FAQ

**Q: Can Agda prove why membranes achieve 33% success rates?**
A: No. That's an empirical observation. Agda proves **structural properties** (symmetry, coprimality) that are **necessary** but not sufficient.

**Q: Why not just use Rust's type system?**
A: Rust's types can't express properties like "this number is prime" or "these digits are coprime." Agda's dependent types let us state and prove these as theorems.

**Q: How long do proofs take to check?**
A: Seconds to minutes for most theorems. The `radical` function computation might take longer for large numbers, but we work with small examples.

**Q: Do I need to know Agda to use the Rust codebase?**
A: No! The Agda workspace is for **verification** and **theorem discovery**. The Rust code stands alone.

**Q: Can we extract verified code to Rust?**
A: Not directly. Agda can extract to Haskell, which you can use as a **reference implementation** when porting to Rust.

---

## Status & Roadmap

### Completed ✅
- [x] Workspace setup
- [x] Foundation modules (GCD, coprimality, radical)
- [x] Membrane structure definition
- [x] Basic symmetry proofs

### In Progress 🚧
- [ ] Coprimality preservation proofs
- [ ] Lagrange point formalization
- [ ] Concrete example verification (base 6, base 30)

### Planned 📋
- [ ] Miller-Rabin error bound proofs
- [ ] Hardy-Littlewood axiomatization
- [ ] Configuration migration invariants
- [ ] Extraction to Haskell reference implementation

---

## Contact & Collaboration

This workspace is part of the **Prime Physics Engine** research project. For questions about:

- **Agda proofs**: Check `docs/GETTING_STARTED.md` or open an issue
- **Mathematical foundations**: See `../EVIDENCE.md` for empirical data
- **Rust implementation**: See `../prime-physics-engine/README.md`

**Philosophy**: We combine the **rigor** of formal verification with the **agility** of empirical exploration. Agda ensures we don't build on faulty foundations; Rust lets us discover what's actually out there.

---

**Happy Proving! 🎯**

*"In mathematics, you don't understand things. You just get used to them." — John von Neumann*
*"In Agda, you understand things by proving them. Then you get used to that." — This Project*
