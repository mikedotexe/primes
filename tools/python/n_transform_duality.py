#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from __future__ import annotations
from dataclasses import dataclass
from fractions import Fraction
from typing import List, Tuple, Dict
import argparse
import math
import random
import json
import sys

# ---------- core number utils ----------
def egcd(a: int, b: int) -> Tuple[int,int,int]:
    if b == 0: return (a, 1, 0)
    g, x1, y1 = egcd(b, a % b)
    return (g, y1, x1 - (a // b) * y1)

def inv_mod(a: int, n: int) -> int | None:
    a %= n
    g, x, _ = egcd(a, n)
    if g != 1: return None
    return x % n

def prime_factors(n: int) -> Dict[int,int]:
    n = abs(n)
    out: Dict[int,int] = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            out[d] = out.get(d, 0) + 1
            n //= d
        d += 1 if d == 2 else 2
    if n > 1: out[n] = out.get(n, 0) + 1
    return out

def terminates_in_base(fr: Fraction, base: int) -> bool:
    # reduce fraction
    fr = Fraction(fr.numerator, fr.denominator)
    den = fr.denominator
    if den == 1: return True
    # a/b terminates in base N iff prime(den) ⊆ prime(base)
    pf_den = prime_factors(den).keys()
    pf_base = set(prime_factors(base).keys())
    return set(pf_den).issubset(pf_base)

# ---------- N× transform mechanics ----------
@dataclass(frozen=True)
class VertexInfo:
    k: int
    value_over_N: Fraction          # (r + kB) / N  (not reduced mod 1)
    residue: int                    # ((r + kB) mod N)
    fracpart: Fraction              # residue / N
    is_integer: bool                # residue == 0
    repeats_in_base10: bool         # fracpart == 1/3 or 2/3 (for N=3) OR generic base10 repeating? here we keep N=3 special
    terminates_in_baseN: bool       # terminating in base N (general criterion)

@dataclass
class TransformReport:
    B: int
    N: int
    r: int
    gcd_BN: int
    residues: List[int]
    fracparts: List[Fraction]
    integer_vertex_k: int | None
    distinct_residue_count: int
    vertices: List[VertexInfo]

def residues_after_transform(B: int, N: int, r: int) -> List[int]:
    # residues for k = 0..N-1
    a = r % N
    b = B % N
    return [ (a + k*b) % N for k in range(N) ]

def vertex_integer_k(B: int, N: int, r: int) -> int | None:
    # find k in [0..N-1] s.t. r + kB ≡ 0 (mod N); exists iff gcd divides r
    g = math.gcd(B % N, N)
    if r % g != 0: return None
    if g == N: return 0  # everything ≡ r (mod N), and r ≡ 0 (mod N)
    Bb = (B // g) % (N // g)
    rb = (r // g) % (N // g)
    inv = inv_mod(Bb, N // g)
    if inv is None: return None
    k0 = (-rb * inv) % (N // g)
    # Lift to 0..N-1 (there are g solutions, spaced by N/g)
    return int(k0)

def analyze_single(B: int, N: int, r: int) -> TransformReport:
    g = math.gcd(B, N)
    res = residues_after_transform(B, N, r)
    fps = [ Fraction(x, N) for x in res ]
    k_int = vertex_integer_k(B, N, r)
    verts: List[VertexInfo] = []
    for k in range(N):
        val = Fraction(r + k*B, N)
        residue = res[k]
        fp = Fraction(residue, N)
        is_int = (residue == 0)
        # repeating in base10 only meaningful for N=3 → 1/3 or 2/3
        repeats = (N == 3) and (fp == Fraction(1,3) or fp == Fraction(2,3))
        termN = terminates_in_base(fp, N)  # exact, factor-base check
        verts.append(VertexInfo(
            k=k, value_over_N=val, residue=residue, fracpart=fp,
            is_integer=is_int, repeats_in_base10=repeats, terminates_in_baseN=termN
        ))
    return TransformReport(
        B=B, N=N, r=r, gcd_BN=g, residues=res, fracparts=fps,
        integer_vertex_k=k_int,
        distinct_residue_count=len(set(res)),
        vertices=verts
    )

# ---------- sweep utilities ----------
def analyze_all_r(B: int, N: int, modulo: int | None = None) -> Dict[str, float]:
    # By default, use remainders mod p = B/2 for B=2p; otherwise all r in [0..B-1]
    if modulo is None:
        modulo = B
    data = [ analyze_single(B, N, r) for r in range(modulo) ]
    # sanity for N=3: when 3 ∤ B, every r should yield residues {0,1,2}
    trio_ok = (N == 3 and (B % 3 != 0)) and all(set(rep.residues) == {0,1,2} for rep in data)
    # distribution of which k is integer (when gcd(B,N)=1, should be uniform)
    ks = [rep.integer_vertex_k for rep in data if rep.integer_vertex_k is not None]
    hist_k: Dict[int,int] = {}
    for x in ks: hist_k[x] = hist_k.get(x,0)+1
    ent = 0.0
    total = sum(hist_k.values())
    if total > 0:
        for c in hist_k.values():
            p = c/total
            ent -= p * (0 if p==0 else math.log(p, 2))
    return {
        "gcd_BN": math.gcd(B, N),
        "modulo": modulo,
        "N3_trio_universal": 1.0 if trio_ok else 0.0,
        "integer_k_entropy_bits": ent,
        "integer_k_support": float(len(hist_k)),
        "integer_k_uniformity": float(int(len(set(hist_k.values()))==1)) if len(hist_k)>0 else 0.0
    }

def sample_MZR(B: int, N: int, alpha: float, trials: int, modulo: int | None = None) -> Dict[str,float]:
    if modulo is None: modulo = B
    # Choose r ≈ alpha * (B/2) (your "0.4 × HZ" rule), allow ±1 jitter
    p = B//2
    base_r = int(round(alpha * p))
    hits_trio = 0
    hits_any_repeat = 0
    for _ in range(trials):
        r = (base_r + random.randint(-1,1)) % modulo
        rep = analyze_single(B, N, r)
        if N == 3 and set(rep.residues) == {0,1,2}: hits_trio += 1
        if any(v.repeats_in_base10 for v in rep.vertices): hits_any_repeat += 1
    return {
        "MZR_trio_rate": hits_trio / trials,
        "MZR_any_repeat_rate": hits_any_repeat / trials
    }

# ---------- CLI ----------
def main():
    ap = argparse.ArgumentParser(prog="n_transform_duality", description="Exact N× transform analysis (no floats)")
    ap.add_argument("--base", type=int, default=106, help="B (default 106=2*53)")
    ap.add_argument("--N", type=int, default=3, help="N× transform (default 3)")
    ap.add_argument("--r", type=int, default=None, help="single remainder r to analyze")
    ap.add_argument("--modulo", type=int, default=None, help="range for r sweep (default: p=B/2 if B even else B)")
    ap.add_argument("--mzr-alpha", type=float, default=0.4, help="alpha in r≈alpha*(B/2)")
    ap.add_argument("--mzr-trials", type=int, default=20, help="trials around alpha for MZR sampling")
    ap.add_argument("--json", action="store_true", help="emit JSON summary")
    args = ap.parse_args()

    B = args.base
    N = args.N
    modulo = args.modulo
    if modulo is None:
        modulo = B//2 if B%2==0 else B

    if args.r is not None:
        rep = analyze_single(B, N, args.r)
        if args.json:
            print(json.dumps({
                "B": B, "N": N, "r": args.r, "gcd_BN": rep.gcd_BN,
                "residues": rep.residues,
                "fracparts": [f"{x.numerator}/{x.denominator}" for x in rep.fracparts],
                "integer_vertex_k": rep.integer_vertex_k,
                "distinct_residue_count": rep.distinct_residue_count,
                "vertices": [{
                    "k": v.k,
                    "value_over_N": f"{v.value_over_N.numerator}/{v.value_over_N.denominator}",
                    "residue": v.residue,
                    "fracpart": f"{v.fracpart.numerator}/{v.fracpart.denominator}",
                    "is_integer": v.is_integer,
                    "repeats_in_base10": v.repeats_in_base10,
                    "terminates_in_baseN": v.terminates_in_baseN
                } for v in rep.vertices]
            }, indent=2))
        else:
            print(f"B={B}, N={N}, r={args.r}, gcd(B,N)={rep.gcd_BN}")
            print(f"residues: {rep.residues}")
            print(f"fracparts: {rep.fracparts}")
            print(f"integer vertex k: {rep.integer_vertex_k}")
            for v in rep.vertices:
                print(f"  k={v.k:2d}  (r+kB)/N={v.value_over_N}  residue={v.residue}  "
                      f"frac={v.fracpart}  int={v.is_integer}  "
                      f"repeat10={v.repeats_in_base10}  term_baseN={v.terminates_in_baseN}")
        return

    # sweep
    summary = analyze_all_r(B, N, modulo=modulo)
    mzr = sample_MZR(B, N, args.mzr_alpha, args.mzr_trials, modulo=modulo)

    if args.json:
        out = {"B":B,"N":N,"modulo":modulo}
        out.update(summary)
        out.update(mzr)
        print(json.dumps(out, indent=2))
    else:
        print(f"[N× transform summary]  B={B}, N={N}, modulo(r)={modulo}")
        print(f"  gcd(B,N)={summary['gcd_BN']}")
        if N==3 and B%3!=0:
            print("  For N=3 and 3∤B: residues are exactly {0,1,2}/3 for every r (universal).")
        elif N==3 and B%3==0:
            print("  For N=3 and 3|B: all three vertices share the same residue (collapse).")
        print(f"  integer-vertex k entropy (bits): {summary['integer_k_entropy_bits']:.3f}")
        print(f"  integer-vertex k support: {summary['integer_k_support']:.0f}  "
              f"uniform? {bool(summary['integer_k_uniformity'])}")
        if N==3:
            print(f"  'trio' universal flag: {bool(summary['N3_trio_universal'])}")
        print(f"  MZR(alpha={args.mzr_alpha}) trio rate: {mzr.get('MZR_trio_rate',0):.3f}  "
              f"any-repeat rate: {mzr.get('MZR_any_repeat_rate',0):.3f}")

if __name__ == "__main__":
    main()
