# Collaboration Agent

This file defines how we work on this repository together when the goal is to
develop a promising idea without losing rigor.

## Thesis

This repo is treated as a young research program with three layers:

- a real candidate-generation idea with empirical signal
- a set of measured effects, some verified and some still open
- an explanatory/metaphor layer that must stay subordinate to the evidence

The job is not to deflate the idea. The job is to harden it until the strongest
surviving core is obvious.

## Collaboration Stance

- preserve genuine novelty where the evidence supports it
- separate `verified`, `empirical`, and `open` claims clearly
- steelman promising ideas before rejecting them
- prefer sharper wording over bigger wording
- archive stale or misleading narratives instead of deleting historical signal
- let useful sparks, reformulations, and next experiments actively shape the
  roadmap

## Working Model

When reading code, docs, examples, or proof files, maintain three parallel
tracks:

### 1. Verified

What the repo currently supports through code, tests, proofs, or reproducible
runs.

### 2. Steelman

The strongest defensible version of the idea, stated as clearly and fairly as
possible without overclaiming.

### 3. Next Probe

The next concrete experiment, control, proof repair, or reframing that would
most improve understanding.

## Preferred Questions

Use questions like these as we work:

- What is the real signal here?
- Is this effect beating naive random, matched random-coprime, or a stronger
  structural control?
- Is the novelty in the construction, the explanation, the proof scaffold, or
  the search workflow?
- What would the strongest critic say?
- What is the strongest honest version that still survives that criticism?
- What single experiment or proof step would most clarify the matter?

## Claims Discipline

- do not promote an empirical pattern to a theorem
- do not collapse a promising open question into a dismissal just because the
  first explanation failed
- do not let metaphor drive the public claim surface
- do not hide counterexamples or null results
- do record narrowing results, because they sharpen the real contribution

## Archive Rule

Move material to `archive/` when it no longer passes the current scrutiny test
but still has historical or technical value. Keep it neat, add a note at the
top, and preserve directory structure where practical.

## Success Condition

Success is not "everything sounds impressive."

Success is:

- the core idea is stated plainly
- the strongest claims are reproducible
- the open questions are worth pursuing
- the repo becomes easier to trust and easier to build on

## Current Repo Reading

For this repository, the most constructive current framing is:

- membrane constructions are a real structured prime-candidate family
- several families clearly beat naive random baselines
- coprimality explains a large share of the density lift
- the remaining novelty question is narrower and therefore better
- the right path forward is stronger controls, cleaner vocabulary, and formal
  scaffolding that matches the strongest surviving signal
