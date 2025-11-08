# Membrane Pattern Reference

A quick reference for understanding membrane notation.

## The Basic Pattern

```
L ... R ... C ... R ... L

Where:
  L = Left boundary (outer wall)
  R = Right boundary (inner wall)  
  C = Center (the seed - our variable)
  ... = Some number of zeros
```

## Real Example

```
Configuration: (3,7) k=(1,1)

Pattern:   3 [0] 7 [0] C [0] 7 [0] 3
           ↑  ↑  ↑  ↑  ↑  ↑  ↑  ↑  ↑
           L  k₁ R  k₂ C  k₂ R  k₁ L

With C=5:  3  0  7  0  5  0  7  0  3
Result:    307050703 (prime)
```

## The k-values (Padding)

```
k = (k₁, k₂) controls the zeros:

k=(0,0):  3 7 C 7 3         → 37573      "Tight"
k=(1,1):  3 0 7 0 C 0 7 0 3 → 307050703  "Symmetric"
k=(0,1):  3 7 0 C 0 7 3     → 3705073    "Breathing" 🫁
```

## Why "Breathing"?

```
Symmetric k=(1,1):        Breathing k=(0,1):
    3                         3
   0 0                       7 7
  7   7        vs           0   0
 0     0                   C     C
C       C                 0       0  
 0     0                 7         7
  7   7                 3           3
   0 0
    3

Even spacing             Compressed/expanded
21.3% primes            30.2% primes ✨
```

## Quick Reference Card

```
┌─────────────────────────────────────┐
│ (L,R) = Boundary digits             │
│ k=(k₁,k₂) = Zero padding           │
│ C = Seed value (our variable)       │
│                                     │
│ Example: (3,7) k=(1,1) seed=5      │
│ → 3 0 7 0 5 0 7 0 3 = 307050703   │
└─────────────────────────────────────┘
```

## The Magic

```
Different configurations have different "personalities":

(3,3) k=(0,1) base 6  → 30% primes! 🎯
(3,7) k=(1,1) base 10 → Works ONLY with C=5
(1,1) k=(0,1) base 2  → Simple but effective

The same pattern, different parameters, 
wildly different behaviors.
```

---

```
Ready to try it?
  Config: (3,7) k=(1,1)
  Seed: 5
  → 307050703
  
  Check it: https://www.wolframalpha.com/input/?i=isprime(307050703)
```