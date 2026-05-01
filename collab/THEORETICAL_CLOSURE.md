# Current Interpretation of Membrane Efficiency

**Updated**: March 2026  
**Purpose**: concise collaborator-facing summary of the current interpretation
of the symmetric digit-template family (repo alias: membrane)

## Summary

The repository's current evidence supports a conservative interpretation:

- the symmetric digit-template family can show much higher prime density than
  naive random integers
- most or all of that lift is explained by coprimality filtering plus ordinary
  prime-density effects from candidate size
- the template arrangement itself has not been shown to provide a statistically
  significant boost beyond sampling numbers coprime to the base

The key control comparison in the repo is template versus random-coprime. Its
reported structure ratio is about `1.020 +/- 0.053`, which is not significantly
different from `1.0`.

## Representative Measured Configurations

| Base | Configuration | Prime Density |
|------|---------------|---------------|
| 6    | `(1,5) k=(0,0)` | 33.0% |
| 10   | `(3,7) k=(0,0)` | 18.5% |
| 30   | `(11,7) k=(0,0)` | 30.0% |

These are measured examples, not claims of a universal optimum.

## Working Model

For candidates of magnitude `X` restricted to values coprime to base `B`, the
useful approximation is:

```text
P(prime | gcd(n, B) = 1) ≈ B / φ(B) × 1 / ln(X)
```

This combines:

1. Prime Number Theorem baseline `1 / ln(X)`
2. Euler/totient coprimality boost `B / φ(B)`
3. ordinary size effects from shorter or more compact constructions

The current repo interpretation is that the template structure is mainly a
convenient way to generate candidates in reduced residue classes.

## Construction Density Atlas Addendum

The construction density atlas is the right next instrument for this
interpretation. It does not replace the matched-control conclusion above; it
turns that conclusion into a visible landscape.

The atlas uses the language:

```text
density drift across controlled affine membrane families
```

and compares good, mediocre, and intentionally lousy lanes with layered
controls:

- raw random numbers with the same decimal digit counts
- random numbers with the same digit counts and coprime to `rad(base)`
- same-slot random constructions with the zero-run grammar preserved
- same-budget scaffold controls with noncanonical zero placement

This lets us say "great construction" in a stricter way: a high-yield affine
membrane surface under specified controls. The atlas is allowed to be exciting
because it shows real drift and real prime witnesses, but its claim boundary is
the same as this note: no global density theorem follows from the visual alone.

## Base 30 Wheel Compact Addendum

The base-30 compact report is a focused explanatory companion to the density
atlas. Its job is not to prove residual density lift. Its job is to make the
classical wheel effect legible inside the affine membrane grammar.

For base `30 = 2 * 3 * 5`, unit boundary digits automatically avoid the first
small-prime traps that dominate naive random comparisons. Compact `k=(0,0)`
lanes then keep candidate size low, which also helps ordinary prime density.
The `(B,7)` walkthrough is a good anchor because it has the exact lane:

```text
N(s) = 272970221 + 900*s
```

but it should be described as strong, not uniquely exceptional. The report's
all-pair heatmap is meant to show that base 30 contains a family of strong
compact unit-pair lanes. The careful sentence is: base 30 is a clean
wheel-compressed affine candidate surface, and the residual-density question
remains open until it survives the matched controls.

## Why This Interpretation Won

Several stronger or more exotic explanations were tested and did not survive the
current evidence review:

- padding-scaling laws
- 2×p resonance explanations
- boundary-digit "magic"
- membrane-specific structure bonuses larger than random-coprime controls

See [`VERIFIED_FACTS_VS_SPECULATION.md`](../VERIFIED_FACTS_VS_SPECULATION.md)
and [`NOVELTY.md`](../NOVELTY.md) for the fuller falsification record.

## What Still Looks Interesting

These are active leads, not settled conclusions:

- connector asymmetry for the canonical pair `10301` / `3007003007003`
- period-dependent effects in some primorial-style experiments
- the diameter-density relationship
- exact structure probes showing membranes are broader than ordinary
  palindromes, while same-budget centered-gap controls still fail to show a
  consistent advantage in the tested families

Each of these still needs either broader replication or a cleaner theoretical
account before being treated as a stable claim.

## Framing Guidance

Good framing:

- "membrane constructions are a structured way to sample coprime candidates"
- "high observed densities are largely explained by classical number theory"
- "the membrane-specific effect beyond coprimality is currently not significant"

Avoid:

- "complete theoretical closure"
- "special membrane physics"
- "genuinely novel mechanism" for the membrane density effect itself
- general claims about connector or Lagrange behavior beyond the canonical pair

## Pointers

- [`README.md`](../README.md): public-facing summary
- [`CLAIMS.md`](../CLAIMS.md): claim registry
- [`NOVELTY.md`](../NOVELTY.md): what is and is not novel here
- [`VERIFIED_FACTS_VS_SPECULATION.md`](../VERIFIED_FACTS_VS_SPECULATION.md):
  audited fact/speculation split
- [`EVIDENCE.md`](../EVIDENCE.md): current curated evidence summary
