# Prime Generation External Comparison

**Updated**: May 2026

This note anchors the first external comparison pass for the fast affine and
Metal transfer-collapse work. It is deliberately conservative: it compares
problem shapes before speed numbers, because most mature prime-generation
systems solve a different problem than visible membrane witness generation.

## Current Claim

Our maintained fast path is best described as:

```text
affine membrane prime family -> affine line N(s)=A+G*s
  -> exact small-modulus residue funnel
  -> deterministic u64 primality confirmation
  -> visible prime witnesses
```

An affine membrane prime family is a fixed symmetric zero-run template lane as
the middle seed varies. Individual primes found in the family are prime
witnesses. The public-friendly gloss is symmetric zero-run template primes.

The maintained Metal path sharpens the systems claim:

```text
lane residue metadata -> dedicated .metal residue kernel
  -> survivor bitmask -> CPU reconstruction and primality confirmation
```

The phrase to use is **candidate-transfer collapse** or **zero candidate
transfer**, not total zero-copy. Candidate values are not sent to the GPU, but
params, residue rows, and survivor bitmasks still live in shared Metal buffers.

The Prime Witness Engine is the repo-owned front door for that capability; see
[`../docs/PRIME_WITNESS_ENGINE.md`](../docs/PRIME_WITNESS_ENGINE.md). Its
large-witness ladder makes the capability claim more explicit:

```text
seed origin -> affine membrane lane -> residue funnel
  -> probable-prime witness -> verification transcript
```

This engine generates large, human-readable prime witnesses by compiling
symmetric digit constructions into affine search lanes with cheap residue
filtering. The comparison point is semantic rarity: ordinary generators can
quickly produce large primes, but they do not naturally target or preserve this
named construction family.

## External Map

| Family | Representative source | Established shape | Boundary against our path |
|---|---|---|---|
| Ordinary prime enumeration | [primesieve](https://github.com/kimwalisch/primesieve) | Cache-aware segmented sieve of Eratosthenes with wheel factorization, bucket sieve, and multithreading up to `2^64`. | This is the right gold-standard foil for interval enumeration. Our affine lane is not trying to list all primes in an interval; it searches a structured visible construction surface. |
| Arbitrary-precision next-prime/probable-prime | [GNU MP](https://gmplib.org/manual/Number-Theoretic-Functions) | Trial division, Baillie-PSW probable-prime testing, and Miller-Rabin rounds. | This is the natural future comparison for BigUint survivor confirmation. Our current maintained benchmark stays in deterministic `u64` scope and measures the candidate funnel first. |
| Cryptographic random prime generation | [OpenSSL BN_generate_prime_ex](https://docs.openssl.org/master/man3/BN_generate_prime/) | Pseudo-random bit-length prime generation with optional congruence constraints, small-prime trial division, and Miller-Rabin error bounds. | This is not a visible-template generator. It is a useful contrast for random/probable prime throughput, not for membrane construction semantics or density claims. |
| Machine-word deterministic primality | [Forisek-Jancina / Jaeschke-Sinclair bases](https://ceur-ws.org/Vol-1326/020-Forisek.pdf) | Deterministic Miller-Rabin-style testing for bounded 32-bit and 64-bit integers using fixed or hashed bases. | This is closest to our current confirmation regime. The live question is how much affine residue structure removes before deterministic `u64` testing. |
| GPU segmented sieving | [CUDASieve](https://github.com/curtisseizert/CUDASieve) | Nvidia CUDA segmented sieve of Eratosthenes for counting and generating all primes in ranges. | CUDASieve is a range-wide GPU sieve. Our Metal kernel is lane-wide affine residue sieving with zero candidate-value transfer. |
| Special-form huge prime searches | [GIMPS](https://www.mersenne.org/various/works.php) | Trial factoring, P-1/ECM where useful, PRP tests, and Lucas-Lehmer verification for Mersenne-form candidates. | These systems optimize very large special forms over long runtimes. They are relevant as GPU-primality precedent, but not directly comparable to 64-bit lane witnesses per second. |

## What Stands Out

The interesting quadrant is not already occupied by the usual tools:

- primesieve owns general interval enumeration.
- OpenSSL/GMP own random or arbitrary-precision probable-prime workflows.
- GIMPS/PrimeGrid-style software owns huge special-form PRP work.
- our maintained path owns affine membrane prime families: structured,
  human-readable affine lanes where the construction itself matters.

That means the fair comparison is not "do we beat primesieve?" It is:

1. For the same magnitude, how many deterministic primality confirmations do
   our exact residue filters avoid?
2. For a fixed visible construction grammar, how quickly can we surface prime
   witnesses?
3. As seed batches scale, does the dedicated `.metal` residue kernel amortize
   dispatch enough to become materially better than CPU residue rows?
4. When we leave `u64`, can the same residue funnel reduce the cost of BigUint
   probable-prime confirmation enough to matter?

## Current Local Reading

The latest maintained benchmark still says: CPU wheels and CPU residue rows are
excellent at medium `u64` sizes, and Metal is not yet a blanket speed win.

But the transfer architecture is clean. A 19-digit visible decimal lane can scan
millions of seeds while sending residue metadata plus a survivor bitmask instead
of a full candidate-value buffer. That is the part worth protecting and scaling.

The benchmark report now includes optional external CLI adapters. If
`primesieve` is installed, it measures an ordinary interval-count row near the
19-digit affine lane. If OpenSSL is installed, it measures repeated 64-bit
random prime generation through `openssl prime -generate -bits 64`.

These rows intentionally include CLI/process overhead and remain
problem-shape comparisons. They are useful as calibration, not as claims that
the tools are solving the same visible-template witness task.

The report now also includes two extension rows:

- A repeated Metal dispatch row that creates the Metal pipeline once, dispatches
  residue-sieve batches repeatedly, and reports setup time separately from GPU
  dispatch time.
- A beyond-`u64` BigUint row that uses the same exact residue funnel before
  Miller-Rabin probable-prime confirmation on a visible 22-digit decimal lane.

The next meaningful benchmark should move the beyond-`u64` survivor funnel onto
the repeated Metal dispatch path and add a GMP/OpenSSL library-level adapter so
the comparison is not dominated by CLI process startup.

The large-witness report is now the focused place to show the superpower above
the `u64` boundary without pretending it is an interval-enumeration contest:

```bash
cargo run --release --example large_affine_witness_ladder_report -- \
  --profile smoke \
  --out-dir /tmp/primes_large_affine_witness_ladder_smoke

cargo run --release --example large_affine_witness_ladder_report -- \
  --profile release \
  --out-dir /tmp/primes_large_affine_witness_ladder
```

Its default release ladder scans visible decimal rungs at `22, 28, 38, 50, 75,
100, 128` digits for the primary `(3,7), k=(2,1)` construction. It reports
time-to-first witness, witnesses per seed, residue survivor share, backend
scope across BigUint/u128/u64, local random and same-slot controls, OpenSSL
probable-prime calibration, primesieve out-of-scope rows above `u64`, and the
template's semantic rarity among same-digit decimal strings.

Fresh release-profile ladder output found probable-prime witnesses across all
seven rungs from the same `20,000` seed prefix:

| Visible digits | Witnesses | Raw hit rate | Seeds per witness | First witness seed |
|---:|---:|---:|---:|---:|
| 22 | 1,019 | 5.10% | 19.6 | 62 |
| 28 | 813 | 4.06% | 24.6 | 53 |
| 38 | 583 | 2.92% | 34.3 | 3 |
| 50 | 384 | 1.92% | 52.1 | 30 |
| 75 | 303 | 1.52% | 66.0 | 24 |
| 100 | 213 | 1.06% | 93.9 | 24 |
| 128 | 156 | 0.78% | 128.2 | 60 |

That is the "flex" claim in its cleanest form: the engine surfaces readable
large witnesses very quickly inside a tiny, named template slice of the
same-digit decimal space. It remains an efficacy and construction-family claim,
not a proof of residual density magic.

The most approachable front-door demo is now:

```bash
cargo run --release --bin seed-to-witness
cargo run --release --bin seed-to-witness -- --seed 60
```

Without `--seed`, the command uses current epoch nanoseconds as the seed origin.
With `--seed`, it uses the provided value as an origin on the default 128-digit
visible decimal lane. Either way it returns the first witness seed found and
prints a transcript with the affine line, residue funnel, confirmation tier,
and copyable WolframAlpha/Mathematica/PARI/Sage checks. The clean phrase is:
one seed origin, one affine membrane lane, one large readable probable-prime
witness.

## Fresh Local Benchmark Reading

Fresh local run:

```bash
cargo run --features metal --release --example metal_affine_benchmark_report -- \
  --out-dir /tmp/primes_affine_benchmark_u128 \
  --seed-count 1000000 --max-primes 10 \
  --biguint-seed-count 20000 --biguint-middle-lengths 12,15,18,28 \
  --u128-seed-count 20000 --u128-middle-lengths 12,15,18,21,24,27,28
```

Headline results:

| Surface | Path | Scope | Prime witnesses or primes | Rate |
|---|---|---:|---:|---:|
| visible decimal affine lane, 16 digits | Metal affine transfer-collapse | 1,000,000 seeds | 70,006 witnesses | about 68k witnesses/s |
| visible decimal affine lane, 19 digits | Metal affine transfer-collapse | 1,000,000 seeds | 58,668 witnesses | about 54k witnesses/s |
| ordinary same-window odd scan, 19 digits | sequential odd baseline | 200,000 candidates | 9,427 primes | about 27k primes/s |
| ordinary same-window small-prime wheel, 19 digits | sequential small-prime wheel | 200,000 candidates | 9,427 primes | about 53k primes/s |
| ordinary interval enumeration near the 19-digit lane | primesieve CLI row | distance 1,000,000 | 23,570 primes counted | about 57k primes/s |
| visible beyond-`u64` decimal lane, 22 digits | BigUint affine residue funnel | 20,000 seeds | 1,019 probable-prime witnesses | 5.10% raw hit rate |
| visible beyond-`u64` decimal lane, 28 digits | BigUint affine residue funnel | 20,000 seeds | 813 probable-prime witnesses | 4.06% raw hit rate |
| visible beyond-`u64` decimal lane, 38 digits | BigUint affine residue funnel | 20,000 seeds | 583 probable-prime witnesses | 2.92% raw hit rate |
| visible beyond-`u64` decimal lane, 38 digits | u128 affine residue funnel | 20,000 seeds | 583 probable-prime witnesses | 34.3 seeds/witness |

An additional local calibration command:

```bash
primesieve 3007000000000007003 --dist=100000000 --time
```

counted `2,353,248` primes in `0.701` seconds. That is the right reminder:
primesieve is a superb interval-counting engine, and it scales past the small
CLI adapter row when the interval is large enough.

Above the `u64` boundary, primesieve is no longer the right direct comparator.
For example:

```bash
primesieve 3007000000000000000007 --dist=1000000 --time
```

fails with a `64-bit unsigned integer overflow` message. That is expected:
primesieve owns bounded machine-word interval enumeration, while the BigUint
affine rows are probable-prime witness searches in structured lanes above that
range.

As a rough CLI calibration for random probable-prime generation, local OpenSSL
process-level runs generated ten `96`-bit primes at about `114` primes/s and
ten `128`-bit primes at about `120` primes/s. Those rows include process
startup overhead and solve a different problem, but they help size the gap
between random prime generation and structured affine witness search.

The u128 result is best read as an efficacy result rather than a speed win. The
straightforward fixed-width u128 Miller-Rabin path is slower than the existing
BigUint Miller-Rabin backend because overflow-safe u128 modular multiplication
needs extra work without a native u256 intermediate. But it confirms that the
same affine/residue grammar reaches the top of the u128 range: the 38-digit lane
keeps a `2.92%` raw witness rate, `7.63%` witness rate among residue survivors,
and about `34.3` seeds per witness.

The novelty claim is therefore not "we beat primesieve." We do not own ordinary
interval enumeration. The interesting claim is that a visible construction
family can be compiled into:

```text
template grammar -> affine lane -> exact residue funnel -> prime witnesses
```

and, on the Metal path:

```text
lane residue metadata -> GPU residue sieve -> survivor bitmask
```

The latest 19-digit Metal row scanned one million structured seeds while
transferring `160` metadata bytes and a `125,000` byte survivor bitmask, avoiding
an `8,000,000` byte `u64` candidate-value buffer. That transfer shape is the
unusual systems trick. The density/yield side is promising but still belongs in
the matched-control framework.

The balanced reading:

- primesieve wins the general "count all primes in an interval" category
- ordinary CPU wheels are already very strong at these medium sizes
- our affine lanes beat naive odd/random same-window scans because they combine
  visible structure, coprimality, and exact residue filtering
- the dedicated `.metal` path is currently a small warm-speed improvement at
  `u64` size, but its more durable novelty is zero candidate-value transfer
- the beyond-`u64` rows show why this could matter later: residue funnels become
  valuable when each survivor confirmation is expensive, and the construction
  still stays visible well past the `u64` boundary
