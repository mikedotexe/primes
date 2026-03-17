# CertifiedResonanceParamDyn Base-6 Sketch

This note preserves the old `Example-Base6-Dual` shell that was previously
embedded inside
[`CertifiedResonanceParamDyn.agda`](./CertifiedResonanceParamDyn.agda).

It remains useful as an expository sketch because it shows the fully
constructive static half of the example together with the single remaining
runtime witness family `proof-pointwiseSafe`. It is intentionally a note rather than an
active Agda module so the main wrapper file can stay a clean-local boundary
surface.

## Sketch

```agda
module Example-Base6-Dual where

  open import Data.Fin using () renaming (zero to fzero ; suc to fsuc)

  example-mid : Fin 6
  example-mid = suc (suc (suc zero))

  example-residues : Vec (Fin 6) 4
  example-residues =
    suc zero ∷
    suc (suc (suc (suc (suc zero)))) ∷
    suc (suc zero) ∷
    suc (suc (suc (suc zero))) ∷
    []

  example-positions : C.List (Fin 6)
  example-positions =
    (suc (suc zero)) C.∷
    (suc (suc (suc (suc zero)))) C.∷
    (suc zero) C.∷
    (suc (suc (suc (suc (suc zero))))) C.∷
    C.[]

  -- Static side: direct constructive pairing, same style as
  -- CertifiedResonanceComplete.
  inv-fn : Fin 6 → Fin 6
  inv-fn fzero                                         = fzero
  inv-fn (fsuc fzero)                                  = fsuc (fsuc (fsuc (fsuc (fsuc fzero))))
  inv-fn (fsuc (fsuc fzero))                           = fsuc (fsuc (fsuc (fsuc fzero)))
  inv-fn (fsuc (fsuc (fsuc fzero)))                    = fsuc (fsuc (fsuc fzero))
  inv-fn (fsuc (fsuc (fsuc (fsuc fzero))))             = fsuc (fsuc fzero)
  inv-fn (fsuc (fsuc (fsuc (fsuc (fsuc fzero)))))      = fsuc fzero

  -- ... same constructive PB / HZ setup as the old embedded example ...

  -- Runtime side: generated code now feeds the smaller PointwiseSafe contract.
  -- If the per-position distance inequalities are available, build the witness
  -- with the maintained helpers instead of supplying a raw StableOrbital proof.
  postulate
    pos₂-safe pos₄-safe pos₁-safe pos₅-safe
      : ∀ {R} → SafePos R (toℕ example-mid) _

  proof-pointwiseSafe : ∀ {R}
                      → PointwiseSafe R (toℕ example-mid) (mapFin example-positions)
  proof-pointwiseSafe =
    pointwiseSafeCons pos₂-safe
      (pointwiseSafeCons pos₄-safe
        (pointwiseSafeCons pos₁-safe
          (pointwiseSafeCons pos₅-safe pointwiseSafeNil)))

  example-dynamic : ∀ {R}
                  → InZone R (toℕ example-mid) (mapFin example-positions)
                  → ⊥
  example-dynamic {R} = ResonanceCertificateDyn.inviolability
                          example-certificate
                          example-positions
                          (proof-pointwiseSafe {R})
```

## Why It Moved

- The only local postulate in the embedded example was the runtime witness
  family. The active wrapper now consumes `PointwiseSafe`, not raw
  `StableOrbital`.
- Keeping that shell inside
  [`CertifiedResonanceParamDyn.agda`](./CertifiedResonanceParamDyn.agda)
  made the whole wrapper read as assumption-heavy even though its active static
  surface was already clean-local.
- Extracting the sketch keeps the signal while making the module boundary more
  honest.

## Current Status

- [`CertifiedResonanceParamDyn.agda`](./CertifiedResonanceParamDyn.agda) is the
  active dual wrapper surface.
- Future generated witnesses should target the maintained
  `pointwiseSafeCons` / `pointwiseSafeNil` helper path and let
  `CertifiedResonanceParamDyn.agda` derive `StableOrbital` internally.
