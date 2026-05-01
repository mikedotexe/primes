import PrimeArithmetic.Structure.AffineTemplate

namespace PrimeArithmetic.Structure

/-!
Exact bounded-`k` compactness arithmetic for the symmetric template lane.

This module packages the small padding parameters that drive the maintained
bounded-`k` reports:

- `kOuter` inserts zeros between the outer and inner digits on both sides,
- `kInner` inserts zeros between the inner digits and the middle block,
- `k = (0, 0)` is the compact baseline,
- increasing either padding coordinate weakly increases width / diameter and
  shifts the digit positions outward.

The statements here are purely algebraic. They do not refer to primality,
density, or any threshold interpretation.
-/

structure BoundedKConfig where
  kOuter : ℕ
  kInner : ℕ
deriving DecidableEq, Repr

def BoundedKConfig.toSymmetricTemplateConfig
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) :
    SymmetricTemplateConfig where
  base := base
  outer := outer
  inner := inner
  kOuter := cfg.kOuter
  kInner := cfg.kInner
  middleWidth := middleWidth

def BoundedKConfig.paddingWeight (cfg : BoundedKConfig) : ℕ :=
  cfg.kOuter + cfg.kInner

def BoundedKConfig.diameter
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) : ℕ :=
  width (cfg.toSymmetricTemplateConfig base outer inner middleWidth)

@[simp] theorem toSymmetricTemplateConfig_kOuter
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) :
    (cfg.toSymmetricTemplateConfig base outer inner middleWidth).kOuter = cfg.kOuter := rfl

@[simp] theorem toSymmetricTemplateConfig_kInner
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) :
    (cfg.toSymmetricTemplateConfig base outer inner middleWidth).kInner = cfg.kInner := rfl

@[simp] theorem paddingWeight_zero_zero :
    ({ kOuter := 0, kInner := 0 } : BoundedKConfig).paddingWeight = 0 := rfl

@[simp] theorem paddingWeight_nonneg
    (cfg : BoundedKConfig) :
    0 ≤ cfg.paddingWeight := by
  omega

theorem zero_zero_min_paddingWeight
    (cfg : BoundedKConfig) :
    ({ kOuter := 0, kInner := 0 } : BoundedKConfig).paddingWeight ≤ cfg.paddingWeight := by
  simp [BoundedKConfig.paddingWeight]

@[simp] theorem rightInnerPosition_toSymmetricTemplateConfig
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) :
    rightInnerPosition (cfg.toSymmetricTemplateConfig base outer inner middleWidth) =
      cfg.kOuter + 1 := by
  simp [BoundedKConfig.toSymmetricTemplateConfig, rightInnerPosition]

@[simp] theorem middlePosition_toSymmetricTemplateConfig
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) :
    middlePosition (cfg.toSymmetricTemplateConfig base outer inner middleWidth) =
      cfg.kOuter + cfg.kInner + 2 := by
  simp [BoundedKConfig.toSymmetricTemplateConfig, middlePosition]

@[simp] theorem leftInnerPosition_toSymmetricTemplateConfig
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) :
    leftInnerPosition (cfg.toSymmetricTemplateConfig base outer inner middleWidth) =
      cfg.kOuter + 2 * cfg.kInner + middleWidth + 2 := by
  simp [BoundedKConfig.toSymmetricTemplateConfig, leftInnerPosition]

@[simp] theorem leftOuterPosition_toSymmetricTemplateConfig
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) :
    leftOuterPosition (cfg.toSymmetricTemplateConfig base outer inner middleWidth) =
      2 * cfg.kOuter + 2 * cfg.kInner + middleWidth + 3 := by
  simp [BoundedKConfig.toSymmetricTemplateConfig, leftOuterPosition]

@[simp] theorem width_toSymmetricTemplateConfig
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) :
    width (cfg.toSymmetricTemplateConfig base outer inner middleWidth) =
      2 * cfg.kOuter + 2 * cfg.kInner + middleWidth + 4 := by
  simp [BoundedKConfig.toSymmetricTemplateConfig, width]

@[simp] theorem diameter_eq_width
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) :
    cfg.diameter base outer inner middleWidth =
      2 * cfg.kOuter + 2 * cfg.kInner + middleWidth + 4 := by
  simp [BoundedKConfig.diameter]

theorem zero_zero_min_diameter
    (cfg : BoundedKConfig) (base outer inner middleWidth : ℕ) :
    ({ kOuter := 0, kInner := 0 } : BoundedKConfig).diameter base outer inner middleWidth ≤
      cfg.diameter base outer inner middleWidth := by
  simp [BoundedKConfig.diameter]

theorem paddingWeight_mono
    {cfg₁ cfg₂ : BoundedKConfig}
    (hOuter : cfg₁.kOuter ≤ cfg₂.kOuter)
    (hInner : cfg₁.kInner ≤ cfg₂.kInner) :
    cfg₁.paddingWeight ≤ cfg₂.paddingWeight := by
  simp [BoundedKConfig.paddingWeight]
  omega

theorem diameter_mono
    {cfg₁ cfg₂ : BoundedKConfig}
    (hOuter : cfg₁.kOuter ≤ cfg₂.kOuter)
    (hInner : cfg₁.kInner ≤ cfg₂.kInner)
    (base outer inner middleWidth : ℕ) :
    cfg₁.diameter base outer inner middleWidth ≤
      cfg₂.diameter base outer inner middleWidth := by
  simp [BoundedKConfig.diameter]
  omega

theorem rightInnerPosition_mono
    {cfg₁ cfg₂ : BoundedKConfig}
    (hOuter : cfg₁.kOuter ≤ cfg₂.kOuter)
    (base outer inner middleWidth : ℕ) :
    rightInnerPosition (cfg₁.toSymmetricTemplateConfig base outer inner middleWidth) ≤
      rightInnerPosition (cfg₂.toSymmetricTemplateConfig base outer inner middleWidth) := by
  simp
  omega

theorem middlePosition_mono
    {cfg₁ cfg₂ : BoundedKConfig}
    (hOuter : cfg₁.kOuter ≤ cfg₂.kOuter)
    (hInner : cfg₁.kInner ≤ cfg₂.kInner)
    (base outer inner middleWidth : ℕ) :
    middlePosition (cfg₁.toSymmetricTemplateConfig base outer inner middleWidth) ≤
      middlePosition (cfg₂.toSymmetricTemplateConfig base outer inner middleWidth) := by
  simp
  omega

theorem leftInnerPosition_mono
    {cfg₁ cfg₂ : BoundedKConfig}
    (hOuter : cfg₁.kOuter ≤ cfg₂.kOuter)
    (hInner : cfg₁.kInner ≤ cfg₂.kInner)
    (base outer inner middleWidth : ℕ) :
    leftInnerPosition (cfg₁.toSymmetricTemplateConfig base outer inner middleWidth) ≤
      leftInnerPosition (cfg₂.toSymmetricTemplateConfig base outer inner middleWidth) := by
  simp
  omega

theorem leftOuterPosition_mono
    {cfg₁ cfg₂ : BoundedKConfig}
    (hOuter : cfg₁.kOuter ≤ cfg₂.kOuter)
    (hInner : cfg₁.kInner ≤ cfg₂.kInner)
    (base outer inner middleWidth : ℕ) :
    leftOuterPosition (cfg₁.toSymmetricTemplateConfig base outer inner middleWidth) ≤
      leftOuterPosition (cfg₂.toSymmetricTemplateConfig base outer inner middleWidth) := by
  simp
  omega

def base10_compact : BoundedKConfig := { kOuter := 0, kInner := 0 }

def base10_offset : BoundedKConfig := { kOuter := 1, kInner := 0 }

example : base10_compact.paddingWeight = 0 := rfl

example : base10_compact.diameter 10 3 7 2 = 6 := by
  native_decide

example : base10_compact.diameter 10 3 7 2 ≤ base10_offset.diameter 10 3 7 2 := by
  exact zero_zero_min_diameter base10_offset 10 3 7 2

end PrimeArithmetic.Structure
