# Prime Verification Action Plan

## Issues Found

1. **False Prime in Documentation**: `300700300703` - divisible by 11 (not found in current docs, may have been removed)
2. **Base-6 Typo**: `15651` should be `15451` in base 6 (FIXED in CLAUDE.md)
3. **Other Composite Numbers Found**:
   - `15651` (decimal) is composite - divisible by 3, 37, 47
   - Several generated patterns are composite

## Game Plan

### 1. Immediate Actions ✅
- [x] Fixed base-6 example in CLAUDE.md
- [x] Created verification scripts
- [x] Identified problematic patterns

### 2. Systematic Verification (Next Steps)

#### Create Master Prime List
```bash
# Run this to generate all primes we claim
cargo run --example generate_all_claimed_primes > claimed_primes.txt
```

#### Verify Each Prime
```bash
# Run comprehensive verification
cargo run --example verify_all_documentation_primes
```

#### Update Documentation
- Remove any false primes found
- Add verification status next to each prime claim
- Use format: `151 ✓ PRIME` or `15651 ✗ COMPOSITE`

### 3. Preventative Measures

#### Add Pre-commit Hook
```bash
#!/bin/bash
# .git/hooks/pre-commit
cargo run --example verify_documentation_primes || {
    echo "Documentation contains false prime claims!"
    exit 1
}
```

#### CI Pipeline Check
```yaml
- name: Verify Prime Claims
  run: cargo run --example verify_documentation_primes
```

### 4. Documentation Standards

When adding prime examples:
1. ALWAYS verify with `is_prime()` first
2. Include structure notation: `30703 (3-0-7-0-3)`
3. Add verification status: `✓ PRIME` or `✗ COMPOSITE`
4. For base conversions, double-check validity

### 5. Common Pitfalls to Avoid

1. **Invalid Base Digits**: Don't use digit 6 in base 6, etc.
2. **Typos in Large Numbers**: Double-check digit transcription
3. **Pattern Assumptions**: Not all membrane patterns generate primes
4. **Base Confusion**: Be explicit about which base you're using

## Why This Matters

False prime claims undermine the credibility of our entire project. Every claimed prime must be:
- Mathematically verified
- Independently confirmable (e.g., Wolfram Alpha)
- Properly documented with structure

## Quick Verification Commands

```bash
# Check a specific number
echo "300700300703" | cargo run --example check_prime

# Verify all examples in a file
cargo run --example verify_file_primes -- CLAUDE.md

# Generate verification report
cargo run --example prime_verification_report > report.txt
```

## Status

- **Base-6 Example**: ✅ FIXED
- **300700300703**: ⚠️ Not found in current docs (may have been removed)
- **Systematic Audit**: 🔄 IN PROGRESS