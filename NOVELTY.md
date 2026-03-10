# What This Project Actually Contributes

**Updated**: March 2026

## The Core Story

This project investigated symmetric "membrane" constructions that produce primes
at 3-7x the rate of random chance. Through systematic empirical testing, we
discovered that the entire efficiency advantage is explained by classical number
theory (Euler's totient, Mertens' theorem, PNT). The membrane structure itself
provides no statistically significant advantage beyond guaranteeing coprimality
to the base.

This is documented in detail in [collab/THEORETICAL_CLOSURE.md](collab/THEORETICAL_CLOSURE.md).

## What Is Novel

### 1. The Falsification Record (Methodological Contribution)

Multiple hypotheses were tested and refuted:

| Hypothesis | Prediction | Result |
|------------|-----------|--------|
| Scaling law k* ~ sqrt(M) | Optimal padding grows with seed length | **Refuted**: k=0 optimal for all M >= 2 |
| 2xp resonance | Base 14 (2x7) special behavior | **Refuted**: k=0 like all other bases |
| Membrane structure effect | Structure boost > 1.0 | **Refuted**: 1.02x, not significant |
| Phase-lock harmonics | Periodic structure in base 12 | **Refuted** |
| Boundary digit magic | Some digits intrinsically special | **Refuted**: coprimality explains all |

The project demonstrates that a seemingly "special" construction is fully
explained by a well-known principle. This kind of systematic disenchantment
is genuinely valuable -- it prevents a literature of false novelty.

### 2. The Coprimality Reduction (Classical Contribution, New Presentation)

The central result -- that membrane efficiency equals coprimality filtering --
is not new mathematics. Euler, Mertens, and PNT established the ingredients
centuries ago. What is new is the **specific empirical demonstration** that a
particular family of constructions (membranes) achieves exactly the predicted
coprimality boost and nothing more.

The structure stability test (membrane vs random-coprime: ratio = 1.020 +/- 0.053,
p > 0.05) is the key piece of evidence. This test is reproducible:
`cargo run --example membrane_vs_random`.

### 3. The Connector Asymmetry Phenomenon (Empirical, Single Instance)

When concatenating two primes with a buffer, the number of prime results differs
by ~2% depending on concatenation order. This is real (p < 10^-20 for the canonical
pair 10301 and 3007003007003) but has been tested on only one pair. Whether it
generalizes is an open question.

### 4. The Implementation

The Hardy-Littlewood framework (`src/hzlib/`) is a clean, tested Rust
implementation of:
- Singular series computation for Goldbach analysis
- Truncated expectations for restricted pair counting
- Statistical tools (Hedges' g, Cliff's delta, Spearman rho, BH correction)
- Sieve infrastructure (boolean, SPF, segmented)

This is useful code, not novel mathematics.

### 5. The Agda Formalization (Partial)

20 clean Agda modules prove properties about residue-class symmetry and
balanced-bucket pairing for specific modular bases. 12 additional modules
type-check with postulates. The certification framework is operational but
covers a narrow slice of the claims.

## What Is Not Novel

- **Membrane constructions favoring primality**: This is coprimality filtering.
  Any construction guaranteeing gcd(n, base) = 1 achieves the same density.
- **The "physics engine" framing**: Gravity, tidal forces, Lagrange points are
  metaphors for visualization. They are not physics and do not predict anything
  that modular arithmetic does not already explain.
- **k=0 dominance**: This follows directly from the diameter-density principle,
  which itself follows from PNT (shorter numbers have higher prime density).
- **Base-specific optimal digits**: This is a restatement of coprimality.

## Summary Classification

| Contribution | Type | Status |
|-------------|------|--------|
| Membrane = coprimality | Classical result, new presentation | Verified |
| Falsification record | Methodological | Complete |
| Connector asymmetry | Empirical, single instance | Open |
| HL Rust implementation | Software engineering | Tested |
| Agda formalization | Partial formal verification | 20 clean modules |
| Physics metaphor | Visualization/pedagogy | Not a contribution to math |

## Honest Framing

The project's value is the journey of systematic investigation: observing a
pattern, forming hypotheses, testing them rigorously, and arriving at classical
explanations. The destination was known to Euler. The path was our own.

The remaining open question -- connector asymmetry -- is the one finding that
is not immediately explained by standard number theory. It deserves further
investigation with additional prime pairs before any claims about generality.
