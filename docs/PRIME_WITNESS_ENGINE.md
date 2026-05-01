# Prime Witness Engine

This is the repo front door for the large-witness path:

```text
seed origin -> affine membrane lane -> residue funnel -> probable-prime witness -> verification transcript
```

The Prime Witness Engine generates large, human-readable prime witnesses inside
named symmetric digit constructions. It starts from a seed origin, walks an
affine membrane lane, uses exact small-prime residue gates to avoid many
expensive primality checks, and returns the first witness together with a
copyable verification transcript.

The default lane is the visible decimal family:

```text
base=10, pair=(3,7), k=(2,1)
3 00 7 0 [middle seed] 0 7 00 3
```

The important distinction is semantic: ordinary prime generators can quickly
produce large primes, but they do not naturally target or preserve this named
construction family.

## Vocabulary

| Term | Meaning |
|---|---|
| Prime Witness Engine | The repo section for seed-origin demos, large affine witness measurement, and verification transcripts. |
| seed origin | The starting seed supplied by a user, timestamp, or deterministic test fixture. It is not a promise that the exact seed is prime. |
| witness seed | The seed that actually produced the first returned witness after walking the lane. |
| affine membrane lane | A fixed symmetric digit template compiled into `N(s) = A + G*s`. |
| residue funnel | Exact small-prime filters applied before primality or probable-prime confirmation. |
| confirmation tier | The local label for how the witness was checked: deterministic `u64`, fixed-width `u128`, or BigUint Miller-Rabin probable-prime. |
| Mersenne class | Exact special-form label for whether the witness is of the form `2^p - 1`; default affine witnesses currently report `not_mersenne`. |
| semantic rarity | The fact that the witness belongs to a tiny named template slice of same-digit decimal strings. |

## Command Map

| Command | Purpose |
|---|---|
| `cargo run --release --bin seed-to-witness` | Timestamp-nanosecond seed origin to one large readable probable-prime witness. |
| `cargo run --release --bin seed-to-witness -- --seed 60` | Canonical fixed-seed 128-digit transcript. |
| `cargo run --release --bin seed-to-witness -- --visible-digits 1024 --max-steps 20000` | Larger timestamp-seeded demo on the same lane. |
| `cargo run --release --example seed_to_witness_demo_report` | Small report bundle for the transcript demo. |
| `cargo run --release --example large_affine_witness_ladder_report -- --profile release` | Measurement entrypoint for the large witness ladder. |
| `cargo run --release --example timestamp_seed_policy_report -- --profile release` | Bounded empirical policy for timestamp-like seed origins. |
| `cargo run --release --example special_form_witness_comparison_report` | Mersenne-style special-form comparison for compact descriptors and non-Mersenne affine witnesses. |
| `scripts/signal_spine.sh witness-engine` | Umbrella smoke group for seed and ladder reports. |

## One-Command Timestamp Demo

Run:

```bash
cargo run --release --bin seed-to-witness
```

When `--seed` is omitted, the CLI uses the current epoch nanoseconds as the
seed origin. The transcript reports the seed origin, witness seed, steps walked,
affine line, residue-funnel counts, local confirmation tier, and copyable
external checks.

This is the simplest story to tell a collaborator: give the engine a moment in
time, and it walks a named construction family until it finds a large readable
probable-prime witness.

## Canonical Seed 60 Demo

Run:

```bash
cargo run --release --bin seed-to-witness -- --seed 60
```

This returns the canonical 128-digit witness in the default decimal lane. In the
current local run, seed `60` is itself the witness seed:

```text
30070000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006007003
```

The transcript includes snippets for WolframAlpha `isprime(...)`, Mathematica
`PrimeQ[...]`, PARI/GP `isprime(...)`, and Sage
`is_prime(Integer("..."))`. It also marks the witness's Mersenne class. For the
default affine lane, the canonical witness reports `not_mersenne`, which is the
interesting comparison against the Mersenne-prime tradition: small input, large
prime-shaped output, but not the `2^p - 1` special form.

## Large Ladder Reading

The release ladder is the measurement entrypoint:

```bash
cargo run --release --example large_affine_witness_ladder_report -- \
  --profile release \
  --out-dir /tmp/primes_large_affine_witness_ladder
```

The release profile scans visible decimal rungs at `22, 28, 38, 50, 75, 100,
128` digits for the primary `(3,7), k=(2,1)` construction. It reports
time-to-first witness, witnesses found, residue survivor share, backend scope,
control rows, semantic rarity, and a witness gallery.

Fresh local release-profile output found probable-prime witnesses at every
rung in a `20,000` seed prefix:

| Visible digits | Witnesses | Raw hit rate | Seeds per witness | First witness seed |
|---:|---:|---:|---:|---:|
| 22 | 1,019 | 5.10% | 19.6 | 62 |
| 28 | 813 | 4.06% | 24.6 | 53 |
| 38 | 583 | 2.92% | 34.3 | 3 |
| 50 | 384 | 1.92% | 52.1 | 30 |
| 75 | 303 | 1.52% | 66.0 | 24 |
| 100 | 213 | 1.06% | 93.9 | 24 |
| 128 | 156 | 0.78% | 128.2 | 60 |

## Larger Local Observations

The seed-to-witness path is not capped at 128 digits. These fresh local
observations use seed origin `60`, the default lane, and `--max-steps 20000`.
They are probable-prime witnesses unless a proof certificate is later added.

| Visible digits | Witness seed | Steps | Residue survivors / tests | Elapsed seconds | Confirmation tier |
|---:|---:|---:|---:|---:|---|
| 128 | 60 | 0 | 1 | 0.002 | `probable_prime_fixed_20_bases` |
| 256 | 60 | 0 | 1 | 0.008 | `probable_prime_fixed_20_bases` |
| 512 | 584 | 524 | 203 | 0.427 | `probable_prime_fixed_20_bases` |
| 1024 | 510 | 450 | 171 | 2.782 | `probable_prime_fixed_20_bases` |

The point is not that these numbers are hard upper limits. They are evidence
that the construction remains lively well past `u64`: the affine/residue funnel
still finds large, readable witnesses quickly enough to be interactive on this
machine.

## Bounded Timestamp Policy

For the question "can we almost always do this for a nanosecond timestamp?",
the repo now uses an explicit bounded empirical policy:

```text
timestamp seed origin -> walk forward on the default lane -> success only if a witness appears inside the max-step budget
```

The release policy samples deterministic timestamp-like origins near
`1777651200000000000`, spaced by `1000003` nanoseconds so adjacent trials do
not simply reuse the same tiny window. Current local release-profile output:

| Policy | Visible digits | Max steps | Trials | Successes | Median steps | P95 steps | P99 steps | Max observed |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `timestamp_full_middle_29d_512_steps` | 29 | 512 | 256 | 256 | 18 | 80 | 116 | 131 |
| `timestamp_large_128d_20000_steps` | 128 | 20000 | 64 | 64 | 77 | 425 | 503 | 503 |

The bounded reading is strong but still precise: in this measured sample, every
timestamp-like seed origin found a nearby probable-prime witness within the
specified budget. That supports the demo policy; it does not prove an
all-timestamps theorem.

## How To Read Probable-Prime Witness

For candidates that fit in `u64`, the repo can use deterministic primality
confirmation. Above that boundary, the current witness engine uses fixed-base
Miller-Rabin through BigUint and labels the result as a probable-prime witness.

That is a useful research artifact, not a proof certificate. A proof path could
later add ECPP, Pocklington-style certificates when structure permits, or
external certificate capture. Until then, docs should say probable-prime
witness for large outputs.

Mersenne classification is separate from primality confirmation. It is an exact
shape check: compute `N + 1` and ask whether it is a power of two. A
probable-prime witness can therefore be confidently labeled `not_mersenne` even
before we add a proof certificate.

## Special-Form Comparison

The repo keeps the Mersenne comparison in a separate report:

```bash
cargo run --release --example special_form_witness_comparison_report -- \
  --out-dir /tmp/primes_special_form_witness_comparison
```

The intended reading is:

```text
Mersenne: p -> 2^p - 1
Affine membrane: seed origin + lane -> A + G*s
```

Both are compact-descriptor prime-witness stories. They differ sharply in
arithmetic grammar and proof maturity. The maintained affine line is that our
witnesses are large, readable, structured, and explicitly `not_mersenne`.

## What This Does Not Claim

- It is not interval enumeration, and it is not trying to replace primesieve at
  counting all primes in a range.
- It is not a density theorem. Yield is measured for specified lanes and
  controls.
- It is not a proof certificate above `u64`.
- It is not proof that the default lane is globally optimal.
- It does not say random prime generators cannot find large primes. It says this
  engine preserves a named readable construction while searching.

## Research Queue

- Add a proof/certificate path for large witnesses.
- Broaden the lane catalog beyond decimal `(3,7), k=(2,1)`.
- Strengthen residue gates and make the max-step policy adaptive.
- Bridge Metal candidate-transfer collapse to BigUint survivor confirmation.
- Keep matched-control density interpretation separate from witness throughput.
