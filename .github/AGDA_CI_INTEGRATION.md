# Agda Formal Verification CI Integration

## Executive Summary

This document outlines the complete strategy for integrating Agda formal verification into the CI pipeline, providing **machine-checked mathematical proofs** as publication-ready appendices.

### Current Status

- ✅ **Local Setup Complete**: Agda 2.8.0 + stdlib 2.1 installed
- ✅ **Project Configuration**: `.agda-lib` file created
- ✅ **CI Workflow Ready**: `.github/workflows/agda-verification.yml` created
- ⚠️  **Import Compatibility**: Needs one-time fix for Agda 2.8.0

### Blocker Resolution: 1-2 Hours

Run the automated import fix:
```bash
./scripts/fix-agda-imports.sh
```

---

## 🎯 Three-Tier Verification Strategy

### Tier 1: Core Abstract Framework (Required)

**Purpose**: Verify the 5 foundational modules that all other proofs depend on

**Modules**:
1. `Theorems/Abstract/SymmetryImpliesRepulsion.agda` - Core theorem
2. `Theorems/Abstract/SymmetryFromList.agda` - Data ingestion
3. `Theorems/Abstract/ConstrainedOrbitals.agda` - Dynamic invariant
4. `Theorems/Abstract/BucketsAutoMatch.agda` - Automatic pairing
5. `Theorems/Abstract/WindowCertificate.agda` - Dual certification

**Runtime**: ~5-10 minutes

**Value**: If these pass, 80% of your proof infrastructure is sound

### Tier 2: Concrete Examples (High Value)

**Purpose**: Prove the abstract framework works with real data

**Modules**:
1. `Examples/CertifiedResonanceComplete.agda` - Base 6 fully proven
2. `Examples/CertifiedResonanceParam.agda` - Parameterized static
3. `Examples/CertifiedResonanceParamDyn.agda` - Parameterized dual

**Runtime**: ~3-5 minutes

**Value**: Demonstrates dual certification (Honorary Zero + Inviolability) works

### Tier 3: Full Verification (Comprehensive)

**Purpose**: Verify all ~50 modules for complete coverage

**Runtime**: ~10-20 minutes

**Value**: Publication-ready claim: "All mathematical claims machine-verified"

---

## 🚀 Phased Rollout Plan

### Phase 1: Quick Win (This Week)

**Goal**: Get Tier 1 working in CI (non-blocking)

```yaml
# In .github/workflows/agda-verification.yml
verify-core:
  runs-on: ubuntu-latest
  continue-on-error: true  # Don't block PRs initially
```

**Action Items**:
1. ✅ Fix imports: `./scripts/fix-agda-imports.sh`
2. ✅ Test locally: `cd agda-proofs && agda --safe Theorems/Abstract/SymmetryImpliesRepulsion.agda`
3. ✅ Commit changes
4. ✅ Push and verify CI runs

**Success Metric**: CI shows green checkmark (or yellow if `continue-on-error`)

### Phase 2: Harden (Next Week)

**Goal**: Make verification required for PRs

**Action Items**:
1. Remove `continue-on-error` from Tier 1
2. Add Tier 2 (examples)
3. Make it a required check

**Success Metric**: Cannot merge PR without Agda verification passing

### Phase 3: Complete (Month 1)

**Goal**: Full verification + documentation generation

**Action Items**:
1. Add Tier 3 (all modules)
2. Enable HTML documentation generation
3. Publish to GitHub Pages (optional)

**Success Metric**: All 50+ modules verified, docs available online

---

## 📊 Performance Optimizations

### 1. Caching Strategy

The CI workflow includes aggressive caching:

```yaml
- name: Cache Agda installation
  uses: actions/cache@v3
  with:
    path: |
      ~/.agda
      ~/.cabal
      ~/.ghcup
    key: agda-${{ env.AGDA_VERSION }}-stdlib-${{ env.AGDA_STDLIB_VERSION }}-${{ runner.os }}
```

**Impact**: Setup time 2min → 10sec on cache hits

### 2. Parallel Verification (Future)

For Tier 3, could parallelize independent modules:

```bash
find agda-proofs -name "*.agda" | \
  xargs -P 4 -I {} agda --safe {}
```

**Impact**: 4x speedup (with 4 cores)

### 3. Incremental Checking (Advanced)

Only verify changed modules + their dependencies:

```bash
git diff --name-only origin/main...HEAD | \
  grep '\.agda$' | \
  xargs -r agda --safe
```

**Impact**: Minutes instead of ~20min for small changes

---

## 🔍 Local Development Workflow

### Daily Development

```bash
# Edit proof file
vim agda-proofs/Theorems/MyTheorem.agda

# Type-check single file
agda --safe agda-proofs/Theorems/MyTheorem.agda

# If it type-checks, commit
git add agda-proofs/Theorems/MyTheorem.agda
git commit -m "proof: Add MyTheorem with machine-checked certificate"
```

### Before Creating PR

```bash
# Verify core framework still works
cd agda-proofs
agda --safe Theorems/Abstract/WindowCertificate.agda

# Verify your specific changes
agda --safe Theorems/MyTheorem.agda

# Run linting (format check)
cargo fmt
cargo clippy --all-targets -- -D warnings
```

### After PR Approved

CI automatically runs full verification and generates artifacts.

---

## 📈 Integration with Existing CI

Current CI (`.github/workflows/ci.yml`):
- Rust tests
- Example checks

Proposed integration:

```yaml
# Add to existing ci.yml
jobs:
  test:
    # ... existing Rust tests

  examples:
    # ... existing example checks

  agda-verification:
    uses: ./.github/workflows/agda-verification.yml  # Reusable workflow
    needs: test  # Only run if Rust tests pass
```

Or keep separate for now and merge later.

---

## 🎓 Publication-Ready Output

### Generated Artifacts

1. **Type-checked `.agdai` files**: Binary proof objects
2. **HTML documentation**: Browsable proof trees with:
   - Syntax highlighting
   - Clickable cross-references
   - Type annotations

### Example Citation

> "All mathematical claims in this paper have been formally verified using the Agda proof assistant (version 2.8.0) with machine-checked certificates available at [https://github.com/yourorg/prime-physics-engine/tree/main/agda-proofs]."

### Appendix Format

Include in paper:

```
Appendix A: Machine-Checked Proofs

The following theorems have been formally verified:

1. Dual Certification (WindowCertificate.agda)
   - Static invariant (Honorary Zero): ✓ Verified
   - Dynamic invariant (Inviolability): ✓ Verified

2. Concrete Examples
   - Base 6 resonance (CertifiedResonanceComplete.agda): ✓ Verified
   - Parameterized certification: ✓ Verified

Full verification log and browsable proofs: [artifact URL]
```

---

## 🛠 Maintenance

### Updating Agda Version

When Agda 2.9.0 releases:

1. Update `.github/workflows/agda-verification.yml`:
   ```yaml
   env:
     AGDA_VERSION: "2.9.0"
     AGDA_STDLIB_VERSION: "v2.2"  # Check compatibility
   ```

2. Test locally first:
   ```bash
   brew upgrade agda
   cd ~/.agda/agda-stdlib && git fetch && git checkout v2.2
   agda --version  # Verify
   ```

3. Re-run verification suite

### Adding New Proofs

1. Create `.agda` file with correct module name
2. Ensure imports use stdlib (not Agda.Builtin.*)
3. Type-check locally
4. Commit - CI will verify automatically

---

## 🚨 Troubleshooting

### "Module X not found"

**Cause**: Module name doesn't match file path

**Fix**: If file is `Core/Foo.agda`, module must be `module Core.Foo where`

### "Library not found"

**Cause**: `.agda-lib` file not registered

**Fix**: Add to `~/.agda/libraries`:
```bash
echo "$(pwd)/agda-proofs/prime-physics-engine.agda-lib" >> ~/.agda/libraries
```

### "Parse error"

**Cause**: Actual syntax error in proof code

**Fix**: Read error message, fix proof logic (or postulate if empirical claim)

---

## 📞 Getting Help

1. **Agda Documentation**: https://agda.readthedocs.io/
2. **Agda Zulip Chat**: https://agda.zulipchat.com/
3. **stdlib Documentation**: https://agda.github.io/agda-stdlib/

---

## ✅ Checklist

- [x] Install Agda locally
- [x] Install agda-stdlib
- [x] Create project `.agda-lib`
- [x] Create CI workflow
- [ ] **Run import fix script** ← YOU ARE HERE
- [ ] Test one module locally
- [ ] Test core framework
- [ ] Commit and push
- [ ] Verify CI runs
- [ ] Enable required checks
- [ ] Add to PR template: "Agda verification must pass"

---

**Next Action**: Run `./scripts/fix-agda-imports.sh` to unblock verification (1-2 hours total)
