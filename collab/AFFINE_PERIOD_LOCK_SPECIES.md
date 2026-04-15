# Affine Period-Lock Species Boundary

## Scope

This tranche exploits the local affine period-lock theorem as an atlas-first
classifier surface. The goal is narrower than a new theorem claim:

- identify where gradient agreement can occur on the direct bounded-`k` lane surface
- separate period lock from the residual shift split inside the locked regime
- ask whether the meaningful `M = 2` species live on low-order ubiquitous locks
  while the residual side-pockets require rarer higher-order locks plus shift
  misalignment

This is still exploratory language for collaborators, not a new public claim.

## Why Direct Lanes Come First

The theorem-facing surface in this pass is the direct comparison
`k = (0, 0) -> each noncompact lane`, not just `k = (0, 0) -> best_k`.

That boundary matters because:

- winner selection can hide local affine structure
- the base-22 higher-order pocket survives clearly on the direct surface
- the same pocket disappears on the maintained `M = 2` winner-active surface

So the direct lane surface is the theorem surface, and the winner lane remains
only a secondary species projection.

## Three Distinct Objects

### Period lock

This is the exact local theorem:

- gradient agreement is possible exactly when the compared middle positions are
  congruent modulo the multiplicative order of the base unit in `ZMod modulus`

This answers where `gradient_equal` can happen at all.

### Locked shift residuals

Inside the locked regime, the remaining split is a shift question:

- `identity` if the local shifts agree
- `gradient_only` if the local shifts do not agree

So period lock explains the possibility of gradient agreement, while the locked
shift residual explains which local relation survives there.

### Species labels

These remain downstream report language:

- `persistent_core`
- `persistence_only`
- `core_only`
- `active_neither`

The species labels are not themselves exact local objects. They are empirical
summaries interpreted through the exact period-lock and shift-residual layers.

## Order Buckets

The maintained atlas keeps exact multiplicative orders in the raw order-cell
rows, but bucketed summaries use only:

- `ord_1`
- `ord_2`
- `ord_ge_3`

This is deliberate. The bucketed view is for species and control summaries; the
exact order values remain available in the raw atlas.

## Current Maintained Reading

On the maintained main active `M = 2` surface:

- the meaningful hinge-family winners are dominated by low-order locked mass
- the period-lock-only primary search has **no exact rule**
- the best period-lock-only frontier is `m2 unlocked_shift_only_count = 0`
- once mixed back with the existing hinge atoms, the old exact rule stays pinned:
  `m1 anomaly_mass_pp > 0 AND m2 boundary_prime_delta_count <= 0`

That means the new period-lock decomposition is real and useful, but it does
not replace the existing exact hinge rule yet.

The sharper species reading is:

- period lock explains where gradient agreement is possible
- locked shift residuals explain what local relation survives there
- the meaningful `M = 2` winner species are still mostly low-order and not
  driven by rare locks
- the base-22 higher-order story is real, but it survives as a **direct-lane
  side pocket**, not as a winner-active species

The maintained report currently finds:

- `80` direct side-pocket rows for the base-22 higher-order `gradient_only`
  regime
- `0` winner-active rows from that pocket on the maintained main `M = 2`
  surface
- base `30` acts like a theorem-facing control here, with no maintained `M = 2`
  active rows

## Boundary

This pass does **not** claim:

- a public class theorem about low-order locks
- a new replacement for the hinge rule
- that higher-order locks are the main explanation of the meaningful `M = 2`
  species

It does claim a useful decomposition:

- period lock is now the exact answer to where gradient agreement can occur
- the locked shift residual is the exact answer to what kind of local relation
  remains there
- the species language should now be read downstream of that split
