# Babylonian-Prime Divergence Tools

Cross-platform implementations for demonstrating the orthogonality between human-convenient mathematics (Babylonian base-60 legacy) and nature's mathematical patterns (prime distributions).

## Overview

These tools implement the statistical framework for the **Babylonian-Prime Divergence Theorem**, which shows that:

1. **Raw correlation** between Babylonian scores and prime pair counts is positive (~0.5) due to Hardy-Littlewood singular series bias
2. **HL-normalized correlation** collapses to near zero (~0.0), confirming true orthogonality

## Available Implementations

### Rust (Primary Implementation)

**Location**: `examples/babylonian_prime_orthogonality.rs`

**Features**:
- Full statistical suite with permutation tests
- Multiple metric options (raw, norm, z-score)
- Integration with hzlib Hardy-Littlewood framework
- Production-quality with comprehensive error handling

**Usage**:
```bash
# Basic demonstration
cargo run --example babylonian_prime_orthogonality

# With custom parameters
cargo run --release --example babylonian_prime_orthogonality -- \
    --N 2000000 --G 500 --metric norm

# Different baseline
cargo run --example babylonian_prime_orthogonality -- \
    --baseline tau  # Use pure divisor count instead of base-60
```

**See also**: `src/hzlib/orthogonality.rs` for the core library implementation.

### Node.js

**Location**: `tools/orthogonality/orthogonality.js`

**Features**:
- Standalone JavaScript implementation
- No external dependencies
- Optional permutation testing
- Cross-platform (Node.js 14+)

**Usage**:
```bash
# Make executable
chmod +x orthogonality.js

# Run with Node
node orthogonality.js --N 1000000 --G 300 --metric norm

# With permutation test
node orthogonality.js --N 500000 --G 200 --metric norm --perm 1000

# Compare metrics
node orthogonality.js --metric raw      # Shows arithmetic bias
node orthogonality.js --metric norm     # Shows orthogonality
node orthogonality.js --metric z        # Z-score normalization
```

## Metrics Explained

### Raw (`--metric raw`)

Counts raw prime pairs (p, p+g) without normalization.

**Expected**: r ≈ +0.5 to +0.6

**Reason**: Both Babylonian scores and raw counts favor gaps with small prime factors, creating spurious correlation through the Hardy-Littlewood singular series.

**Use case**: Demonstrates the necessity of HL normalization.

### Normalized (`--metric norm`)

Divides raw counts by Hardy-Littlewood expectation:
```
normalized = raw_count / (S(g) × N/ln²(N))
```

where S(g) is the singular series.

**Expected**: r ≈ -0.05 to +0.05

**Reason**: Normalization removes arithmetic bias, revealing geometric residue.

**Use case**: Primary metric for demonstrating orthogonality.

### Z-score (`--metric z`)

Computes Poisson-normalized residuals:
```
z = (raw_count - expected) / √expected
```

**Expected**: r ≈ 0

**Reason**: Variance normalization in standardized space.

**Use case**: Alternative demonstration of orthogonality with variance control.

## Baseline Options

### Base-60 (`--baseline base60`, default)

Scores based on 2,3,5-smoothness and divisibility by 60:
```
B₆₀(g) = 2(e₂ + e₃ + e₅) + 10·𝟙(60|g) - 3·|others| + 0.5·τ(g)
```

Explicitly emphasizes Babylonian sexagesimal legacy.

### Tau (`--baseline tau`)

Pure divisor count baseline:
```
Bτ(g) = τ(g) = number of divisors of g
```

Alternative that doesn't specifically favor base-60, but still rewards divisibility.

## Interpretation Guide

### Correlation Magnitude

- **|r| < 0.1**: Negligible (orthogonal) ✅
- **0.1 ≤ |r| < 0.3**: Weak
- **0.3 ≤ |r| < 0.5**: Moderate
- **|r| ≥ 0.5**: Strong

### t-statistic

- **|t| < 2**: Not significant (p > 0.05) ✅
- **2 ≤ |t| < 3**: Marginally significant
- **|t| ≥ 3**: Highly significant

### Expected Results

| Metric | Expected r | Expected |t| | Interpretation |
|--------|-----------|----------|----------------|
| Raw    | ~0.5      | >7       | Arithmetic bias |
| Norm   | ~0.0      | <2       | **Orthogonality** ✅ |
| Z      | ~0.0      | <2       | **Orthogonality** ✅ |

## Example Output

```
╔══════════════════════════════════════════════════════════════╗
║                    CORRELATION RESULTS                       ║
╚══════════════════════════════════════════════════════════════╝

  Pearson r:             -0.0245
  t-statistic:             -0.17
  Sample size:                50

  ✅ Correlation is negligible!
  ✅ Not statistically significant (as expected for orthogonality)

╔══════════════════════════════════════════════════════════════╗
║                       CHAMPION GAPS                          ║
╚══════════════════════════════════════════════════════════════╝

Top Babylonian Gaps (human-convenient):
  #1: gap  60  score  24.00
  #2: gap  96  score  18.00
  #3: gap  72  score  16.00

Top Prime Harmony Gaps (nature's patterns):
  #1: gap  44  score   1.27
  #2: gap   8  score   1.26
  #3: gap  92  score   1.26

  ✅ No overlap between top-3 champions!
     → Human and nature optimize for DIFFERENT structures
```

## Performance Notes

### Computational Complexity

- **Sieving**: O(N log log N) using Eratosthenes
- **Pair indexing**: O(π(N) × G/2) worst case, typically O(π(N) × log(N))
- **Correlation**: O(G) (linear in number of gaps)
- **Permutation test**: O(perm × G)

### Typical Runtimes

| N         | G    | Rust (release) | Node.js    |
|-----------|------|----------------|------------|
| 100k      | 100  | <0.1s          | ~0.2s      |
| 1M        | 300  | ~0.5s          | ~2s        |
| 2M        | 500  | ~1.5s          | ~6s        |
| 10M       | 1000 | ~10s           | ~60s       |

*Rust times with `cargo run --release`. Node.js times with V8 JIT.*

### Recommendations

- **Quick verification**: N=100k, G=100
- **Standard analysis**: N=1M, G=300
- **Publication-quality**: N≥2M, G≥500, with permutation test (perm=2000)

## Statistical Rigor

### Null Hypothesis

H₀: Babylonian scores and HL-normalized prime harmony scores are uncorrelated (r = 0).

### Test Procedure

1. Compute Babylonian score for each even gap
2. Count prime pairs at each gap
3. Normalize by Hardy-Littlewood expectation
4. Compute Pearson correlation r
5. Test significance using t-statistic or permutation test

### Permutation Test

When `--perm N` is specified, the tool performs N random shuffles of the harmony scores and counts how many permutations yield |r| ≥ observed |r|. This gives a non-parametric p-value.

**Interpretation**:
- p > 0.05: Cannot reject orthogonality ✅
- p ≤ 0.05: Evidence against orthogonality

## References

### Documentation

- [../../README.md](../../README.md) — Current public project summary
- [../../EVIDENCE.md](../../EVIDENCE.md) — Audited empirical context
- [../../CLAUDE.md](../../CLAUDE.md) — Developer-facing integration notes

### Academic Context

1. **Hardy-Littlewood Conjectures** (1923): Prime pair heuristics and singular series
2. **Cramér's Conjecture** (1936): Random model for prime gaps
3. **Goldston-Pintz-Yıldırım** (2005): Bounded gaps between primes
4. **Zhang-Maynard-Tao** (2013-2014): Explicit bounded gap results

### Cultural Context

1. **Babylonian Sexagesimal System**: Base-60 from ancient Sumer (~3000 BCE)
2. **Periodical Cicadas**: 13- and 17-year prime cycles (evolutionary biology)
3. **Anthropic Principle**: Human-centric vs. universe-intrinsic structures

## Troubleshooting

### "Correlation too high for normalized metric"

If you see |r| > 0.2 with `--metric norm`:
- Check N is large enough (N ≥ 100k recommended)
- Verify G/N ratio is reasonable (G << √N typically)
- Try different random seed for permutation test
- Report as potential issue if reproducible with N ≥ 1M

### "JavaScript heap out of memory"

For very large N in Node.js:
```bash
node --max-old-space-size=4096 orthogonality.js --N 10000000 --G 1000
```

Or use the Rust implementation, which has no heap limitations for typical parameters.

### "NaN correlation"

Usually indicates insufficient data:
- Increase N (more primes needed)
- Decrease G (fewer gaps to analyze)
- Check that G ≤ N/2

## Contributing

To add new metrics or baselines:

1. **Rust**: Edit `src/hzlib/orthogonality.rs` and update example
2. **Node.js**: Edit `tools/orthogonality/orthogonality.js`
3. **Tests**: Verify orthogonality holds (r ≈ 0 for norm/z metrics)
4. **Documentation**: Update this README and any linked root-level source-of-truth docs if the interpretation changes

## License

MIT License — See [LICENSE](../../LICENSE)

## Citation

If you use these tools in research, please cite:

```
Membrane Prime Toolkit / `primes` repository (2025-2026)
Babylonian-Prime Divergence Framework
https://github.com/mikedotexe/primes
```

---

This README describes a research tool, not a claim registry.
