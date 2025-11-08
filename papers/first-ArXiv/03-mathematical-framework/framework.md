# Mathematical Framework

## Membrane Polynomial Definition

A membrane polynomial of width w over base b with boundary parameters (L,R) and seed C is defined as:

$$M_{L,R,b,w}(C) = L \cdot b^{w-1} + R \cdot b^{w-2-r_1} + C \cdot b^{\lfloor w/2 \rfloor} + R \cdot b^{r_2+1} + L$$

where $r_1$ and $r_2$ represent zero-padding positions. The symmetric structure with boundary digits L and R creates a "membrane" around the central seed value C.

## Residue Space

For a set of primes $\{p_1, p_2, ..., p_k\}$, we define residue space as the Cartesian product:

$$\mathcal{R} = \mathbb{Z}/p_1\mathbb{Z} \times \mathbb{Z}/p_2\mathbb{Z} \times ... \times \mathbb{Z}/p_k\mathbb{Z}$$

Each integer n maps to a vector $(n \bmod p_1, n \bmod p_2, ..., n \bmod p_k)$ in this space.

[Stub: Expand with theorem about linear trajectories, proof of affine structure, connection to classical sieving]