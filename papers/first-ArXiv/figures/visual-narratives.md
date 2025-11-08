# Visual Narratives for the Paper

## Figure Concepts That Tell Our Story

### Figure 1: The Membrane Structure
```
Traditional prime search:
    ?  ?  ?  ?  ?  ?  ?  ?  ?  ?
    ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓
   101 102 103 104 105 106 107 108 109 110
    ✗  ✗  ✓  ✗  ✗  ✗  ✓  ✗  ✓  ✗
    
Membrane prime search:
    L  .  R  .  C  .  R  .  L
    ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓
    5  0  5  0  2  0  5  0  5  (base 6)
    └──────────────────────┘
              ↓
            257₁₀ ✓ (prime!)
```

Caption: Traditional search tests arbitrary sequential numbers with ~10% success. Membrane polynomials create structured numbers with 25-30% success.

### Figure 2: The Affine Transform Visualization
```
Before (expensive):
M(c) = 245 + 6c
M(0) = 245 → 245 ÷ 7 = 35 R 0  ✗ divisible
M(1) = 251 → 251 ÷ 7 = 35 R 6  ✓ 
M(2) = 257 → 257 ÷ 7 = 36 R 5  ✓
M(3) = 263 → 263 ÷ 7 = 37 R 4  ✓
[Complex division every time]

After (linear):
s = 0, g = 6 (precomputed)
c=0: (0 + 0×6) mod 7 = 0  ✗
c=1: (0 + 1×6) mod 7 = 6  ✓
c=2: (0 + 2×6) mod 7 = 5  ✓
c=3: (0 + 3×6) mod 7 = 4  ✓
[Simple multiply-add pattern]
```

Caption: The affine transform converts expensive modular division into predictable multiply-add operations.

### Figure 3: Residue Space Trajectories
```
    mod 5
    4 ·   ·   ·   · M(4)
    3 · M(3) ·   ·   ·
    2 ·   · M(2) ·   ·
    1 ·   ·   · M(1) ·
    0 · M(0) ·   ·   ·
      0   1   2   3   4
          mod 3

Legend: · = point in residue space
        M(c) = membrane trajectory
        Lines = divisibility walls
```

Caption: Membrane sequences trace linear paths through residue space, systematically avoiding divisibility walls (where coordinate = 0).

### Figure 4: GPU Performance Evolution
```
     200M │                                    ╱━━━ 186.6M
          │                                   ╱
     150M │                                  ╱
          │                                 ╱
     100M │                            ╱━━━╱ (reciprocal)
          │                      ╱━━━━╱ (SIMD ballot)
      50M │               ╱━━━━╱ (batch size)
          │         ╱━━━━╱ (threadgroup)
          │    ╱━━━╱ (affine)
        0 ├━━━━┴────┴────┴────┴────┴────┴────
          │ CPU  GPU  Opt1  Opt2  Opt3  Opt4  Final
            270k  297k  3M   10M   31M   52M   186M
```

Caption: Each optimization enables the next, creating superlinear speedup from 270k to 186.6M candidates/second.

### Figure 5: Memory Access Pattern
```
Bad (random access):          Good (coalesced access):
Thread 0 → memory[1000]       Thread 0 → memory[0]  ┐
Thread 1 → memory[47]         Thread 1 → memory[1]  ├─ One
Thread 2 → memory[823]        Thread 2 → memory[2]  │  cache
Thread 3 → memory[91]         Thread 3 → memory[3]  │  line!
   ↓           ↓                 ↓           ↓       ┘
4 cache misses               1 coalesced read
```

Caption: Sequential access patterns enable memory coalescing, reducing bandwidth by 32x.

### Figure 6: The Exclusive Configuration Mystery
```
Configuration (3,7) k=(1,1) base 10:

Seed:  0    1    2    3    4    5    6    7    8    9
       ↓    ↓    ↓    ↓    ↓    ↓    ↓    ↓    ↓    ↓
       ✗    ✗    ✗    ✗    ✗    ✓    ✗    ✗    ✗    ✗
                              307050703
                            (unique prime!)

Statistical expectation: ~1 prime
Actual result: Exactly 1 prime, always seed 5
Probability if random: 0.387
Probability observed: 1.000
```

Caption: Some configurations work with exactly one seed value, suggesting deep mathematical constraints.

### Figure 7: Base-Dependent Prime Density
```
    35% │      ╭─╮
        │     ╱   ╲                ╭╮
    30% │    ╱ 6   ╲              ╱  ╲
        │   ╱       ╲            ╱ 12  ╲
    25% │  ╱         ╰──╮  ╭────╯      ╲
        │ ╱             ╰──╯            ╲
    20% │╱                               ╲
        ├────┬────┬────┬────┬────┬────┬────┬────
         2    4    6    8   10   12   14   16
                        Base

Bases 6 and 12 (products of first two primes) show 
anomalously high prime density.
```

Caption: Prime density varies dramatically with base, with bases 6 and 12 achieving championship performance.

### Figure 8: Threadgroup Memory Impact
```
Without threadgroup memory:        With threadgroup memory:
┌─────────────────────┐           ┌─────────────────────┐
│   Global Memory     │           │   Global Memory     │
│  ┌─────────────┐   │           │  ┌─────────────┐   │
│  │ Signatures  │   │           │  │ Signatures  │   │
│  └─────────────┘   │           │  └──────┬──────┘   │
│    ↑ ↑ ↑ ... ↑     │           │         ↓ (once)    │
│    │ │ │     │     │           │  ┌─────────────┐   │
│   1024 threads     │           │  │ Threadgroup │   │
│   102,400 reads!   │           │  │   Memory    │   │
└─────────────────────┘           │  └──────┬──────┘   │
                                  │     ↑ ↑ ↑ ↑        │
                                  │    1024 threads    │
                                  │    (fast access)   │
                                  └─────────────────────┘
```

Caption: Cooperative loading into threadgroup memory reduces global memory traffic by 100x.

### Figure 9: The Computational Naturalness Hierarchy
```
                    Natural
                      ↑
    Membrane ●        │
    Primes   ╲        │
              ╲       │      ● Quantum
               ╲      │      Simulation  
                ╲     │    ╱
    FFT ●────────╲────┼───╱─────────● Matrix
                  ╲   │ ╱            Multiply
                   ╲  │╱
    ────────────────╲─┼──────────────
                     ╲│
                      ●│ Traditional
                       │ Algorithms
                       │
                    Forced
```

Caption: Natural computations align mathematical structure with hardware capabilities. Forced computations fight against the grain.

### Figure 10: Collaborative Intelligence Network
```
           Human
         (intuition)
            ╱ ╲
           ╱   ╲
    "What if?" "Why?"
         ╱       ╲
        ╱         ╲
    Claude      o3-pro
  (explore)    (optimize)
       ╲         ╱
        ╲       ╱
    "Test"  "Speed"
          ╲   ╱
           ╲ ╱
         Results
         1000x
```

Caption: Three different types of intelligence created synergistic discovery through complementary strengths.

### Figure 11: The Breathing Pattern Effect
```
Symmetric k=(1,1):           Breathing k=(0,1):
                            
     3 1 7 1 C 1 7 1 3          3 7 1 C 1 7 3
     ● ● ● ● ● ● ● ● ●          ●●●●●●●●●
     Even spacing               Asymmetric clustering
     
     Density: 21.3%             Density: 30.2%
```

Caption: Asymmetric "breathing" patterns achieve 50% higher prime density than symmetric configurations.

### Figure 12: The Complete Pipeline
```
   CPU          Transfer       GPU         Transfer      CPU
   ═════════════════════════════════════════════════════════
   
   Generate  →  4M values  →  Affine   →  800k bits →  Miller-
   membranes     (16MB)       sieve       (100KB)      Rabin
   parallel                   parallel                  parallel
   
   2.7ms         0.3ms        21.4ms       0.2ms       42.6ms
   
   ───────────────────────────────────────────────────────────
                    Total: 66.7ms → 60M candidates/second
```

Caption: The complete pipeline leverages each processor's strengths: CPU for complex logic, GPU for massive parallelism.

### Creating These Figures

For the actual paper, these ASCII diagrams would be recreated as:
1. Vector graphics (TikZ for LaTeX)
2. High-resolution plots (matplotlib/pgfplot)
3. Professional diagrams (draw.io/Illustrator)
4. Performance charts (gnuplot/plotly)

But the ASCII versions capture the essential information architecture of what we need to communicate visually!