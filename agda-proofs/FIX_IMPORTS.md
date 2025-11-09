# Agda Import Compatibility Fix

## Problem

The Agda proof files use outdated import paths that don't exist in modern Agda (2.8.0+):

```agda
-- ❌ DOESN'T EXIST in Agda 2.8.0
open import Agda.Builtin.Empty using (⊥)
open import Agda.Builtin.Sigma using (Σ; _,_)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat; zero; suc)
```

## Solution

Replace with standard library imports:

```agda
-- ✅ CORRECT for Agda 2.8.0 + stdlib 2.1
open import Data.Empty using (⊥)
open import Data.Product using (Σ; _,_; proj₁; proj₂)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Data.Nat using (ℕ; zero; suc; _+_; _*_)
```

## Automated Fix

Run this script to fix all imports:

```bash
./scripts/fix-agda-imports.sh
```

## Manual Fix Reference

### Common Replacements

| Old Import | New Import |
|------------|------------|
| `Agda.Builtin.Empty` | `Data.Empty` |
| `Agda.Builtin.Sigma` | `Data.Product` |
| `Agda.Builtin.Equality` | `Relation.Binary.PropositionalEquality` |
| `Agda.Builtin.Nat` | `Data.Nat` |
| `Agda.Builtin.Bool` | `Data.Bool` |
| `Agda.Builtin.List` | `Data.List` |

### Type Name Changes

Some types also need renaming:

| Old | New |
|-----|-----|
| `Nat` | `ℕ` (in most stdlib modules) |

## Verification Workflow

After fixing imports:

```bash
# Test single module
cd agda-proofs
agda --safe Theorems/Abstract/SymmetryImpliesRepulsion.agda

# Test all core modules
./scripts/verify-agda-core.sh

# Generate full verification report
./scripts/verify-agda-all.sh
```

## CI Integration

Once imports are fixed, the CI workflow in `.github/workflows/agda-verification.yml` will:

1. ✓ Install Agda 2.8.0 + stdlib 2.1
2. ✓ Verify core abstract framework (Tier 1)
3. ✓ Verify concrete examples (Tier 2)
4. ✓ Verify all ~50 modules (Tier 3)
5. ✓ Generate HTML documentation
6. ✓ Upload artifacts for inspection

## Timeline Estimate

- **Automated fix script**: 10 minutes to write
- **Run script + test**: 5 minutes
- **Manual fixups if needed**: 30-60 minutes
- **Full verification test**: 10-20 minutes

**Total**: 1-2 hours to complete

## Why This Happened

The Agda proofs were written for an older Agda version where some types were built-in primitives. In Agda 2.6+, these were moved to the standard library to provide better organization and more features.

This is a **one-time fix** - once updated, the proofs will work with all modern Agda versions.
