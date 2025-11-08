# N× Transform Duality Tool (Python Reference Implementation)

## Overview

This is the **reference implementation** of the N× transform analysis in Python. It provides exact arithmetic (via `Fraction`) to test mathematical properties without floating-point errors.

The production implementation is in the Rust unified CLI (`../prime_unified_cli.rs`), which is faster and integrates with the main data pipeline.

## Purpose

Test the **MZR hypothesis**: Does the MZR selection rule (r ≈ 0.4×HZ) bias which vertex k becomes the integer one in the N× transform?

## Key Mathematical Fixes

This implementation corrects several issues in earlier approaches:

1. **Exact residue calculation**: Uses integer arithmetic `(r + k*B) % N` instead of float division
2. **Correct termination test**: Factors denominator and checks `primes(den) ⊆ primes(base)`
3. **Universal trio property**: For N=3 and 3∤B, residues are **always** {0,⅓,⅔} (structural, not MZR-dependent)
4. **Integer vertex formula**: For gcd(B,N)=1: `k_int ≡ -r·B⁻¹ (mod N)`

## Usage

### Single Remainder Analysis

```bash
python3 n_transform_duality.py --base=998 --N=3 --r=332
```

Output:
```
B=998, N=3, r=332, gcd(B,N)=1
residues: [2, 1, 0]
fracparts: [Fraction(2, 3), Fraction(1, 3), Fraction(0, 1)]
integer vertex k: 2
  k= 0  (r+kB)/N=332/3  residue=2  frac=2/3  int=False  repeat10=True  term_baseN=True
  k= 1  (r+kB)/N=1330/3  residue=1  frac=1/3  int=False  repeat10=True  term_baseN=True
  k= 2  (r+kB)/N=776  residue=0  frac=0  int=True  repeat10=False  term_baseN=True
```

### Sweep All Remainders

```bash
python3 n_transform_duality.py --base=106 --N=3
```

Output:
```
[N× transform summary]  B=106, N=3, modulo(r)=53
  gcd(B,N)=1
  For N=3 and 3∤B: residues are exactly {0,1,2}/3 for every r (universal).
  integer-vertex k entropy (bits): 1.584
  integer-vertex k support: 3  uniform? False
  'trio' universal flag: True
  MZR(alpha=0.4) trio rate: 1.000  any-repeat rate: 1.000
```

### JSON Output

```bash
python3 n_transform_duality.py --base=106 --N=3 --json
```

Output:
```json
{
  "B": 106,
  "N": 3,
  "modulo": 53,
  "gcd_BN": 1,
  "N3_trio_universal": 1.0,
  "integer_k_entropy_bits": 1.584445579974916,
  "integer_k_support": 3.0,
  "integer_k_uniformity": 0.0,
  "MZR_trio_rate": 1.0,
  "MZR_any_repeat_rate": 1.0
}
```

## Parameters

- `--base B`: Base value (default: 106)
- `--N N`: N× transform value (default: 3)
- `--r R`: Single remainder to analyze (default: sweep all)
- `--modulo M`: Range for sweep [0, M) (default: B/2 for even B, else B)
- `--mzr-alpha α`: MZR selection parameter (default: 0.4)
- `--mzr-trials T`: Sampling trials for MZR (default: 20)
- `--json`: Emit JSON output

## Verification Against Rust Implementation

Compare Python reference with Rust production:

```bash
# Python
python3 n_transform_duality.py --base=106 --N=3 --json

# Rust
../prime_unified --run=ntransform --ntransform-bases=106 --out-dir=./test_out
cat ./test_out/ntransform_summary.csv
```

Expected match:
- Python `integer_k_entropy_bits`: 1.584445579974916
- Rust `integer_k_entropy_bits`: 1.584446 (6 decimal places)

## Mathematical Background

### The N× Transform

Given base B and remainder r, compute N fractional vertices:

```
v_k = (r + k·B) / N    for k = 0, 1, ..., N-1
```

Each vertex has:
- **Residue**: `(r + k·B) mod N`
- **Fractional part**: `residue / N`
- **Integer vertex**: k where residue = 0

### Key Theorem (N=3, gcd(B,3)=1)

For N=3 and 3∤B, the three residues are **always exactly** {0, 1, 2} in some order, for every r.

**Proof sketch**: Since gcd(B,3)=1, B generates the full group Z/3Z. Adding r just rotates the triple.

**Implication**: The "100% {0,⅓,⅔} coverage" is **structural**, not a filter effect. MZR can only affect *which* vertex becomes the integer one, not whether all three fractions appear.

### Integer Vertex Formula

When gcd(B,N)=1, the integer vertex is:

```
k_int ≡ -r·B⁻¹ (mod N)
```

where B⁻¹ is the modular inverse of B modulo N.

### Uniformity vs Entropy

- **Support**: How many distinct k values can be k_int across all r
- **Uniformity**: Whether each k appears equally often (true ⇔ gcd(B,N)=1)
- **Entropy**: Shannon entropy of k_int distribution (max = log₂(N) for uniform)

For gcd(B,N)=1:
- Support = N (all k values possible)
- Uniformity = True (each appears |r|/N times)
- Entropy ≈ log₂(N) = 1.585 for N=3

## Relationship to Main Research

### CCRT Analysis
If MZR biases k_int, it might correlate with:
- Complementary vs single-zero coverage rates
- Which residue classes dominate Goldbach pairs

### MDR Analysis
Integer vertex bias could explain:
- Why certain remainder ranges show stronger PNT deviation
- Correlation between k_int entropy and `ratio_w_over_pred`

## Dependencies

**Zero**. Uses only Python 3.8+ standard library:
- `fractions.Fraction` for exact arithmetic
- `dataclasses` for clean data structures
- `argparse` for CLI
- `json` for structured output

## License

MIT (same as parent project)
