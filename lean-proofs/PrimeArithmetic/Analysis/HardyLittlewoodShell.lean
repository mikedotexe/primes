import Mathlib
import PrimeArithmetic.Density.RadicalFilter

open scoped BigOperators

namespace PrimeArithmetic.Analysis

open PrimeArithmetic.Density

/-!
Conservative Hardy-Littlewood shell.

This module does not claim a new asymptotic theorem. It only fixes a small,
classical notation layer and the exact local-factor bookkeeping that the repo
already uses on the runtime side:

- ordered versus unordered pair conventions,
- the multiplicative Goldbach local factors `(p - 1) / (p - 2)` for odd prime
  divisors,
- the observation that this singular-series support depends only on distinct
  odd prime factors, hence only on `radical n`,
- the standard notation for the logarithmic main-term scale and Poisson-style
  coverage transform.
-/

/-- Pair-counting convention for Goldbach-style heuristics. -/
inductive PairCount where
  | ordered
  | unordered
  deriving DecidableEq, Repr

/-- Exact multiplicity attached to the pair-counting convention. -/
def pairMultiplicityQ : PairCount → ℚ
  | .ordered => 2
  | .unordered => 1

/-- Real-valued multiplicity, used only for the notation shell. -/
def pairMultiplicityR (pairing : PairCount) : ℝ :=
  (pairMultiplicityQ pairing : ℝ)

@[simp] theorem pairMultiplicityQ_ordered :
    pairMultiplicityQ PairCount.ordered = 2 := rfl

@[simp] theorem pairMultiplicityQ_unordered :
    pairMultiplicityQ PairCount.unordered = 1 := rfl

@[simp] theorem pairMultiplicityR_ordered :
    pairMultiplicityR PairCount.ordered = 2 := by
  norm_num [pairMultiplicityR]

@[simp] theorem pairMultiplicityR_unordered :
    pairMultiplicityR PairCount.unordered = 1 := by
  norm_num [pairMultiplicityR]

/-- Front factor `κ` written in terms of the twin-prime constant `C₂`. -/
def kappaShellQ (pairing : PairCount) (C2 : ℚ) : ℚ :=
  pairMultiplicityQ pairing * C2

/-- Real-valued front factor `κ` written in terms of the twin-prime constant `C₂`. -/
def kappaShell (pairing : PairCount) (C2 : ℝ) : ℝ :=
  pairMultiplicityR pairing * C2

@[simp] theorem kappaShellQ_unordered (C2 : ℚ) :
    kappaShellQ PairCount.unordered C2 = C2 := by
  simp [kappaShellQ]

@[simp] theorem kappaShellQ_ordered (C2 : ℚ) :
    kappaShellQ PairCount.ordered C2 = 2 * C2 := by
  simp [kappaShellQ]

@[simp] theorem kappaShell_unordered (C2 : ℝ) :
    kappaShell PairCount.unordered C2 = C2 := by
  simp [kappaShell]

@[simp] theorem kappaShell_ordered (C2 : ℝ) :
    kappaShell PairCount.ordered C2 = 2 * C2 := by
  simp [kappaShell]

theorem kappaShell_ordered_eq_two_mul_unordered (C2 : ℝ) :
    kappaShell PairCount.ordered C2 = 2 * kappaShell PairCount.unordered C2 := by
  simp [kappaShell]

/-- Local multiplicative Goldbach factor `(p - 1)/(p - 2)` for an odd prime `p > 2`. -/
def goldbachLocalFactorQ (p : ℕ) : ℚ :=
  ((p : ℚ) - 1) / ((p : ℚ) - 2)

theorem goldbachLocalFactorQ_pos {p : ℕ} (hp : 2 < p) :
    0 < goldbachLocalFactorQ p := by
  unfold goldbachLocalFactorQ
  have hpQ : (2 : ℚ) < p := by
    exact_mod_cast hp
  have hNum : 0 < (p : ℚ) - 1 := by
    linarith
  have hDen : 0 < (p : ℚ) - 2 := by
    linarith
  exact div_pos hNum hDen

@[simp] theorem goldbachLocalFactorQ_three :
    goldbachLocalFactorQ 3 = 2 := by
  norm_num [goldbachLocalFactorQ]

@[simp] theorem goldbachLocalFactorQ_five :
    goldbachLocalFactorQ 5 = (4 : ℚ) / 3 := by
  norm_num [goldbachLocalFactorQ]

/-- Distinct odd prime support contributing to the classical Goldbach local factor. -/
def oddPrimeSupport (n : ℕ) : Finset ℕ :=
  n.primeFactors.erase 2

theorem oddPrimeSupport_two_mul (n : ℕ) :
    oddPrimeSupport (2 * n) = oddPrimeSupport n := by
  by_cases hn : n = 0
  · subst hn
    simp [oddPrimeSupport]
  · unfold oddPrimeSupport
    rw [Nat.primeFactors_mul (by decide : 2 ≠ 0) hn,
      (show Nat.Prime 2 by decide).primeFactors]
    simp

theorem oddPrimeSupport_two_pow_mul (n k : ℕ) (hk : k ≠ 0) :
    oddPrimeSupport (2 ^ k * n) = oddPrimeSupport n := by
  by_cases hn : n = 0
  · subst hn
    simp [oddPrimeSupport]
  · unfold oddPrimeSupport
    rw [Nat.primeFactors_mul (pow_ne_zero _ (by decide : 2 ≠ 0)) hn,
      Nat.primeFactors_prime_pow hk (show Nat.Prime 2 by decide)]
    simp

/-- Every factor in the odd-prime support is an odd prime, hence strictly larger than `2`. -/
theorem two_lt_of_mem_oddPrimeSupport {n p : ℕ} (hp : p ∈ oddPrimeSupport n) :
    2 < p := by
  unfold oddPrimeSupport at hp
  rcases Finset.mem_erase.mp hp with ⟨hpNeTwo, hpMem⟩
  have hpPrime : p.Prime := Nat.prime_of_mem_primeFactors hpMem
  exact lt_of_le_of_ne hpPrime.two_le (Ne.symm hpNeTwo)

/-- Exact multiplicative singular-series shell over the distinct odd prime support. -/
def goldbachSingularSeriesQ (n : ℕ) : ℚ :=
  Finset.prod (oddPrimeSupport n) fun p => goldbachLocalFactorQ p

theorem goldbachSingularSeriesQ_two_mul (n : ℕ) :
    goldbachSingularSeriesQ (2 * n) = goldbachSingularSeriesQ n := by
  unfold goldbachSingularSeriesQ
  rw [oddPrimeSupport_two_mul]

theorem goldbachSingularSeriesQ_two_pow_mul (n k : ℕ) (hk : k ≠ 0) :
    goldbachSingularSeriesQ (2 ^ k * n) = goldbachSingularSeriesQ n := by
  unfold goldbachSingularSeriesQ
  rw [oddPrimeSupport_two_pow_mul n k hk]

theorem goldbachSingularSeriesQ_pos (n : ℕ) :
    0 < goldbachSingularSeriesQ n := by
  unfold goldbachSingularSeriesQ
  exact Finset.prod_pos fun p hp => goldbachLocalFactorQ_pos (two_lt_of_mem_oddPrimeSupport hp)

theorem oddPrimeSupport_radical (n : ℕ) :
    oddPrimeSupport (radical n) = oddPrimeSupport n := by
  unfold oddPrimeSupport
  rw [primeFactors_radical]

theorem goldbachSingularSeriesQ_radical (n : ℕ) :
    goldbachSingularSeriesQ (radical n) = goldbachSingularSeriesQ n := by
  unfold goldbachSingularSeriesQ
  rw [oddPrimeSupport_radical]

theorem goldbachSingularSeriesQ_two : goldbachSingularSeriesQ 2 = 1 := by
  native_decide

theorem goldbachSingularSeriesQ_six : goldbachSingularSeriesQ 6 = 2 := by
  native_decide

theorem goldbachSingularSeriesQ_thirty : goldbachSingularSeriesQ 30 = (8 : ℚ) / 3 := by
  native_decide

/-- Classical logarithmic scale `n / (log n)^2` using the natural logarithm. -/
noncomputable def goldbachLogScale (n : ℝ) : ℝ :=
  n / (Real.log n) ^ 2

/-- Symbolic Hardy-Littlewood main term `κ · S₂(n) · n / (log n)^2`. -/
noncomputable def goldbachLambdaShell (pairing : PairCount) (C2 singularSeries n : ℝ) : ℝ :=
  kappaShell pairing C2 * singularSeries * goldbachLogScale n

/-- Symbolic truncated Hardy-Littlewood main term with an externally supplied truncated scale. -/
noncomputable def goldbachLambdaTruncatedShell
    (pairing : PairCount) (C2 singularSeries truncatedScale : ℝ) : ℝ :=
  kappaShell pairing C2 * singularSeries * truncatedScale

/-- Poisson-style coverage transform `1 - e^{-λ}`. -/
noncomputable def goldbachCoverageFromLambda (lam : ℝ) : ℝ :=
  1 - Real.exp (-lam)

theorem goldbachLambdaShell_ordered_eq_two_mul_unordered
    (C2 singularSeries n : ℝ) :
    goldbachLambdaShell PairCount.ordered C2 singularSeries n =
      2 * goldbachLambdaShell PairCount.unordered C2 singularSeries n := by
  simp [goldbachLambdaShell, goldbachLogScale, kappaShell]
  ring

theorem goldbachLambdaShell_singularSeriesQ_two_mul
    (pairing : PairCount) (C2 n : ℝ) (m : ℕ) :
    goldbachLambdaShell pairing C2 (goldbachSingularSeriesQ (2 * m)) n =
      goldbachLambdaShell pairing C2 (goldbachSingularSeriesQ m) n := by
  rw [goldbachSingularSeriesQ_two_mul]

theorem goldbachLambdaShell_singularSeriesQ_two_pow_mul
    (pairing : PairCount) (C2 n : ℝ) (m k : ℕ) (hk : k ≠ 0) :
    goldbachLambdaShell pairing C2 (goldbachSingularSeriesQ (2 ^ k * m)) n =
      goldbachLambdaShell pairing C2 (goldbachSingularSeriesQ m) n := by
  rw [goldbachSingularSeriesQ_two_pow_mul m k hk]

theorem goldbachLambdaTruncatedShell_ordered_eq_two_mul_unordered
    (C2 singularSeries truncatedScale : ℝ) :
    goldbachLambdaTruncatedShell PairCount.ordered C2 singularSeries truncatedScale =
      2 * goldbachLambdaTruncatedShell PairCount.unordered C2 singularSeries truncatedScale := by
  simp [goldbachLambdaTruncatedShell, kappaShell]
  ring

theorem goldbachLambdaTruncatedShell_singularSeriesQ_two_mul
    (pairing : PairCount) (C2 truncatedScale : ℝ) (m : ℕ) :
    goldbachLambdaTruncatedShell pairing C2 (goldbachSingularSeriesQ (2 * m)) truncatedScale =
      goldbachLambdaTruncatedShell pairing C2 (goldbachSingularSeriesQ m) truncatedScale := by
  rw [goldbachSingularSeriesQ_two_mul]

theorem goldbachLambdaTruncatedShell_singularSeriesQ_two_pow_mul
    (pairing : PairCount) (C2 truncatedScale : ℝ) (m k : ℕ) (hk : k ≠ 0) :
    goldbachLambdaTruncatedShell pairing C2 (goldbachSingularSeriesQ (2 ^ k * m)) truncatedScale =
      goldbachLambdaTruncatedShell pairing C2 (goldbachSingularSeriesQ m) truncatedScale := by
  rw [goldbachSingularSeriesQ_two_pow_mul m k hk]

@[simp] theorem goldbachCoverageFromLambda_zero :
    goldbachCoverageFromLambda 0 = 0 := by
  simp [goldbachCoverageFromLambda]

theorem goldbachCoverageFromLambda_nonneg {lam : ℝ} (hLam : 0 ≤ lam) :
    0 ≤ goldbachCoverageFromLambda lam := by
  unfold goldbachCoverageFromLambda
  have hExpLe : Real.exp (-lam) ≤ 1 := by
    have hNeg : -lam ≤ 0 := by linarith
    simpa using Real.exp_le_exp.mpr hNeg
  linarith

theorem goldbachCoverageFromLambda_lt_one (lam : ℝ) :
    goldbachCoverageFromLambda lam < 1 := by
  unfold goldbachCoverageFromLambda
  have hPos : 0 < Real.exp (-lam) := Real.exp_pos (-lam)
  linarith

theorem goldbachCoverageFromLambda_pos {lam : ℝ} (hLam : 0 < lam) :
    0 < goldbachCoverageFromLambda lam := by
  unfold goldbachCoverageFromLambda
  have hExpLt : Real.exp (-lam) < 1 := by
    have hNeg : -lam < 0 := by
      linarith
    simpa using Real.exp_lt_exp.mpr hNeg
  linarith

theorem goldbachCoverageFromLambda_le_one (lam : ℝ) :
    goldbachCoverageFromLambda lam ≤ 1 :=
  le_of_lt (goldbachCoverageFromLambda_lt_one lam)

theorem goldbachCoverageFromLambda_monotone :
    Monotone goldbachCoverageFromLambda := by
  intro a b hab
  unfold goldbachCoverageFromLambda
  have hNeg : -b ≤ -a := by
    linarith
  have hExp : Real.exp (-b) ≤ Real.exp (-a) := Real.exp_le_exp.mpr hNeg
  linarith

end PrimeArithmetic.Analysis
