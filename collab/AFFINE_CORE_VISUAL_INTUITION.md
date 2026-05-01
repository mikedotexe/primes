# Affine Core And Visual Intuition

**Status**: collaborator-facing internal note
**Scope**: affine seed-search geometry, residue-torus visualization, and
audience-facing explanation discipline
**Claim boundary**: exact local arithmetic yes; new prime-density theorem no

## One-Sentence Anchor

We are not visualizing where primes magically appear. We are visualizing the
arithmetic surface that a fixed symmetric template forces every candidate to
live on.

That sentence is the bridge between wonder and auditability. It lets the visual
story stay alive without asking the reader to accept more than the math proves.

## Family Language

The clean internal name for the maintained object is:

```text
affine membrane prime family
```

That means a fixed symmetric zero-run membrane lane viewed as a family while
the middle seed varies. The accessible gloss is:

```text
symmetric zero-run template primes
```

Individual primes found inside the family are **prime witnesses**. For example,
`3007000000002907003` is a witness in the family:

```text
3 00 7 0 [seed] 0 7 00 3
```

This is analogous to saying "palindromic primes" are a family, but the rule is
different. Palindromic primes require the whole digit string to read the same
forward and backward. Affine membrane prime families require a symmetric
zero-run frame around a seed; the seed block can be arbitrary and need not be
palindromic.

## The Affine Core

A fixed membrane/template lane has:

- a base `b`
- outer and inner boundary digits
- a middle length `M`
- a padding lane `k=(k_outer,k_inner)`
- a seed `s` filling the middle digits

The maintained grammar is:

```text
outer + 0^k_outer + inner + 0^k_inner + seed
      + 0^k_inner + inner + 0^k_outer + outer
```

The zero-run lengths mirror around the seed, and in the current maintained
membrane grammar the boundary digits mirror too. A number with symmetric
zero-run placement but non-mirrored boundary digits is a nearby scaffold
generalization, not this exact lane family. That generalization may be worth
studying, but it should be named separately.

Once the lane is fixed, the seed is the only moving part. The candidate is:

```text
N(s) = A + G*s
```

Here `A` is the fixed shift and `G` is the fixed gradient. More explicitly, if
the left fixed prefix has value `P`, the right fixed suffix has value `Q`, and
the suffix has length `r`, then:

```text
N(s) = P*b^(M+r) + s*b^r + Q
A    = P*b^(M+r) + Q
G    = b^r
```

This is why the poetic-looking symmetric construction becomes exact modular
geometry. The template is not just a shape; it is an affine seed line.

## Residue Filters

Modulo a small prime `p`, the same lane becomes:

```text
N(s) = A_p + G_p*s mod p
```

When `gcd(G_p,p)=1`, exactly one seed class is excluded:

```text
s = -A_p / G_p mod p
```

This is the smallest rigorous unit of the signal story. Prime generation still
requires ordinary primality, but the template gives exact residue filters before
the primality test ever runs.

## Comparing Lanes

Two `k` lanes can be compared by their local affine data:

```text
N_from(s) = A_from + G_from*s
N_to(s)   = A_to   + G_to*s
```

The gradient is a power of the base, so gradient equality modulo `p` is not a
mystery. It is controlled by multiplicative order:

```text
G_from = G_to mod p
iff b^delta = 1 mod p
iff delta = 0 mod ord_p(b)
```

where `delta` is the difference in the two middle positions.

That is the period lock. It is an exact local condition, not a fitted pattern.

## Local Relation Vocabulary

The affine comparison has four useful local relation labels:

| Shift | Gradient | Label | Reading |
|---|---|---|---|
| same | same | `identity` | Same local affine lane modulo `p` |
| different | same | `gradient_only` | Same step, different start |
| same | different | `shift_only` | Same start, different step |
| different | different | `shift_and_gradient` | Both local features differ |

For human explanation, `gradient_only` can be read as parallel modular motion:
the lanes walk through seed space with the same modular step, but they begin on
different residues.

## What The Visuals Show

The preferred visualization ladder is:

| Step | Visual | Purpose |
|---|---|---|
| construction | `construction_template.png` | Show the zero-run grammar and the center seed |
| affine line | `affine_line.png` | Show `N(s)=A+G*s` after the lane is fixed |
| residue filters | `residue_filter_wheel.png` | Show the excluded seed class for a modulus |
| lane comparison | `canonical_walkthrough.png` | Compare shifts and gradients between two lanes |
| period lock | `unwrapped_torus.png` | Flatten `delta mod ord_p(base)` for skeptical reading |
| torus | `residue_torus.png` | Restore the cyclic visual intuition |
| witnesses | `example_gallery.png` | Show real prime constructions |

### Residue Torus

The residue torus is a cyclic picture of:

```text
delta mod ord_p(base)
```

The lock meridian is residue `0`. Points on that meridian have matching affine
gradients. Points off that meridian do not.

The current maintained report surface is especially clean: observed affine
gradient equality lands exactly on the lock meridian. In the current run:

- comparison rows: `6672`
- modulus rows: `60048`
- locked rows: `17100`
- period-lock mismatches: `0`

These counts are report-surface facts, not density claims.

### Unwrapped Torus

The unwrapped torus turns the same cyclic condition into a chart:

```text
x-axis: delta mod order / order
y-axis: ord_p(base)
```

This is the accessibility view. A skeptical reader can see that gradient
agreement is an order/residue phenomenon rather than a decorative plot.

### Canonical Walkthrough

The base-22/mod-5 pocket is the teaching example:

```text
base = 22
modulus = 5
pair = (H,J)
M = 2
k=(0,0) -> k=(2,2)
ord_5(22) = 4
position delta = 4
delta mod order = 0
```

The two lanes are:

```text
k=(0,0): N(s) = 92063043 + 484*s
k=(2,2): N(s) = 4808275622545646169 + 113379904*s
```

Modulo `5`:

```text
k=(0,0): N(s) = 3 + 4*s mod 5
k=(2,2): N(s) = 4 + 4*s mod 5
```

Same gradient, different shift. That is `gradient_only`.

This is visually a torus story because the gradient lock is caused by cycling
back to the residue-0 meridian of the multiplicative-order circle. It is
arithmetically a shift story because the intercepts remain separated.

### Construction Gallery

Prime witnesses show that these are real constructions, not just residue
diagrams. Current maintained examples include:

| Example | Construction | Decimal value |
|---|---:|---:|
| Decimal visible zero-run | `300702007003` base `10` | `300702007003` |
| Decimal one-one zero-run | `3070050703` base `10` | `3070050703` |
| Decimal deep zero-run | `10070004007001` base `10` | `10070004007001` |
| Decimal nonpalindromic center | `300100030001003` base `10` | `300100030001003` |
| Base 22 side pocket | `H00J000D00J00H` base `22` | `4808275624019584921` |
| Base 22 compact lane | `HJ0AJH` base `22` | `92067883` |
| Base 10 identity pocket | `30301303` base `10` | `30301303` |
| Base 14 persistent core | `DB0180BD` base `14` | `1453260983` |
| Base 6 bridge witness | `15451` base `6` | `2551` |

The right reading is: the constructions are alive. The wrong reading is: these
witnesses prove a global density theorem.

## Curt-Style Explanation Ladder

Use this ladder when explaining the work to a smart skeptical audience:

1. We begin with symmetric digit templates.
2. Fixing a template turns seed search into `N(s)=A+G*s`.
3. Modulo small primes, this gives exact residue filters.
4. Comparing template lanes becomes comparing shifts and gradients.
5. Gradient agreement is governed exactly by multiplicative order.
6. The residue torus shows that order/residue condition as a visual object.
7. Prime witnesses show that the constructions actually produce primes.
8. Density claims remain conservative and must pass matched controls.

## What We Are Seeing

The strongest current reading is:

- symmetric templates are structured affine search surfaces
- period lock exactly explains where affine gradients can agree
- base-22/mod-5 is a vivid `gradient_only` side pocket
- identity pockets and gradient-only pockets differ by shift alignment
- prime witnesses demonstrate viable constructions
- density lift is still mainly attributed to coprimality, candidate size, and
  residue filtering unless a matched-control residual survives

The exciting part is not a secret formula for primes. The exciting part is that
a visual, symmetric construction has hardened into exact modular geometry that
can be audited row by row.

## Fast Generation Throughput Funnel

The maintained fast-generation story should be explained as a funnel:

```text
raw seeds -> residue-admissible seeds -> deterministic primality tests -> prime witnesses
```

For medium visible lanes, the candidate is deterministic and `u64`-bounded:

```text
N(s) = A + G*s
```

The generator can therefore precompute `A`, `G`, and a small-prime residue
wheel, then test only surviving seed classes with deterministic
`primal::is_prime(u64)`.

This is a throughput claim for specified lanes, not a density theorem. The old
`membrane-prime-optimized` and `membrane-prime-ultra` binaries should be read
as optimization sketches until their formulas are reconciled with the
maintained base-aware grammar.

## Construction Density Atlas

The density-atlas view adds the missing contrast class:

```text
good lanes / mediocre lanes / intentionally lousy lanes
```

It should be read as a map of density drift across controlled affine membrane
families. The report asks how much of a lane's prime rate is explained by:

- candidate size
- boundary digits that are units modulo the base
- exact small-prime residue survival
- same-digit-count random controls
- coprime random controls
- same-slot random controls
- same-budget scaffold controls

This is where "great construction" gets a precise internal meaning: a
high-yield affine membrane surface under specified controls. It is not a
synonym for a density theorem.

The useful visual sentence is:

```text
template family -> affine lane -> residue filters -> controls -> witnesses
```

The exciting part is that we can now show lousy constructions on purpose. They
make the good lanes easier to understand, because the reader can see which
parts of the grammar are doing ordinary residue work and which parts, if any,
remain interesting after controls.

## Base 30 Wheel Compact

The base-30 compact report is the cleanest current place to show the forest
inside one tree. Base `30 = 2 * 3 * 5` already acts like a classical wheel:
when the boundary digits are units, the construction avoids the first trivial
composite traps before any special membrane story begins.

The canonical walkthrough is `(B,7)`, `M=2`, `k=(0,0)`:

```text
N(s) = 272970221 + 900*s
```

That lane is strong, but the point of the report is that it is not lonely. The
all-pair heatmap scans all 64 ordered unit pairs, the residue funnel shows the
exact small-prime exclusions after the base wheel has done its first job, and
the witness gallery keeps real prime examples visible without turning them
into a density theorem.

The strong public sentence is:

```text
base 30 is a clean wheel-compressed affine candidate surface
```

The caution sentence stays beside it:

```text
this is a gorgeous classical wheel effect, not yet residual density magic
```

The reversal-asymmetry companion report asks a smaller question inside that
same surface: what changes when the same two unit residues swap roles? The
template remains symmetric after roles are chosen, but `(outer, inner)` and
`(inner, outer)` have different affine shifts with the same gradient. That
makes reversal asymmetry a residue-phase question, not a new global density
claim.

For compact base-30 lanes:

```text
N_{o,i}(s) - N_{i,o}(s) = (o-i)(30-1)(30^(M+2)-1)
```

The report's useful visual sentence is:

```text
same residue pair -> swapped affine shift -> changed residue phase -> changed witness yield
```

The reversal-residual companion report then asks how much of that witness-yield
difference survives the first honest decompositions:

```text
raw prime-rate delta
-> size / PNT-expected delta
-> residue-survivor delta
-> prime-rate-among-survivors residual
```

This is the right posture for the `(1,B)` versus `(B,1)` lead. The smaller
outer digit makes smaller candidates, so some advantage is expected before any
template-specific story begins. The affine shift still changes the residue
phase while the compact gradient remains fixed. Whatever is left after those
layers is a research lead to visualize and control, not a density theorem.

The cross-base affine phase residual atlas makes that posture portable. It
keeps compact `k=(0,0)` fixed, compares swapped ordered roles across
`6,10,14,22,26,30,34`, and ranks the rows where the finite exact signal still
looks interesting after size/PNT and residue-survivor accounting. This gives us
a lead queue for coherent local affine phase effects instead of a single
favorite anecdote.

## Shift-Phase Signal Mining

The reduced public phrase is:

```text
shift-phase residual
```

The plain-language version is:

```text
same slope, different intercept, different residue weather
```

That sentence is load-bearing. In a same-gradient pair, the template grammar
and affine gradient `G` stay fixed. The swapped role assignment changes only
the affine shift `A` in `N(s)=A+G*s`. The visual question becomes: how does a
shifted line meet the small-prime residue gates?

The curated signal-mining report therefore uses this walkthrough order:

```text
construction
-> affine line
-> same-gradient reversal
-> residue gate profile
-> survivor yield
-> residual decomposition
-> lead queue
```

The report follows focus leads and foils into a mature `M=4` surface when the
lane remains deterministic `u64`-safe. Rows are labeled as `persistent`,
`amplifies`, `fades`, `reverses`, `volatile`, or `residue_only`. Those labels
are intentionally humble: they help us decide where to probe next, not what to
claim.

For a smart external audience, the best first explanation is not "we found a
new prime-density law." It is: we have a structured way to create same-slope
candidate lines, shift their intercepts, read their residue gate profiles, and
rank the local phase residuals that remain after ordinary explanations are
separated.

## Unit-Cycle Phase Signal

The next reduction is:

```text
unit-cycle phase signal
```

This asks whether the shift-phase story survives when digit names are replaced
by positions on the base's ordered unit-residue cycle. The report does not ask
"is `(1,T)` magical in base 30?" It asks whether arc geometry such as
`diameter`, `long_arc`, `wrap_edge`, `low_edge`, and `base_complement` keeps
showing up after the same size and residue-survivor accounting.

The visual sentence is:

```text
same-gradient swap
-> unit-cycle arc orientation
-> residue gate profile
-> survivor-yield residual
```

This is the least anecdotal way to keep chasing the reversal lead. A coherent
bucket means "probe here next." A fading or scattered bucket is also useful:
it tells us a beautiful base-local story may not be a base-invariant one.

## Unit-Cycle Base Neighbors

The base-neighbor scout keeps the same geometry but changes the question:

```text
nearby base -> unit bead count -> diameter/complement examples
-> exact compact phase scan -> payload/transcoding caution
```

Base `57` and base `58` are a useful teaching pair because the circles are not
different sizes in the report. Both are normalized radius-1 circles. What
changes is the number of unit-residue beads: base `57` has `36`, while base
`58` has `28`. So base `57` has tighter bead spacing even though the base value
is smaller.

This also separates two tempting but different ideas. For arbitrary base58
payloads, moving to base57 is still radix transcoding: preserve the underlying
integer or bytes, then emit a different alphabet/base representation. The
affine/residue shortcut only becomes conceptually different if we generate the
identifier directly inside a constrained base-57 grammar instead of converting
an arbitrary base58 payload after the fact.

## Base57 Affine Codec

The maintained base57 experiment turns that caution into a concrete artifact.
It deliberately keeps two modes side by side:

```text
ordinary bytes -> base58/base57 radix text
ordinary bytes -> framed base57 affine membrane chunks
```

The invariant helper vocabulary is:

```text
canonical payload -> base rendering
canonical payload -> affine envelope
```

The canonical payload is the byte string. Hex/base16, base58, and base57 are
ordinary base renderings of that same payload. The affine envelope is a
structured representative of that payload inside the membrane grammar.

The first path is just a codec. It round-trips arbitrary bytes through BigUint
radix conversion and preserves leading zero bytes with leading `1` characters.
Base57 is expected to be slightly longer than base58 because it has one fewer
symbol.

The second path is the new notation idea. Payload bytes are packed into small
seed chunks with nonce bits, then searched inside a fixed base-57 membrane lane:

```text
base = 57
outer = 1
inner = 56
k = (0,0)
M = 6
N(s) = A + G*s
```

The emitted identifiers are:

```text
a57r1:<payload_len>:<body>  residue-filtered chunks
a57p1:<payload_len>:<body>  prime-witness chunks
```

This is not a shorter encoding claim. The useful claim is that the notation
creates a structured identifier namespace where validity can be checked through
the same affine lane, residue gates, and optional deterministic primality tests
used elsewhere in the repo. That makes it closer to "generate identifiers
inside a grammar" than "convert base58 into base57 faster."

The collaborator sentence is:

```text
base57 affine IDs trade compactness for structure: fixed grammar, fast residue
validation, and optional prime-witness chunks
```

## Metal Candidate-Transfer Collapse

The maintained Metal prototype sharpens a useful systems claim:

```text
affine lane metadata -> GPU residue sieve -> survivor bitmask -> CPU prime confirmation
```

The key point is modest but real. The GPU does not need a buffer of full
candidate values. It receives small residue rows derived from
`N(s) = A + G*s`, tests seed positions directly, and writes back a compact
bitmask of survivors. The candidate values are reconstructed only for survivor
seeds before deterministic `u64` primality confirmation.

This should be called candidate-transfer collapse or zero candidate transfer,
not total zero-copy. Params, residue rows, and output masks still move through
shared Metal buffers.

Implementation boundary: the affine residue loop belongs in the dedicated
`.metal` shader (`shaders/sieve_affine.metal::sieve_affine_lane`). Rust's
`metal` crate is the host layer: it loads the metallib, manages shared buffers,
dispatches the kernel, and reads back the survivor bitmask. This distinction is
load-bearing for benchmarks; host-side Rust Metal plumbing alone is not the
optimization.

For the first external comparison frame, see
`collab/PRIME_GENERATION_EXTERNAL_COMPARISON.md`. The short version is that
primesieve remains the correct gold-standard comparison for ordinary interval
enumeration, GMP/OpenSSL represent mature probable-prime pipelines, and our
lane engine should be judged first on structured witness throughput and
candidate-transfer collapse rather than as a replacement for general sieving.

## Guardrails

Do not say:

- the torus predicts primes
- gradient lock is enough for primality
- witnesses prove density lift
- the affine lane signal has escaped matched-control accounting

Do say:

- the torus visualizes exact gradient agreement
- residue filters are exact for each fixed lane and modulus
- witnesses show the construction is real
- density remains a separate empirical and theoretical question

## Reproduce

```bash
cargo run --release --example residue_torus_period_lock_report -- \
  --out-dir /tmp/primes_residue_torus_period_lock

cargo run --release --example membrane_prime_throughput_report -- \
  --out-dir /tmp/primes_fast_generation

cargo run --release --bin membrane-prime-fast -- \
  --base 10 --outer 3 --inner 7 --k 2,1 --middle-length 2 \
  --seed-count 10000 --max-primes 5

cargo run --features metal --release --bin membrane-prime-metal-fast -- \
  --base 10 --outer 3 --inner 7 --k 2,1 --middle-length 2 \
  --seed-count 10000 --max-primes 5

cargo run --features metal --release --example metal_affine_transfer_collapse_report -- \
  --out-dir /tmp/primes_metal_affine_transfer

cargo run --features metal --release --example metal_affine_benchmark_report -- \
  --out-dir /tmp/primes_metal_affine_benchmark

cargo run --release --example prime_witness_engine_visual_atlas -- \
  --out-dir /tmp/primes_prime_witness_engine_visual_atlas

cargo run --release --example base30_wheel_compact_report -- \
  --out-dir /tmp/primes_base30_wheel_compact

cargo run --release --example base30_reversal_asymmetry_report -- \
  --out-dir /tmp/primes_base30_reversal_asymmetry

cargo run --release --example base30_reversal_residual_report -- \
  --out-dir /tmp/primes_base30_reversal_residual

cargo run --release --example affine_phase_residual_atlas_report -- \
  --out-dir /tmp/primes_affine_phase_residual_atlas

cargo run --release --example shift_phase_signal_mining_report -- \
  --out-dir /tmp/primes_shift_phase_signal_mining

cargo run --release --example unit_cycle_phase_signal_report -- \
  --out-dir /tmp/primes_unit_cycle_phase_signal

cargo run --release --example unit_cycle_base_neighbor_report -- \
  --out-dir /tmp/primes_unit_cycle_base_neighbor

cargo run --release --example base57_affine_codec_report -- \
  --out-dir /tmp/primes_base57_affine_codec

cargo run --bin base57-affine-codec -- value-map \
  --input Cn8eVZg --input-format base58 --affine both

cargo run --bin base57-affine-codec -- affine-encode \
  --input hello --input-format text --mode prime

scripts/signal_spine.sh affine
scripts/signal_spine.sh fast-generation
scripts/signal_spine.sh phase-residual
scripts/signal_spine.sh shift-phase
scripts/signal_spine.sh unit-cycle
scripts/signal_spine.sh base-neighbor
scripts/signal_spine.sh base57-codec
```

Primary code and report surfaces:

- `src/validation/affine_period_lock.rs`
- `src/validation/bounded_k.rs`
- `src/validation/fast_affine.rs`
- `src/validation/metal_affine.rs`
- `src/validation/base57_affine_codec.rs`
- `src/bin/base57-affine-codec.rs`
- `examples/residue_torus_period_lock_report.rs`
- `examples/membrane_prime_throughput_report.rs`
- `examples/metal_affine_transfer_collapse_report.rs`
- `examples/metal_affine_benchmark_report.rs`
- `examples/prime_witness_engine_visual_atlas.rs`
- `examples/base30_wheel_compact_report.rs`
- `examples/base30_reversal_asymmetry_report.rs`
- `examples/base30_reversal_residual_report.rs`
- `examples/affine_phase_residual_atlas_report.rs`
- `examples/shift_phase_signal_mining_report.rs`
- `examples/unit_cycle_phase_signal_report.rs`
- `examples/unit_cycle_base_neighbor_report.rs`
- `examples/base57_affine_codec_report.rs`
- `examples/affine_period_lock_report.rs`
