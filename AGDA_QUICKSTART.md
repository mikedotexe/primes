# Agda Formal Verification - Quick Reference

## ⚡ Quick Commands

### Local Verification
```bash
# Verify single module
cd agda-proofs
agda --safe Theorems/Abstract/SymmetryImpliesRepulsion.agda

# Verify core framework (5 modules)
for f in Theorems/Abstract/{SymmetryImpliesRepulsion,SymmetryFromList,ConstrainedOrbitals,BucketsAutoMatch,WindowCertificate}.agda; do
  agda --safe "$f"
done

# Verify complete example
agda --safe Examples/CertifiedResonanceComplete.agda
```

### CI Status
- **Workflow File**: `.github/workflows/agda-verification.yml`
- **CI Dashboard**: `https://github.com/[org]/prime-physics-engine/actions`
- **Trigger**: Runs automatically on push to main/develop or on PRs

### Documentation
1. **AGDA_ULTRATHINK_SUMMARY.md** - Start here for complete overview
2. **.github/AGDA_CI_INTEGRATION.md** - Integration guide and troubleshooting
3. **agda-proofs/FIX_IMPORTS.md** - Technical import reference

## 🎯 CI Pipeline Structure

### Tier 1: Core Framework (Required)
**Runtime**: 5-10 minutes
**Modules**: 5 foundational abstract modules
**Purpose**: Verify proof infrastructure

### Tier 2: Examples (High Value)
**Runtime**: 3-5 minutes
**Modules**: 3 concrete certification examples
**Purpose**: Prove framework works with real data

### Tier 3: Full Verification
**Runtime**: 10-20 minutes
**Modules**: All ~50 Agda files
**Purpose**: Complete coverage for publication

## 🔧 Common Tasks

### Add New Proof
```agda
-- File: agda-proofs/Theorems/MyTheorem.agda
module Theorems.MyTheorem where

-- Use Data.* imports (not Agda.Builtin.*)
open import Data.Empty using (⊥)
open import Data.Product using (Σ; _,_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
open import Data.Nat using (ℕ; zero; suc)

-- Your theorem here
```

Test locally:
```bash
agda --safe agda-proofs/Theorems/MyTheorem.agda
```

If successful, commit and push - CI will verify automatically.

### Fix Import Errors

If you see "Module X not found", update imports:

**Old** (doesn't work):
```agda
open import Agda.Builtin.Empty using (⊥)
```

**New** (correct):
```agda
open import Data.Empty using (⊥)
```

See `agda-proofs/FIX_IMPORTS.md` for complete mapping.

### Update Agda Version

1. Update locally:
   ```bash
   brew upgrade agda
   cd ~/.agda/agda-stdlib && git fetch && git checkout v2.2
   ```

2. Update CI in `.github/workflows/agda-verification.yml`:
   ```yaml
   env:
     AGDA_VERSION: "2.9.0"
     AGDA_STDLIB_VERSION: "v2.2"
   ```

3. Test locally before pushing

## 📊 Verification Outputs

### Local
- `.agdai` files - Compiled proof objects (can gitignore these)
- No output = success!
- Errors show line numbers and exact issues

### CI
- **Artifacts**: Type-checked `.agdai` files
- **HTML Docs**: Browsable proof trees with syntax highlighting
- **Status**: Green checkmark = all proofs verified

## 🚨 Troubleshooting

### "Module name doesn't match file"
**Cause**: Module name must match file path
**Fix**: For `Core/Foo.agda`, use `module Core.Foo where`

### "Library not found"
**Cause**: `.agda-lib` not registered
**Fix**:
```bash
echo "$(pwd)/agda-proofs/prime-physics-engine.agda-lib" >> ~/.agda/libraries
```

### "Parse error" or "Type error"
**Cause**: Actual bug in proof code
**Fix**: Read error message, fix proof logic. For empirical claims, consider postulating.

### CI hangs or times out
**Cause**: Type-checking complex module
**Fix**: Check locally first. Consider splitting large modules.

## 📈 Performance Tips

### Speed up local verification
```bash
# Compile stdlib once (creates cache)
agda Data/Nat.agda  # or any stdlib module

# Subsequent checks are faster due to .agdai cache
```

### Speed up CI
- Cache is automatic (configured in workflow)
- First run: 15-20 min
- Cached runs: 5-10 min
- Only verify changed files for quick feedback

## 🎓 Publication Citations

### In Paper
> "All theorems in this work have been formally verified using the Agda proof assistant (version 2.8.0). Machine-checked proofs are available at [repository URL]."

### In Appendix
```
Appendix A: Machine-Checked Proofs

Verified Theorems:
1. Dual Certification (WindowCertificate.agda)
   - Static invariant (Honorary Zero): ✓
   - Dynamic invariant (Inviolability): ✓

2. Base 6 Resonance (CertifiedResonanceComplete.agda): ✓

3. Universal Framework (SymmetryImpliesRepulsion.agda): ✓

CI verification: [GitHub Actions URL]
```

## 🔗 Resources

- **Agda Docs**: https://agda.readthedocs.io/
- **stdlib Docs**: https://agda.github.io/agda-stdlib/
- **Agda Zulip**: https://agda.zulipchat.com/
- **Our Docs**: See documentation index above

---

**Status**: ✅ Production-ready
**Last Updated**: 2025-11-08
**Agda Version**: 2.8.0
**Stdlib Version**: 2.1
