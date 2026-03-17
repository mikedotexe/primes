# WindowCertificate Base-14 Sketch

This note preserves the old hypothetical `Example-Base14` usage shell that was
previously embedded inside
[`Theorems/Abstract/WindowCertificate.agda`](../Theorems/Abstract/WindowCertificate.agda).

It is intentionally a note rather than an active Agda module. The goal is to
keep the core dual-certificate builder module clean-local while still retaining
the shape of the intended integration example.

## Sketch

```agda
module Example-Base14 where

  open import Data.Nat using (ℕ)
  open import Data.Fin using (Fin)

  open import Theorems.Abstract.SymmetryImpliesRepulsion
    using (SymmetryData ; HonoraryZero)
  open import Theorems.Abstract.ConstrainedOrbitals
    using (PointwiseSafe ; pointwiseSafeCons ; pointwiseSafeNil)
  open import Theorems.Abstract.BucketsAutoMatch
    using (BalancedBuckets)
  open import Theorems.Abstract.WindowCertificate
    using ( WindowData
          ; StaticContracts
          ; DualCertificate
          ; buildDualCertificate
          )

  base : ℕ
  base = 14

  postulate
    S : SymmetryData (Fin 14)
    n : ℕ
    W : WindowData 14 n
    contracts : StaticContracts S W
    bb : BalancedBuckets S (WindowData.residues W) (WindowData.count W)
    -- In a generated certificate, pointwise-safe evidence is the dynamic
    -- input. The builder derives StableOrbital internally.
    ps : PointwiseSafe (WindowData.radius W)
                       (WindowData.window-mid W)
                       (WindowData.positions W)

  certificate : DualCertificate S W
  certificate = buildDualCertificate S W contracts bb ps

  proof-of-void : HonoraryZero S (MS-fromResid (WindowData.residues W))
  proof-of-void = DualCertificate.honorary-zero certificate
```

## Why It Moved

- The old embedded example made
  [`Theorems/Abstract/WindowCertificate.agda`](../Theorems/Abstract/WindowCertificate.agda)
  a `with local postulates` module even though the actual builder surface was
  constructive.
- The active builder now consumes `PointwiseSafe` directly, so generated notes
  should show helper-driven dynamic construction rather than a raw
  `StableOrbital` witness.
- Extracting the sketch keeps the signal while making the contract boundary in
  the active certification stack more honest.

## Current Status

- The builder in
  [`Theorems/Abstract/WindowCertificate.agda`](../Theorems/Abstract/WindowCertificate.agda)
  is the active proof surface.
- This file is historical/expository scaffolding for future concrete generated
  examples, and those generated examples should target `PointwiseSafe`
  construction first.
