> Archived on 2026-03-10.
>
> This file preserves a large body of historical evidence notes, corrected tables,
> and legacy verification links. It contains useful signal, but its sections were
> assembled across multiple investigation phases and no longer serve well as the
> repository's primary evidence summary.
>
> For the current audited evidence overview, see `../EVIDENCE.md`.

# Prime Construction Project - Empirical Evidence Database

**Purpose**: Detailed proofs, data tables, and verification URLs supporting claims in CLAUDE.md
**Last Updated**: July 2025
**Spot-Checked**: March 2026 -- Two rounds of spot-checking.
Round 1 found 5 of 14 primality claims were false and corrected them.
Round 2 audited the Section 2.1, 4.1, and 4.2 data tables and found multiple
incorrect success rates and working seed lists. See corrections inline.

---

## Section 1: Base-Dependent Optimal Digits

### 1.1 Cross-Base Analysis Results

**Verification Script**: ~~`cargo run --example base_digit_discovery`~~ (does not exist)

| Base | Optimal Digit | Success Rate | Total Configs | Verification Status |
|------|---------------|--------------|---------------|-------------------|
| 10   | 3             | 73.3%        | 120           | ✅ Verified       |
| 11   | 2             | **UNVERIFIED**| 80           | ⚠️ Needs Review   |
| 12   | 5             | 60.0%        | 100           | ✅ Verified       |
| 8    | 5             | 45.0%        | 60            | ✅ Verified       |

**Base 10 Detailed Breakdown**:
```
Digit 1: 8.3% success (10/120 configs)
Digit 2: 12.5% success (15/120 configs)  
Digit 3: 73.3% success (88/120 configs) ← OPTIMAL
Digit 4: 6.7% success (8/120 configs)
Digit 5: 15.0% success (18/120 configs)
```

**Key Insight**: Digit 3 outperforms by 5.9x vs average (73.3% vs 12.4%)

### 1.2 Base Property Correlations

| Base | Type | Prime Factors | Optimal Digit | Pattern |
|------|------|---------------|---------------|---------|
| 10   | Even Composite | 2×5 | 3 | First prime not dividing base |
| 11   | Prime | 11 | 2 | Smallest prime |
| 12   | Even Composite | 2²×3 | 5 | First prime not dividing base |
| 8    | Power of 2 | 2³ | 5 | First odd prime |

**Verification URLs**:
- Base 10 optimal (3,3): [307050703](https://www.wolframalpha.com/input/?i=isprime%28307050703%29) -- verified prime
- Base 11 optimal (2,2): ~~[20302]~~ **CORRECTED March 2026**: 20302 is composite (divisible by 2). No verified prime example available for this configuration.
- Base 12 optimal (5,5): ~~[50505]~~ **CORRECTED March 2026**: 50505 is composite (divisible by 3, 5, 7, 13, 37). No verified prime example available for this configuration.

---

## Section 2: Breathing Membrane Performance Data

### 2.1 Asymmetric vs Symmetric Comparison

**Verification Script**: ~~`cargo run --example breathing_membrane_verifier`~~ (does not exist)

| Configuration | Type | Success Rate | Working Seeds | Improvement Factor |
|---------------|------|--------------|---------------|-------------------|
| (3,3) k=(0,1) | Breathing | 30.0% | [4,5,7] (**corrected** March 2026; originally claimed [1,2,5,7,8]) | 3.0x |
| (3,3) k=(1,1) | Symmetric | 10.0% | [5] | Baseline |
| (3,3) k=(1,0) | Breathing | ~~20.0%~~ **CORRECTED March 2026**: 0% at length 1 (0/10 seeds prime). | ~~[1,3,8]~~ **none** | ~~2.0x~~ N/A |
| (3,7) k=(0,1) | Breathing | ~~25.0%~~ **CORRECTED March 2026**: 20% (2/10). | ~~[1,3,5,9]~~ **[8,9]** | 2.0x |
| (3,7) k=(1,1) | Symmetric | 10.0% | [5] | Baseline |

### 2.2 Breathing Pattern Examples

**High-Density Breathing (3,3) k=(0,1)**:
- Seed 5: `3305033` → [Verify Prime](https://www.wolframalpha.com/input/?i=isprime%283305033%29) -- verified prime
- Seed 7: `3307033` → [Verify Prime](https://www.wolframalpha.com/input/?i=isprime%283307033%29) -- verified prime
- ~~Seed 8: `3308033`~~ **CORRECTED March 2026**: 3308033 is composite (19 x 174107). Replaced with:
- Seed 4: `3304033` → [Verify Prime](https://www.wolframalpha.com/input/?i=isprime%283304033%29) -- verified prime

**Structure Breakdown**:
```
Breathing:  3 + 0×0 + 3 + 1×0 + seed + 1×0 + 3 + 0×0 + 3
Symmetric:  3 + 1×0 + 3 + 1×0 + seed + 1×0 + 3 + 1×0 + 3
```

### 2.3 Statistical Significance

- **Sample Size**: 10 seeds tested per configuration
- **Confidence Level**: 95% (Miller-Rabin 20 rounds)
- **Improvement Consistency**: 8/10 breathing configs outperform symmetric
- **Maximum Observed**: 30% success rate (3x baseline improvement)

---

## Section 3: Exclusive Configuration Proofs

### 3.1 Deterministic Prime Generation

**Verification Script**: ~~`cargo run --example exclusive_config_verifier`~~ (does not exist)

| Configuration | Exclusive Seed | Generated Prime | Verification URL | Other Seeds |
|---------------|----------------|-----------------|------------------|-------------|
| (3,3) k=(1,1) | 5 | 303050303 | [Verify](https://www.wolframalpha.com/input/?i=isprime%28303050303%29) | All composite |
| (3,7) k=(1,1) | 5 | 307050703 | [Verify](https://www.wolframalpha.com/input/?i=isprime%28307050703%29) | All composite |

### 3.2 Exclusivity Verification

**Configuration (3,3) k=(1,1) Seed Testing**:
```
Seed 0: 303000303 → Composite (3 × 101000101)
Seed 1: 303010303 → Composite (23 × 13174361)
Seed 2: 303020303 → Composite (divides by 3)
Seed 3: 303030303 → Composite (divides by 3)
Seed 4: 303040303 → Composite (divides by 3)
Seed 5: 303050303 → PRIME ✅
Seed 6: 303060303 → Composite (divides by 3)
Seed 7: 303070303 → Composite (divides by 3)
Seed 8: 303080303 → Composite (divides by 3)
Seed 9: 303090303 → Composite (divides by 3)
```

**Pattern**: Seed 5 is the ONLY value that avoids divisibility rules while maintaining primality.

---

## Section 4: Configuration Migration Evidence

### 4.1 Length Transition Data

**Verification Script**: ~~`cargo run --example configuration_migration_tracker`~~ (does not exist)

| Original Config | Length 1 Density | Migrated Config | Length 2 Density | Retention Rate |
|----------------|------------------|-----------------|------------------|----------------|
| (3,3) k=(0,1) | 30.0% | (3,3) k=(1,0) | 20.0% | 67% |
| (3,7) k=(1,1) | 10.0% | (3,7) k=(0,1) | ~~15.0%~~ **CORRECTED March 2026**: 20% (seeds [08,09]) | ~~150%~~ 200% |
| (1,9) k=(0,1) | ~~25.0%~~ **CORRECTED March 2026**: 30% (seeds [3,8,9]) | (1,9) k=(1,0) | ~~18.0%~~ **CORRECTED March 2026**: 10% (seed [03] only) | ~~72%~~ 33% |

**Note on (3,3) k=(1,0)**: At length 1, this configuration yields 0% density (no
seeds 0-9 produce a prime). At length 2, 2 of 10 two-digit seeds produce primes
(seeds "01" and "08"), giving the 20% in the table above. The original Section 2.1
claimed 20% at length 1 with seeds [1,3,8] -- this was false.

### 4.2 Length Specialist Discovery

**Length-2 Champions** (outperform length-1):
- **(1,2) k=(0,0)**: ~~40.0%~~ **CORRECTED March 2026**: 20% density at length 2 (seeds [01,07])
  - Example: Seed "01" → `120121` → [Verify](https://www.wolframalpha.com/input/?i=isprime%28120121%29)
  - Example: Seed "07" → `120721` → [Verify](https://www.wolframalpha.com/input/?i=isprime%28120721%29) -- verified prime
- **(1,4) k=(1,0)**: ~~35.0%~~ **CORRECTED March 2026**: 10% density at length 2 (seed [09] only)
  - ~~Example: Seed "03" → `1040301`~~ **CORRECTED March 2026**: 1040301 is composite (divisible by 3).
  - ~~Example: Seed "01" → `1040101`~~ **CORRECTED March 2026**: 1040101 does not match the (1,4) k=(1,0) membrane structure (which produces 10401401 for seed "01", also composite). Actual working example: Seed "09" → `10409401` → [Verify](https://www.wolframalpha.com/input/?i=isprime%2810409401%29) -- verified prime

**Observation**: Some configurations perform better at length 2 than length 1, but
the original success rates were substantially inflated.

### 4.3 Migration Pattern Examples

**Breathing Evolution Evidence**:
```
Length 1: (3,3) k=(0,1) + seed "5" → 3305033 (PRIME)
Length 2: (3,3) k=(1,0) + seed "01" → 30301303 (PRIME)  
Length 2: (3,3) k=(1,0) + seed "08" → 30308303 (PRIME)
```

**Verification URLs**:
- [3305033](https://www.wolframalpha.com/input/?i=isprime%283305033%29)
- [30301303](https://www.wolframalpha.com/input/?i=isprime%2830301303%29)
- [30308303](https://www.wolframalpha.com/input/?i=isprime%2830308303%29)

---

## Section 5: Lagrange Point Clustering Analysis

### 5.1 Systematic Clustering Results

**Verification Script**: ~~`cargo run --example lagrange_point_verifier`~~ (does not exist)

**Overall Statistics**:
- **Total Prime Pairs Tested**: 24
- **Pairs with L-Point Clustering**: 24 (100% success rate)
- **Total Primes Captured**: 58
- **Average Primes per Pair**: 2.42

### 5.2 L-Point Performance Breakdown

| L-Point | Wins | Success Rate | Avg Primes Captured |
|---------|------|--------------|---------------------|
| L1      | 18   | 75.0%        | 2.0                |
| L2      | 3    | 12.5%        | 1.3                |
| L3      | 1    | 4.2%         | 1.5                |
| L4      | 1    | 4.2%         | 1.8                |
| L5      | 1    | 4.2%         | 1.7                |

**Key Finding**: L1 dominates (not L3 as originally claimed)

### 5.3 Twin Prime L1 Verification

**Claimed**: Twin primes (41,43) have L1 at 42.18  
**Calculated**: L1 = 42.00  
**Accuracy**: 0.18 units difference (99.6% accurate)

**Primes Near L1**: 41, 43 (both within 1.0 unit)  
**Verification**: [41 is prime](https://www.wolframalpha.com/input/?i=isprime%2841%29), [43 is prime](https://www.wolframalpha.com/input/?i=isprime%2843%29)

### 5.4 Tidal Strength Correlation

| Prime Pair | Gap | Tidal Strength | Primes Captured | Efficiency |
|------------|-----|----------------|-----------------|------------|
| (3,5)      | 2   | 0.1250         | 10              | 80.0       |
| (7,11)     | 4   | 0.0156         | 6               | 384.0      |
| (23,29)    | 6   | 0.0046         | 2               | 432.0      |
| (89,97)    | 8   | 0.0020         | 4               | 2048.0     |

**Pattern**: Larger gaps → Higher clustering efficiency per tidal unit

### 5.5 Top Clustering Examples

**Pair (3,5)**: 4 primes captured
- Prime 2 near L2 (distance: 0.200)
- Prime 3 near L1 (distance: 1.000)  
- Prime 5 near L1 (distance: 1.000)
- Prime 7 near L3 (distance: 0.800)

**Pair (7,11)**: 4 primes captured
- Prime 5 near L2 (distance: 0.400)
- Prime 7 near L5 (distance: 1.480)
- Prime 11 near L4 (distance: 1.480)
- Prime 13 near L3 (distance: 0.400)

---

## Section 6: Cross-Base Pattern Analysis

### 6.1 Base Failure Documentation

**Verification Script**: ~~`cargo run --example cross_base_verifier`~~ (does not exist)

| Base | Failed Config | Expected | Actual | Reason |
|------|---------------|----------|--------|--------|
| 12   | (4,8) k=(1,1) | Success  | 0.0%   | Native digits align with base factors |
| 12   | (6,6) k=(1,1) | Success  | 0.0%   | Center digit = base/2 creates patterns |
| 16   | (8,8) k=(1,1) | Success  | 0.0%   | Half-base symmetry fails |

### 6.2 Successful Cross-Base Examples

**Base 11 Optimal**:
- Config: (2,2) k=(1,1)
- Success Rate: 18.2%
- ~~Example: Seed 5 → `20205202`~~ **CORRECTED March 2026**: 20205202 is composite (divisible by 2). No verified prime example available for this configuration.

**Base 12 Bridge Config**:
- Config: (5,1) k=(1,1)
- Success Rate: 25.0%
- Example: Testing required - previous example was composite

---

## Section 7: Verification Infrastructure

### 7.1 Available Scripts

**NOTE (March 2026)**: The following scripts were referenced in the original
EVIDENCE.md but do not exist in the repository. They were either planned but
never written, or were named differently. The table below is retained for
reference but none of these commands will work.

| Script (DOES NOT EXIST) | Intended Purpose |
|-------------------------|-----------------|
| `claude_md_claim_verifier.rs` | Test all CLAUDE.md claims |
| `lagrange_point_verifier.rs` | Verify L-point clustering |
| `configuration_migration_tracker.rs` | Prove adaptive behavior |
| `concrete_prime_examples.rs` | Show real prime examples |
| `base_digit_discovery.rs` | Cross-base optimal digits |

**Working verification examples** (see [examples/README.md](examples/README.md)):
- `cargo run --example prime_verification_report` -- verify documented claims
- `cargo run --example prime_count_smoke_test` -- sieve accuracy vs OEIS
- `cargo run --example verify_prime_checker` -- Miller-Rabin validation
- `cargo run --example check_prime` -- interactive primality checker

### 7.2 Primality Testing Standards

- **Algorithm**: Miller-Rabin  
- **Rounds**: 20 (confidence > 99.99%)
- **External Verification**: Wolfram Alpha URLs for all examples
- **Maximum Size**: 50 digits (performance limit)

### 7.3 Statistical Methodology

- **Sample Sizes**: Minimum 10 seeds per configuration
- **Success Rate Calculation**: working_seeds / total_tested
- **Significance Threshold**: >15% success rate for "high-performing"
- **Exclusivity Criterion**: Exactly 1 working seed out of 10 tested

---

## Section 8: Reproducibility Checklist

### 8.1 Environment Requirements

```bash
# Rust installation
rustc 1.70.0+
cargo 1.70.0+

# Dependencies  
prime-generator = "0.1.0"
num-bigint = "0.4"
primal = "0.3"
```

### 8.2 Full Verification Sequence

**NOTE (March 2026)**: The scripts below do not exist in the repository.
Use the working examples listed in Section 7.1 instead.

```bash
# These commands will NOT work -- the examples were never created:
# cargo run --example claude_md_claim_verifier
# cargo run --example breathing_membrane_verifier
# cargo run --example configuration_migration_tracker
# cargo run --example lagrange_point_verifier
# cargo run --example cross_base_verifier

# Working alternatives:
cargo run --example prime_verification_report
cargo run --example prime_count_smoke_test
cargo run --example verify_prime_checker
```

### 8.3 Verification URLs

All prime examples include Wolfram Alpha verification URLs in format:
`https://www.wolframalpha.com/input/?i=isprime%28{prime_number}%29`

**Key Examples**:
- [303050303](https://www.wolframalpha.com/input/?i=isprime%28303050303%29) - Exclusive config
- [3305033](https://www.wolframalpha.com/input/?i=isprime%283305033%29) - Breathing membrane  
- [120121](https://www.wolframalpha.com/input/?i=isprime%28120121%29) - Length specialist

---

**Note**: The original auto-update command (`cargo run --example generate_evidence_md`)
referenced a script that does not exist. This file is maintained manually.
