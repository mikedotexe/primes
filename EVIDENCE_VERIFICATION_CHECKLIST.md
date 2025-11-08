# EVIDENCE.md Verification Checklist

**Purpose**: Systematic verification of all claims, data tables, and references in EVIDENCE.md
**Status**: Not yet verified - this document guides the verification process
**Created**: 2025-11-07

## Verification Approach

For each section in EVIDENCE.md:
1. Identify specific claims and data points
2. Locate verification script or example that should validate it
3. Run the verification and compare results
4. Document any discrepancies
5. Update EVIDENCE.md with verified data or remove unverifiable claims

---

## Section 1: Base-Dependent Optimal Digits

### Claims to Verify

**Table Data (lines 13-18):**
```
| Base | Optimal Digit | Success Rate | Total Configs | Verification Status |
|------|---------------|--------------|---------------|-------------------|
| 10   | 3             | 73.3%        | 120           | ✅ Verified       |
| 11   | 2             | **UNVERIFIED**| 80           | ⚠️ Needs Review   |
| 12   | 5             | 60.0%        | 100           | ✅ Verified       |
| 8    | 5             | 45.0%        | 60            | ✅ Verified       |
```

**Verification Steps:**
- [ ] Locate script: `cargo run --example base_digit_discovery` (referenced in EVIDENCE.md)
  - **Status**: Need to find or verify if this example exists
  - **Alternative**: Check `comprehensive_base_analysis` example
- [ ] Run for base 10, verify 73.3% success rate for digit 3
- [ ] Run for base 11, verify claim about digit 2
- [ ] Run for base 12, verify 60.0% success rate for digit 5
- [ ] Run for base 8, verify 45.0% success rate for digit 5
- [ ] Verify "Total Configs" counts (120, 80, 100, 60)

**Base 10 Detailed Breakdown (lines 20-26):**
```
Digit 1: 8.3% success (10/120 configs)
Digit 2: 12.5% success (15/120 configs)
Digit 3: 73.3% success (88/120 configs) ← OPTIMAL
Digit 4: 6.7% success (8/120 configs)
Digit 5: 15.0% success (18/120 configs)
```

**Verification Steps:**
- [ ] Verify each digit's success rate
- [ ] Verify exact counts: 10, 15, 88, 8, 18 configs
- [ ] Confirm 10+15+88+8+18 = 119 (should be 120?)
- [ ] Check arithmetic: 88/120 = 73.33%

**Issues to Investigate:**
- Missing example script `base_digit_discovery`
- Base 11 marked as UNVERIFIED - intentional or needs data?
- Breakdown totals 119 not 120 - off by one error?

---

## Section 2: Breathing Membrane Performance Data

### Claims to Verify

**Performance Comparison (lines ~30-50):**
- Symmetric k=(1,1) vs Breathing k=(0,1) comparison
- Specific success rates for different configurations
- "42% improvement" claim

**Verification Steps:**
- [ ] Locate relevant example (likely `proper_membrane_generator`)
- [ ] Run symmetric k=(1,1) configurations, measure success rate
- [ ] Run breathing k=(0,1) configurations, measure success rate
- [ ] Verify 42% improvement calculation
- [ ] Check if data matches current code output

**Data Tables:**
- [ ] Verify all percentage values are calculated correctly
- [ ] Check sample sizes are documented
- [ ] Ensure reproducibility with current codebase

---

## Section 3: Exclusive Configuration Proofs

### Claims to Verify

**Exclusive Configuration Claim:**
- Base 10, config (3,7) k=(1,1) works ONLY with seed 5
- Produces: 307050703 (prime)

**Verification Steps:**
- [ ] Test with `proper_membrane_generator` or `check_prime`
- [ ] Verify 307050703 is prime (external: WolframAlpha)
- [ ] Test seeds 0-9, confirm only seed=5 produces prime
- [ ] Test additional seeds beyond 0-9 to verify exclusivity
- [ ] Document if any other seeds work

**WolframAlpha URL:**
- [ ] Verify: `https://www.wolframalpha.com/input/?i=isprime(307050703)`

---

## Section 4: Configuration Migration Evidence

### Claims to Verify

**Migration Patterns:**
- Optimal configurations change as seed length increases
- "Length specialist" configurations exist

**Verification Steps:**
- [ ] Identify examples that test multi-length seeds
- [ ] Run tests with seed lengths 1, 2, 3, 4+ digits
- [ ] Document which configurations maintain/improve success rate
- [ ] Verify "migration" behavior is real vs. statistical noise

**Data Requirements:**
- [ ] Success rates across seed lengths
- [ ] Clear definition of "migration"
- [ ] Statistical significance of observations

---

## Section 5: Lagrange Point Clustering Analysis

### Claims to Verify

**Core Claim:**
- "100% clustering success across 24 prime pairs"

**Verification Steps:**
- [ ] Locate Lagrange verification examples:
  - `lagrange_full_verification`
  - `lagrange_mechanics`
  - `lagrange_clustering_verifier`
- [ ] Run each example, capture output
- [ ] Verify "100% clustering" claim
- [ ] Count actual prime pairs tested (should be 24)
- [ ] Document clustering definition and criteria

**Specific Examples Referenced:**
- [ ] Verify: 10301 ↔ 3007003007003 Lagrange points
- [ ] Test positions and digit placements
- [ ] Confirm 23-digit prime results

---

## Section 6: Cross-Base Pattern Analysis

### Claims to Verify

**Failure Documentation:**
- Native base-10 configurations fail in other bases
- Successful adaptation strategies exist

**Verification Steps:**
- [ ] Test claimed failures (document which configs, which bases)
- [ ] Test claimed successes (adaptations that work)
- [ ] Verify this isn't just restating coprimality requirement
- [ ] Provide specific examples of failure → adaptation → success

---

## Section 7: Verification Infrastructure

### Claims to Verify

**Available Scripts:**
- Complete list of verification examples
- All scripts actually exist and compile

**Verification Steps:**
- [ ] Cross-reference with `example_test_results.txt` (63 working examples)
- [ ] Verify each referenced script exists:
  - `base_digit_discovery` ← May not exist
  - `prime_count_smoke_test` ← Verified working
  - `proper_membrane_generator` ← Verified working
  - Others listed in section
- [ ] Test that examples produce expected output format
- [ ] Document runtime for each verification script

---

## Section 8: Reproducibility Checklist

### Verification Steps

- [ ] Follow reproducibility instructions step-by-step on clean system
- [ ] Document any missing dependencies or unclear instructions
- [ ] Verify all external URLs work (WolframAlpha links)
- [ ] Test that verification produces same results

---

## High-Priority Verification Tasks

### Immediate (Before Next Commit)

1. **Find or create missing scripts:**
   - `base_digit_discovery` (referenced but may not exist)
   - Document what exists vs. what's referenced

2. **Verify key claims:**
   - 33% density for Base-6 (1,5) k=(0,0)
   - 286,200+ tests claim
   - 100% clustering across 24 prime pairs

3. **Fix arithmetic errors:**
   - Base 10 breakdown: 10+15+88+8+18 = 119 not 120
   - Any percentage calculations

### Medium Priority

4. **Test exclusive configurations:**
   - 307050703 primality
   - Seed exclusivity (only seed=5 works)

5. **Verify data tables:**
   - All success rates match current code output
   - Sample sizes documented correctly

6. **Check external references:**
   - All WolframAlpha URLs work
   - All EVIDENCE.md section anchors exist

### Lower Priority

7. **Statistical rigor:**
   - Confidence intervals on success rates
   - Sample size justification
   - Multiple comparison correction if needed

8. **Reproducibility:**
   - Test on different machines
   - Document exact Rust version requirements
   - Verify deterministic results

---

## Discrepancy Resolution Protocol

When discrepancies are found:

1. **Document the issue:**
   - What EVIDENCE.md claims
   - What verification found
   - Exact commands run
   - Output captured

2. **Investigate root cause:**
   - Code changed since EVIDENCE.md written?
   - Claim was always incorrect?
   - Statistical variance?
   - Missing context?

3. **Resolution options:**
   - **Update EVIDENCE.md** with verified data
   - **Remove claim** if unverifiable
   - **Add caveat** if partially true
   - **Fix code** if code is wrong
   - **Add TODO** if needs more investigation

4. **Update this checklist:**
   - Mark item as verified or failed
   - Document actual values found
   - Link to commit that fixes issue

---

## Verification Log

### Session 1: [Date TBD]

**Tasks completed:**
- [ ] Section 1 verification
- [ ] Section 2 verification
- [ ] Section 3 verification
- [ ] Section 4 verification
- [ ] Section 5 verification
- [ ] Section 6 verification
- [ ] Section 7 verification
- [ ] Section 8 verification

**Discrepancies found:**
- TBD

**Files updated:**
- TBD

**Confidence level:**
- Not yet assessed

---

## Tools for Verification

### Examples to Run
```bash
# Core verification
cargo run --example prime_count_smoke_test
cargo run --example proper_membrane_generator
cargo run --example statistical_prime_generator
cargo run --example comprehensive_base_analysis

# Lagrange verification
cargo run --example lagrange_full_verification
cargo run --example lagrange_mechanics
cargo run --example lagrange_clustering_verifier

# Check specific prime
cargo run --example check_prime
# Then enter: 307050703
```

### External Verification
```
WolframAlpha base: https://www.wolframalpha.com/input/?i=isprime(NUMBER)
```

### Data Collection Scripts
```bash
# Run verification and capture output
cargo run --example proper_membrane_generator 2>&1 | tee verification_output.txt

# Compare against EVIDENCE.md claims
diff -u expected_output.txt verification_output.txt
```

---

## Success Criteria

EVIDENCE.md verification is complete when:

- [ ] All 8 sections verified against actual code output
- [ ] All referenced examples exist and work
- [ ] All data tables match current measurements
- [ ] All arithmetic is correct
- [ ] All external URLs work
- [ ] All claims have supporting evidence OR are removed
- [ ] Discrepancy log documents any changes made
- [ ] Confidence level ≥ 95% in all numerical claims

---

## Notes for Future Verifiers

1. **Be skeptical**: Question every number, percentage, and claim
2. **Run the code**: Don't assume, verify with actual execution
3. **Document everything**: Capture outputs, note discrepancies
4. **Update as you go**: Fix EVIDENCE.md immediately when errors found
5. **Statistical rigor**: Small sample sizes need confidence intervals
6. **Version control**: Git commit after each section verified

---

**Next Steps:**
1. Review this checklist for completeness
2. Schedule verification session
3. Run systematic checks section by section
4. Update EVIDENCE.md with verified data
5. Commit verified version with "VERIFIED" tag in commit message
