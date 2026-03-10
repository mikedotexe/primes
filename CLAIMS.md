# Claim-Evidence Registry

**Last verified**: March 2026 (updated after EVIDENCE.md audit rounds 1-2)

Every significant claim made in this repository's public documents (README.md,
CLAUDE.md) is listed below with its status, evidence source, and verification
command. If a verification command fails, the claim should be reviewed.

**EVIDENCE.md audit note**: Two audit rounds found significant errors in
EVIDENCE.md (5/14 false primality claims, 6 inflated data table entries). All
corrections are recorded with strikethrough notation in EVIDENCE.md. The claims
below reflect corrected values.

## Verified Empirical Claims

| # | Claim | Status | Evidence | Verification |
|---|-------|--------|----------|--------------|
| 1 | Membrane (1,5) base 6 achieves ~33% prime density | `empirical` | VERIFIED_FACTS_VS_SPECULATION.md Fact 1, n=1000, p<0.001 | `cargo run --example prime_verification_report` |
| 2 | Base 30 (11,7) k=(0,0) achieves ~30% prime density | `empirical` | VERIFIED_FACTS_VS_SPECULATION.md, n=1000 | `cargo run --example comprehensive_base_analysis` |
| 3 | Base 10 (3,7) k=(0,0) achieves ~18.5% prime density | `empirical` | VERIFIED_FACTS_VS_SPECULATION.md, n=1000 | `cargo run --example comprehensive_base_analysis` |
| 4 | k=0 dominates for seed length M >= 2 (except base 10 M=2) | `empirical` | VERIFIED_FACTS_VS_SPECULATION.md Facts 1-4, 5+ bases | `cargo run --example prime_verification_report` |
| 5 | Coprimality of boundary digits to base is required | `empirical` | VERIFIED_FACTS_VS_SPECULATION.md, 100% of top configs are coprime | `cargo run --example comprehensive_base_analysis` |
| 6 | Diameter-density law: compactness predicts density (rho > 0.77) | `empirical` | VERIFIED_FACTS_VS_SPECULATION.md, Spearman rho > 0.77, p < 1e-20 | (verified in scaling exploration, not in a standalone example) |
| 7 | Base 10 M=2 is a uniquely isolated exception where k=1 beats k=0 | `empirical` | VERIFIED_FACTS_VS_SPECULATION.md Fact 2, delta=+5.9pp, p~0.01 | `cargo run --example prime_verification_report` |
| 8 | Membrane density advantage is largely explained by coprimality filtering | `empirical` | collab/THEORETICAL_CLOSURE.md, structure boost ~1.02x (not significant) | `cargo run --example membrane_vs_random` |

## Infrastructure Claims

| # | Claim | Status | Evidence | Verification |
|---|-------|--------|----------|--------------|
| 9 | 174 library tests pass | `verified` | CI | `cargo test --lib` |
| 10 | Clippy clean on all targets | `verified` | CI | `cargo clippy --all-targets -- -D warnings` |
| 11 | 32 curated examples compile | `verified` | examples/README.md | `for f in examples/*.rs; do cargo build --example "$(basename "$f" .rs)" 2>/dev/null || echo "FAIL: $f"; done` |
| 12 | 32/80 Agda modules type-check (20 clean, 12 with postulates) | `verified` | agda-proofs/STATUS.md | Local: `agda <module>` for each listed module |
| 13 | Miller-Rabin with 20 rounds (error rate < 1e-12) | `implemented` | src/lib.rs `is_prime` function | `cargo run --example verify_prime_checker` |
| 14 | Sieve matches OEIS A000720 reference counts | `verified` | pi(10^k) smoke test | `cargo run --example prime_count_smoke_test` |

## Falsified Claims (Documented for Honesty)

| # | Claim | Status | Reference |
|---|-------|--------|-----------|
| F1 | Scaling law k* ~ sqrt(M) | `falsified` | VERIFIED_FACTS_VS_SPECULATION.md, measured exponent ~0, R^2 ~0 |
| F2 | 2xp resonance hypothesis | `falsified` | VERIFIED_FACTS_VS_SPECULATION.md Fact 2b, base 14 shows k*=0 |
| F3 | GPU 50x speedup | `removed` | No benchmark existed; claim removed from lib.rs in Track 2 |
| F4 | `simd` feature | `removed` | Never existed in Cargo.toml; reference removed from lib.rs |

## Open Questions (Not Claims)

| # | Question | Status |
|---|----------|--------|
| O1 | Why does M=1 prefer k>0 while all larger M prefer k=0? | `open` |
| O2 | Is directional asymmetry in prime connectors a general phenomenon? | `open` (tested on one pair only) |
| O3 | Can the diameter-density law be proven from k-tuple conjecture theory? | `open` |

## Methodology

- **Primality testing**: Miller-Rabin, 20 rounds, deterministic bases for small numbers
- **Sample sizes**: Minimum n=100 per configuration, n=1000 for key claims
- **Statistical tests**: z-test for proportions, Hedges' g for effect sizes, Spearman rho for correlations
- **Falsifiability**: Each verified fact includes explicit criteria for refutation
- **Full statistical details**: [VERIFIED_FACTS_VS_SPECULATION.md](VERIFIED_FACTS_VS_SPECULATION.md)
