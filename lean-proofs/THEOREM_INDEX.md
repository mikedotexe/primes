# Lean Theorem Index

**Updated**: March 2026

This index is the shortest current answer to the question:

What mathematically durable signal has been recovered from the repository's
older prose, and where does it now live in Lean 4?

Throughout, standard mathematical language comes first. Repository aliases such
as "membrane" or "honorary zero" are kept only when they help connect the Lean
surface back to older project artifacts.

## Recovered Signal

| Prose intuition | Standard mathematical statement | Lean module(s) | Status |
|-----------------|---------------------------------|----------------|--------|
| midpoint void / honorary zero | a fixed point of an involution cannot occur in a perfectly paired residue family | `PrimeArithmetic/Symmetry/MidpointObstruction`, `PrimeArithmetic/Symmetry/UnitResidueComplementWitness`, `PrimeArithmetic/Symmetry/ZModUnitNegationWitness` | proved |
| modular mirror symmetry has only the trivial fixed residues | on `Fin B`, the complement involution `r ↦ -r mod B` fixes only `0` and `B / 2`, and this yields reusable finite reflection certificates and mirror-family midpoint obstruction theorems | `PrimeArithmetic/Symmetry/ModularReflection`, `PrimeArithmetic/Symmetry/CertificateReflection`, `PrimeArithmetic/Symmetry/MirrorObstruction` | proved |
| balanced reflected residue buckets support constructive pairing | support lists of indices with residue `r` and `-r mod B` are exact finite supports; on balanced disjoint supports, listwise `zipPair` gives an involutive roundtrip on the left support | `PrimeArithmetic/Symmetry/BalancedBucketSupport` | proved |
| balanced bucket counts certify midpoint exclusion automatically | balanced residue counts, support-count agreement, and fixed-point exclusion determine a reflection certificate without hand-written mate functions, and the same data can be packaged into a narrow per-window static/dynamic certificate shell | `PrimeArithmetic/Symmetry/BalancedBucketReflection`, `PrimeArithmetic/Symmetry/WindowCertificate` | proved |
| small finite windows can be certified end to end in Lean | explicit base-6 and base-10 finite residue data instantiate the balanced-bucket and window-certificate APIs and derive both midpoint exclusion and inviolability | `PrimeArithmetic/Symmetry/WindowCertificateExamples` | proved |
| non-coprime boundaries fail | primes larger than the base must be coprime to the base, hence to `rad(base)` | `PrimeArithmetic/Density/CoprimeFilter`, `PrimeArithmetic/Density/RadicalFilter` | proved |
| admissible endings are base-invariant | admissible residues mod `B` are exactly the units, with cardinality `φ(B)` | `PrimeArithmetic/Density/UnitResidues`, `PrimeArithmetic/Density/ZModUnits` | proved |
| symmetry comes from complements | unit residues for bases `> 2` are closed under complement / negation, and the midpoint class is excluded for even bases | `PrimeArithmetic/Density/UnitResidueSymmetry`, `PrimeArithmetic/Density/ZModUnitNegation` | proved |
| there are exactly half as many symmetry orbits as units | admissible residues and unit groups split into two-element complement / negation orbits of size `φ(B) / 2`, and the same quotient can be stated as the orbit space of the order-two subgroup `{1, -1}` acting on `(ZMod B)ˣ` | `PrimeArithmetic/Density/UnitResiduePairs`, `PrimeArithmetic/Density/ZModUnitOrbits`, `PrimeArithmetic/Density/ZModUnitAction` | proved |
| base families decompose locally | wheel bases and coprime product bases decompose by CRT into local prime-factor conditions and unit-group factors, both as recursive tuple decompositions and as canonical finite families over the prime factors | `PrimeArithmetic/Density/CoprimeBaseProducts`, `PrimeArithmetic/Density/ZModUnitCRT`, `PrimeArithmetic/Density/WheelBases`, `PrimeArithmetic/Density/WheelUnitCRT`, `PrimeArithmetic/Density/WheelUnitProductEquiv`, `PrimeArithmetic/Density/WheelResidueClassifier` | proved |
| fixed layout makes the construction linear in the middle block | for fixed base, boundary digits, zero padding, and middle width, the symmetric template has affine form `shift + gradient * seed` | `PrimeArithmetic/Structure/AffineTemplate` | proved |
| base divisors see only the outer digit, while coprime moduli see a seed permutation | every divisor of the base yields a seed-independent residue determined by the outer digit, while every modulus coprime to the base yields an affine permutation of seed residues and a unique seed class for each target residue | `PrimeArithmetic/Structure/AffineResidueSearch`, `PrimeArithmetic/Structure/AffineSeedClasses` | proved |
| fixed-width connector scans admit exact residue filters | forward and reverse concatenations reduce to the same boundary-plus-connector sum whenever `base ≡ 1 (mod m)`; for the canonical decimal pair, this yields exact `mod 3` and `mod 9` exclusion classes for connector values | `PrimeArithmetic/Connector/ConcatenationFilters` | proved |
| whole connector families inherit the same residue logic | once the left/right pair has a known residue class modulo `m`, forward and reverse concatenations reduce to a generic connector-shift profile, giving reusable admissibility lemmas for the entire family | `PrimeArithmetic/Connector/ConcatenationFamilies` | proved |

## Mathematician-Facing Entry Points

If the goal is to show a mathematician the strongest current Lean surface with
minimal repository-specific vocabulary, the best modules to start with are:

- `PrimeArithmetic/Symmetry/MidpointObstruction`
- `PrimeArithmetic/Symmetry/ModularReflection`
- `PrimeArithmetic/Symmetry/MirrorObstruction`
- `PrimeArithmetic/Symmetry/BalancedBucketReflection`
- `PrimeArithmetic/Symmetry/WindowCertificate`
- `PrimeArithmetic/Density/ZModUnitNegation`
- `PrimeArithmetic/Density/ZModUnitCRT`
- `PrimeArithmetic/Density/WheelUnitProductEquiv`
- `PrimeArithmetic/Density/ZModUnitOrbits`
- `PrimeArithmetic/Density/ZModUnitAction`
- `PrimeArithmetic/Structure/AffineTemplate`
- `PrimeArithmetic/Structure/AffineResidueSearch`
- `PrimeArithmetic/Connector/ConcatenationFilters`
- `PrimeArithmetic/Connector/ConcatenationFamilies`

These give:

- the abstract fixed-point obstruction
- the concrete modular reflection classifier and its mirror-family corollary
- the automatic bucket-to-certificate symmetry layer and its narrow window shell
- the standard unit-group negation formulation
- the CRT decomposition on unit groups
- the canonical finite-family wheel-base CRT theorem
- the explicit `φ(B) / 2` orbit count
- the same orbit count in group-action language
- the exact affine dependence on the middle block
- the modular search theorem that turns the affine form into explicit residue
  and seed-class control
- the exact residue filters for fixed-width connector concatenation, including
  the canonical decimal `mod 3` / `mod 9` exclusions
- the reusable connector-family admissibility layer beyond the single canonical pair

## Candidate Future Tranches

The current Lean package is now strong enough that the next gains should come
from extending standard theorem families, not from adding new repository
metaphors. The highest-value next tranches are:

| Priority | Target theorem family | Intended Lean surface | Why it is durable |
|----------|-----------------------|-----------------------|-------------------|
| 1 | window-export / generated proof artifact path | connect generated residue-bucket and position data to the narrow Lean certificate shell in a reproducible artifact pipeline | this would make the new certification lane practically reusable for larger empirical passes |
| 2 | broader connector-family examples | instantiate the new connector-family layer on a few additional maintained pairs or bases so the generic API has concrete, non-canonical clients | this would demonstrate that the new connector surface is genuinely family-level |
| 3 | cautious analytic shell | add only conservative Hardy-Littlewood or singular-series scaffolding once the exact arithmetic lane is stable enough to support it | this keeps any future analytic layer disciplined and classical |
| 4 | certificate-export ergonomics | add helper surfaces that make finite certificate construction from generated data less verbose without widening the theorem boundary | this improves usability without changing the mathematics |

## Still Open Or Non-Formal

The following ideas may still contain useful heuristics or experimental signal,
but they are **not** currently Lean theorems and should not be presented as
formal conclusions:

- a template-specific density mechanism beyond coprimality filtering
- a proof of the diameter-density law
- a general theorem for connector asymmetry beyond the canonical pair
- the gravity / Lagrange / tidal visualization layer as mathematical evidence

## Suggested Reading Order

1. `PrimeArithmetic/Symmetry/MidpointObstruction`
2. `PrimeArithmetic/Symmetry/ModularReflection`
3. `PrimeArithmetic/Symmetry/MirrorObstruction`
4. `PrimeArithmetic/Symmetry/BalancedBucketReflection`
5. `PrimeArithmetic/Symmetry/WindowCertificate`
6. `PrimeArithmetic/Density/UnitResidues`
7. `PrimeArithmetic/Density/ZModUnitNegation`
8. `PrimeArithmetic/Density/ZModUnitCRT`
9. `PrimeArithmetic/Density/WheelUnitProductEquiv`
10. `PrimeArithmetic/Density/ZModUnitAction`
11. `PrimeArithmetic/Structure/AffineTemplate`
12. `PrimeArithmetic/Structure/AffineResidueSearch`
13. `PrimeArithmetic/Connector/ConcatenationFilters`
14. `PrimeArithmetic/Connector/ConcatenationFamilies`

## Related Files

- [`README.md`](README.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`PrimeArithmetic.lean`](PrimeArithmetic.lean)
