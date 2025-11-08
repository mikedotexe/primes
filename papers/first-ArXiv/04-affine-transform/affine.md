# The Affine Transform

## Core Innovation

The fundamental breakthrough is recognizing that membrane polynomials create affine sequences in residue space. For any prime p and membrane polynomial M(C), we can write:

$$M(C) \equiv s_p + g_p \cdot C \pmod{p}$$

where $s_p$ and $g_p$ are constants depending only on the membrane parameters and p, not on C.

## Pre-computation Phase

For each prime $p_i$ in our sieving set, we compute:
- Signature: $s_i = M(0) \bmod p_i$
- Generator: $g_i = (M(1) - M(0)) \bmod p_i$

This transforms divisibility testing from:
```
if M(C) % p == 0 then composite
```
to:
```
if (s + C * g) % p == 0 then composite
```

[Stub: Prove the affine property, show computational advantages, discuss invertibility and information preservation]