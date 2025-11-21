# Membrane Scaling MVP - Archival Summary

**Date**: November 19, 2025
**Action**: Historical archival and documentation of scaling hypothesis investigation
**Status**: ✅ Complete

---

## What Was Done

### 1. Created Historical Archive Structure

```
historical/
└── membrane_scaling_investigation/
    ├── README.md                      (NEW - comprehensive historical documentation)
    ├── run_membrane_mvp.sh            (MOVED from root)
    ├── membrane_mvp_adapter.rs        (MOVED from root)
    ├── membrane_scaling_mvp.py        (MOVED from root)
    └── MVP_FINDINGS_SUMMARY.md        (MOVED from root)
```

### 2. Created Comprehensive Historical README

**File**: `historical/membrane_scaling_investigation/README.md`

**Contents**:
- Original hypothesis (k* ∝ M^(1/2)) and motivation
- Complete methodology and test parameters
- Results showing β≈0.0 (not 0.5) - hypothesis refuted
- Discovery of Minimal Padding Principle
- Timeline of subsequent investigations (Phase 1, Path A, M=2 analysis, M∈{5-10})
- Current verified status
- Scientific significance and lessons learned
- Reproducibility instructions

**Purpose**: Preserves the complete investigative journey showing how discovery evolved from hypothesis testing.

### 3. Updated Main Documentation

**File**: `prime-physics-engine/CLAUDE.md`

**Added Section**: "Research Evolution: From Hypothesis to Discovery"

**Location**: After "Cross-Base Pattern Failures", before "Methodology Overview" (lines 153-240)

**Contents**:
- November 2025 timeline of discovery
- Original scaling hypothesis and its refutation
- Discovery of Minimal Padding Principle
- Rigorous validation timeline
- Current verified status
- References to historical documentation

**Purpose**: Provides readers with historical context showing rigorous scientific methodology.

### 4. Preserved Cross-References

All existing references to data and findings remain valid:
- `SCALING_LAW_FINDINGS.md` - Still comprehensive synthesis
- `EXPERIMENTAL_RESULTS_SUMMARY.md` - Still current findings
- `CRITICAL_ANALYSIS_M2_ANOMALIES.md` - Still statistical analysis
- Data files remain accessible

---

## How to Access Historical Materials

### View Original Investigation

```bash
cd historical/membrane_scaling_investigation
cat README.md  # Read comprehensive historical documentation
```

### Reproduce Original MVP Test

```bash
cd historical/membrane_scaling_investigation
./run_membrane_mvp.sh  # Run complete MVP pipeline
```

**Expected result**: Shows β≈0.0, not β≈0.5 (scaling hypothesis refuted)

### Understand Research Evolution

```bash
# Read main documentation with historical context
cat CLAUDE.md | grep -A 100 "Research Evolution"
```

---

## What This Accomplishes

- Preserves original investigation (hypothesis → test → refutation → discovery)
- Maintains reproducibility (all code, instructions, data preserved)
- Documents evolution from scaling hypothesis to Minimal Padding Principle

---

## Current State of Knowledge

### The Minimal Padding Principle (Verified)

```
M=1:      78.4% k*=0  (mixed regime)
M=2:      99.1% k*=0  (near-perfect, 4 noise anomalies)
M=3:     100.0% k*=0  (perfect universality, p<0.001)
M∈{5-10}: k=0 dominance (mean Δ=2-4.5pp, all significant)
```

**Cross-Validation**: 8+ bases tested
**Statistical Confidence**: p<0.001 for M=3, comprehensive noise refutation for M=2

---

## Key Documents

### Historical Documentation
- `historical/membrane_scaling_investigation/README.md` - Origin story

### Current Findings
- `SCALING_LAW_FINDINGS.md` - 800+ line comprehensive synthesis
- `EXPERIMENTAL_RESULTS_SUMMARY.md` - Recent M=2/M∈{5-10} validation
- `CRITICAL_ANALYSIS_M2_ANOMALIES.md` - 12,500-word statistical analysis

### Main Entry Point
- `CLAUDE.md` - Executive summary (now with research evolution section)

---

## For Publication

Testing k* ∝ M^(1/2) scaling yielded β≈0.0 (R²≈0.0), revealing instead that k*=0 for M≥2. Subsequent validation confirmed M=3 universality (100%, p<0.001), M=2 near-universality (99.1%), and M∈{5-10} k=0 dominance.

---

## Next Steps

### Completed ✅
- [x] Archive MVP artifacts
- [x] Create historical README
- [x] Update CLAUDE.md with research evolution
- [x] Preserve cross-references

### Optional Enhancements
- [ ] Add historical note to SCALING_LAW_FINDINGS.md
- [ ] Create timeline visualization
- [ ] Add to publication manuscript as "Methods - Discovery Process"

### Ready for Publication
The complete research narrative (hypothesis → refutation → discovery → validation) is now
documented and ready for manuscript inclusion.

---

## Summary

**What Changed**:
- MVP files moved to `historical/membrane_scaling_investigation/`
- Historical README created explaining original hypothesis
- CLAUDE.md updated with research evolution section
- All cross-references preserved

**What Stayed the Same**:
- Current findings unchanged (Minimal Padding Principle verified)
- All data files accessible
- Reproducibility maintained
- Publication-ready status

**Bottom Line**: Complete documentation of the investigation from scaling hypothesis to Minimal Padding Principle discovery.

---

**Archival Date**: November 19, 2025
**Preserved By**: Claude (Anthropic) in collaboration with Michael Purvis
**Status**: Complete and ready for publication preparation
