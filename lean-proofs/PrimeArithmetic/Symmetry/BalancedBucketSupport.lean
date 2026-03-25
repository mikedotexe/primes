import PrimeArithmetic.Symmetry.ModularReflection

namespace PrimeArithmetic.Symmetry.BalancedBucketSupport

open PrimeArithmetic.Symmetry.ModularReflection

/-!
List-based support buckets for modular reflection.

This module isolates the constructive support machinery used by the Agda
auto-matching development:

- enumerate the indices with a given residue,
- prove the corresponding support lists are balanced and disjoint,
- pair two support lists elementwise with `zipPair`,
- prove the resulting pairing roundtrips on the left support.

The follow-on auto-certificate layer can then be built on top of these exact
list lemmas instead of relying on proof-transport-heavy subtype equivalences.
-/

def supportList {base n : ℕ} [NeZero base]
    (residue : Fin n → Fin base) (r : Fin base) : List (Fin n) :=
  (List.finRange n).filter fun i => decide (residue i = r)

@[simp] theorem mem_supportList {base n : ℕ} [NeZero base]
    {residue : Fin n → Fin base} {r : Fin base} {i : Fin n} :
    i ∈ supportList residue r ↔ residue i = r := by
  simp [supportList]

theorem nodup_supportList {base n : ℕ} [NeZero base]
    {residue : Fin n → Fin base} {r : Fin base} :
    (supportList residue r).Nodup := by
  simpa [supportList] using (List.nodup_finRange n).filter (fun i => decide (residue i = r))

theorem mem_supportList_self {base n : ℕ} [NeZero base]
    {residue : Fin n → Fin base} (i : Fin n) :
    i ∈ supportList residue (residue i) := by
  simp

theorem supportList_disjoint_of_ne {base n : ℕ} [NeZero base]
    {residue : Fin n → Fin base} {r s : Fin base} (hneq : r ≠ s) :
    List.Disjoint (supportList residue r) (supportList residue s) := by
  rw [List.disjoint_left]
  intro i hi
  have hr : residue i = r := (mem_supportList (residue := residue) (r := r) (i := i)).1 hi
  intro hs
  have hs' : residue i = s := (mem_supportList (residue := residue) (r := s) (i := i)).1 hs
  exact hneq (hr.symm.trans hs')

theorem supportList_disjoint_of_observedMove {base n : ℕ} [NeZero base]
    {residue : Fin n → Fin base}
    (hMove : ∀ i, reflect base (residue i) ≠ residue i) (r : Fin base) :
    List.Disjoint (supportList residue r) (supportList residue (reflect base r)) := by
  rw [List.disjoint_left]
  intro i hi
  have hr : residue i = r := (mem_supportList (residue := residue) (r := r) (i := i)).1 hi
  intro hs
  have hs' : residue i = reflect base r :=
    (mem_supportList (residue := residue) (r := reflect base r) (i := i)).1 hs
  apply hMove i
  calc
    reflect base (residue i) = reflect base r := by rw [hr]
    _ = residue i := hs'.symm

structure BalancedSupportLengths {base n : ℕ} [NeZero base]
    (residue : Fin n → Fin base) where
  balanced :
    ∀ r, (supportList residue r).length =
      (supportList residue (reflect base r)).length

def zipPair {n : ℕ} : List (Fin n) → List (Fin n) → Fin n → Fin n
  | [], _, i => i
  | _, [], i => i
  | x :: xs, y :: ys, i =>
      if i = x then y else if i = y then x else zipPair xs ys i

@[simp] theorem zipPair_nil_left {n : ℕ} (ys : List (Fin n)) (i : Fin n) :
    zipPair ([] : List (Fin n)) ys i = i := by
  cases ys <;> rfl

@[simp] theorem zipPair_nil_right {n : ℕ} (xs : List (Fin n)) (i : Fin n) :
    zipPair xs ([] : List (Fin n)) i = i := by
  cases xs <;> rfl

@[simp] theorem zipPair_left_head {n : ℕ} {x y : Fin n} {xs ys : List (Fin n)} :
    zipPair (x :: xs) (y :: ys) x = y := by
  simp [zipPair]

@[simp] theorem zipPair_right_head {n : ℕ} {x y : Fin n} {xs ys : List (Fin n)}
    (hxy : y ≠ x) :
    zipPair (x :: xs) (y :: ys) y = x := by
  simp [zipPair, hxy]

theorem zipPair_skips_heads {n : ℕ} {x y i : Fin n} {xs ys : List (Fin n)}
    (hix : i ≠ x) (hiy : i ≠ y) :
    zipPair (x :: xs) (y :: ys) i = zipPair xs ys i := by
  simp [zipPair, hix, hiy]

theorem tail_disjoint {n : ℕ} {x y : Fin n} {xs ys : List (Fin n)}
    (h : List.Disjoint (x :: xs) (y :: ys)) :
    List.Disjoint xs ys := by
  intro i hi hj
  exact h
    (show i ∈ x :: xs from by simpa [List.mem_cons] using (Or.inr hi : i = x ∨ i ∈ xs))
    (show i ∈ y :: ys from by simpa [List.mem_cons] using (Or.inr hj : i = y ∨ i ∈ ys))

theorem tail_mem_ne_right_head {n : ℕ} {x y : Fin n} {xs ys : List (Fin n)}
    (h : List.Disjoint (x :: xs) (y :: ys)) {i : Fin n} (hi : i ∈ xs) :
    i ≠ y := by
  intro hEq
  exact h
    (show i ∈ x :: xs from by simpa [List.mem_cons] using (Or.inr hi : i = x ∨ i ∈ xs))
    (show i ∈ y :: ys from by
      subst hEq
      simp)

theorem right_tail_mem_ne_left_head {n : ℕ} {x y : Fin n} {xs ys : List (Fin n)}
    (h : List.Disjoint (x :: xs) (y :: ys)) {i : Fin n} (hi : i ∈ ys) :
    i ≠ x := by
  intro hEq
  exact h
    (show i ∈ x :: xs from by
      subst hEq
      simp)
    (show i ∈ y :: ys from by simpa [List.mem_cons] using (Or.inr hi : i = y ∨ i ∈ ys))

theorem disjoint_heads_ne {n : ℕ} {x y : Fin n} {xs ys : List (Fin n)}
    (h : List.Disjoint (x :: xs) (y :: ys)) :
    x ≠ y := by
  intro hEq
  exact h
    (show x ∈ x :: xs from by simp)
    (show x ∈ y :: ys from by
      subst hEq
      simp)

theorem zipPair_mem_right_of_mem_left {n : ℕ} :
    ∀ {xs ys : List (Fin n)},
      xs.length = ys.length →
      xs.Disjoint ys →
      ∀ {i : Fin n}, i ∈ xs → zipPair xs ys i ∈ ys
  | [], _, _, _, i, hi => by cases hi
  | _ :: _, [], hLen, _, _, _ => by cases hLen
  | x :: xs, y :: ys, hLen, hDisj, i, hi => by
      by_cases hix : i = x
      · subst hix
        simp
      · have hiTail : i ∈ xs := by simpa [List.mem_cons, hix] using hi
        have hiy : i ≠ y := tail_mem_ne_right_head hDisj hiTail
        have hTail :
            zipPair xs ys i ∈ ys :=
          zipPair_mem_right_of_mem_left (xs := xs) (ys := ys)
            (Nat.succ.inj hLen) (tail_disjoint hDisj) hiTail
        rw [zipPair_skips_heads hix hiy]
        exact show zipPair xs ys i ∈ y :: ys from by
          simpa [List.mem_cons] using
            (Or.inr hTail : zipPair xs ys i = y ∨ zipPair xs ys i ∈ ys)

theorem zipPair_roundtrip_of_nodup_disjoint {n : ℕ} :
    ∀ {xs ys : List (Fin n)},
      xs.Nodup →
      ys.Nodup →
      xs.length = ys.length →
      xs.Disjoint ys →
      ∀ {i : Fin n}, i ∈ xs → zipPair ys xs (zipPair xs ys i) = i
  := by
  intro xs
  induction xs with
  | nil =>
      intro ys hNodupX hNodupY hLen hDisj i hi
      cases hi
  | cons x xs ih =>
      intro ys hNodupX hNodupY hLen hDisj i hi
      cases ys with
      | nil =>
          cases hLen
      | cons y ys =>
          rcases List.nodup_cons.mp hNodupX with ⟨hxFresh, hNodupX'⟩
          rcases List.nodup_cons.mp hNodupY with ⟨hyFresh, hNodupY'⟩
          by_cases hix : i = x
          · subst i
            simp [zipPair]
          · have hiTail : i ∈ xs := by
              simpa [List.mem_cons, hix] using hi
            have hiy : i ≠ y := tail_mem_ne_right_head hDisj hiTail
            have hjMem :
                zipPair xs ys i ∈ ys :=
              zipPair_mem_right_of_mem_left (xs := xs) (ys := ys)
                (Nat.succ.inj hLen) (tail_disjoint hDisj) hiTail
            have hjNeY : zipPair xs ys i ≠ y := by
              intro hEq
              exact hyFresh (hEq ▸ hjMem)
            have hjNeX : zipPair xs ys i ≠ x :=
              right_tail_mem_ne_left_head hDisj hjMem
            calc
              zipPair (y :: ys) (x :: xs) (zipPair (x :: xs) (y :: ys) i)
                  = zipPair (y :: ys) (x :: xs) (zipPair xs ys i) := by
                      rw [zipPair_skips_heads hix hiy]
              _ = zipPair ys xs (zipPair xs ys i) := by
                    rw [zipPair_skips_heads hjNeY hjNeX]
              _ = i := ih hNodupX' hNodupY' (Nat.succ.inj hLen) (tail_disjoint hDisj) hiTail

end PrimeArithmetic.Symmetry.BalancedBucketSupport
