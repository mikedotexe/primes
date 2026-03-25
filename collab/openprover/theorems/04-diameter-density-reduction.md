Investigate the repository's open diameter-density question in a conservative
way.

Problem:

The repository reports a strong empirical relationship between compactness
(shorter or lower-diameter symmetric digit templates, repo alias: membranes)
and prime density. The current interpretation is that much of the overall lift
is explained by classical coprimality filtering and ordinary prime-density
effects.

Open question:

Can the observed diameter-density relationship be reduced to classical
number-theory effects, or is there any residual structural claim worth
isolating after same-budget controls?

Repo pointers:

- `CLAIMS.md` open question O3
- `collab/THEORETICAL_CLOSURE.md`
- `VERIFIED_FACTS_VS_SPECULATION.md`
- `agda-proofs/Integration/PrimeDensityFramework.agda`

What counts as success:

- a reduction of the question into clearly separated components
- a list of what is already explained by PNT, Euler/totient filtering, and
  candidate magnitude
- a short list of residual hypotheses that would still need new evidence
- if literature is relevant, point to the classical tools most likely to matter

It is acceptable if the outcome is:

- "mostly explained already"
- "open but narrower than stated"
- "needs a different control design before formalization"

Do not:

- claim a new mechanism unless it survives the repo's coprime-dominated framing
- treat current empirical correlation as a proof
