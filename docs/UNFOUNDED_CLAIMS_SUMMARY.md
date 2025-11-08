# ⚠️ Unfounded Claims Summary

## Overview

Our comprehensive verification found that **7 out of 12 tested claims (58.3%)** in the documentation are FALSE. This document summarizes what needs to be corrected.

## Critical False Claims

### 1. Base 6 "41% Champion" Claim
- **File**: THREAD_SUMMARY.md, line 11
- **Claimed**: 41% 
- **Reality**: 25.55% (tested with 10,000 samples)
- **Error**: Overstated by 15.45 percentage points

### 2. Breathing Pattern "30.2%" Claim
- **File**: VISUAL_DISCOVERIES.md, line 83
- **Claimed**: (3,3) k=(0,1) achieves 30.2%
- **Reality**: 0.00% - generates ZERO primes!
- **Error**: Configuration (3,3) is non-coprime to base 6

### 3. Breathing "42% Enhancement" Claim  
- **Files**: THREAD_SUMMARY.md & VISUAL_DISCOVERIES.md
- **Claimed**: Breathing patterns give 42% boost
- **Reality**: When tested with coprime digits, breathing gives only 2-6% improvement
- **Error**: Massively overstated

### 4. Base 12 "28.9%" Claim
- **File**: BASE12_DISCOVERIES.md, line 37
- **Claimed**: (5,7) k=(0,1) achieves 28.9%
- **Reality**: 17.72% (tested with 10,000 samples)
- **Error**: Overstated by 11.18 percentage points

### 5. Base 4 "28%" Claim
- **File**: QUICK_REFERENCE_CARD.md, line 14
- **Claimed**: (3,1) achieves 28%
- **Reality**: 18.26% (tested with 10,000 samples)
- **Error**: Overstated by 9.74 percentage points

## Surprising Finding: Even Bases Win MORE Than Claimed!

- **Claimed**: 44% advantage
- **Reality**: ~81% advantage
- **Note**: The even base effect is STRONGER than documented

## Breathing Pattern Reality Check

From our systematic testing of all coprime configurations:

### Base 6 Breathing Results:
- (1,5): Symmetric 20.1% → Breathing 26.6% (+6.5%)
- (5,1): Symmetric 19.0% → Breathing 24.2% (+5.2%)

### Base 12 Breathing Results:
- Best improvement: (7,1) with +6.4%
- Worst improvement: (7,11) with +1.3%
- Average improvement: ~4%

**Conclusion**: Breathing patterns do provide a small benefit (2-6%), NOT the claimed 42%.

## Files Requiring Updates

1. **THREAD_SUMMARY.md** - 3 false claims
2. **VISUAL_DISCOVERIES.md** - 2 false claims  
3. **BASE12_DISCOVERIES.md** - 1 false claim
4. **QUICK_REFERENCE_CARD.md** - 1 false claim

## Verification Data

- **Total configurations tested**: 14,600+
- **Total primality tests**: 14.6 million
- **CSV data file**: `exhaustive_verification_data.csv` (14,601 rows)
- **Test scripts**: 
  - `comprehensive_claim_verifier.rs`
  - `exhaustive_data_dump.rs`

## Action Items

1. **Immediate**: Add warning to files with false claims
2. **This week**: Update all percentages to verified values
3. **Going forward**: Require verification before publishing performance claims

---

*Remember: It's better to understate and overdeliver than to make unfounded claims.*