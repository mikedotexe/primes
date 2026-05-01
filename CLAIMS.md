# Claim-Evidence Registry

**Last verified**: March 2026 (updated after the `EVIDENCE.md` audit rounds)

This file records the active claims made in the canonical repository documents
(`README.md`, `CLAUDE.md`, `AGENTS.md`, and linked audited summaries). Each
entry lists the claim status, the primary evidence source, and the verification
command to rerun. If a verification command fails, the corresponding claim
should be re-audited before reuse.

**Audit note**: two audit rounds found material errors in `EVIDENCE.md`
(false primality claims and inflated data-table entries). Those corrections are
recorded in `EVIDENCE.md`; the registry below reflects the corrected values.

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
| 9 | Membrane families tested exactly are broader than the ordinary palindrome subset | `empirical` | EVIDENCE.md structural exact probes; non-palindromic subsets retain prime density in tested families | `cargo run --example membrane_palindrome_probe` |
| 10 | Exact same-budget centered-gap controls do not show a consistent advantage in the tested families | `empirical` | EVIDENCE.md structural exact probes; fixed-anchor and independent-digit probes both fail to show a stable centered-gap lift | `cargo run --example membrane_scaffold_probe` |
| 11 | In the matched width-5..7 zero-padded single-digit scan, the canonical connector pair `10301` / `3007003007003` yields 11 forward prime hits and 13 reverse prime hits after the exact direction-independent `mod 3` / `mod 9` residue filter | `empirical` | examples/connector_signal_report.rs, collab/CONNECTOR_SIGNAL.md | `cargo run --example connector_signal_report` |
| 12 | In the density-aware rerun of that matched connector scan, the canonical pair retains a corrected residual gap, but the sign of that gap does not persist across the maintained membrane, twin-prime, and Sophie Germain comparison family | `empirical` | examples/connector_signal_report.rs, collab/CONNECTOR_SIGNAL.md | `cargo run --example connector_signal_report` |

## Infrastructure Claims

| # | Claim | Status | Evidence | Verification |
|---|-------|--------|----------|--------------|
| 13 | 174 library tests pass | `verified` | CI | `cargo test --lib` |
| 14 | Clippy clean on all targets | `verified` | CI | `cargo clippy --all-targets -- -D warnings` |
| 15 | Curated top-level examples compile | `verified` | examples/README.md, STATUS.md | `for f in examples/*.rs; do cargo build --example "$(basename "$f" .rs)" 2>/dev/null || echo "FAIL: $f"; done` |
| 16 | 81/81 Agda modules type-check individually (40 clean-local, 41 with local postulates, 0 failing) | `verified` | agda-proofs/STATUS.md | Local: `agda <module>` for each listed module, or `cd agda-proofs && ./scripts/verify-clean-spine.sh` for the maintained clean spine |
| 17 | Miller-Rabin with 20 rounds (error rate < 1e-12) | `implemented` | src/lib.rs `is_prime` function | `cargo run --example verify_prime_checker` |
| 18 | Sieve matches OEIS A000720 reference counts | `verified` | pi(10^k) smoke test | `cargo run --example prime_count_smoke_test` |

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
| O4 | Can any narrower centered-gap family show a robust advantage after same-budget matching? | `open` |

## Maintained Research Programs (Not Claims)

This section is intentionally non-claim language. It records the repo's most
promising live programs in restrained wording so the research framing stays
stable without broadening the verified claim surface. Fuller constructive
steelman language lives in
[collab/HARDENED_RESEARCH_PROGRAMS.md](collab/HARDENED_RESEARCH_PROGRAMS.md).

| Program | Current maintained wording | Strongest current evidence | Upgrade condition |
|---|---|---|---|
| bounded-`k` threshold / compactness regime | The strongest live arithmetic signal is a bounded-`k` transition from anomaly-rich `M=1` through a sparse `M=2` boundary layer into a compact stable regime by `M=3` on the maintained catalog. | `examples/m_transition_curve_report.rs`, `examples/m2_m3_transition_report.rs`, `examples/chaos_threshold_translation_report.rs` | A proof-quality reduction or broader maintained sweeps that preserve the same threshold shape under stronger stress tests. |
| cross-base hinge / species classification | The short-length lane now looks species-like rather than universal, with base `14` as the strongest current overlap-led hinge witness and base `34` as a different boundary-led emergent species. | `examples/two_p_hinge_mask_report.rs`, `examples/two_p_hinge_report.rs`, `examples/base34_boundary_species_report.rs` | An exact discriminator that explains the hinge split in transfer or residue language rather than by case-by-case narrative. |
| connector directional asymmetry | Connector asymmetry remains a narrow but live program: some controlled connector families still show directional residuals, but the current maintained evidence is taxonomic rather than universal. | `examples/connector_signal_report.rs`, `examples/comparative_signal_report.rs`, `collab/CONNECTOR_SIGNAL.md` | Broader controlled family coverage and a smaller classifier that survives cross-base or family stress tests. |
| formal exact substrate | Lean and Agda now certify exact bounded-`k` and transfer combinatorics that make the threshold story intelligible, without yet proving the catalog-wide empirical narrative. | `lean-proofs/THEOREM_INDEX.md`, `collab/LEAN_THRESHOLD_SUBSTRATE.md`, `collab/AGDA_THRESHOLD_SUBSTRATE.md` | More report language translated into exact finite statements, especially where overlap-led and boundary-led distinctions can be restated without prose-only labels. |
| geometry / visual discovery surface | The visual lane is maintained as a disciplined discovery aid: useful when geometric patterns are treated as embeddings of exact arithmetic objects and translated back into counts, masks, or residue relations before promotion. | `examples/chaos_threshold_translation_report.rs`, `scripts/plot_chaos_threshold_translation.py`, `collab/CHAOS_THRESHOLD_TRANSLATION.md` | Repeated visual patterns that translate back into exact arithmetic features and survive quantitative checks across multiple bases or families. |

## Methodology

- **Primality testing**: Miller-Rabin, 20 rounds, deterministic bases for small numbers
- **Sample sizes**: Minimum n=100 per configuration, n=1000 for key claims
- **Statistical tests**: z-test for proportions, Hedges' g for effect sizes, Spearman rho for correlations
- **Falsifiability**: Each verified fact includes explicit criteria for refutation
- **Full statistical details**: [VERIFIED_FACTS_VS_SPECULATION.md](VERIFIED_FACTS_VS_SPECULATION.md)
