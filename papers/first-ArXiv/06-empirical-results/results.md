# Empirical Results

## Prime Density Comparison

| Base | Configuration | Prime Density | vs Random |
|------|--------------|---------------|-----------|
| 6 | (5,5) k=(0,0) | 25.5% | 5.7x |
| 6 | (3,3) k=(0,1) | 30.2% | 6.7x |
| 10 | (3,7) k=(1,1) | 10% (exclusive) | N/A |
| 12 | (5,7) k=(0,0) | 23.2% | 5.2x |

*Random 32-bit integer baseline: 4.5% (1/ln(2³²))*

## Performance Benchmarks

Testing with 4 million candidates on Apple M2 Max:

| Implementation | Throughput | Time | Speedup |
|----------------|------------|------|---------|
| CPU Baseline | 270k c/s | 14.8s | 1x |
| GPU Original | 297k c/s | 13.5s | 1.1x |
| GPU Optimized | 31.9M c/s | 0.125s | 118x |
| GPU Kernel Only | 186.6M c/s | 0.021s | 691x |

## Statistical Validation

Chi-square tests confirm prime distributions significantly differ from random (p < 0.001).

[Stub: Add more bases, configuration migration patterns, large prime examples, statistical tests]