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

### Affine Membrane Prime Family

A fixed membrane lane viewed as a family of prime candidates as the middle seed
varies. Once the base, boundary digits, middle length, and zero-run padding are
fixed, every candidate in the family has the affine form:

```text
N(s) = A + G*s
```

Use this as the precise collaborator term for the maintained structured prime
families in this repo.

### Symmetric Zero-Run Template Prime

Accessible name for a prime that appears inside an affine membrane prime
family. It highlights the visible digit shape:

```text
outer 0...0 inner 0...0 seed 0...0 inner 0...0 outer
```

This is adjacent to palindromic primes, but different: the symmetric frame is
constrained, while the middle seed block need not itself be palindromic.

### Prime Witness

An individual prime found inside a structured family. For example,
`3007000000002907003` is a prime witness for the base-10 family
`3 00 7 0 [seed] 0 7 00 3`.

### Prime Witness Engine

Repo section for the workflow:

```text
seed origin -> affine membrane lane -> residue funnel -> probable-prime witness -> verification transcript
```

The demo entrypoint is `seed-to-witness`; the measurement entrypoint is
`large_affine_witness_ladder_report`. See
[`docs/PRIME_WITNESS_ENGINE.md`](docs/PRIME_WITNESS_ENGINE.md).

### Seed Origin

The starting seed supplied to the Prime Witness Engine by a user, timestamp, or
test fixture. It is a start point on the affine lane, not a guarantee that the
exact seed is a witness.

### Witness Seed

The seed that actually produces the returned witness after walking forward from
the seed origin. The witness seed may equal the seed origin, or it may be a
later seed in the same affine membrane lane.

### Residue Funnel

Exact small-prime filtering applied before primality or probable-prime
confirmation. The funnel removes seed classes that are provably divisible by
configured small moduli, then sends survivors to the confirmation tier.

### Confirmation Tier

The local label for how a witness was checked: deterministic `u64`, fixed-width
`u128`, or BigUint Miller-Rabin probable-prime. Above `u64`, current docs say
probable-prime witness unless a proof certificate is added.

### Mersenne Class

Exact label for whether a witness is of the form `2^p - 1`. This classification
is independent of primality confirmation: if `N + 1` is not a power of two, the
witness is definitely `not_mersenne`, even when the prime check above `u64` is
still labeled probable-prime.

### Semantic Rarity

The fact that a witness belongs to a tiny named construction slice of the
same-digit space, such as a specific base, boundary pair, zero-run shape, middle
length, and seed. It is a construction-family claim, not a density theorem.

### Density Drift

Measured variation in prime rate across specified affine membrane prime
families and their controls. This is a report-surface term: it describes how
candidate yield changes with base, boundary digits, middle length, zero-run
padding, and control design. It does not by itself assert a density theorem.

### Wheel-Compressed Affine Surface

A fixed affine membrane family in a base such as `30 = 2 * 3 * 5`, where the
base itself removes the first small-prime residue traps whenever the boundary
digits are units. This is classical wheel behavior expressed through the
membrane grammar, not a claim of residual density magic.

### Ordered-Pair Phase Asymmetry

Measured difference between a compact lane `(outer, inner)` and its reversal
`(inner, outer)`. The two lanes share the same compact grammar and gradient,
but their affine shifts differ, so they can pass through later residue gates in
different phases. This is a local residue-phase observation, not a standalone
density theorem.

### Reversal Residual

The signed remainder in an ordered-pair phase comparison after separating the
raw prime-rate delta from ordinary size/PNT expectation and exact small-prime
residue survival. It is a lead for further controls, not a theorem that one
role assignment is intrinsically better.

### Affine Phase Residual

Cross-base generalization of reversal residual for compact membrane lanes. It
compares two lanes with the same grammar and gradient but swapped ordered
boundary roles, then ranks what remains after size and exact residue-survivor
accounting. This is signal-discovery language, not theorem language.

### Shift-Phase Residual

Preferred public phrase for the curated affine phase residual follow-up:
same slope, different intercept, different residue weather. It compares
same-gradient lanes and asks whether the shifted affine intercept leaves a
measurable survivor-yield difference after size and residue accounting.

### Unit-Cycle Phase Signal

Base-normalized follow-up to shift-phase residuals. Unit digits are placed on
their ordered unit-residue cycle for the base, then same-gradient swaps are
compared by arc geometry, edge/complement status, residue gate profile, and
survivor yield. This is cross-base signal-mining language, not theorem
language.

### Unit-Cycle Base Neighbor

Adjacent or nearby bases compared as normalized unit-cycle geometries. The
circle radius is fixed to `1`; what changes is the number of unit-residue beads,
their arc/chord spacing, and which digit pairs become diameters, complements,
edges, or interior arcs. This is a geometry scout for later residue and density
questions, not a radix-conversion shortcut.

### Base57 Affine Codec

Experimental notation layer that compares ordinary base58/base57 transcoding
against identifiers generated directly inside a constrained base-57 affine
membrane grammar. The baseline base57 alphabet is Bitcoin-style base58 with
`z` removed, while preserving leading-zero byte semantics with leading `1`
characters. The affine notation uses framed chunks such as `a57r1:<len>:<body>`
for residue-filtered identifiers and `a57p1:<len>:<body>` for prime-witness
identifiers. This is a structured identifier namespace with fast residue
validation, not compression magic or a shortcut around radix conversion.

### Canonical Payload

The base-invariant byte string or integer value being represented. Base16 hex,
base58, base57, and affine envelopes can all render the same canonical payload.
The spelling changes; the decoded bytes must not.

### Base Rendering

An ordinary reversible textual representation of a canonical payload, such as
hex/base16, base58, or base57. A base rendering is a costume for the invariant
payload, not a new arithmetic object.

### Affine Envelope

A structured representative of a canonical payload inside an affine membrane
grammar. In the base57 codec experiment, payload bytes are packed into seed
chunks with nonce bits, then emitted as residue-filtered or prime-witness
chunks. An affine envelope is usually longer than an ordinary base rendering
because it carries validation structure.

### Same-Gradient Pair

Two membrane lanes with the same affine gradient `G` in `N(s)=A+G*s` but
different affine shifts `A`. Compact ordered-role reversals are the main
current example: `(outer, inner)` versus `(inner, outer)`.

### Shift Phase

The residue-facing position of the affine shift `A` for a fixed lane. Changing
the shift while holding the gradient fixed changes which seed classes are
excluded by small-prime residue gates.

### Residue Gate Profile

The small-prime moduli, affine shift residues, gradient residues, and excluded
seed classes attached to a lane. It is the exact finite bookkeeping layer
between raw candidate generation and primality testing.

### Survivor Yield

The prime rate among seeds that survive the exact residue gate profile used in
a report. It is narrower than raw prime density because obvious small-prime
composites have already been filtered out.

### Mature Lane

A larger middle-length follow-up lane used to check whether a short-lane lead
persists beyond tiny seed spaces. In the shift-phase signal mining report,
`M=4` is the default mature follow-up when it remains deterministic `u64`-safe.

### Phase Residual Lead

A ranked empirical lead where a same-gradient comparison still looks
interesting after size/PNT and exact residue-survivor layers are separated.
This is explicitly a research queue item, not a density theorem.

### Great Construction

Internal shorthand for a high-yield affine membrane surface under specified
controls. Use it only with the relevant measurement surface attached, such as
deterministic `u64` density-atlas rows or matched-control exports.

### Configuration

The fixed part of a membrane construction: base, boundary digits, and padding
parameters.

Example: `(outer, inner) = (3, 7)` with `k=(1,1)`.

### Seed

The variable middle part of a membrane candidate. The seed is an integer index
written as exactly `M` digits in the configured base.

- single-digit seed: `5`
- base 10, `M=2`, seed `10` is written as `"10"`
- base 6, `M=2`, seed `10` decimal is written as `"14"`
- base 22, `M=2`, seed `10` decimal is written as `"0A"`

This base-aware distinction is important: the seed index is an ordinary
integer, but its middle digit representation depends on the base.

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

### Connector hit

Arithmetic-first name for one bounded connector case:

```text
(pair, width, position, digit, direction)
```

In the current repo, this specifically means a zero-padded single-digit
connector in a fixed-width base-10 scan.

### Canonical pair

The prime pair used most heavily in connector experiments:

- left: `10301`
- right: `3007003007003`

Several connector findings currently apply only to this pair.

### Directional asymmetry

An observed difference between forward and reverse concatenation success rates.
The repo treats this as a real empirical effect for the canonical pair, but not
yet as a general law.

### Residue-admissible

A connector candidate that survives the exact small-modulus exclusion layer.

For the maintained base-10 connector lane, the exact proved filters currently
highlight `mod 3` and `mod 9` behavior.

### Resonance position

A width/position bucket with multiple working digits in the same matched scan.
This is an empirical pattern label, not a proved general mechanism.

### Lagrange point

Repository shorthand for a productive insertion position in a connector buffer.
This is an analogy borrowed from physics, not a literal physical model.

Use this term carefully:

- acceptable: "Lagrange point" as the repo's connector vocabulary
- acceptable: using it as an optional alias for a connector hit or resonance
  position
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
