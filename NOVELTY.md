# What This Project Actually Contributes

**Updated**: March 2026

## The Core Story

This project investigated symmetric "membrane" constructions that produce primes
at 3-7x the rate of naive random chance. Through systematic empirical testing,
the repo established a narrower result: current controls explain most of that
lift using classical number theory (Euler's totient, Mertens' theorem, PNT),
and do not yet show a statistically significant membrane-specific advantage
beyond matched coprime controls.

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

The project demonstrates that a seemingly "special" construction can often be
reduced to well-known principles. This kind of systematic disenchantment is
genuinely valuable; it prevents a literature of false novelty.

### 2. The Coprimality Reduction (Classical Contribution, New Presentation)

The central result -- that membrane efficiency is largely captured by
coprimality filtering --
is not new mathematics. Euler, Mertens, and PNT established the ingredients
centuries ago. What is new is the **specific empirical demonstration** that a
particular family of constructions (membranes) lands close to the predicted
coprimality boost, with current controls remaining consistent with no extra
lift.

The structure stability test (membrane vs random-coprime: ratio = 1.020 +/- 0.053,
p > 0.05) is the key piece of evidence. This test is reproducible:
`cargo run --example membrane_vs_random`.

### 3. Membranes Are Broader Than the Palindrome Subset (Empirical)

Exact enumeration in `examples/membrane_palindrome_probe.rs` shows that the
membrane family is not reducible to ordinary palindromes. In the tested base-10
and base-6 families, non-palindromic membrane subsets retain real prime density,
and at even total lengths the palindromic subset can disappear entirely while
the non-palindromic membrane subset still contains primes.

This is a real structural distinction, but it is not yet evidence that the
canonical mirror-zero layout creates a new prime-density mechanism.

### 4. Centered-Gap Symmetry Remains Unconfirmed (Empirical Negative Control)

The newer same-budget scaffold controls do not show a consistent centered-gap
advantage in the tested base-10 and base-6 families. Both fixed-anchor membrane
templates and broader independent-digit spacing families produce small,
sign-changing deltas once compared against matched asymmetric controls.

That narrows the open question further: if there is extra structural signal, it
lies in a narrower family than "symmetric zero-padding alone."

### 5. The Connector Asymmetry Phenomenon (Empirical, Single Instance)

When concatenating two primes with a buffer, the number of prime results differs
by ~2% depending on concatenation order. This is real (p < 10^-20 for the canonical
pair 10301 and 3007003007003) but has been tested on only one pair. Whether it
generalizes is an open question.

### 6. The Implementation

The Hardy-Littlewood framework (`src/hzlib/`) is a clean, tested Rust
implementation of:
- Singular series computation for Goldbach analysis
- Truncated expectations for restricted pair counting
- Statistical tools (Hedges' g, Cliff's delta, Spearman rho, BH correction)
- Sieve infrastructure (boolean, SPF, segmented)

This is useful code, not novel mathematics.

### 7. The Agda Formalization (Partial)

20 clean Agda modules prove properties about residue-class symmetry and
balanced-bucket pairing for specific modular bases. 12 additional modules
type-check with postulates. The certification framework is operational but
covers a narrow slice of the claims.

## What Is Not Novel

- **A solved membrane-specific density mechanism**: Current evidence does not
  establish one. The best control result is still consistent with coprimality
  filtering plus ordinary size effects.
- **The "physics engine" framing**: Gravity, tidal forces, Lagrange points are
  metaphors for visualization. They are not physics and do not predict anything
  that modular arithmetic does not already explain.
- **k=0 dominance**: This follows directly from the diameter-density principle,
  which itself follows from PNT (shorter numbers have higher prime density).
- **Base-specific optimal digits**: This is a restatement of coprimality.

## Summary Classification

| Contribution | Type | Status |
|-------------|------|--------|
| Membrane lift largely matches coprimality filtering | Classical result, new presentation | Verified empirically |
| Membranes broader than ordinary palindromes | Empirical structural distinction | Verified in tested families |
| Centered-gap symmetry advantage | Empirical control question | Not detected consistently in tested families |
| Falsification record | Methodological | Complete |
| Connector asymmetry | Empirical, single instance | Open |
| HL Rust implementation | Software engineering | Tested |
| Agda formalization | Partial formal verification | 20 clean modules |
| Physics metaphor | Visualization/pedagogy | Not a contribution to math |

## Honest Framing

The project's value is the journey of systematic investigation: observing a
pattern, forming hypotheses, testing them rigorously, and narrowing the part
that is still genuinely open. Much of the density story points back to classical
number theory; some structural questions remain alive.

The remaining open question -- connector asymmetry -- is the one finding that
is least explained by the current writeups. The centered-scaffold versus
same-budget-control question also remains open after the new exact probes.
Both deserve further investigation before any claims about generality.
