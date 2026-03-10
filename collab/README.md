# Collaborator Reference

**Updated**: 2026-03-09
**Repository**: prime-physics-engine

This folder contains curated synthesis documents for collaborators. For the
full codebase, see the repo root.

## Start Here

### The Most Important Finding

**[THEORETICAL_CLOSURE.md](./THEORETICAL_CLOSURE.md)** -- The membrane efficiency
gains (3-7x over random) are **fully explained by classical coprimality filtering**
(Euler + Mertens + PNT). The membrane structure itself contributes no statistically
significant advantage beyond guaranteeing gcd(candidate, base) = 1. This was
confirmed by a structure stability test: membrane vs random-coprime efficiency
ratio = 1.020 +/- 0.053, not significantly different from 1.0 (p > 0.05).

This means the project's value lies not in "special membrane magic" but in:
1. A convenient construction that guarantees coprimality
2. The systematic empirical methodology that led to this understanding
3. The Hardy-Littlewood statistical framework for prime density analysis
4. The honest falsification record (multiple hypotheses tested and refuted)

### Practical Guide

**[PRIMORIAL_MEMBRANE_OPTIMIZATION_GUIDE.md](./PRIMORIAL_MEMBRANE_OPTIMIZATION_GUIDE.md)**
-- If you want to generate primes efficiently, this guide covers three optimization
axes: base selection (primorials), boundary digits (L=1), and seed length (period-6
resonance). Combined efficiency up to ~5.2x PNT.

### Discovery Narrative

**[EXPLORATION_SYNTHESIS.md](./EXPLORATION_SYNTHESIS.md)** -- The material landscape
framework: orthogonal X-Y axes (geometric quality vs cycle purity), Prime Core
Fraction metric, and the path from "what works" to "why it works."

**[PERIOD6_RESONANCE_DISCOVERY.md](./PERIOD6_RESONANCE_DISCOVERY.md)** -- Period-6
resonance in primorial membranes. Real effect (~24% gain) but optimal phase requires
empirical testing per base. Nuanced: the 31% figure in the original report was
pre-stability-testing.

## Key Repo Artifacts (Not in This Folder)

These are the repo's strongest verified documents:

| Document | What It Contains |
|----------|-----------------|
| [VERIFIED_FACTS_VS_SPECULATION.md](../VERIFIED_FACTS_VS_SPECULATION.md) | Rigorous fact/speculation separation with p-values and falsifiability criteria |
| [EVIDENCE.md](../EVIDENCE.md) | Empirical data tables and Wolfram Alpha verification URLs |
| [examples/README.md](../examples/README.md) | 32 curated examples organized by category |
| [src/hzlib/](../src/hzlib/) | Hardy-Littlewood framework, stats, sieves, density analysis |
| [ROADMAP.md](../ROADMAP.md) | Current hardening status and track progress |

## Quick Verification

```bash
cargo test --lib                                    # 174 tests pass
cargo run --example prime_count_smoke_test           # Sieve vs OEIS reference
cargo run --example proper_membrane_generator        # Generate membrane primes
cargo run --example prime_verification_report        # Full verification report
```

## What We Know For Certain

1. Membrane constructions achieve 3-7x prime density over random (empirical, n=1000, p<0.001)
2. The efficiency is fully explained by coprimality filtering (classical, December 2025)
3. Coprimality of boundary digits to the base is required (empirical, 100% of top configs)
4. k=0 padding dominates for seed length M >= 2 (empirical, 8 bases, p<0.001)
5. Base 10 M=2 is a uniquely isolated exception (1/8 bases, p<0.05)

## What Remains Open

1. Why is Base 10 M=2 exceptional? (No structural explanation found)
2. Does the period-6 resonance have a deeper explanation beyond ord(10)?
3. Can the HL framework predict membrane density a priori?
4. What do the Agda formal proofs actually establish? (32 of 80 modules type-check; 20 clean, 12 with postulates)
