import Mathlib
import PrimeArithmetic.Density.CoprimeFilter
import PrimeArithmetic.Density.RadicalFilter

namespace PrimeArithmetic.Density

/-!
`unitResidues base` is the exact residue-class surface left after the classical
base filter: residues strictly below `base` that are coprime to `base`.

Its cardinality is `Nat.totient base`, so this module separates:

- `radical base`: which prime divisors matter for divisibility filtering
- `Nat.totient base`: how many admissible residue classes survive
-/

def unitResidues (base : ℕ) : Finset ℕ :=
  (Finset.range base).filter fun a => base.Coprime a

theorem mem_unitResidues {base a : ℕ} :
    a ∈ unitResidues base ↔ a < base ∧ base.Coprime a := by
  simp [unitResidues]

theorem card_unitResidues (base : ℕ) :
    (unitResidues base).card = Nat.totient base := by
  simpa [unitResidues] using Nat.totient_eq_card_coprime base

theorem primeGtBaseModCoprime
    {p base : ℕ} (hBase : 2 ≤ base) (hPrime : Nat.Prime p) (hGt : base < p) :
    (p % base).Coprime base := by
  exact (ZMod.coprime_mod_iff_coprime p base).2 <| primeGtBaseCoprime hBase hPrime hGt

theorem primeGtBaseModCoprimeToRadical
    {p base : ℕ} (hBase : 2 ≤ base) (hPrime : Nat.Prime p) (hGt : base < p) :
    (p % base).Coprime (radical base) := by
  exact
    (primeGtBaseModCoprime hBase hPrime hGt).of_dvd_right <|
      radical_dvd (n := base)

theorem primeGtBaseMod_memUnitResidues
    {p base : ℕ} (hBase : 2 ≤ base) (hPrime : Nat.Prime p) (hGt : base < p) :
    p % base ∈ unitResidues base := by
  refine mem_unitResidues.2 ?_
  constructor
  · exact Nat.mod_lt _ (lt_of_lt_of_le Nat.zero_lt_two hBase)
  · exact (primeGtBaseModCoprime hBase hPrime hGt).symm

theorem primeGtBaseModGcdRadicalEqOne
    {p base : ℕ} (hBase : 2 ≤ base) (hPrime : Nat.Prime p) (hGt : base < p) :
    Nat.gcd (p % base) (radical base) = 1 :=
  (primeGtBaseModCoprimeToRadical hBase hPrime hGt).gcd_eq_one

end PrimeArithmetic.Density
