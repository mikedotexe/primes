# Prime Physics Engine - Quick Reference Glossary

**For detailed explanations, see [RESEARCHER_QUICKSTART.md](RESEARCHER_QUICKSTART.md) or [CLAUDE.md](../CLAUDE.md)**

---

## Not a Palindrome (Read This First)

You might think: "Is this just finding palindromic primes?"

**No.** Critical distinction:

```
┌─────────────────────────────────────────────────────┐
│  PALINDROME (digits must match positions):         │
│                                                     │
│     1  2  3  4  3  2  1                            │
│     ↑  ↑  ↑  ↑  ↑  ↑  ↑                            │
│     └──────┴──┴──┴──┴──────┘                       │
│     Position 1 = Position 7 (must both be 1)       │
│     Position 2 = Position 6 (must both be 2)       │
│     etc.                                           │
│                                                     │
│  MEMBRANE (structure, not specific digits):        │
│                                                     │
│     3  ◯  7  ◯  5  ◯  7  ◯  3                      │
│     ↑     ↑     ↑     ↑     ↑                      │
│     │     │     │     │     └─ outer boundary      │
│     │     │     └───────────── variable seed       │
│     │     └─────────────────── inner boundary      │
│     └───────────────────────── outer boundary      │
│                                                     │
│   We DON'T require 3=3 at ends (that's palindrome) │
│   We only require STRUCTURE: outer-inner-SEED      │
│   The VALUES (3,7,5) can be anything coprime!      │
│                                                     │
│   Different seed → same structure, different prime │
│      seed=5 → 307050703 ✓ prime                    │
│      seed=4 → 307040703 ✗ composite                │
│      seed=7 → 307070703 ✗ composite                │
└─────────────────────────────────────────────────────┘
```

**The Big Difference**:
- **Palindrome**: Fixed digits in fixed positions (1 specific number)
- **Membrane**: Fixed structure, variable values (1 structure → MULTIPLE primes)

**Why membranes are powerful**: One structure produces many primes by varying the seed.

---

## Core Concepts

### Membrane
A symmetric number construction with boundary digits wrapping around a central seed.

**Structure**: `outer + padding + inner + padding + SEED + padding + inner + padding + outer`

**Example**: `3-0-7-0-5-0-7-0-3` → `307050703` (prime)

**Why "membrane"?** Like a biological membrane that wraps and protects cell contents, these digit patterns wrap around the central seed with symmetric boundary layers.

---

### Configuration
The fixed structural parameters of a membrane: boundary digits and padding amounts.

**Notation**: `(outer, inner)` with `k=(k₁,k₂)`

**Example**: Configuration `(3,7)` with `k=(1,1)` means outer=3, inner=7, one zero between each layer

**Key Point**: ONE configuration can produce MULTIPLE primes by varying the seed.

---

### Seed
The central variable portion of a membrane that changes while the configuration stays fixed.

**Single-digit seed**: `5` → becomes central digit `5`

**Multi-digit seed**: `'01'` → becomes central pair `01` (zero then one)

**Note**: Quotes indicate string concatenation, not numeric value. Seed `'01'` ≠ seed `1`.

---

### k=(k₁,k₂) - Zero Padding Parameters
Controls the number of zeros between boundary layers.

**k₁**: Number of zeros between outer and inner boundary digits

**k₂**: Number of zeros between inner boundary and seed

**Examples**:
- `k=(0,0)`: No padding - `3-7-5-7-3` → `37573`
- `k=(1,1)`: One zero each - `3-0-7-0-5-0-7-0-3` → `307050703`
- `k=(2,1)`: Two outer, one inner - `3-00-7-0-5-0-7-00-3` → `300705070003`

**Surprising finding**: `k=(0,0)` (minimal padding) often performs best.

---

### Breathing Pattern
A membrane configuration with asymmetric padding where k₁ ≠ k₂.

**Example**: `(3,3)` with `k=(0,1)` → `3-3-0-5-0-3-3` → `3305033`

**Why "breathing"?** The alternating tight-loose-tight pattern creates a rhythm in the structure, like breathing in-out. (Though the numbers are static.)

**Performance**: Often outperforms symmetric patterns - e.g., 30% vs 10% success

---

### Symmetric Pattern
A membrane configuration with equal padding where k₁ = k₂.

**Example**: `(3,3)` with `k=(1,1)` → `3-0-3-0-5-0-3-0-3` → `303050303`

**Characteristic**: Perfect structural symmetry

**Performance**: Generally lower than breathing patterns, despite aesthetic appeal

---

### Success Rate / Prime Density
The fraction of seeds that produce prime numbers for a given configuration.

**Formula**: `(primes found) / (seeds tested)`

**Example**: Configuration `(3,3)` k=(0,1) with seeds 0-9:
- Seeds 4, 5, 7 produce primes (3 primes)
- Total seeds tested: 10
- Success rate: 3/10 = **30%**

**Comparison**: Random odd numbers ~5%, optimal membranes ~33%

**Note**: We use "success rate" and "prime density" interchangeably.

---

### Coprime to Base
A digit d is coprime to base b if gcd(d, b) = 1 (they share no prime factors).

**Example (base 10)**:
- Digits 1,3,7,9 are coprime to 10 ✓
- Digit 2 shares factor 2 with 10 (not coprime) ✗
- Digit 5 shares factor 5 with 10 (not coprime) ✗

**Why it matters**: Non-coprime boundary digits create systematic divisibility.
- Using 2 in base 10 → every membrane is even (composite)
- Using 5 in base 10 → every membrane divisible by 5

**Key Finding**: 100% of top-performing configurations use coprime boundary digits.

---

## Advanced Concepts

### Lagrange Points
Positions in the zero buffer between two concatenated primes where specific non-zero digits maintain primality of the entire number.

**Analogy**: Like gravitational Lagrange points between Earth and Moon where objects remain stable, these are "mathematical equilibrium points" between prime "masses."

**Example**:
```
Prime 1:  97
Buffer:   00000 (7 zeros)
Prime 2:  303050303

With all zeros: 97000000303050303 → COMPOSITE ✗
With digit at L₁: 97009000303050303 → PRIME ✓
```

**Requirements**: Needs TWO primes (single primes don't create this effect)

**Status**: 100% clustering success across 24 tested prime pairs

---

### Exclusive Configuration
A configuration that produces primes with only ONE specific seed value.

**Example**: `(3,7)` k=(1,1) works ONLY with seed=5
- Seed 5 → `307050703` ✓ PRIME
- Seeds 0,1,2,3,4,6,7,8,9 → all composite ✗

**Significance**: Demonstrates deterministic prime generation - 100% success rate for that specific seed.

**Verification**: We test ALL seeds (0-9), not cherry-picked successes.

---

### Configuration Migration
As seed length increases, optimal configurations "evolve" to maintain high prime density.

**Example**: Different configs dominate for:
- Single-digit seeds: `(3,3)` k=(0,1)
- Two-digit seeds: Different patterns emerge
- Three-digit seeds: Further evolution

**Key Insight**: Some configurations are "length specialists" - natively optimized for specific seed lengths, not degraded versions of shorter patterns.

---

## Bases and Representation

### Base (Radix)
The number system used to construct the membrane pattern.

**Base 10 (decimal)**: Digits 0-9
**Base 6**: Digits 0-5
**Base 12**: Digits 0-9,A,B (or 0-9,10,11)

**Example**: In base 6, the number "15451" means:
```
1×6⁴ + 5×6³ + 4×6² + 5×6 + 1 = 1296 + 1080 + 144 + 30 + 1 = 2551 (decimal)
```

**IMPORTANT**: Membrane patterns are **built in the specified base** but primality is **tested in decimal**.

---

### Base-Dependent Optimization
Each number base has its own optimal boundary digits and configurations.

**Key Finding**: No "universally magical" digit exists - optimization is base-specific.

**Examples**:
- Base 6 optimal: `(1,5)` k=(0,0) → 33% success
- Base 10 optimal: `(3,3)` k=(0,1) → 30% success
- Base 30 optimal: `(11,7)` k=(0,0) → 30% success

**Why?**: Different bases have different factorization properties (6=2×3, 10=2×5, 30=2×3×5)

---

## Verification and Testing

### Systematic Testing
Testing ALL possible values in a defined parameter space, not cherry-picking successes.

**Protocol**: For each configuration:
1. Test ALL single-digit seeds (0-9 in base 10, 0-5 in base 6, etc.)
2. Record EVERY result (both primes and composites)
3. Calculate success rate honestly

**Transparency**: We report both successes AND failures.

**Example**: Configuration `(3,7)` k=(1,1):
```
Seed 0: 307000703 → composite ✗
Seed 1: 307010703 → composite ✗
Seed 2: 307020703 → composite ✗
Seed 3: 307030703 → composite ✗
Seed 4: 307040703 → composite ✗
Seed 5: 307050703 → PRIME ✓
Seed 6: 307060703 → composite ✗
Seed 7: 307070703 → composite ✗
Seed 8: 307080703 → composite ✗
Seed 9: 307090703 → composite ✗

Success rate: 1/10 = 10%
```

---

### Miller-Rabin Primality Test
A probabilistic algorithm for testing whether a number is prime.

**Our Standard**: 20 rounds

**Confidence**: Probability of false positive (composite called prime) < (1/4)²⁰ ≈ 10⁻¹²

**Across 286,200 tests**: Expected false positives ≈ 0.0003 (essentially zero)

**External Verification**: Key examples also verified via Wolfram Alpha for complete confidence.

---

### Verified vs Unverified
**Verified**: Tested with current scripts and independently confirmed

**Unverified**: Claimed in earlier exploration but not yet re-checked with current infrastructure

**Why keep unverified claims?** Transparency. This is active research, not a polished textbook. We show what's been double-checked vs what's in our notes.

---

## Statistical Terms

### Random Baseline (5%)
The prime density of random odd numbers with similar length to our membrane constructions.

**Not the same as PNT density**: Prime Number Theorem gives overall density ~1/ln(n) ≈ 10% for this range

**Our 5% baseline**: Specifically random odd numbers of the same digit length as our membranes

**Why the difference?**: We're comparing apples-to-apples - same length range, odd numbers only

---

### Hardy-Littlewood (HL) Framework
Advanced statistical framework for predicting prime distributions based on analytic number theory.

**Purpose**: Connects our empirical findings (membranes) to theoretical predictions

**Complexity Level**: Graduate-level number theory

**Note**: Understanding basic membrane results does not require HL framework. Skip this section unless you want deep theoretical connections.

**Key Tool**: Predicts expected number of Goldbach pairs, twin primes, etc.

---

## Common Notation

### π(n)
The prime-counting function: number of primes ≤ n

**Example**: π(100) = 25 (there are 25 primes up to 100)

---

### gcd(a,b)
Greatest common divisor: largest integer that divides both a and b

**Example**: gcd(12, 18) = 6

**Special case**: gcd(a,b) = 1 means a and b are coprime (share no factors)

---

### ◯ (Circle Symbol)
Visual representation of zero in membrane patterns for clarity

**Example**: `3-◯-7-◯-5-◯-7-◯-3` = `3-0-7-0-5-0-7-0-3` = `307050703`

**Purpose**: Makes the structure more visible than writing `300705070003`

---

## Quick Reference: Common Confusions

| **Confusion** | **Clarification** |
|--------------|-------------------|
| "More padding = better" | NO! k=(0,0) minimal padding often wins |
| "Symmetric = optimal" | NO! Breathing (asymmetric) often outperforms |
| "Base 10 is best" | NO! Base 6 achieves 33% vs base 10's 18.5% |
| "5% seems low" | Correct for random odds of same length |
| "Is this physics?" | NO! Pure number theory, physics terms are analogies |
| "Seed '01' = seed 1" | NO! '01' is string pattern (zero-one), not value 1 |
| "25 primes from one config" | NO! 10 seeds tested, 3 primes found = 30% |

---

## For More Information

- **Getting Started**: [RESEARCHER_QUICKSTART.md](RESEARCHER_QUICKSTART.md) - 15-minute guided tour
- **Full Context**: [../CLAUDE.md](../CLAUDE.md) - Executive summary of all discoveries
- **Empirical Data**: [EVIDENCE.md](EVIDENCE.md) - Detailed proofs and verification
- **The 3-5-7 Mystery**: [FIVE_SEVEN_MYSTERY.md](FIVE_SEVEN_MYSTERY.md) - Why these digits appear everywhere
- **Command Reference**: [COMMAND_REFERENCE.md](COMMAND_REFERENCE.md) - Quick lookup for all examples

---

**Last Updated**: 2025-10-29
**Status**: Living document - will be updated as new concepts emerge
