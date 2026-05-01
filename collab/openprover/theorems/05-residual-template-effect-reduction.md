Investigate a conservative theorem-level route for any residual template effect.

Problem:

The repository's current evidence says symmetric digit templates beat naive
random baselines, but the best matched controls do not yet show a stable
template-specific lift beyond coprimality filtering, candidate magnitude, and
same-budget scaffold effects.

Open question:

If a future matched-control study *does* find a stable positive residual across
families, what exact structural feature could still plausibly carry that
residual, and what parts of the effect are already explained by classical
arithmetic?

Repo pointers:

- `README.md` verified statements and open questions
- `NOVELTY.md`
- `collab/THEORETICAL_CLOSURE.md`
- `examples/membrane_vs_random.rs`
- `examples/membrane_scaffold_probe.rs`
- `lean-proofs/PrimeArithmetic/Structure/AffineTemplate.lean`
- `lean-proofs/PrimeArithmetic/Structure/AffineResidueSearch.lean`
- `lean-proofs/PrimeArithmetic/Density/UnitResidues.lean`
- `lean-proofs/PrimeArithmetic/Analysis/HardyLittlewoodShell.lean`

What counts as success:

- a separation between factors already explained by:
  - reduced residue-class filtering
  - candidate magnitude / logarithmic density
  - exact affine residue search in the seed variable
  - same-budget scaffold symmetries already ruled out empirically
- a short list of residual structural hypotheses that are still genuinely live
- one or more candidate theorem shells or impossibility statements sharp enough
  to guide later Lean work
- an explicit note if the correct conclusion is currently "no theorem-level
  mechanism route is justified until better controls exist"

Helpful output shape:

1. Already-explained components
2. Empirically eliminated structural stories
3. Smallest remaining structural candidate
4. Candidate formal statements
5. What extra empirical evidence would be needed before treating those
   statements as mechanism-level targets

Do not:

- claim a new density theorem without residual empirical support
- treat correlation or elevated raw density as a mechanism
- treat the physics-metaphor layer as theorem evidence
