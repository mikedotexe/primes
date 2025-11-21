# Agda Formal Verification for Prime Construction Project

This directory contains formal proofs in Agda for mathematical claims in the prime construction project.

## Installation

### Install Agda

```bash
# Option 1: Via GHCup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh
ghcup install ghc 9.4.7
ghcup install cabal latest
cabal update
cabal install Agda

# Option 2: Via package manager (may be older version)
# macOS:
brew install agda

# Ubuntu/Debian:
sudo apt-get install agda

# Arch:
sudo pacman -S agda
```

### Install Agda Standard Library

```bash
# Clone the standard library
git clone https://github.com/agda/agda-stdlib.git ~/.agda/agda-stdlib
cd ~/.agda/agda-stdlib
git checkout v2.0  # or latest stable version

# Configure Agda to find it
mkdir -p ~/.agda
cat > ~/.agda/libraries <<EOF
~/.agda/agda-stdlib/standard-library.agda-lib
EOF

cat > ~/.agda/defaults <<EOF
standard-library
EOF
```

### Verify Installation

```bash
agda --version
# Should show: Agda version 2.6.x or later
```

## Project Structure

```
agda-proofs/
├── Core/                   # Fundamental definitions
│   ├── Config.agda        # Membrane configuration
│   ├── Polynomial.agda    # Membrane polynomial
│   ├── Primality.agda     # Primality predicates
│   └── Radical.agda       # Radical function
│
├── Theorems/              # Main mathematical proofs
│   ├── AffineTransform.agda      # Affine transform theorem ⭐⭐⭐⭐⭐
│   ├── RadicalProperties.agda    # Radical vs totient ⭐⭐⭐⭐
│   ├── Coprimality.agda          # Coprimality necessity ⭐⭐⭐⭐
│   ├── TrioUniversality.agda     # N=3 universality ⭐⭐⭐
│   └── ExclusiveConfigs.agda     # Unique seed proofs ⭐⭐⭐⭐
│
├── Empirical/             # Postulates for observed patterns
│   └── ObservedPatterns.agda
│
└── Utils/                 # Helper functions and lemmas
    ├── ModularArithmetic.agda
    ├── GCD.agda
    └── Divisibility.agda
```

### Executable Specification Layer

The `Specs/` directory contains **executable specifications** that mirror the Rust implementation exactly:

- **`Specs/SpacingResidueModel`**: Core DP algorithm for counting residue classes
  - `countsDP`: Dynamic programming over residues (matches Rust implementation)
  - `countsEnum`: Brute-force enumerator for validation
  - `countZeroViaL`: LCM lift (single DP at L → per-prime counts)
  - Executable tests: `Test₁/₂/₃` verify DP ≡ enumeration on small specs

- **`Specs/PalindromeEvenDivides`**: Even-palindrome divisibility theorem
  - Proves: even-length palindrome ⟹ (b+1) ∣ n
  - Characterizes 2-digit exception (when b+1 is prime)
  - Justifies `mirror && even-length → obstructed` in density-explorer

- **`Specs/Tests`**: Unified test harness
  - Smoke tests for DP correctness
  - Palindrome divisibility sanity checks
  - Hook for extending formal guarantees

**Purpose**: These modules serve as a **single source of truth** for the spacing/residue model, ensuring Rust and Agda agree exactly.

```bash
# Verify executable specs
cd agda-proofs
agda --safe Specs/Tests.agda  # All tests normalize to refl ✓
```

## Verification Priority

### Phase 1: Quick Wins (Weeks 1-2)
- ✅ **Radical properties** - Easy, high pedagogical value
- ✅ **Trio universality** - Fundamental, should be straightforward

### Phase 2: Core Theorems (Months 1-2)
- 🎯 **Affine transform** - Highest priority, moderate difficulty
- 🎯 **Coprimality necessity** - High value, requires careful formalization

### Phase 3: Advanced Claims (Months 2-4)
- 📐 **Exclusive configurations** - Constructive proofs via divisibility
- 📐 **GCD collapse properties** - Complex but achievable

### Phase 4: Ambitious Goals (Months 4+)
- 🚀 **Goldbach construction** - May need probabilistic models
- 🚀 **Lagrange point properties** - Requires careful definition

## Building the Proofs

```bash
# Type-check a single file
agda Core/Radical.agda

# Type-check entire project
agda Theorems/AffineTransform.agda  # Will check dependencies

# Generate HTML documentation
agda --html Theorems/AffineTransform.agda
# Opens in browser: html/Theorems.AffineTransform.html
```

## Current Status

### Completed
- ✅ Project structure
- ✅ README and installation guide
- ✅ Radical formalization (Core/Radical.agda)
- ✅ Affine transform scaffolding (Theorems/AffineTransform.agda)

### In Progress
- 🚧 Radical property proofs
- 🚧 Affine transform main theorem

### Planned
- ⏳ Coprimality theorem
- ⏳ Trio universality
- ⏳ Exclusive configuration proofs

## Connection to Rust Code

These Agda proofs verify mathematical claims made in:
- `examples/affine_transform_verifier.rs` → `Theorems/AffineTransform.agda`
- `src/hzlib/density.rs` (radical) → `Core/Radical.agda`
- `examples/gcd_paradox_resolver.rs` → (future work)
- `examples/proper_membrane_generator.rs` → `Theorems/ExclusiveConfigs.agda`

## Learning Resources

### Agda Tutorials
- [Agda Documentation](https://agda.readthedocs.io/)
- [Programming Language Foundations in Agda](https://plfa.github.io/)
- [Verified Functional Programming in Agda](https://www.amazon.com/Verified-Functional-Programming-Agda-Books/dp/1970001240)

### Number Theory in Agda
- [agda-stdlib Data.Nat.Primality](https://agda.github.io/agda-stdlib/Data.Nat.Primality.html)
- [agda-stdlib Data.Nat.GCD](https://agda.github.io/agda-stdlib/Data.Nat.GCD.html)

## Contributing

When adding new proofs:
1. Start with clear specification comments
2. State the theorem precisely
3. Break into lemmas
4. Document assumptions and postulates
5. Cross-reference to Rust code and FORMAL_VERIFICATION_ASSESSMENT.md

## Notes

- **Primality**: We may need to postulate primality checking or use probabilistic models for large numbers
- **Performance**: Agda proofs are for verification, not execution
- **Completeness**: Some empirical claims may not be fully provable - we can postulate observed patterns
