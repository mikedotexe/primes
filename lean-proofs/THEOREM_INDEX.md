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
| generated residue lists and position lists feed the maintained certificate shell directly | a concrete residue list induces the observed residue map and support counts automatically, so generated window artifacts can enter the static and dual certificate shells without hand-written count functions | `PrimeArithmetic/Symmetry/WindowCertificateGenerated` | proved |
| generated artifacts can be passed around as compact proof objects | balanced counts, fixed-point exclusion, and pointwise safety can be bundled into a single generated evidence object, from which the same static or dual window certificates are rebuilt | `PrimeArithmetic/Symmetry/WindowCertificateErgonomics` | proved |
| small finite windows can be certified end to end in Lean | explicit base-6 and base-10 finite residue data instantiate the balanced-bucket and window-certificate APIs and derive both midpoint exclusion and inviolability | `PrimeArithmetic/Symmetry/WindowCertificateExamples` | proved |
| non-coprime boundaries fail | primes larger than the base must be coprime to the base, hence to `rad(base)` | `PrimeArithmetic/Density/CoprimeFilter`, `PrimeArithmetic/Density/RadicalFilter` | proved |
| admissible endings are base-invariant | admissible residues mod `B` are exactly the units, with cardinality `φ(B)` | `PrimeArithmetic/Density/UnitResidues`, `PrimeArithmetic/Density/ZModUnits` | proved |
| symmetry comes from complements | unit residues for bases `> 2` are closed under complement / negation, and the midpoint class is excluded for even bases | `PrimeArithmetic/Density/UnitResidueSymmetry`, `PrimeArithmetic/Density/ZModUnitNegation` | proved |
| there are exactly half as many symmetry orbits as units | admissible residues and unit groups split into two-element complement / negation orbits of size `φ(B) / 2`, and the same quotient can be stated as the orbit space of the order-two subgroup `{1, -1}` acting on `(ZMod B)ˣ` | `PrimeArithmetic/Density/UnitResiduePairs`, `PrimeArithmetic/Density/ZModUnitOrbits`, `PrimeArithmetic/Density/ZModUnitAction` | proved |
| base families decompose locally | wheel bases and coprime product bases decompose by CRT into local prime-factor conditions and unit-group factors, with exact local-to-global residue classification and roundtrip recovery, both as recursive tuple decompositions and as canonical finite families over the prime factors | `PrimeArithmetic/Density/CoprimeBaseProducts`, `PrimeArithmetic/Density/ZModUnitCRT`, `PrimeArithmetic/Density/WheelBases`, `PrimeArithmetic/Density/WheelUnitCRT`, `PrimeArithmetic/Density/WheelUnitProductEquiv`, `PrimeArithmetic/Density/WheelResidueClassifier` | proved |
| fixed layout makes the construction linear in the middle block | for fixed base, boundary digits, zero padding, and middle width, the symmetric template has affine form `shift + gradient * seed` | `PrimeArithmetic/Structure/AffineTemplate` | proved |
| bounded-`k` compactness is exact | the padding coordinates determine an exact width / diameter, `k = (0, 0)` minimizes that compactness quantity, and increasing either padding coordinate weakly pushes the template positions outward | `PrimeArithmetic/Structure/BoundedKTemplate` | proved |
| base divisors see only the outer digit, while coprime moduli see a seed permutation | every divisor of the base yields a seed-independent residue determined by the outer digit, while every modulus coprime to the base yields an affine permutation of seed residues and a unique seed class for each target residue | `PrimeArithmetic/Structure/AffineResidueSearch`, `PrimeArithmetic/Structure/AffineSeedClasses` | proved |
| local affine lane comparison has an exact map-equality and zero-seed grammar | at one coprime modulus, local affine map equality is exactly shift-plus-gradient equality; equal shift and gradient force equal zero-seed classes, and under equal gradients zero-seed equality is equivalent to shift equality | `PrimeArithmetic/Structure/AffineLaneComparison` | proved |
| local affine gradient equality is exactly period lock | for one coprime modulus, local affine gradients agree exactly when the two middle positions are congruent modulo the multiplicative order of the base unit in `ZMod modulus`; in the locked regime the local relation label collapses to `identity` vs `gradientOnly` according to shift agreement, and in the unlocked regime it collapses to `shiftOnly` vs `shiftAndGradient` | `PrimeArithmetic/Structure/AffinePeriodLock` | proved |
| bounded-`k` lane comparisons inherit the same period-lock grammar | the bounded-`k` gradient position is exactly `kOuter + kInner + 2`, direct bounded-`k` lane comparisons inherit the same period-lock criterion, and the special case `k = (0, 0) → (kOuter, kInner)` is equivalent to `kOuter + kInner ≡ 0` modulo the multiplicative order of the base unit | `PrimeArithmetic/Structure/BoundedKPeriodLock` | proved |
| the base-22 / mod-5 residual pocket has an exact local affine law | for base `22`, middle width `M = 2`, and the lane comparison `k = (0, 0) → (2, 2)`, the mod-`5` gradients agree, the shift difference is exactly `-inner`, zero-seed equality is equivalent to `5 ∣ inner`, and the local relation is `identity` iff `5 ∣ inner` and `gradientOnly` otherwise | `PrimeArithmetic/Density/Base22GradientPocket` | proved |
| aligned mask transfer has exact finite combinatorics | shared admissible overlap, admissible deltas, same-mask counts, zero-union counts, and signed good-delta decompositions are exact finite identities on the five transfer buckets `stableZero`, `gainZero`, `lossZero`, `stableNonzero`, `nonzeroChurn` | `PrimeArithmetic/Structure/FiniteMaskTransfer`, `PrimeArithmetic/Generated/BoundedKTransferShell` | proved |
| direct bounded-`k` lane comparisons have an exact local profile object | for fixed base, pair, middle width, and lane, each coprime modulus contributes a precise forbidden seed class, and lane agreement can be stated as equality of those local profiles | `PrimeArithmetic/Structure/BoundedKResidueProfile`, `PrimeArithmetic/Generated/BoundedKResidueProfileShell` | proved |
| local profile agreement forces transfer collapse, but the full maintained `M = 3` surface does not satisfy that hypothesis universally | exact profile agreement implies candidatewise mask equality, zero gain/loss/churn, and zero admissible delta; the tracked base-6 and base-30 direct-lane witnesses show that a stronger public `M >= 3` theorem is not yet supported on the full maintained direct surface | `PrimeArithmetic/Structure/BoundedKTransferCollapse`, `PrimeArithmetic/Density/BoundedKTwoPrimeCollapse`, `PrimeArithmetic/Density/BoundedKWheelCollapse`, `PrimeArithmetic/Generated/BoundedKProfiles/Base6M3Pair15K00ToK01`, `PrimeArithmetic/Generated/BoundedKProfiles/Base30M3Pair11K00ToK01` | proved conditional criterion; negative boundary tracked |
| odd-only segmented sieve arithmetic is exact | odd candidates in a segment are exactly `lo + 2 * idx`, the inverse index is `(n - lo) / 2`, the first marked odd multiple of an odd prime is explicit, and later marks advance by `2p` | `PrimeArithmetic/Sieve/SegmentedSieve` | proved |
| the runtime odd-segment layout has an exact arithmetic capacity | the Rust constants `SEG_BYTES`, `SEG_BITS`, and `SEG_ODDS` match a segment of `262144` odd candidates, the last valid index gives the expected arithmetic span, and every in-range odd candidate has index below the segment capacity | `PrimeArithmetic/Sieve/SegmentLayout` | proved |
| the runtime cross-off loop has an exact arithmetic start and progression | the executable sieve starts at `p^2` once `p^2` enters the segment, otherwise at the first multiple at or above `segLo`, shifts even starts by one `p`, and then marks the arithmetic progression with stride `2p` | `PrimeArithmetic/Sieve/RuntimeCrossOff` | proved |
| the runtime collection loop has an exact odd upper bound and index witness | the executable sieve truncates the raw segment upper bound to an odd endpoint and then every odd candidate in that adjusted interval is exactly `lo + 2 * idx` for some bounded collection index | `PrimeArithmetic/Sieve/RuntimeCollection` | proved |
| the odd-only writer and reader use the same byte/bit coordinates | after the shared odd index `idx = (n - lo) / 2`, both `mark_composite` and `is_prime` address byte `idx / 8` and bit `idx % 8`; those coordinates reconstruct the index and determine the odd candidate in an odd segment | `PrimeArithmetic/Sieve/SegmentBitCoordinates` | proved |
| the odd-only mask update sets exactly the bit that the reader tests | once the shared odd-only bit coordinate is fixed, the executable mask `1 << bit` and the executable readback `((byte >> bit) & 1)` are equivalent to setting and testing the same target bit, while all other bits are preserved | `PrimeArithmetic/Sieve/SegmentBitMasks` | proved |
| bounded multi-mark writes are exact on disjoint byte slots | in any bounded sieve-style byte family, one-mark writes are correct, writes to other byte slots preserve existing reads, and a finite family of pairwise slot-disjoint marks can all be read back as `1` after the whole update sequence | `PrimeArithmetic/Sieve/BoundedByteFamilies` | proved |
| same-byte collisions collapse cleanly into aggregated mask writes | in any bounded sieve-style byte family, a finite list of target bits in one byte can be OR-compressed into a single mask update, every listed bit reads back as `1`, and the repeated same-byte write trace agrees exactly with that aggregated update | `PrimeArithmetic/Sieve/BoundedByteMasks` | proved |
| grouped multi-byte plans unify the disjoint-slot and same-byte routes | a bounded family of per-byte plans flattens exactly to the corresponding single-bit write trace, and when the planned byte slots are pairwise distinct every planned bit reads back as `1` after the whole family update | `PrimeArithmetic/Sieve/BoundedBytePlans` | proved |
| a tiny shared coordinate shell bridges local runtime coordinates into grouped plans | fixed local read/write shells can discharge written-readback once via their `ByteMark` bridge, and grouped coordinate plans can reuse the generic grouped-plan theorem family without repeating byte-plan boilerplate | `PrimeArithmetic/Sieve/BoundedByteCoordinates` | proved |
| the odd-only single-byte update matches the executable write/read pattern and embeds into the generic family | after selecting the bounded target byte in the odd segment, updating that byte with the proved mask semantics changes exactly that slot, readback from the same slot returns `1`, and the runtime shell agrees with the generic bounded byte-family API | `PrimeArithmetic/Sieve/SegmentByteArray` | proved |
| short odd-only runtime mark families sit directly on grouped byte plans | in-range runtime odd candidates, including those coming from `runtimeMarkedBy`, can be bucketed by byte slot and then discharged by the generic grouped-plan readback theorem | `PrimeArithmetic/Sieve/SegmentRuntimePlans` | proved |
| bounded runtime cross-off steps lift directly to grouped byte plans | if `runtimeMarkedBy p segLo step` stays inside the current odd segment, that bounded step can be packaged directly as a grouped-plan coordinate, canonically bucketed by byte slot, and read back as marked without manually rewrapping it as a segment candidate first | `PrimeArithmetic/Sieve/SegmentRuntimeSteps` | proved |
| wheel30 compression is exactly the odd domain with `3` and `5` removed | the surviving residue classes modulo `30` are exactly the units `{1,7,11,13,17,19,23,29}`, wheel30 representability is equivalent to oddness together with `mod 3` and `mod 5` exclusion, and the local residues mod `2`, `3`, and `5` reconstruct the canonical `mod 30` class through the base-30 CRT representative | `PrimeArithmetic/Sieve/Wheel30Residues`, `PrimeArithmetic/Sieve/Wheel30Agreement` | proved |
| the runtime wheel30 bit layout has an exact index formula | the wheel30 slot order is fixed, candidates have the form `base + 30 * cycle + residue`, the linear index is `cycle * 8 + slot`, the byte / bit split is exactly `idx / 8`, `idx % 8`, and any representable candidate inside one aligned segment recovers a bounded runtime `(cycle, slot)` with that exact index | `PrimeArithmetic/Sieve/Wheel30Indexing` | proved |
| the wheel30 writer and reader use the same byte/bit coordinates | for a runtime wheel30 candidate, the writer path through `wheel_index` and the reader path through `wheel_idx = cycle * 8 + slot` both recover the same `some (cycle, slot)` byte/bit coordinates | `PrimeArithmetic/Sieve/Wheel30BitCoordinates` | proved |
| the wheel30 mask update sets exactly the bit that the reader tests | once a wheel30 candidate has a concrete `some bit` coordinate, the executable mask `1 << bit` and the executable readback `((byte >> bit) & 1)` are equivalent to setting and testing the same bit, with candidate-level corollaries for the runtime slot order | `PrimeArithmetic/Sieve/Wheel30BitMasks` | proved |
| the wheel30 single-byte update matches the executable write/read pattern and embeds into the generic family | after selecting the bounded byte slot attached to a runtime wheel cycle, updating that byte with the slot mask changes exactly that slot, readback at the same `(cycle, slot)` returns `1`, and the runtime shell agrees with the generic bounded byte-family API | `PrimeArithmetic/Sieve/Wheel30ByteArray` | proved |
| short wheel30 runtime mark families sit directly on grouped byte plans | runtime `(cycle, slot)` coordinates can be bucketed by cycle byte slot and then discharged by the generic grouped-plan readback theorem | `PrimeArithmetic/Sieve/Wheel30RuntimePlans` | proved |
| wheel30 runtime families can be rebased without changing grouped readback | rebasing a grouped runtime `(cycle, slot)` family across the phantom `base` parameter leaves both the by-byte regrouped write and the already-grouped plan write unchanged, and transports the needed membership witnesses directly to the rebased family | `PrimeArithmetic/Sieve/Wheel30RuntimeRebase`, `PrimeArithmetic/Sieve/Wheel30RuntimePlans` | proved |
| fixed-width connector scans admit exact residue filters | forward and reverse concatenations reduce to the same boundary-plus-connector sum whenever `base ≡ 1 (mod m)`; for the canonical decimal pair, this yields exact `mod 3` and `mod 9` exclusion classes for connector values | `PrimeArithmetic/Connector/ConcatenationFilters` | proved |
| whole connector families inherit the same residue logic | once the left/right pair has a known residue class modulo `m`, forward and reverse concatenations reduce to a generic connector-shift profile, giving reusable admissibility lemmas for the entire family | `PrimeArithmetic/Connector/ConcatenationFamilies` | proved |
| the connector-family API works on maintained non-canonical pairs | the same family theorems instantiate cleanly on the zero-padded membrane pair, a twin-prime pair, and a Sophie Germain pair, producing explicit `mod 3` / `mod 9` connector classes beyond the canonical asymmetry example | `PrimeArithmetic/Connector/ConcatenationProfileExamples` | proved |
| runtime prime windows can be moved into the Lean certificate shell without hand transcription | a runtime exporter emits Lean modules that define a `GeneratedWindowPayload`, a compact dual-evidence object, and the resulting midpoint-exclusion / inviolability theorems directly from extracted positions and residues; the tracked catalog now spans bases `6`, `10`, `12`, `30`, and `210` | `PrimeArithmetic/Generated/Examples/WindowP3Base6Span5`, `PrimeArithmetic/Generated/Examples/WindowP5Base10Span5`, `PrimeArithmetic/Generated/Examples/WindowP5Base12Span17`, `PrimeArithmetic/Generated/Examples/WindowP11Base30Span5`, `PrimeArithmetic/Generated/Examples/WindowP101Base30Span29`, `PrimeArithmetic/Generated/Examples/WindowP163Base30Span35`, `PrimeArithmetic/Generated/Examples/WindowP41Base210Span5`, `src/bin/export_window_certificate.rs` | implemented |
| bounded-`k` transfer witnesses can be moved into Lean without hand transcription | a runtime exporter emits `GeneratedTransferPayload` modules for aligned `k = (0, 0) → best_k` witnesses, and the tracked catalog now distinguishes overlap-led and boundary-led `M = 2` cases at bases `10`, `14`, `22`, and `34` using exact bucket counts and integer sign comparisons | `PrimeArithmetic/Generated/BoundedKTransferShell`, `PrimeArithmetic/Generated/BoundedK/Base10M2Pair33`, `PrimeArithmetic/Generated/BoundedK/Base14M2PairDB`, `PrimeArithmetic/Generated/BoundedK/Base22M2PairHJ`, `PrimeArithmetic/Generated/BoundedK/Base34M2PairP9`, `src/bin/export_bounded_k_transfer_witness.rs` | implemented |
| the analytic layer can be stated conservatively in classical notation | pair-count conventions, odd-prime Goldbach local factors, radical invariance of the local-factor support, and the standard logarithmic and Poisson-style transforms can be fixed in Lean without asserting a new density theorem | `PrimeArithmetic/Analysis/HardyLittlewoodShell` | proved |

## Mathematician-Facing Entry Points

If the goal is to show a mathematician the strongest current Lean surface with
minimal repository-specific vocabulary, the best modules to start with are:

- `PrimeArithmetic/Symmetry/MidpointObstruction`
- `PrimeArithmetic/Symmetry/ModularReflection`
- `PrimeArithmetic/Symmetry/MirrorObstruction`
- `PrimeArithmetic/Symmetry/BalancedBucketReflection`
- `PrimeArithmetic/Symmetry/WindowCertificate`
- `PrimeArithmetic/Symmetry/WindowCertificateGenerated`
- `PrimeArithmetic/Symmetry/WindowCertificateErgonomics`
- `PrimeArithmetic/Density/ZModUnitNegation`
- `PrimeArithmetic/Density/ZModUnitCRT`
- `PrimeArithmetic/Density/WheelUnitProductEquiv`
- `PrimeArithmetic/Density/ZModUnitOrbits`
- `PrimeArithmetic/Density/ZModUnitAction`
- `PrimeArithmetic/Structure/AffineTemplate`
- `PrimeArithmetic/Structure/AffineLaneComparison`
- `PrimeArithmetic/Structure/AffinePeriodLock`
- `PrimeArithmetic/Structure/BoundedKPeriodLock`
- `PrimeArithmetic/Structure/BoundedKTemplate`
- `PrimeArithmetic/Structure/FiniteMaskTransfer`
- `PrimeArithmetic/Structure/AffineResidueSearch`
- `PrimeArithmetic/Structure/BoundedKResidueProfile`
- `PrimeArithmetic/Structure/BoundedKTransferCollapse`
- `PrimeArithmetic/Density/BoundedKTwoPrimeCollapse`
- `PrimeArithmetic/Density/BoundedKWheelCollapse`
- `PrimeArithmetic/Generated/BoundedKResidueProfileShell`
- `PrimeArithmetic/Generated/BoundedKTransferShell`
- `PrimeArithmetic/Sieve/SegmentedSieve`
- `PrimeArithmetic/Sieve/SegmentLayout`
- `PrimeArithmetic/Sieve/RuntimeCrossOff`
- `PrimeArithmetic/Sieve/RuntimeCollection`
- `PrimeArithmetic/Sieve/SegmentBitCoordinates`
- `PrimeArithmetic/Sieve/SegmentBitMasks`
- `PrimeArithmetic/Sieve/BoundedByteFamilies`
- `PrimeArithmetic/Sieve/BoundedByteMasks`
- `PrimeArithmetic/Sieve/BoundedBytePlans`
- `PrimeArithmetic/Sieve/BoundedByteCoordinates`
- `PrimeArithmetic/Sieve/SegmentByteArray`
- `PrimeArithmetic/Sieve/SegmentRuntimePlans`
- `PrimeArithmetic/Sieve/SegmentRuntimeSteps`
- `PrimeArithmetic/Sieve/Wheel30Agreement`
- `PrimeArithmetic/Sieve/Wheel30Indexing`
- `PrimeArithmetic/Sieve/Wheel30BitCoordinates`
- `PrimeArithmetic/Sieve/Wheel30BitMasks`
- `PrimeArithmetic/Sieve/Wheel30ByteArray`
- `PrimeArithmetic/Sieve/Wheel30RuntimePlans`
- `PrimeArithmetic/Sieve/Wheel30RuntimeRebase`
- `PrimeArithmetic/Connector/ConcatenationFilters`
- `PrimeArithmetic/Connector/ConcatenationFamilies`
- `PrimeArithmetic/Connector/ConcatenationProfileExamples`
- `PrimeArithmetic/Analysis/HardyLittlewoodShell`

These give:

- the abstract fixed-point obstruction
- the concrete modular reflection classifier and its mirror-family corollary
- the automatic bucket-to-certificate symmetry layer, its narrow window shell,
  the generated-data entrypoint from residue and position lists, and the
  compact proof-object layer for exported artifacts
- the standard unit-group negation formulation
- the CRT decomposition on unit groups
- the canonical finite-family wheel-base CRT theorem
- the explicit `φ(B) / 2` orbit count
- the same orbit count in group-action language
- the exact affine dependence on the middle block
- the exact local shift / gradient / zero-seed comparison language behind the affine hinge atlas
- the exact local period-lock law controlling when affine gradients can agree at all
- the exact bounded-`k` compactness / diameter layer
- the exact five-bucket mask-transfer combinatorics beneath the bounded-`k` reports
- the modular search theorem that turns the affine form into explicit residue
  and seed-class control
- the direct lane-comparison residue-profile object for bounded `k`
- the universal conditional transfer-collapse criterion and its matched `2p`
  / wheel-base wrapper families
- the generated direct-lane witness shell and tracked counterexamples pinning
  the current negative theorem boundary
- the exact odd-segment encoding and first-mark arithmetic behind the
  segmented sieve candidate domain
- the runtime odd-segment capacity and raw range bound used by the bit-packed
  segment
- the runtime cross-off start branch and exact `2p` marking progression
- the runtime odd-endpoint adjustment and exact collection-index witness
- the exact byte/bit coordinates shared by the odd-only segment writer and reader
- the step-indexed odd-only cross-off family surface, so bounded
  `runtimeMarkedBy` steps can be canonically bucketed by byte slot and
  discharged without manual candidate rewrapping
- the equivalence between wheel30 candidates and the filtered odd domain
- the runtime wheel30 linear index and byte/bit decomposition, together with the converse recovery of bounded `(cycle, slot)` data for representable in-segment candidates
- the exact byte/bit coordinates shared by the wheel30 writer and reader
- the base-invariant rebasing shell for grouped wheel30 runtime families
- the exact residue filters for fixed-width connector concatenation, including
  the canonical decimal `mod 3` / `mod 9` exclusions
- the reusable connector-family admissibility layer
- explicit maintained connector-profile examples beyond the single canonical pair
- the generated-data shell and witness catalog for exact bounded-`k` transfer counts
- a conservative Hardy-Littlewood notation shell with exact local-factor support
  bookkeeping

## Candidate Future Tranches

The current Lean package is now strong enough that the next gains should come
from extending standard theorem families and export ergonomics, not from adding
new repository metaphors. The highest-value next tranches are:

| Priority | Target theorem family | Intended Lean surface | Why it is durable |
|----------|-----------------------|-----------------------|-------------------|
| 1 | concrete first-step or first-byte runtime families, only if needed | add more explicit small runtime family lemmas only when a later theorem or executable agreement argument really needs them | the generic grouped-plan and coordinate shell layers are already in place, so further work here is now selective rather than foundational |
| 2 | optional further catalog growth | add more exported windows only when they support a concrete later argument, not just to increase file count | this keeps the generated catalog curated rather than sprawling |
| 3 | exact Lagrange extraction | revisit the Lagrange-facing code only by extracting connector or residue lemmas that can be restated in standard arithmetic terms | this preserves potentially useful signal without formalizing the simulation metaphor |
| 4 | repo-wide verification and staging hygiene | rerun the full Rust, Lean, and Agda verification surfaces and stage the remaining work in a clean split | the highest-value structural simplifications are now mostly done, so the next durable gain is integration confidence |

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
6. `PrimeArithmetic/Symmetry/WindowCertificateGenerated`
7. `PrimeArithmetic/Symmetry/WindowCertificateErgonomics`
8. `PrimeArithmetic/Density/UnitResidues`
9. `PrimeArithmetic/Density/ZModUnitNegation`
10. `PrimeArithmetic/Density/ZModUnitCRT`
11. `PrimeArithmetic/Density/WheelUnitProductEquiv`
12. `PrimeArithmetic/Density/ZModUnitAction`
13. `PrimeArithmetic/Structure/AffineTemplate`
14. `PrimeArithmetic/Structure/AffineLaneComparison`
15. `PrimeArithmetic/Structure/AffinePeriodLock`
16. `PrimeArithmetic/Structure/BoundedKPeriodLock`
17. `PrimeArithmetic/Structure/AffineResidueSearch`
18. `PrimeArithmetic/Sieve/SegmentedSieve`
19. `PrimeArithmetic/Sieve/SegmentLayout`
20. `PrimeArithmetic/Sieve/RuntimeCrossOff`
21. `PrimeArithmetic/Sieve/RuntimeCollection`
22. `PrimeArithmetic/Sieve/SegmentBitCoordinates`
23. `PrimeArithmetic/Sieve/SegmentBitMasks`
24. `PrimeArithmetic/Sieve/BoundedByteFamilies`
25. `PrimeArithmetic/Sieve/BoundedByteMasks`
26. `PrimeArithmetic/Sieve/BoundedBytePlans`
27. `PrimeArithmetic/Sieve/BoundedByteCoordinates`
28. `PrimeArithmetic/Sieve/SegmentByteArray`
29. `PrimeArithmetic/Sieve/SegmentRuntimePlans`
30. `PrimeArithmetic/Sieve/SegmentRuntimeSteps`
30. `PrimeArithmetic/Sieve/Wheel30Agreement`
31. `PrimeArithmetic/Sieve/Wheel30Indexing`
32. `PrimeArithmetic/Sieve/Wheel30BitCoordinates`
33. `PrimeArithmetic/Sieve/Wheel30BitMasks`
34. `PrimeArithmetic/Sieve/Wheel30ByteArray`
35. `PrimeArithmetic/Sieve/Wheel30RuntimePlans`
36. `PrimeArithmetic/Sieve/Wheel30RuntimeRebase`
37. `PrimeArithmetic/Connector/ConcatenationFilters`
38. `PrimeArithmetic/Connector/ConcatenationFamilies`
39. `PrimeArithmetic/Connector/ConcatenationProfileExamples`
40. `PrimeArithmetic/Analysis/HardyLittlewoodShell`

## Related Files

- [`README.md`](README.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`PrimeArithmetic.lean`](PrimeArithmetic.lean)
