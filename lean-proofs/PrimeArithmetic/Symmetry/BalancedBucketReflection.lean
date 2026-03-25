import PrimeArithmetic.Symmetry.BalancedBucketSupport
import PrimeArithmetic.Symmetry.CertificateReflection

namespace PrimeArithmetic.Symmetry.BalancedBucketReflection

open PrimeArithmetic.Foundation
open PrimeArithmetic.Symmetry.ModularReflection
open PrimeArithmetic.Symmetry.CertificateReflection
open PrimeArithmetic.Symmetry.BalancedBucketSupport

/-!
Automatic reflection certificates from balanced residue buckets.

This module is the Lean counterpart to the Agda auto-matching layer:

- a caller supplies residue counts,
- proves those counts are balanced under modular reflection,
- proves the counts match the observed support lists,
- excludes the fixed residues `0` and `base / 2`,
- and receives a reusable `ReflectionCertificate`.

The combinatorial heart is the `supportList` / `zipPair` substrate from
`BalancedBucketSupport`.
-/

def SupportCountsAgree {base n : ℕ} [NeZero base]
    (residue : Fin n → Fin base) (count : Fin base → ℕ) : Prop :=
  ∀ r, (supportList residue r).length = count r

structure BalancedBuckets {base n : ℕ} [NeZero base]
    (residue : Fin n → Fin base) (count : Fin base → ℕ) where
  balanced : ∀ r, count r = count (reflect base r)

def supportLengthsFromCounts {base n : ℕ} [NeZero base]
    {residue : Fin n → Fin base} {count : Fin base → ℕ}
    (buckets : BalancedBuckets residue count)
    (supports : SupportCountsAgree residue count) :
    BalancedSupportLengths residue where
  balanced := by
    intro r
    calc
      (supportList residue r).length = count r := supports r
      _ = count (reflect base r) := buckets.balanced r
      _ = (supportList residue (reflect base r)).length := by
            symm
            exact supports (reflect base r)

def autoMate {base n : ℕ} [NeZero base]
    (residue : Fin n → Fin base) (i : Fin n) : Fin n :=
  zipPair
    (supportList residue (residue i))
    (supportList residue (reflect base (residue i)))
    i

theorem autoMate_mem_reflectedSupport {base n : ℕ} [NeZero base]
    {residue : Fin n → Fin base}
    (supportLengths : BalancedSupportLengths residue)
    (hMove : ∀ i, reflect base (residue i) ≠ residue i) (i : Fin n) :
    autoMate residue i ∈ supportList residue (reflect base (residue i)) := by
  let r := residue i
  have hi : i ∈ supportList residue r := by
    simp [r]
  have hZip :
      zipPair (supportList residue r) (supportList residue (reflect base r)) i
        ∈ supportList residue (reflect base r) := by
    exact zipPair_mem_right_of_mem_left
      (xs := supportList residue r)
      (ys := supportList residue (reflect base r))
      (by simpa [r] using supportLengths.balanced r)
      (by simpa [r] using supportList_disjoint_of_observedMove (residue := residue) hMove r)
      hi
  simpa [autoMate, r] using hZip

theorem autoMate_equivariant_from {base n : ℕ} [NeZero base]
    {residue : Fin n → Fin base}
    (supportLengths : BalancedSupportLengths residue)
    (hMove : ∀ i, reflect base (residue i) ≠ residue i) (i : Fin n) :
    reflect base (residue i) = residue (autoMate residue i) := by
  exact ((mem_supportList (residue := residue) (r := reflect base (residue i))
    (i := autoMate residue i)).1
      (autoMate_mem_reflectedSupport supportLengths hMove i)).symm

theorem autoMate_involutive_from {base n : ℕ} [NeZero base]
    {residue : Fin n → Fin base}
    (supportLengths : BalancedSupportLengths residue)
    (hMove : ∀ i, reflect base (residue i) ≠ residue i) :
    Function.Involutive (autoMate residue) := by
  intro i
  let r := residue i
  have hi : i ∈ supportList residue r := by
    simp [r]
  have hSecond :
      autoMate residue (autoMate residue i) =
        zipPair
          (supportList residue (reflect base r))
          (supportList residue r)
          (autoMate residue i) := by
    have hAuto :
        residue
            (zipPair
              (supportList residue r)
              (supportList residue (reflect base r))
              i) = reflect base r := by
      exact (mem_supportList
        (residue := residue)
        (r := reflect base r)
        (i := zipPair
          (supportList residue r)
          (supportList residue (reflect base r))
          i)).1
            (zipPair_mem_right_of_mem_left
              (xs := supportList residue r)
              (ys := supportList residue (reflect base r))
              (supportLengths.balanced r)
              (supportList_disjoint_of_observedMove (residue := residue) hMove r)
              hi)
    unfold autoMate
    rw [hAuto]
    have hInv : reflect base (reflect base r) = r := by
      simpa using (reflect_involutive (base := base) r)
    rw [hInv]
  have hRound :
      zipPair
          (supportList residue (reflect base r))
          (supportList residue r)
          (zipPair
            (supportList residue r)
            (supportList residue (reflect base r))
            i) = i := by
    exact zipPair_roundtrip_of_nodup_disjoint
      (xs := supportList residue r)
      (ys := supportList residue (reflect base r))
      (nodup_supportList (residue := residue) (r := r))
      (nodup_supportList (residue := residue) (r := reflect base r))
      (supportLengths.balanced r)
      (supportList_disjoint_of_observedMove (residue := residue) hMove r)
      hi
  calc
    autoMate residue (autoMate residue i) =
        zipPair
          (supportList residue (reflect base r))
          (supportList residue r)
          (autoMate residue i) := hSecond
    _ = i := by
          simpa [autoMate, r] using hRound

structure BalancedBucketReflectionCertificate {base n : ℕ} [NeZero base]
    (hEven : Even base) (residue : Fin n → Fin base) where
  count : Fin base → ℕ
  balancedWitness : BalancedBuckets residue count
  supportCounts : SupportCountsAgree residue count
  fixedPointExclusion : ObservedFixedPointExclusion residue

def supportLengths {base n : ℕ} [NeZero base] {hEven : Even base}
    {residue : Fin n → Fin base}
    (cert : BalancedBucketReflectionCertificate hEven residue) :
    BalancedSupportLengths residue :=
  supportLengthsFromCounts cert.balancedWitness cert.supportCounts

def reflectionCertificate {base n : ℕ} [NeZero base] {hEven : Even base}
    {residue : Fin n → Fin base}
    (cert : BalancedBucketReflectionCertificate hEven residue) :
    ReflectionCertificate hEven residue where
  mate := autoMate residue
  mateInvolutive := autoMate_involutive_from (supportLengths cert)
    (observedResiduesMove cert.fixedPointExclusion)
  equivariant := autoMate_equivariant_from (supportLengths cert)
    (observedResiduesMove cert.fixedPointExclusion)
  fixedPointExclusion := cert.fixedPointExclusion

def pairing {base n : ℕ} [NeZero base] {hEven : Even base}
    {residue : Fin n → Fin base}
    (cert : BalancedBucketReflectionCertificate hEven residue) :
    PerfectPairing (symmetryData base hEven) residue :=
  CertificateReflection.pairing (reflectionCertificate cert)

theorem midpoint_not_visited {base n : ℕ} [NeZero base] {hEven : Even base}
    {residue : Fin n → Fin base}
    (cert : BalancedBucketReflectionCertificate hEven residue) (i : Fin n) :
    residue i ≠ midpoint base :=
  CertificateReflection.midpoint_not_visited (reflectionCertificate cert) i

theorem midpoint_not_in_range {base n : ℕ} [NeZero base] {hEven : Even base}
    {residue : Fin n → Fin base}
    (cert : BalancedBucketReflectionCertificate hEven residue) :
    midpoint base ∉ Set.range residue :=
  CertificateReflection.midpoint_not_in_range (reflectionCertificate cert)

theorem zero_not_in_range {base n : ℕ} [NeZero base] {hEven : Even base}
    {residue : Fin n → Fin base}
    (cert : BalancedBucketReflectionCertificate hEven residue) :
    (0 : Fin base) ∉ Set.range residue :=
  CertificateReflection.zero_not_in_range (reflectionCertificate cert)

end PrimeArithmetic.Symmetry.BalancedBucketReflection
