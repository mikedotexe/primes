# Membrane Prime Toolkit Glossary

Short definitions for the main terms used in this repository.

For a broader technical summary, see [`CLAUDE.md`](CLAUDE.md). For audited
claims, see [`CLAIMS.md`](CLAIMS.md) and
[`VERIFIED_FACTS_VS_SPECULATION.md`](VERIFIED_FACTS_VS_SPECULATION.md).

## Core Terms

### Membrane

A structured number pattern built from boundary digits, zero padding, and a
central seed.

General form:

```text
outer + 0...0 + inner + 0...0 + seed + 0...0 + inner + 0...0 + outer
```

Example:

```text
3 0 7 0 5 0 7 0 3  ->  307050703
```

### Configuration

The fixed part of a membrane construction: base, boundary digits, and padding
parameters.

Example: `(outer, inner) = (3, 7)` with `k=(1,1)`.

### Seed

The variable middle part of a membrane candidate.

- single-digit seed: `5`
- multi-digit seed: `"01"` means the digit string `0,1`, not the number `1`

### `k=(k_outer, k_inner)`

The zero-padding counts in a symmetric membrane.

- `k_outer`: zeros between outer and inner boundary digits
- `k_inner`: zeros between inner boundary digits and the seed

Examples:

- `k=(0,0)`: `3 7 5 7 3`
- `k=(1,1)`: `3 0 7 0 5 0 7 0 3`
- `k=(2,1)`: `3 00 7 0 5 0 7 00 3`

### Symmetric membrane

A membrane where the left and right sides mirror each other using the same
padding counts.

### Breathing membrane

A membrane with asymmetric left/right padding. In this repository, "breathing"
is a structural label, not a theoretical claim about a physical process.

## Evidence Terms

### Prime density / success rate

The fraction of tested seeds that produce primes for a given configuration.

Example: `3` primes out of `10` seeds means `30%` prime density.

### Random baseline

A comparison against randomly chosen integers of comparable size. This baseline
depends on the exact denominator and candidate set, so it should be read from
the relevant experiment rather than treated as a universal constant.

### Random-coprime control

A stricter comparison where random candidates are required to be coprime to the
base. This is the important control for asking whether membrane structure adds
anything beyond coprimality filtering.

### Coprime to the base

A digit or number is coprime to base `b` if its greatest common divisor with
`b` is `1`.

Example in base 10:

- `1, 3, 7, 9` are coprime to `10`
- `2, 4, 5, 6, 8` are not

Why it matters: non-coprime boundary digits can force trivial divisibility.

### Diameter / compactness

A shorthand for how long or spread out a construction is. In this repo's
empirical results, more compact constructions tend to show higher prime density.

### Verified

Checked against the current code or current audited documents.

### Open

A question or interpretation that remains unresolved or only partially tested.

## Connector Terms

### Connector system

The decimal concatenation framework for inserting a variable connector between
two fixed primes.

Core API: `connector::ConcatenationSystem`.

### Canonical pair

The prime pair used most heavily in connector experiments:

- left: `10301`
- right: `3007003007003`

Several connector findings currently apply only to this pair.

### Directional asymmetry

An observed difference between forward and reverse concatenation success rates.
The repo treats this as a real empirical effect for the canonical pair, but not
yet as a general law.

### Lagrange point

Repository shorthand for a productive insertion position in a connector buffer.
This is an analogy borrowed from physics, not a literal physical model.

Use this term carefully:

- acceptable: "Lagrange point" as the repo's connector vocabulary
- not acceptable: implying a proven general equilibrium law for prime pairs

## Number-Theory and API Terms

### Hardy-Littlewood framework

The repo's statistical and number-theoretic tooling under [`src/hzlib`](src/hzlib).
It includes singular series calculations, sieve helpers, and statistical tools.

### `PairCount`

An enum used in the Hardy-Littlewood APIs to distinguish:

- ordered pairs `(p, q)`
- unordered pairs `{p, q}`

### `rad(b)`

The radical of `b`: the product of the distinct prime factors of `b`.

Example:

- `rad(12) = 2 * 3 = 6`

### Miller-Rabin

The primality test used in the repo for large integers. The repo standard is
20 rounds in the documented empirical checks.

## Common Clarifications

- A membrane is not the same thing as a palindrome.
- The repository is not claiming a new physical theory of primes.
- The strongest current interpretation is that membrane efficiency is largely a
  coprimality-filtering effect.
- The quoted densities such as `33%`, `30%`, and `18.5%` are representative
  measured configurations, not universal maxima.
- Base-specific high performers exist, but "best" depends on the exact
  construction family and experiment.

## Current Representative Examples

These are useful anchor points, not exhaustive winners:

| Base | Boundary Digits | k | Prime Density |
|------|-----------------|---|---------------|
| 6    | (1, 5)          | (0, 0) | 33.0% |
| 30   | (11, 7)         | (0, 0) | 30.0% |
| 10   | (3, 7)          | (0, 0) | 18.5% |

## See Also

- [README.md](README.md)
- [CLAUDE.md](CLAUDE.md)
- [CLAIMS.md](CLAIMS.md)
- [NOVELTY.md](NOVELTY.md)
- [EVIDENCE.md](EVIDENCE.md)
