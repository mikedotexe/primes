# Zero-Padded Connector Notes

This subdirectory contains the narrow Agda formalization for one zero-padded
connector case study centered on the pair `(10301, 3007003007003)`.

## Current Status

According to [`../../STATUS.md`](../../STATUS.md), all three modules here
currently type-check with postulates:

- `Alphabet036.agda`
- `Examples036.agda`
- `Asymmetry.agda`

That means the subtree is useful as a structured specification layer, but it is
not a clean machine-checked proof stack.

## What The Subtree Covers

- a restricted `{0,3,6}` alphabet representation
- concrete connector examples for the canonical pair
- a directional-asymmetry formal shell built around empirical scan results

## Limits

- this is a single-pair case study, not a general connector theorem
- primality facts and some statistical conclusions are still represented via
  postulates
- active repo claims should not describe this as settled or universal

For current claim classification, use [`../../../CLAIMS.md`](../../../CLAIMS.md)
and [`../../../VERIFIED_FACTS_VS_SPECULATION.md`](../../../VERIFIED_FACTS_VS_SPECULATION.md).

## Local Verification

```bash
cd agda-proofs
agda LagrangePoints/ZeroPaddedPrimes/Asymmetry.agda
```

The older discovery-first narrative is preserved at
[`../../../archive/agda-proofs/LagrangePoints/ZeroPaddedPrimes/README_legacy.md`](../../../archive/agda-proofs/LagrangePoints/ZeroPaddedPrimes/README_legacy.md).
