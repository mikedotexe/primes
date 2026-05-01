# Collaborator Research Memo

This memo is meant to be constructive.

The goal is not to flatten the ambition of the repository. The goal is to help
the project preserve its strongest signal, reduce avoidable overreach, and make
the surviving core easier for skeptical collaborators to trust and build on.

This reading is broadly aligned with the collaboration stance in
[AGENT.md](AGENT.md), the current project summary in [AGENTS.md](AGENTS.md),
the evidence record in [EVIDENCE.md](EVIDENCE.md), and the verified/open split
in [VERIFIED_FACTS_VS_SPECULATION.md](VERIFIED_FACTS_VS_SPECULATION.md).

## Short Read

The repository currently reads best as a disciplined exploratory research
program, not yet as a finished unifying theory.

That is a good place to be.

The strongest assets are:

- it keeps negative results alive
- it contains real local formal structure
- it exposes runnable verification surfaces
- it often distinguishes metaphor from mathematics

The main risk is not lack of effort or lack of interesting structure. The main
risk is that local formal or empirical signal may be read too quickly as
support for a broader explanatory story than the current evidence warrants.

## What To Protect

### 1. Protect Negative-Result Memory

[NOVELTY.md](NOVELTY.md) is one of the most valuable documents in the project.
It shows that the repository is willing to say:

- a proposed scaling law failed
- a resonance story did not survive testing
- a pattern was not detected under stronger controls

That is a strength, not a weakness. It makes the rest of the repo easier to
trust.

Recommendation:

- keep failed hypotheses visible
- promote refutations into the main claim surface when they materially narrow
  the story
- treat "narrowing results" as progress, not embarrassment

### 2. Protect The Narrow Formal Core

[lean-proofs/THEOREM_INDEX.md](lean-proofs/THEOREM_INDEX.md) contains real
value. It supports genuine local structure around:

- residues and admissibility
- symmetry and pairing
- midpoint obstruction
- affine template structure
- exact sieve arithmetic
- connector residue filters

This is the strongest "hard" surface in the repo. It should remain the
canonical answer to the question:

"What is actually proved here?"

Recommendation:

- keep the theorem inventory legible and narrow
- keep using it to support local claims
- avoid letting local proof automatically inflate broader mechanism claims

### 3. Protect The Verification Surface

[EVIDENCE.md](EVIDENCE.md), [STATUS.md](STATUS.md), and the existing examples
make the repo feel operational rather than rhetorical.

The fact that the repo includes explicit commands like:

- `cargo test --lib`
- `cargo clippy --lib -- -D warnings`
- `cd lean-proofs && lake build`

is important. So is the fact that some empirical claims are paired with
explicit verification entrypoints.

Recommendation:

- preserve runnable examples and verification commands
- make every important empirical claim point to one concrete reproduction path

### 4. Protect Scope Disclaimers

The project is at its best when it clearly says which layer a statement belongs
to:

- verified
- empirical
- open
- metaphor

The repo already does some of this well, especially around the visualization
layer.

Recommendation:

- keep the metaphor layer clearly subordinate
- make the "what layer is this?" answer easy to see in top-level docs

## What To Prune

### 1. Prune Narrative Bleed

The biggest thing to watch is the slide from:

- local theorem
- or empirical pattern

into:

- broader explanatory mechanism

without a sufficiently strong bridge.

The repo does not need to become timid. It just needs to make the bridge
requirements explicit.

Recommendation:

- whenever a broader interpretation is offered, state what evidence tier it is
  currently in
- do not let the existence of formal local structure imply that the larger
  narrative is now supported

### 2. Prune Metaphor From The Default Reading Path

Metaphor can be useful. It can also be sticky.

If a new reader meets tidal, gravity, resonance, or related language too early,
they may over-credit it as mechanism even when the repo itself is more careful.

Recommendation:

- keep metaphor available, but not as the main front door for skeptical readers
- make the evidence-first route easier to find than the metaphor-first route

### 3. Prune One-Off Anomaly Weight

Some of the exploratory scripts ask good questions. That is valuable. But
single-base or single-window stories should stay narrow until they survive:

- matched controls
- replication
- holdout evaluation

Recommendation:

- label one-off findings explicitly as local probes
- promote them only after they survive a stronger comparison ladder

### 4. Prune Claim Drift Across Documents

The same idea can currently appear in several forms:

- prose intuition
- theorem inventory
- script hypothesis
- narrative interpretation

This creates room for drift.

Recommendation:

- maintain one canonical claim ledger with fields like:
  - claim
  - current status
  - evidence type
  - strongest supporting artifact
  - strongest limiting artifact
  - next falsifier

## Three Experiments That Would Most Improve Credibility

### 1. Run A Prospective Holdout Prediction Challenge

Pick a small number of narrow, measurable hypotheses from the exploratory
analysis scripts and freeze them before evaluation.

Good candidates include:

- prime outer-digit anomaly expectations
- discriminant-linked effects
- `k=0` vs `k=1` selection stories
- Goldbach-richness correlations

Then test them on unseen bases, seed ranges, or template families.

Why this matters:

- it distinguishes explanation from hindsight
- it turns "interesting pattern" into a genuine predictive test
- it is one of the fastest ways to gain credibility with skeptics

Desired output:

- one markdown table with pre-registered hypotheses, target cohorts, and pass /
  fail / mixed outcomes

### 2. Build A Strong Null-Model Ladder

The most important empirical question is not just "does this beat naive
random?" It is:

- does this beat random?
- does it beat random-coprime?
- does it beat a residue-matched control?
- does it beat a permutation-preserving structural control?

[EVIDENCE.md](EVIDENCE.md) already points toward this discipline. Expanding it
would make the repo much stronger.

Why this matters:

- many interesting effects shrink once the control is strengthened
- if something still survives that ladder, it becomes much more meaningful

Desired output:

- a standard control ladder used by all major empirical claims
- one table per claim family with effect size, uncertainty, and correction for
  multiple testing

### 3. Force One Formal Claim To Cash Out Empirically

Choose one narrow theorem-backed structural claim and derive one specific,
measurable prediction from it.

The point is not to prove the whole narrative. The point is to create one clean
bridge from:

- local formal structure

to:

- empirical consequence

that can be tested against a classical baseline.

Why this matters:

- it is the most promising route from "there is real local math here" to "some
  of this structure may actually explain observed behavior"
- even a negative result would be clarifying

Desired output:

- one explicit theorem-to-prediction note
- one benchmark or script that tests the prediction against a baseline
- one conclusion that says either "bridge supported", "bridge weakened", or
  "bridge failed"

## Suggested Near-Term Repo Improvements

If the team wants a tight next tranche, I would do these in order:

1. Add a canonical claim ledger.
2. Add one prospective holdout experiment.
3. Standardize the control ladder for empirical claims.
4. Add one theorem-to-prediction bridge note.
5. Make the evidence-first reading path the default path for new readers.

## Best Current Framing

The strongest honest version of the project currently looks like this:

- membrane constructions are a real structured prime-candidate family
- several families beat naive random baselines
- coprimality and classical density effects explain much of the lift
- some narrower structural questions remain open and worth testing
- the formal work supports local arithmetic structure
- the broader explanatory story is still under refinement and should remain
  clearly scoped

That is not a weak framing. It is a credible one.

It gives the project room to mature without asking readers to accept more than
the evidence currently supports.
