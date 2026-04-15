import Mathlib
import PrimeArithmetic.Sieve.RuntimeCrossOff
import PrimeArithmetic.Sieve.SegmentLayout

namespace PrimeArithmetic.Sieve

/-!
Runtime-facing arithmetic for the odd collection loop in `src/prime_sieve.rs`.

The runtime first truncates the raw segment upper bound to an odd number and
then collects exactly the odd candidates in that adjusted interval.
-/

/-- Adjusted odd upper bound used by the runtime collection loop. -/
def runtimeSegmentHi (lo limit : ℕ) : ℕ :=
  let hi := rawSegmentHi lo limit
  if hi % 2 = 0 then hi - 1 else hi

theorem runtimeSegmentHi_le_raw (lo limit : ℕ) :
    runtimeSegmentHi lo limit ≤ rawSegmentHi lo limit := by
  unfold runtimeSegmentHi
  by_cases hEven : rawSegmentHi lo limit % 2 = 0
  · simp [hEven]
  · simp [hEven]

theorem runtimeSegmentHi_le_limit (lo limit : ℕ) :
    runtimeSegmentHi lo limit ≤ limit := by
  exact le_trans (runtimeSegmentHi_le_raw lo limit) (rawSegmentHi_le_limit lo limit)

theorem runtimeSegmentHi_odd {lo limit : ℕ}
    (hLoLe : lo ≤ limit) (hLoPos : 0 < lo) :
    Odd (runtimeSegmentHi lo limit) := by
  by_cases hEven : rawSegmentHi lo limit % 2 = 0
  · have hRawPos : 0 < rawSegmentHi lo limit := lt_of_lt_of_le hLoPos (rawSegmentHi_ge_lo hLoLe)
    have hRawEven : Even (rawSegmentHi lo limit) := Nat.even_iff.mpr hEven
    rcases hRawEven with ⟨k, hk⟩
    have hEven' : (k + k) % 2 = 0 := by simpa [hk] using hEven
    refine ⟨k - 1, ?_⟩
    simp [runtimeSegmentHi, hk, hEven']
    omega
  · have hOddMod : rawSegmentHi lo limit % 2 = 1 := by
      have hCases := Nat.mod_two_eq_zero_or_one (rawSegmentHi lo limit)
      omega
    have hOddMod' : rawSegmentHi lo limit % 2 = 1 := hOddMod
    rw [Nat.odd_iff]
    simp [runtimeSegmentHi, hOddMod']

theorem runtimeSegmentHi_ge_lo {lo limit : ℕ}
    (hLoLe : lo ≤ limit) (hLoOdd : Odd lo) :
    lo ≤ runtimeSegmentHi lo limit := by
  by_cases hEven : rawSegmentHi lo limit % 2 = 0
  · have hRawGe : lo ≤ rawSegmentHi lo limit := rawSegmentHi_ge_lo hLoLe
    have hLoMod : lo % 2 = 1 := by simpa [Nat.odd_iff] using hLoOdd
    have hRawNeLo : rawSegmentHi lo limit ≠ lo := by
      intro hEq
      have : lo % 2 = 0 := by simpa [hEq] using hEven
      omega
    have hRawGt : lo < rawSegmentHi lo limit := lt_of_le_of_ne hRawGe hRawNeLo.symm
    simp [runtimeSegmentHi, hEven]
    omega
  · have hRawGe : lo ≤ rawSegmentHi lo limit := rawSegmentHi_ge_lo hLoLe
    simpa [runtimeSegmentHi, hEven] using hRawGe

theorem oddSegmentNumber_le_runtimeSegmentHi_of_index_le {lo limit idx : ℕ}
    (hLoLe : lo ≤ limit) (hLoPos : 0 < lo) (hLoOdd : Odd lo)
    (hIdx : idx ≤ oddSegmentIndex lo (runtimeSegmentHi lo limit)) :
    oddSegmentNumber lo idx ≤ runtimeSegmentHi lo limit := by
  have hHiGe : lo ≤ runtimeSegmentHi lo limit := runtimeSegmentHi_ge_lo hLoLe hLoOdd
  have hHiOdd : Odd (runtimeSegmentHi lo limit) := runtimeSegmentHi_odd hLoLe hLoPos
  have hHiEq :
      oddSegmentNumber lo (oddSegmentIndex lo (runtimeSegmentHi lo limit)) =
        runtimeSegmentHi lo limit := by
    exact oddSegmentNumber_oddSegmentIndex hHiGe hLoOdd hHiOdd
  unfold oddSegmentNumber at *
  omega

theorem exists_index_of_in_runtimeCollection {lo limit n : ℕ}
    (hLoLe : lo ≤ limit) (hLoPos : 0 < lo) (hLoOdd : Odd lo)
    (hLo : lo ≤ n) (hHi : n ≤ runtimeSegmentHi lo limit) (hNOdd : Odd n) :
    ∃ idx, idx ≤ oddSegmentIndex lo (runtimeSegmentHi lo limit) ∧
      oddSegmentNumber lo idx = n := by
  refine ⟨oddSegmentIndex lo n, ?_, ?_⟩
  · have hHiGe : lo ≤ runtimeSegmentHi lo limit := runtimeSegmentHi_ge_lo hLoLe hLoOdd
    have hHiOdd : Odd (runtimeSegmentHi lo limit) := runtimeSegmentHi_odd hLoLe hLoPos
    have hNEq : oddSegmentNumber lo (oddSegmentIndex lo n) = n :=
      oddSegmentNumber_oddSegmentIndex hLo hLoOdd hNOdd
    have hHiEq :
        oddSegmentNumber lo (oddSegmentIndex lo (runtimeSegmentHi lo limit)) =
          runtimeSegmentHi lo limit :=
      oddSegmentNumber_oddSegmentIndex hHiGe hLoOdd hHiOdd
    unfold oddSegmentNumber at *
    omega
  · exact oddSegmentNumber_oddSegmentIndex hLo hLoOdd hNOdd

theorem exists_index_of_runtimeMarkedBy_in_runtimeCollection
    {lo limit p segLo step : ℕ}
    (hLoLe : lo ≤ limit) (hLoPos : 0 < lo) (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ runtimeSegmentHi lo limit) :
    ∃ idx, idx ≤ oddSegmentIndex lo (runtimeSegmentHi lo limit) ∧
      oddSegmentNumber lo idx = runtimeMarkedBy p segLo step := by
  exact exists_index_of_in_runtimeCollection
    hLoLe hLoPos hLoOdd hLo hHi (odd_runtimeMarkedBy hpOdd)

theorem runtimeMarkedBy_index_le_runtimeSegmentHi
    {lo limit p segLo step : ℕ}
    (hLoLe : lo ≤ limit) (hLoPos : 0 < lo) (hLoOdd : Odd lo) (hpOdd : Odd p)
    (hLo : lo ≤ runtimeMarkedBy p segLo step)
    (hHi : runtimeMarkedBy p segLo step ≤ runtimeSegmentHi lo limit) :
    oddSegmentIndex lo (runtimeMarkedBy p segLo step) ≤
      oddSegmentIndex lo (runtimeSegmentHi lo limit) := by
  rcases exists_index_of_runtimeMarkedBy_in_runtimeCollection
      hLoLe hLoPos hLoOdd hpOdd hLo hHi with
    ⟨idx, hIdx, hEq⟩
  rw [← hEq, oddSegmentIndex_oddSegmentNumber]
  exact hIdx

end PrimeArithmetic.Sieve
