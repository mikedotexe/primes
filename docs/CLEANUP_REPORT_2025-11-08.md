# Repository Cleanup Report - Errant/Incomplete Work

**Date**: 2025-11-08
**Task**: Identify sloppy, half-done, or errant work in the repository

## Summary of Issues Found

### 1. **examples-not-running/** Directory (75 broken examples)
- **Status**: INCOMPLETE/BROKEN
- **Description**: 75 example files that don't compile or run
- **Size**: ~1MB of code (20,161 lines)
- **Impact**: Clutters repository, confuses users
- **Recommendation**:
  - Either fix these examples and move to main `examples/`
  - Or delete them entirely if they're obsolete
  - The CHANGELOG mentions "74 examples in experimental/ need syntax restoration"

### 2. Duplicate/Redundant Documentation (MOVED to docs/)
The following duplicate documentation was found and consolidated:
- RELEASE_NOTES.md vs RELEASE_NOTES_v1.0.0.md vs RELEASE_SUMMARY.md vs RELEASE_TLDR.md
- COMMIT_FINAL_SUMMARY.md vs COMMIT_READY_SUMMARY.md
- SBOM.md vs SBOM.txt (both formats kept)
- CHECKSUMS.md vs checksums.txt

**Action Taken**: All moved to `docs/` directory for organization

### 3. Timestamped Test Output Files (MOVED to archive/)
Old test run outputs with hardcoded dates from 2025-07-17 and 2025-07-18:
- atomic_membranes_20250717_225125.txt
- base_parity_results_20250717_225008.txt
- breathing_patterns_20250717_225831.txt
- grand_unified_primes_20250717_230124.txt
- mega_base_analysis_20250717_233905.txt
- membrane_findings_20250718_043553.txt
- membrane_prime_reference_20250717_230124.txt
- prime_garden_20250717_225842.txt

**Action Taken**: Moved to `archive/old-test-runs/`

### 4. Empty/Temporary Files (DELETED)
- tui_error.txt (0 bytes)
- tui_output.txt (0 bytes)

**Action Taken**: Deleted

### 5. Build Artifacts in Repository
- prime-physics-engine-complete-20250719.tar.gz (6.7KB)

**Action Taken**: Moved to `archive/`

### 6. Duplicate HTML Demos
- index.html (Membrane Prime Explorer)
- index-tailwind.html (Alternative styling)
- Both duplicated functionality in wasm-demo/www/

**Action Taken**: Moved to `archive/`

## Files Reorganized

### Created New Directory Structure:
```
├── data/              # CSV and data files (17 files)
├── outputs/           # Generated results and outputs
│   └── images/       # PNG visualizations (6 files)
├── archive/          # Old artifacts and duplicates
│   └── old-test-runs/ # Timestamped test outputs
├── docs/             # All documentation (45+ MD files)
└── scripts/          # All scripts (shell + Python)
```

### Root Directory - Before vs After:
**Before**: 100+ files (docs, data, scripts, outputs mixed)
**After**: 10 essential files only
- README.md
- CLAUDE.md
- EVIDENCE.md
- CHANGELOG.md
- LICENSE
- VERSION
- Cargo.toml
- build.rs
- deny.toml
- .gitignore

## Recommendations for Future Cleanup

1. **examples-not-running/**: Decide whether to fix or delete these 75 broken examples
2. **Archive pruning**: Review archive/ directory and delete anything truly obsolete
3. **.gitignore updates**: Add patterns to prevent future data/output commits
4. **Documentation consolidation**: Many docs in docs/ seem redundant and could be merged
5. **SBOM**: Choose one format (MD or TXT) and delete the other

## What Should NOT Be Committed

Based on this cleanup, the following should be gitignored:
- `*.csv` (data files - unless they're test fixtures)
- `*.png` (generated images)
- `*.json` (result files - unless they're config/fixtures)
- `*_output.txt` (test outputs)
- `*_results.txt` (result files)
- Timestamped files matching `*_202[0-9][0-9][0-9][0-9][0-9]_*.txt`
- `*.tar.gz` (build artifacts)
- Demo HTML files (unless they're the canonical versions)
