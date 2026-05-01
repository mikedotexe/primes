# Signal Spine Review Brief

## What Changed

The repo now has a maintained signal spine for the arithmetic research stack.
It runs the core verification, membrane reports, connector reports, affine
reports, transfer reports, matched controls, and non-default follow-up groups
from one reproducible script.

The latest committed stack also includes the base57 affine codec experiment,
which separates ordinary base rendering from structured affine envelopes.
The large affine witness ladder is now the companion report for showing the
prime-generation "flex": large readable witnesses in a named construction
family, with backend scope and fair comparison rows kept explicit.
The Prime Witness Engine front door lives at
[`../docs/PRIME_WITNESS_ENGINE.md`](../docs/PRIME_WITNESS_ENGINE.md). The
seed-to-witness demo is the smallest public-facing version of that story: one
seed origin, one affine membrane lane, one large readable probable-prime
witness, and a copyable verification transcript.

## How To Rerun

Default maintained spine:

```bash
scripts/signal_spine.sh
```

Full local spine:

```bash
SIGNAL_SPINE_RUN_ID=post-commit-full \
scripts/signal_spine.sh --all
```

CI-style smoke:

```bash
SIGNAL_SPINE_RUN_ID=ci-smoke-local \
SIGNAL_SPINE_OUT_DIR=/tmp/primes_signal_spine_smoke \
scripts/signal_spine.sh core affine fast-generation base57-codec
```

Large-witness smoke:

```bash
scripts/signal_spine.sh large-witness
```

Seed-to-witness smoke:

```bash
scripts/signal_spine.sh seed-witness
```

Prime Witness Engine umbrella smoke:

```bash
scripts/signal_spine.sh witness-engine
```

Timestamp-origin bounded policy:

```bash
cargo run --release --example timestamp_seed_policy_report -- \
  --profile release \
  --out-dir /tmp/primes_timestamp_seed_policy
```

Curate a small snapshot:

```bash
scripts/curate_signal_spine_snapshot.py \
  --run-dir reports/signal-spine/post-commit-full
```

## Conservative Claims

- Fixed symmetric templates define affine seed-search surfaces:
  `N(s) = A + G*s`.
- Exact residue filters explain much of the prime-candidate efficiency.
- Period lock explains affine gradient agreement through multiplicative order.
- The fast-generation and Metal paths are throughput and transfer-collapse
  tools for specified lanes, not density theorems.
- The large-witness path shows capability inside named visible affine membrane
  families; above `u64`, witnesses are probable primes unless explicitly
  labeled deterministic.
- The seed-to-witness CLI treats the seed as a start point on the affine lane,
  not a guarantee that the exact seed is prime.
- The Prime Witness Engine frames these as seed-origin workflows with explicit
  witness seeds, residue funnels, confirmation tiers, and semantic rarity.
- Base57 affine envelopes are structured representatives of a canonical
  payload, not shorter replacements for base58.

## Throughput Calibration

Fresh local benchmark command:

```bash
cargo run --features metal --release --example metal_affine_benchmark_report -- \
  --out-dir /tmp/primes_affine_benchmark_u128 \
  --seed-count 1000000 --max-primes 10 \
  --biguint-seed-count 20000 --biguint-middle-lengths 12,15,18,28 \
  --u128-seed-count 20000 --u128-middle-lengths 12,15,18,21,24,27,28
```

The 19-digit visible decimal affine lane found `58,668` deterministic prime
witnesses from `1,000,000` structured seeds. The Metal transfer-collapse path
reported about `54k` witnesses/s, comparable to ordinary small-prime wheel
scanning and the local primesieve CLI calibration at this size.

Same-window ordinary odd/random scans were about `26.5k-26.8k` primes/s, while
ordinary small-prime wheel scanning and the local primesieve CLI calibration
landed around `53k-54k` primes/s. The fair takeaway is that affine membranes are
fast structured witness generators, not replacements for general interval
enumeration.

The same report now tracks beyond-`u64` BigUint and fixed-width `u128`
probable-prime rows. In the fresh run, the 38-digit lane found `583`
probable-prime witnesses from `20,000` structured seeds: `2.92%` raw hit rate,
`7.63%` hit rate among residue survivors, or about `34.3` seeds per witness.
That is the efficacy metric to keep separate from the speed of a particular
Miller-Rabin backend. primesieve cannot directly follow those rows because it
is bounded to `u64` interval enumeration.

For an intentionally simpler witness-flex run:

```bash
cargo run --release --example large_affine_witness_ladder_report -- \
  --profile smoke \
  --out-dir /tmp/primes_large_affine_witness_ladder_smoke
```

The smoke profile scans 22- and 38-digit visible decimal rungs in the primary
`(3,7), k=(2,1)` lane. It exports the layered funnel, BigUint/u128/u64 backend
scope, local controls, OpenSSL calibration, primesieve scope rows, semantic
rarity, and a witness gallery. The release profile extends the same ladder to
`50, 75, 100, 128` visible digits.

The local release-profile ladder found probable-prime witnesses at every rung
from `22` to `128` visible digits. The 128-digit row found `156` witnesses from
`20,000` structured seeds, a `0.78%` raw hit rate, or about `128.2` seeds per
witness. The first 128-digit witness appeared at seed `60`.

The front-door demo for that same witness is:

```bash
cargo run --release --bin seed-to-witness
cargo run --release --bin seed-to-witness -- --seed 60
```

Without `--seed`, it uses current epoch nanoseconds as the seed origin. With
`--seed 60`, it prints the construction, affine line, residue funnel, local
fixed-base Miller-Rabin confirmation, and copyable
WolframAlpha/Mathematica/PARI/Sage checks for the canonical 128-digit witness.

The timestamp policy report makes the casual timestamp claim bounded. In the
fresh release-profile run, `256/256` full-middle 29-digit timestamp-like seed
origins found a witness within `512` steps, with p95 at `80` steps and max
observed at `131`. The 128-digit policy found `64/64` witnesses within `20000`
steps, with p95 at `425` and max observed at `503`.

## Explicit Non-Claims

- The residue torus does not predict primality.
- Prime witnesses do not prove a global density theorem.
- Base 30 is not claimed as residual density magic beyond classical wheel
  effects without controls.
- Shift-phase and unit-cycle leads are ranked empirical leads, not laws.
- Base57 affine notation is not compression and is not a shortcut around radix
  conversion.

## Verification Commands

```bash
git diff --check
cargo fmt -- --check
cargo clippy --lib -- -D warnings
cargo test --lib
scripts/signal_spine.sh --all
```

## Next Research Queue

- `phase-residual`: broaden compact same-gradient reversal leads.
- `shift-phase`: follow mature leads and foils through M4 surfaces.
- `unit-cycle`: test whether arc geometry survives base normalization.
- `witness-engine`: add proof/certificate paths, extend the visible ladder to
  more bases, strengthen residue gates, and eventually put BigUint survivor
  confirmation behind the Metal transfer-collapse path.
- Density/control reports: separate coprimality, size, residue survival, and
  residual template effects.
- Base57 structured envelopes: explore payload-bearing arithmetic namespaces
  without overstating compression or encoding gains.
