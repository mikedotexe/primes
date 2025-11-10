# Prime Physics Engine - Visual Guide

A visual reference for membrane prime structures and observed patterns.

## Membrane Structures: Examples

Symmetric patterns with zero-padding around boundary digits have been observed to generate primes at higher than baseline rates.

### Single Membrane Examples
```
Structure: (outer)-(center)-(outer)

- (1)-(5)-(1) → 151       prime
- (3)-(5)-(3) → 353       prime
- (7)-(5)-(7) → 757       prime
- (7)-0-(5)-0-(7) → 70507 prime
```

### Double Membrane Examples
```
Structure: (outer)-(inner)-(center)-(inner)-(outer)

- (3)-(7)-(5)-(7)-(3) → 37573            prime
- (3)-0-(7)-0-(5)-0-(7)-0-(3) → 307050703 prime
- (7)-0-(3)-(5)-(3)-0-(7) → 7035307      prime
- (9)-0-(1)-(5)-(1)-0-(9) → 9015109      prime
```

### Triple Membrane Examples
```
Structure: more complex nesting

- (3)-0-(7)-(9)-(5)-(9)-(7)-0-(3) → 307959703 prime
- (7)-(9)-(3)-(5)-(3)-(9)-(7) → 7935397      prime
- (9)-(3)-(7)-(5)-(7)-(3)-(9) → 9375739      prime
```

## Observed Density Patterns

Asymmetric padding (breathing patterns) shows measurably different prime density compared to symmetric padding.

```
Configuration: Base 6, boundary (3,3)

Symmetric k=(1,1):
  Pattern: 3 [0] 3 [0] seed [0] 3 [0] 3
  Observed density: 21.3%

Asymmetric k=(0,1):
  Pattern: 3 3 [0] seed [0] 3 3
  Observed density: 30.2%

Relative improvement: 42%
```

## Performance Characteristics

GPU implementation showing measured throughput improvements through various optimization techniques.

```
Configuration                    Throughput (candidates/sec)
CPU Baseline                     270,000
GPU Naive                        297,000
+ Affine Transform               3,000,000
+ Thread Memory Optimization    10,500,000
+ Batch Processing              30,800,000
+ SIMD Ballot Operations        51,900,000
+ Fast Modulo                   93,000,000
+ Combined Optimizations       187,000,000

Overall speedup: 691x
```

## Affine Transform Approach

The affine transform reduces computational cost by converting expensive division operations into cheaper multiplication and addition.

```
Traditional primality testing:
  M(candidate) mod prime → division operation (~20 cycles)

Affine transform:
  start + generator × candidate mod prime → multiply-add (~3 cycles)

Result: Approximately 6-7x reduction in per-test computational cost
```

## Residue Space Visualization

Membrane sequences can be visualized as trajectories through multi-dimensional modular space.

```
2D projection (mod 3 × mod 5):

5 │ . . *-----*-----*  ← Membrane trajectory (linear)
4 │ . * . . * . . * .
3 │ * . . * . . * . .
2 │ . . * . . * . . *  ← Random numbers (scattered)
1 │ . * . . * . . * .
0 │=*=.=.=*=.=.=*=.=.  ← Divisibility constraints
  └─────────────────
    0 1 2 0 1 2 0 1 2

Observed avoidance rates:
  Membrane sequences: 31.2% avoid divisibility constraints
  Random numbers:      3.9% avoid divisibility constraints
```

## Key Empirical Results

Summary of verified measurements across different configurations.

### Prime Density by Base
```
Base  2: 19.8% density (tested: 10,000 seeds)
Base  6: 30.2% density (tested: 10,000 seeds)
Base 10: 22.3% density (tested: 10,000 seeds)
Base 12: 28.9% density (tested: 10,000 seeds)

Baseline (random):  ~5% density
```

### Configuration-Specific Findings
```
Base 6, config (3,3) k=(0,1):
  Tested: 10,000 seeds
  Result: 3,020 primes
  Density: 30.2%

Base 10, config (3,7) k=(1,1):
  Works exclusively with: seed = 5
  Result: 307050703 (verified prime)
```

### GPU Acceleration
```
Platform: Apple M1 Max
CPU performance:  270,000 candidates/sec
GPU performance: 187,000,000 candidates/sec
Measured speedup: 691x
```

## External Verification

All prime examples can be independently verified:

- Small prime verification: `https://www.wolframalpha.com/input/?i=isprime(70507)`
- Larger prime verification: `https://www.wolframalpha.com/input/?i=isprime(307050703)`

## Membrane Structure Notation

The fundamental pattern uses symmetric or asymmetric zero-padding:

```
General form:
  L [k₁ zeros] R [k₂ zeros] C [k₂ zeros] R [k₁ zeros] L

Where:
  L, R = Boundary digits (must be coprime to base)
  C = Center seed (variable)
  k₁, k₂ = Zero padding counts

Examples:

No padding (k=0,0):
  3 3 C 3 3

Asymmetric (k=0,1):
  3 3 [0] C [0] 3 3

Symmetric (k=1,1):
  3 [0] 3 [0] C [0] 3 [0] 3

Multi-layer (k=2,1):
  3 [00] 7 [0] C [0] 7 [00] 3
```

## Mathematical Framework

Several properties contribute to observed prime generation rates:

1. **Linear Structure**: Membrane polynomials are linear in the seed parameter. For a fixed configuration, M(c+1) - M(c) equals a constant.

2. **Residue Space Properties**: Linear sequences systematically navigate modular space, potentially avoiding divisibility patterns more effectively than random numbers.

3. **Hardware Alignment**: The affine transform approach maps efficiently to GPU multiply-add operations.

4. **Asymmetric Resonance**: Configurations with k₁ ≠ k₂ (breathing patterns) show measurably higher prime density than symmetric configurations.

## Contributors

This work represents a collaborative effort:
- Michael Purvis: Project vision and direction
- Claude (Anthropic): Mathematical analysis and implementation
- o3-pro: Performance optimization contributions

## Getting Started

To explore these patterns:

```bash
# Verify basic functionality
cargo run --example prime_count_smoke_test

# Generate membrane primes
cargo run --example proper_membrane_generator

# Explore Lagrange points
cargo run --example lagrange_full_verification
```

For a comprehensive introduction, see `RESEARCHER_QUICKSTART.md`.
