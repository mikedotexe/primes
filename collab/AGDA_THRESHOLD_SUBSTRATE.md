# Agda Threshold Substrate Boundary

This note freezes the scope of the new Agda bounded-`k` tranche so we do not
accidentally let the arithmetic substrate inherit the empirical threshold prose.

## Exact Agda Vocabulary

- `BoundedKConfig`
  exact padding coordinates for the bounded-`k` template lane
- `paddingWeight`
  exact compactness quantity
- `diameter`
  exact width-style compactness quantity for a fixed middle length
- `TransferBucket`
  exact five-way transfer partition:
  `stableZero`, `gainZero`, `lossZero`, `stableNonzero`, `nonzeroChurn`
- `sharedAdmissibleCount`
  exact `stableZero` count
- `admissibleCountFrom`, `admissibleCountTo`
  exact counts recovered from the transfer buckets
- `bucketGoodFromCount`, `bucketGoodToCount`
  exact bucketwise event counts on a finite aligned witness list
- `TransferWitnessSummary`
  generated-style Agda shell that records exact finite bucket counts and exact
  signed delta summaries for curated bounded-`k` witnesses
- `export_bounded_k_transfer_agda_summary`
  Rust regeneration path for the maintained Agda witness catalog

## What Agda Proves Exactly

- `Theorems/BoundedKCompactness.agda`
  proves exact padding/diameter arithmetic and the minimization of `k = (0,0)`
  inside the bounded-`k` shell
- `Theorems/Abstract/FiniteMaskTransfer.agda`
  proves the exact five-bucket partition and the exact count identities for
  shared admissible overlap, admissible totals, same-mask totals, zero-union
  totals, and bucketwise good-count decomposition
- `Examples/BoundedKTransferWitnessShell.agda`
  fixes the exact Agda witness vocabulary
- `Examples/Generated/BoundedKTransferWitnessCatalog.agda`
  is regenerated from the Rust bounded-`k` lane and certifies the maintained
  base-10, base-14, base-22, and base-34 `M = 2` cases
- `Examples/BoundedKTransferWitnesses.agda`
  is the stable wrapper path that re-exports the shell plus generated catalog

## What Remains Empirical

- the `M = 1 → 2 → 3` collapse itself
- catalog-wide survival rates and anomaly-mass percentages
- overlap-led / boundary-led language as a global regime statement
- the visual lane and any geometry built on top of it

Those stay in the Rust reporting and visualization surfaces.

## Why Density Stays A Guardrail

The bounded-`k` threshold lane is currently strongest as exact finite transfer
arithmetic plus empirical catalog summaries. Agda is certifying the finite
combinatorics that make the threshold story intelligible; it is not asserting a
density theorem or an asymptotic prime-yield law.

## Why “Chaos Threshold” Stays Report Language

The phrase “chaos threshold” is still metaphor/report language in this repo.
The Agda contribution is narrower and better:

- exact compactness arithmetic
- exact transfer-bucket arithmetic
- exact finite witness summaries

That is the proof substrate beneath the threshold story, not a formalization of
the story's prose itself.
