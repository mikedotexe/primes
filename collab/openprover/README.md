# OpenProver Companion Workflow

**Updated**: March 2026

This directory is a staging area for using OpenProver against the repository's
current theorem-decomposition and proof-planning backlog.

## Operational Summary

OpenProver is currently useful in this repository as a companion system for:

- theorem decomposition
- lemma dependency extraction
- counterexample search
- literature-guided reduction of open questions
- Lean 4 proof search against the in-repo package in `lean-proofs/`

It is **not** currently a direct verifier for the Agda workspace.

## Supported Uses

### 1. Discovery and Reduction

Use OpenProver on problem statements in `theorems/` when the task is to reduce a
large question into smaller claims, candidate lemmas, or failure modes.

Appropriate targets include:

- open questions listed in `CLAIMS.md`
- proof shells in `agda-proofs/Theorems/`
- reductions of empirical questions such as the diameter-density relation

### 2. Agda Companion Planning

Use OpenProver to prepare proof artifacts for Agda without treating it as an
Agda checker.

Expected outputs:

- cleaned theorem statements
- lemma orderings
- constructive versus nonconstructive branch points
- classical proof sketches suitable for later porting
- explicit failure notes for routes that collapse

### 3. Lean 4 Companion Verification

When machine checking is required through OpenProver, the practical target is
the in-repo Lean package under `lean-proofs/`.

Best candidates:

- finite involution and pairing lemmas
- midpoint-obstruction theorems
- coprimality and radical filters
- unit-residue, unit-group orbit, and finite-CRT lemmas
- direct bridges from `ZMod`-unit statements back into the abstract symmetry API

This is a companion lane, not a replacement for the Agda workspace.

## Task Inventory

| File | Mathematical role | Repository alignment |
|------|-------------------|----------------------|
| `theorems/01-base10-prime-filter.md` | base-10 coprimality filter | `agda-proofs/Examples/Base10ResidueFilter.agda`, Lean density entry lemma |
| `theorems/02-symmetry-midpoint-obstruction.md` | finite symmetry certification | Agda abstract certification stack, Lean midpoint lane, `ZMod`-unit symmetry witness lane |
| `theorems/03-base-radical-prime-filter.md` | radical and wheel-base filter facts | Agda radical layer, Lean radical/unit-residue/wheel-base/unit-group CRT lane |
| `theorems/04-diameter-density-reduction.md` | reduction of an empirical open question | `CLAIMS.md` open question O3 |
| `theorems/05-residual-template-effect-reduction.md` | theorem-level route for any residual template effect | `README.md` open questions, Lean affine/residue shell, matched-control example lane |

## Example Commands

Discovery pass:

```bash
cd /Users/mikepurvis/other/openprover
python3 -m openprover \
  --theorem /Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/collab/openprover/theorems/04-diameter-density-reduction.md \
  --max-time 30m \
  --headless \
  --no-isolation
```

Focused theorem decomposition:

```bash
cd /Users/mikepurvis/other/openprover
python3 -m openprover \
  --theorem /Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/collab/openprover/theorems/02-symmetry-midpoint-obstruction.md \
  --max-time 20m \
  --headless
```

Lean-backed checking against the canonical package:

```bash
cd /Users/mikepurvis/other/openprover
python3 -m openprover \
  --theorem /Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/collab/openprover/theorems/02-symmetry-midpoint-obstruction.md \
  --lean-project /Users/mikepurvis/Library/CloudStorage/Dropbox/Kairos/primes/prime-physics-engine/lean-proofs \
  --lean-theorem /path/to/THEOREM.lean
```

## Task-Writing Rules For This Repo

Include:

- claim status: verified, open, or shell-only
- target module or theorem family
- objective: proof, reduction, counterexample search, or literature scan
- success criterion stated in repository terms

Avoid:

- asking for an Agda formalization without a bounded theorem surface
- treating postulated Agda modules as settled theorems
- asking for a template-specific density proof when the current interpretation
  is dominated by coprimality and candidate-size effects

## Near-Term Priority Order

1. Strengthen the Lean midpoint and symmetry witness lane, especially the direct
   `ZMod`-unit bridge.
2. Reduce postulates around the radical/coprimality and wheel-base CRT layer.
3. Run disciplined reduction passes on connector asymmetry, the
   diameter-density question, and any residual-template-effect route.
4. Accumulate reusable sublemmas before any broader formalization sprint.

## Agda Backend Outlook

An Agda backend remains technically plausible, but it does not exist in the
current OpenProver tool layer.

The minimum viable extension would provide:

- `agda_verify(code_or_file)`
- `agda_store(code)`
- `agda_grep(query)` or an indexed Agda-stdlib search interface

Until such a layer exists, OpenProver should be treated as an upstream theorem
assistant for the Agda workflow and a direct companion only for Lean.
