# Theorems Directory

## ElbowEvents.agda

### Quick Start

1. **View existing examples**:
   ```bash
   agda --safe ElbowEvents.agda
   ```

2. **Add a new elbow**:
   ```agda
   -- 1. Define config
   my-elbow-config : ElbowConfig
   my-elbow-config = mkElbowConfig
     base outer inner M-from M-to k-from k-to

   -- 2. Define densities (as rationals)
   ρ-from : ℚ
   ρ-from = num / den

   ρ-to : ℚ
   ρ-to = num' / den'

   -- 3. Create evidence
   my-elbow : ElbowEvent
   my-elbow = mkElbowEvidence
     my-elbow-config
     ρ-from
     ρ-to
     refl      -- M-step
     proof     -- k-increases
     refl      -- density improves
   ```

3. **Verify it type-checks**: `agda --safe ElbowEvents.agda`

### Integration with Ridge Extractor

From `ridge_elbows.csv`:
```csv
base,outer,inner,M_from,M_to,k_from,k_to,density_from,density_to
15,13,1,1,2,0,1,0.071429,0.114286
```

Becomes:
```agda
base15-13-1-elbow-config = mkElbowConfig 15 13 1 1 2 0 1
ρ-base15-M1-k0 = 1 / 14   -- ≈ 0.071429
ρ-base15-M2-k1 = 4 / 35   -- ≈ 0.114286
```

### Types

**ElbowConfig**: Combinatorial description
- `base : ℕ` - Positional base
- `outer : ℕ` - Outer membrane digit
- `inner : ℕ` - Inner membrane digit
- `M-from : ℕ` - Old seed length
- `M-to : ℕ` - New seed length
- `k-from : ℕ` - Old padding
- `k-to : ℕ` - New padding

**ElbowEvidence** (positive elbow):
- Density improves: `density-from ≤ℚ density-to`
- K increases: `k-from < k-to`
- M steps: `M-to ≡ suc M-from`

**ContrarianElbowEvidence**:
- Density decreases: `density-to ≤ℚ density-from`
- K still increases: `k-from < k-to`
- M steps: `M-to ≡ suc M-from`

### Auto-Generation Workflow

**Automatic**: Generate Agda witnesses directly from CSV data

```bash
# From the collab/ directory:
python3 generate_elbow_agda_from_csv.py ridge_elbows.csv > ../agda-proofs/Theorems/ElbowsFromCSV.agda

# Verify generated code:
cd ../agda-proofs
agda Theorems/ElbowsFromCSV.agda
```

**What it does**:
- Reads `ridge_elbows.csv` (output from `ridge_extractor.py`)
- Converts float densities → exact rationals (ℚ) using `Fraction.limit_denominator(1000000)`
- Classifies events: `ElbowEvent` (density↑) vs `ContrarianElbowEvent` (density↓)
- Generates complete Agda code with:
  - Config definitions (`mkElbowConfig`)
  - Density constants (`num / den`)
  - Evidence terms (`mkElbowEvidence` or `mkContrarianElbowEvidence`)
  - Automatic proofs for common cases (e.g., `k-0<1` for 0→1 transitions)
  - Proof holes for manual completion (other k-transitions)

**Example output**:
```agda
-- Event 1: base=15, outer=13, inner=1, M=1→2, k=0→1
cfg_b15_o13_i1_M1_to_M2_k0_to_k1 : ElbowConfig
cfg_b15_o13_i1_M1_to_M2_k0_to_k1 = mkElbowConfig 15 13 1 1 2 0 1

ρ_b15_o13_i1_M1_to_M2_k0_to_k1_from : ℚ
ρ_b15_o13_i1_M1_to_M2_k0_to_k1_from = 1 / 7

ρ_b15_o13_i1_M1_to_M2_k0_to_k1_to : ℚ
ρ_b15_o13_i1_M1_to_M2_k0_to_k1_to = 24 / 133

elbow_b15_o13_i1_M1_to_M2_k0_to_k1 : ElbowEvent
elbow_b15_o13_i1_M1_to_M2_k0_to_k1 = mkElbowEvidence
  cfg_b15_o13_i1_M1_to_M2_k0_to_k1
  ρ_b15_o13_i1_M1_to_M2_k0_to_k1_from
  ρ_b15_o13_i1_M1_to_M2_k0_to_k1_to
  refl    -- M-step: M-to ≡ suc M-from
  k-0<1   -- k-increases: 0 < 1
  refl    -- density-weakly-improves: _≤ℚ_ density-from density-to ≡ true
```

### Manual Workflow

```
ridge_elbows.csv
    ↓
Extract (base, outer, inner, M, k, density)
    ↓
Encode as ℚ rationals
    ↓
Create ElbowEvent with proofs
    ↓
agda --safe ElbowEvents.agda
    ↓
✓ Machine-checked verification
```

### Connection to Honorary Zero

Any elbow with an even base admits an `HZBase`:

```agda
elbowHasHZBase : (e : ElbowConfig) → Even (base e) → HZBase
```

This connects empirical elbows to the formal honorary-zero framework.
