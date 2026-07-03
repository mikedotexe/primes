import PrimeArithmetic.Structure.AffineTemplate

namespace PrimeArithmetic.Witness

open PrimeArithmetic.Structure

/-!
Generic proof object for finite witness-search replay certificates.

This module formalizes only the residue-funnel replay layer.  It does not claim
that a surviving witness is prime; large witness primality remains outside this
certificate and is tracked separately as probable-prime metadata.
-/

def ResidueRejected
    (config : SymmetricTemplateConfig) (rejections : List (ℕ × ℕ)) (seed : ℕ) : Prop :=
  ∃ modulus, (seed, modulus) ∈ rejections ∧ templateValue config seed % modulus = 0

def ResidueSurvives
    (config : SymmetricTemplateConfig) (residueModuli : List ℕ) (seed : ℕ) : Prop :=
  ∀ {modulus : ℕ}, modulus ∈ residueModuli -> templateValue config seed % modulus ≠ 0

def ReplayRowAccounted
    (config : SymmetricTemplateConfig) (residueModuli : List ℕ) (witnessSeed : ℕ)
    (rejections : List (ℕ × ℕ)) (survivors : List ℕ) (seed : ℕ) : Prop :=
  ResidueRejected config rejections seed ∨
    (seed ∈ survivors ∧ seed ≠ witnessSeed ∧ ResidueSurvives config residueModuli seed)

def RejectedSeeds (rejections : List (ℕ × ℕ)) : List ℕ :=
  rejections.map Prod.fst

def contiguousReplaySeeds (inputSeed scannedSeedCount : ℕ) : List ℕ :=
  (List.range scannedSeedCount).map (fun offset => inputSeed + offset)

theorem contiguousReplaySeeds_length (inputSeed scannedSeedCount : ℕ) :
    (contiguousReplaySeeds inputSeed scannedSeedCount).length = scannedSeedCount := by
  simp [contiguousReplaySeeds]

theorem mem_contiguousReplaySeeds {inputSeed scannedSeedCount seed : ℕ} :
    seed ∈ contiguousReplaySeeds inputSeed scannedSeedCount ↔
      inputSeed ≤ seed ∧ seed < inputSeed + scannedSeedCount := by
  constructor
  · intro h
    rw [contiguousReplaySeeds, List.mem_map] at h
    rcases h with ⟨offset, hOffset, hSeed⟩
    rw [List.mem_range] at hOffset
    rw [← hSeed]
    constructor <;> omega
  · intro h
    rw [contiguousReplaySeeds, List.mem_map]
    refine ⟨seed - inputSeed, ?_, ?_⟩
    · rw [List.mem_range]
      omega
    · omega

structure SearchReplayCertificate where
  config : SymmetricTemplateConfig
  residueModuli : List ℕ
  replaySeeds : List ℕ
  witnessSeed : ℕ
  residueRejections : List (ℕ × ℕ)
  residueSurvivors : List ℕ
  nonAcceptedResidueSurvivors : List ℕ
  acceptedResidueSurvivors : List ℕ
  witnessInReplay : witnessSeed ∈ replaySeeds
  witnessSurvivor : witnessSeed ∈ residueSurvivors
  witnessAccepted : witnessSeed ∈ acceptedResidueSurvivors
  preWitnessClassified :
    ∀ {seed : ℕ}, seed ∈ replaySeeds -> seed < witnessSeed ->
      (∃ modulus, (seed, modulus) ∈ residueRejections) ∨ seed ∈ residueSurvivors
  rejectionsReject :
    ∀ {seed modulus : ℕ}, (seed, modulus) ∈ residueRejections ->
      templateValue config seed % modulus = 0
  survivorsSurvive :
    ∀ {seed modulus : ℕ}, seed ∈ residueSurvivors -> modulus ∈ residueModuli ->
      templateValue config seed % modulus ≠ 0
  survivorsInReplay :
    ∀ {seed : ℕ}, seed ∈ residueSurvivors -> seed ∈ replaySeeds
  replaySurvivorsComplete :
    ∀ {seed : ℕ}, seed ∈ replaySeeds -> ResidueSurvives config residueModuli seed ->
      seed ∈ residueSurvivors
  replayPartition :
    ∀ {seed : ℕ}, seed ∈ replaySeeds ↔
      seed ∈ RejectedSeeds residueRejections ∨ seed ∈ residueSurvivors
  replayPartitionDisjoint :
    ∀ {seed : ℕ}, seed ∈ RejectedSeeds residueRejections -> seed ∈ residueSurvivors ->
      False
  survivorAcceptancePartition :
    ∀ {seed : ℕ}, seed ∈ residueSurvivors ↔
      seed ∈ nonAcceptedResidueSurvivors ∨ seed ∈ acceptedResidueSurvivors
  survivorAcceptanceDisjoint :
    ∀ {seed : ℕ}, seed ∈ nonAcceptedResidueSurvivors ->
      seed ∈ acceptedResidueSurvivors -> False
  preWitnessSurvivorsNonAccepted :
    ∀ {seed : ℕ}, seed ∈ residueSurvivors -> seed < witnessSeed ->
      seed ∈ nonAcceptedResidueSurvivors
  acceptedSurvivorsAreWitness :
    ∀ {seed : ℕ}, seed ∈ acceptedResidueSurvivors -> seed = witnessSeed

namespace SearchReplayCertificate

def PreWitnessComplete (cert : SearchReplayCertificate) : Prop :=
  ∀ {seed : ℕ}, seed ∈ cert.replaySeeds -> seed < cert.witnessSeed ->
    ReplayRowAccounted cert.config cert.residueModuli cert.witnessSeed
      cert.residueRejections cert.residueSurvivors seed

def WitnessSurvives (cert : SearchReplayCertificate) : Prop :=
  ResidueSurvives cert.config cert.residueModuli cert.witnessSeed

def Sound (cert : SearchReplayCertificate) : Prop :=
  cert.PreWitnessComplete ∧ cert.witnessSeed ∈ cert.replaySeeds ∧ cert.WitnessSurvives

def SurvivorListExact (cert : SearchReplayCertificate) : Prop :=
  ∀ {seed : ℕ}, seed ∈ cert.residueSurvivors ↔
    seed ∈ cert.replaySeeds ∧ ResidueSurvives cert.config cert.residueModuli seed

def ReplayPartitionExact (cert : SearchReplayCertificate) : Prop :=
  (∀ {seed : ℕ}, seed ∈ cert.replaySeeds ↔
    seed ∈ RejectedSeeds cert.residueRejections ∨ seed ∈ cert.residueSurvivors) ∧
  (∀ {seed : ℕ}, seed ∈ RejectedSeeds cert.residueRejections ->
    seed ∈ cert.residueSurvivors -> False)

def ReplayCountExact (cert : SearchReplayCertificate)
    (scannedSeedCount residueRejectedCount residueSurvivorCount : ℕ) : Prop :=
  cert.replaySeeds.length = scannedSeedCount ∧
    (RejectedSeeds cert.residueRejections).length = residueRejectedCount ∧
    cert.residueSurvivors.length = residueSurvivorCount ∧
    scannedSeedCount = residueRejectedCount + residueSurvivorCount

def ReplayAccountingExact (cert : SearchReplayCertificate)
    (scannedSeedCount residueRejectedCount residueSurvivorCount : ℕ) : Prop :=
  cert.ReplayPartitionExact ∧
    cert.ReplayCountExact scannedSeedCount residueRejectedCount residueSurvivorCount

def SurvivorAcceptanceExact (cert : SearchReplayCertificate) : Prop :=
  (∀ {seed : ℕ}, seed ∈ cert.residueSurvivors ↔
    seed ∈ cert.nonAcceptedResidueSurvivors ∨
      seed ∈ cert.acceptedResidueSurvivors) ∧
  (∀ {seed : ℕ}, seed ∈ cert.nonAcceptedResidueSurvivors ->
    seed ∈ cert.acceptedResidueSurvivors -> False)

def AcceptedSurvivorExact (cert : SearchReplayCertificate) : Prop :=
  ∀ {seed : ℕ}, seed ∈ cert.acceptedResidueSurvivors ↔ seed = cert.witnessSeed

def PreWitnessSurvivorsNonAccepted (cert : SearchReplayCertificate) : Prop :=
  ∀ {seed : ℕ}, seed ∈ cert.residueSurvivors -> seed < cert.witnessSeed ->
    seed ∈ cert.nonAcceptedResidueSurvivors ∧
      seed ∉ cert.acceptedResidueSurvivors

def FirstAcceptedSurvivor (cert : SearchReplayCertificate) : Prop :=
  cert.witnessSeed ∈ cert.acceptedResidueSurvivors ∧
    cert.WitnessSurvives ∧
    cert.PreWitnessSurvivorsNonAccepted ∧
    (∀ {seed : ℕ}, seed ∈ cert.acceptedResidueSurvivors -> cert.witnessSeed ≤ seed)

theorem preWitnessComplete (cert : SearchReplayCertificate) :
    cert.PreWitnessComplete := by
  intro seed hSeed hPre
  rcases cert.preWitnessClassified hSeed hPre with hRejected | hSurvivor
  · left
    rcases hRejected with ⟨modulus, hRow⟩
    exact ⟨modulus, hRow, cert.rejectionsReject hRow⟩
  · right
    exact ⟨hSurvivor, Nat.ne_of_lt hPre, by
      intro modulus hModulus
      exact cert.survivorsSurvive hSurvivor hModulus⟩

theorem witnessSurvives (cert : SearchReplayCertificate) :
    cert.WitnessSurvives := by
  intro modulus hModulus
  exact cert.survivorsSurvive cert.witnessSurvivor hModulus

theorem sound (cert : SearchReplayCertificate) :
    cert.Sound :=
  ⟨preWitnessComplete cert, cert.witnessInReplay, witnessSurvives cert⟩

theorem survivorListExact (cert : SearchReplayCertificate) :
    cert.SurvivorListExact := by
  intro seed
  constructor
  · intro hSeed
    exact ⟨cert.survivorsInReplay hSeed, by
      intro modulus hModulus
      exact cert.survivorsSurvive hSeed hModulus⟩
  · intro hSeed
    exact cert.replaySurvivorsComplete hSeed.1 hSeed.2

theorem survivor_mem_iff (cert : SearchReplayCertificate) {seed : ℕ} :
    seed ∈ cert.residueSurvivors ↔
      seed ∈ cert.replaySeeds ∧ ResidueSurvives cert.config cert.residueModuli seed :=
  survivorListExact cert

theorem replayPartitionExact (cert : SearchReplayCertificate) :
    cert.ReplayPartitionExact :=
  ⟨fun {_} => cert.replayPartition, fun {_} => cert.replayPartitionDisjoint⟩

theorem replayCountExact (cert : SearchReplayCertificate)
    {scannedSeedCount residueRejectedCount residueSurvivorCount : ℕ}
    (h :
      cert.replaySeeds.length = scannedSeedCount ∧
        (RejectedSeeds cert.residueRejections).length = residueRejectedCount ∧
        cert.residueSurvivors.length = residueSurvivorCount ∧
        scannedSeedCount = residueRejectedCount + residueSurvivorCount) :
    cert.ReplayCountExact scannedSeedCount residueRejectedCount residueSurvivorCount :=
  h

theorem replayAccountingExact (cert : SearchReplayCertificate)
    {scannedSeedCount residueRejectedCount residueSurvivorCount : ℕ}
    (hCount : cert.ReplayCountExact
      scannedSeedCount residueRejectedCount residueSurvivorCount) :
    cert.ReplayAccountingExact scannedSeedCount residueRejectedCount residueSurvivorCount :=
  ⟨replayPartitionExact cert, hCount⟩

theorem survivorAcceptanceExact (cert : SearchReplayCertificate) :
    cert.SurvivorAcceptanceExact :=
  ⟨fun {_} => cert.survivorAcceptancePartition,
    fun {_} => cert.survivorAcceptanceDisjoint⟩

theorem acceptedSurvivorExact (cert : SearchReplayCertificate) :
    cert.AcceptedSurvivorExact := by
  intro seed
  constructor
  · intro hAccepted
    exact cert.acceptedSurvivorsAreWitness hAccepted
  · intro hSeed
    rw [hSeed]
    exact cert.witnessAccepted

theorem preWitnessSurvivorsNonAcceptedExact (cert : SearchReplayCertificate) :
    cert.PreWitnessSurvivorsNonAccepted := by
  intro seed hSurvivor hPre
  refine ⟨cert.preWitnessSurvivorsNonAccepted hSurvivor hPre, ?_⟩
  intro hAccepted
  have hEq := cert.acceptedSurvivorsAreWitness hAccepted
  exact Nat.ne_of_lt hPre hEq

theorem firstAcceptedSurvivor (cert : SearchReplayCertificate) :
    cert.FirstAcceptedSurvivor := by
  refine ⟨cert.witnessAccepted, witnessSurvives cert,
    preWitnessSurvivorsNonAcceptedExact cert, ?_⟩
  intro seed hAccepted
  have hEq := cert.acceptedSurvivorsAreWitness hAccepted
  rw [hEq]

end SearchReplayCertificate

end PrimeArithmetic.Witness
