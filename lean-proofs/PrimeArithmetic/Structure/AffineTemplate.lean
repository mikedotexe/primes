import Mathlib

namespace PrimeArithmetic.Structure

/-!
Affine form of the symmetric digit-template construction.

Once the base, boundary digits, zero-padding counts, and middle-block width are
fixed, the constructed value is affine in the middle block:

`template(seed) = shift + gradient * seed`.

This captures one of the strongest exact structural signals from the older repo
prose in a standard arithmetic form. The theorem is purely algebraic: no
primality or density interpretation is used here.
-/

structure SymmetricTemplateConfig where
  base : ℕ
  outer : ℕ
  inner : ℕ
  kOuter : ℕ
  kInner : ℕ
  middleWidth : ℕ

def rightInnerPosition (conf : SymmetricTemplateConfig) : ℕ :=
  conf.kOuter + 1

def middlePosition (conf : SymmetricTemplateConfig) : ℕ :=
  conf.kOuter + conf.kInner + 2

def leftInnerPosition (conf : SymmetricTemplateConfig) : ℕ :=
  conf.kOuter + 2 * conf.kInner + conf.middleWidth + 2

def leftOuterPosition (conf : SymmetricTemplateConfig) : ℕ :=
  2 * conf.kOuter + 2 * conf.kInner + conf.middleWidth + 3

def width (conf : SymmetricTemplateConfig) : ℕ :=
  2 * conf.kOuter + 2 * conf.kInner + conf.middleWidth + 4

/-- Numeric value of the symmetric template with a fixed-width middle block. -/
def templateValue (conf : SymmetricTemplateConfig) (seed : ℕ) : ℕ :=
  conf.outer * conf.base ^ leftOuterPosition conf
    + conf.inner * conf.base ^ leftInnerPosition conf
    + seed * conf.base ^ middlePosition conf
    + conf.inner * conf.base ^ rightInnerPosition conf
    + conf.outer

def templateShift (conf : SymmetricTemplateConfig) : ℕ :=
  templateValue conf 0

def templateGradient (conf : SymmetricTemplateConfig) : ℕ :=
  conf.base ^ middlePosition conf

theorem templateValue_eq_shift_add_gradient
    (conf : SymmetricTemplateConfig) (seed : ℕ) :
    templateValue conf seed = templateShift conf + seed * templateGradient conf := by
  unfold templateShift templateGradient
  simp [templateValue]
  ac_rfl

theorem templateValue_modEq_affine
    (conf : SymmetricTemplateConfig) (seed modulus : ℕ) :
    templateValue conf seed ≡ templateShift conf + seed * templateGradient conf [MOD modulus] := by
  rw [Nat.ModEq, templateValue_eq_shift_add_gradient]

theorem templateValue_mod_eq_affine
    (conf : SymmetricTemplateConfig) (seed modulus : ℕ) :
    templateValue conf seed % modulus =
      (templateShift conf + seed * templateGradient conf) % modulus := by
  simpa [Nat.ModEq] using templateValue_modEq_affine conf seed modulus

def base6_15 : SymmetricTemplateConfig where
  base := 6
  outer := 1
  inner := 5
  kOuter := 0
  kInner := 0
  middleWidth := 1

def base10_37 : SymmetricTemplateConfig where
  base := 10
  outer := 3
  inner := 7
  kOuter := 1
  kInner := 1
  middleWidth := 1

theorem width_base6_15 : width base6_15 = 5 := by
  native_decide

theorem width_base10_37 : width base10_37 = 9 := by
  native_decide

theorem templateValue_base6_15_seed0 : templateValue base6_15 0 = 2407 := by
  native_decide

theorem templateValue_base6_15_seed4 : templateValue base6_15 4 = 2551 := by
  native_decide

theorem templateValue_base10_37_seed0 : templateValue base10_37 0 = 307000703 := by
  native_decide

theorem templateValue_base10_37_seed5 : templateValue base10_37 5 = 307050703 := by
  native_decide

theorem templateValue_base6_15_mod7_seed4 :
    templateValue base6_15 4 % 7 = 3 := by
  native_decide

theorem templateAffine_base6_15_mod7_seed4 :
    (templateShift base6_15 + 4 * templateGradient base6_15) % 7 = 3 := by
  native_decide

theorem templateValue_base10_37_mod11_seed5 :
    templateValue base10_37 5 % 11 = 3 := by
  native_decide

theorem templateAffine_base10_37_mod11_seed5 :
    (templateShift base10_37 + 5 * templateGradient base10_37) % 11 = 3 := by
  native_decide

end PrimeArithmetic.Structure
