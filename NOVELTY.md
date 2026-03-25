# Contribution Classification

**Updated**: March 2026

This document classifies what the repository currently contributes, what it
does not establish, and which questions remain open. Throughout the active
docs, "membrane" denotes the symmetric digit-template family implemented in the
crate.

## Verified Contributions

### 1. Empirical Reduction to Classical Filtering

The strongest current result is not a new prime-density theorem. It is the
empirical reduction showing that the observed density lift of the symmetric
digit-template family is largely accounted for by classical coprimality
filtering together with ordinary candidate-size effects.

The key control comparison is template versus random-coprime sampling. The
reported structure ratio is approximately `1.020 +/- 0.053`, with `p > 0.05`,
so the current evidence is consistent with no additional template-specific lift.

### 2. Negative Results With High Explanatory Value

Several stronger hypotheses were tested and rejected:

| Hypothesis | Predicted consequence | Current result |
|------------|-----------------------|----------------|
| `k* ~ sqrt(M)` scaling law | Optimal padding grows with seed length | Refuted: `k=0` dominates for `M >= 2` |
| `2p` resonance | Bases of the form `2p` should be exceptional | Refuted in tested cases |
| Large template-specific bonus | Template/random-coprime ratio should materially exceed `1` | Not detected |
| Boundary-digit specialness | Some digits should remain preferred after coprimality matching | Not detected |
| Phase-lock harmonic story | Base-12 periodic structure should survive controls | Not detected |

This falsification record is methodologically useful because it narrows the
live claim surface and removes unsupported mechanisms from the active
interpretation.

### 3. Structural Distinction From Ordinary Palindromes

Exact enumeration shows that the symmetric digit-template family is broader than
the ordinary palindrome subset. In the tested base-10 and base-6 families,
non-palindromic subsets retain nontrivial prime density, and at even total
length the palindromic subset can vanish while the larger template family still
contains primes.

This is a verified structural distinction. It is not, by itself, evidence for a
new template-specific density mechanism.

### 4. Connector Asymmetry as a Narrow Empirical Signal

For the canonical pair `10301` and `3007003007003`, the number of prime outputs
depends measurably on concatenation order. This asymmetry is statistically real
for that pair, but it has not yet been generalized beyond a single instance.

The correct classification is therefore: verified single-instance phenomenon,
open general theorem.

## Software and Formalization Contributions

### 1. Rust Mathematical Toolkit

The crate contains a tested implementation of:

- prime sieves and primality checks
- Hardy-Littlewood singular-series computations
- truncated Goldbach expectations
- effect sizes, rank-based statistics, and multiple-testing correction
- connector concatenation arithmetic

This is useful software infrastructure. It should be treated as an engineering
contribution rather than new mathematics.

### 2. Agda and Lean Formalization Surfaces

The repository now has two active proof lanes:

- `agda-proofs/`: broader formal workspace, with audited clean-local and
  postulated strata
- `lean-proofs/`: narrower mathlib-backed package focused on midpoint
  obstruction, exact residue-class filters, radicals, unit residues, and
  wheel-base structure

These formalization surfaces are real contributions to rigor and proof
organization, but they do not yet amount to a complete formal proof of the full
prime-density interpretation.

## Non-Claims

The repository does **not** currently establish:

- a proved template-specific density mechanism beyond coprimality filtering
- a theoretical derivation of the diameter-density relationship
- a general theorem covering connector asymmetry
- a mathematical result supported by the physics-metaphor layer

The gravity, tidal, Lagrange, and related APIs remain visualization or legacy
interfaces. They should not be cited as mathematical evidence.

## Residual Open Questions

The highest-signal remaining questions are:

1. Can the diameter-density relationship be reduced to classical analytic
   number-theory effects, or does any narrower residual structural claim remain?
2. Why does the `M=1` case behave differently from the `M >= 2` regime?
3. Does connector asymmetry persist for families beyond the canonical pair?
4. Can any centered-gap subclass survive same-budget controls with a stable
   excess over matched asymmetric families?

## Summary Table

| Item | Classification | Current status |
|------|----------------|----------------|
| Coprimality-dominated interpretation | Classical mathematics, new empirical reduction | Verified empirically |
| Falsification record | Methodological contribution | Verified |
| Distinction from ordinary palindromes | Structural empirical result | Verified in tested families |
| Connector asymmetry | Empirical single-instance result | Open generalization |
| Rust analysis toolkit | Software engineering | Implemented and tested |
| Agda and Lean proof surfaces | Formalization infrastructure | Active but incomplete |
| Physics-metaphor framing | Visualization / compatibility layer | Not mathematical evidence |
