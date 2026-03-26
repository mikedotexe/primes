# Hardening Roadmap

**Updated**: 2026-03-25
**Purpose**: current tranche-based hardening plan for the active repository

Previous roadmap history is preserved at
[`archive/ROADMAP_legacy.md`](archive/ROADMAP_legacy.md). That file remains
valuable as historical context, but it no longer reflects the current repo
state.

Throughout this roadmap, "membrane" denotes the symmetric digit-template family
implemented in the crate. Active tracks should be read in the standard
mathematical vocabulary of residue classes, coprimality filters, midpoint
symmetry, and exact or empirical claims.

## Current Assessment

### Strongest Verified Spine

- `cargo test --lib` passes 174 library tests
- `cargo clippy --lib -- -D warnings` passes cleanly
- repo-level mutable counts live in [`STATUS.md`](STATUS.md)
- active source-of-truth surfaces are [`CLAIMS.md`](CLAIMS.md),
  [`README.md`](README.md), [`CLAUDE.md`](CLAUDE.md),
  [`EVIDENCE.md`](EVIDENCE.md), [`VERIFIED_FACTS_VS_SPECULATION.md`](VERIFIED_FACTS_VS_SPECULATION.md),
  and [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md)
- symmetric digit templates clearly beat naive random baselines in several
  tested families
- the best current template-vs-random-coprime control does not establish a
  statistically significant extra lift beyond coprimality filtering
- exact enumeration now shows the template family is broader than the ordinary
  palindrome subset in the tested families
- the repo now has two active formalization lanes:
  [`agda-proofs/`](agda-proofs/) and the in-repo Lean package
  [`lean-proofs/`](lean-proofs/)
- the first Lean 4 milestone is live and builds with `lake build`, covering the
  abstract midpoint-obstruction theorem plus a concrete base-6 certified witness
- the Lean lane now also has a conservative exact arithmetic layer covering
  affine template structure, coprimality/radical/unit-residue results,
  `ZMod`/CRT/wheel-base structure, reusable symmetry certificates, explicit
  finite certificate examples, exact connector residue filters, and a
  prime-engine correctness lane for odd-only segmented sieve arithmetic and
  wheel30 candidate agreement

### Public Signal Still Above Support

- several subtree docs outside the root source-of-truth set have not yet been
  hardened to the new claim standard
- some public comments and exploratory outputs still use metaphor-first or
  non-technical language
- the centered-scaffold question is sharper now: the family is broader than
  palindromes, but the canonical mirror-zero layout has not been isolated as
  the driver
- the Lean lane is still much narrower than the Agda lane, so repo-level docs
  must keep describing it as a symmetry-first foundation rather than a broad
  second proof stack

### Next Lean Tranches

The Lean lane now has enough structure that future work can focus on a small
set of durable mathematical targets instead of exploratory module growth. The
affine modular-search tranche is now complete, the wheel-base CRT theorem is
available in canonical finite-family form, the negation quotient is restated in
group-action language via the order-two subgroup `{1, -1}`, and the symmetry
lane now has both a modular reflection layer and a reusable reflection
certificate wrapper. The constructive balanced-bucket support substrate is now
also live, and the Lean symmetry lane now also has both the balanced-bucket
reflection wrapper and the narrow window-certificate shell on top of it. The
exact connector arithmetic lane is now also live, including fixed-width
concatenation formulas and canonical decimal `mod 3` / `mod 9` exclusion
filters, and the family-level residue-profile layer is now live as well. The
prime-engine correctness lane is now also live: the Lean package has exact
odd-only segmented sieve arithmetic, the wheel30 admissible residues, and the
agreement theorem connecting wheel30 candidates back to the filtered odd
candidate domain. The Lean symmetry lane now also has a generated-data
entrypoint from residue lists and position lists into the maintained
window-certificate shell, plus a compact proof-object layer for generated
artifacts. The external runtime-export path is now also live: the Rust binary
[`src/bin/export_window_certificate.rs`](src/bin/export_window_certificate.rs)
emits Lean-shaped certificate artifacts from extracted prime-window positions
and residues, and the package now keeps a tracked cross-base exported catalog
live. That catalog now also has a scripted regeneration / verification path via
[`scripts/lean_generated_catalog.sh`](scripts/lean_generated_catalog.sh). The
package also now includes a conservative Hardy-Littlewood shell with standard
pair-count conventions, odd-prime local-factor bookkeeping, radical invariance
of the local-factor support, and the standard logarithmic / coverage transforms,
without treating that as a density proof. The sieve lane now also reaches the
runtime layout surface: Lean matches the odd-segment capacity/span constants and
the wheel30 linear byte/bit index formulas used by the executable sieve, and it
now also matches the runtime cross-off start/progression and adjusted odd
collection window, plus the shared byte/bit coordinates used by the writer and
reader paths, plus the exact `1 << bit` mask/update and `((byte >> bit) & 1)`
readback semantics on those same coordinates, plus a bounded single-byte array
update shell for both the odd-only and wheel30 layouts, and a generic bounded
multi-mark family on disjoint byte slots, plus a cleaner aggregated same-byte
mask route for repeated writes in one byte, now unified by grouped per-byte
plans, a tiny shared coordinate bridge, and the first runtime-facing odd-only
and wheel30 mark-family shells. The next tranches to prefer are:

1. add more explicit first-step or first-byte runtime-family lemmas only if a
   later executable agreement argument truly needs them
2. grow the exported catalog further only when a later theorem or example
   actually needs it
3. revisit the Lagrange / gravity / tidal side later only by extracting exact
   connector or residue lemmas that can be restated in standard arithmetic terms
4. rerun the full Rust, Lean, and Agda verification surfaces and clean up
   staging before a push

The Lagrange / gravity / tidal surface remains a later hardening target, but
not as a direct Lean formalization of the simulation layer. Any future Lean
engagement there should begin by extracting exact arithmetic content from the
current metaphor-oriented code, not by treating force-field, clustering, or
equilibrium heuristics as mathematical primitives.

The detailed Lean-only queue lives in
[`lean-proofs/ROADMAP.md`](lean-proofs/ROADMAP.md) and the mathematician-facing
summary of durable current signal lives in
[`lean-proofs/THEOREM_INDEX.md`](lean-proofs/THEOREM_INDEX.md).

### Why The Next Track Order Is Correct

1. Public identity and claim wording had to be aligned first so the repo stops
   sending mixed signals.
2. Remaining subtree docs are the next biggest drift source because they can
   reintroduce overclaims after the root docs were hardened.
3. Better controls are the next highest-signal research task because the
   previous tranche clarified the exact open question.
4. Drift guards come last because they should encode the stabilized surfaces,
   not the pre-hardening ones.
5. Lean should now move in small classical steps from the new symmetry base,
   because a narrow live package is more valuable than a broad aspirational one.

## Active Tranche

### Lean Formalization Consolidation

Status: `complete for the current foundation tranche`

What is now in place:
- the in-repo Lean package under [`lean-proofs/`](lean-proofs/) builds cleanly and is wired into CI
- the symmetry lane includes the abstract midpoint-obstruction theorem, concrete witnesses, modular reflection, reusable certificate wrappers, balanced-bucket support/reflection, a narrow window-certificate shell, and explicit finite certificate examples
- the exact arithmetic lane includes affine template structure, coprimality, radicals, unit residues, `ZMod` units, CRT/product decomposition, wheel-base families, negation-orbit structure, and exact connector residue filters
- the connector lane now also includes a reusable family-level residue-profile API rather than only the maintained decimal pair

Where the detailed Lean record lives:
- theorem ledger: [`lean-proofs/ROADMAP.md`](lean-proofs/ROADMAP.md)
- theorem map: [`lean-proofs/THEOREM_INDEX.md`](lean-proofs/THEOREM_INDEX.md)
- local workflow: [`lean-proofs/README.md`](lean-proofs/README.md)

Next Lean priorities:
1. add more explicit first-step or first-byte runtime-family lemmas only if they are needed later
2. grow the tracked generated certificate catalog further only when a later theorem or example actually needs it
3. revisit the Lagrange / gravity / tidal side later only through exact arithmetic extraction
4. rerun the full verification surfaces and clean staging before a push

Deferred assessment note:
- the Lagrange / gravity / tidal surface should be revisited later as a
  hardening task, but only to extract exact arithmetic statements worth
  formalizing; it should not be treated as a direct Lean target in its current
  simulation/metaphor form

### Track 6: Public Surface Identity Alignment

Status: `complete`

Why this matters:
- Root docs were already hardened, but crate docs, package metadata, CLI/example
  output, and novelty wording still reflected an older "Prime Physics Engine"
  narrative.
- Leaving those surfaces stale would make the repository look internally
  inconsistent even after the README cleanup succeeded.

Todo:
- [x] align crate-level docs and prelude wording with the active repo framing
- [x] update package metadata and stale example-count comments
- [x] rename the verification report header away from "Prime Physics Engine"
- [x] soften novelty wording so it matches the current empirical support
- [x] propagate the exact structural probe into the claim registry
- [x] replace the stale roadmap with a current tranche-based roadmap

Acceptance criteria:
- no active root/public surface presents "Prime Physics Engine" as the repo
  identity
- the active novelty/docs stack does not claim complete closure of the membrane
  density question
- the structural exact-probe result is represented in the source-of-truth docs
- `cargo test --lib`, `cargo clippy --lib -- -D warnings`, and the top-level
  example build sweep all pass

Verification:
```bash
cargo test --lib
cargo clippy --lib -- -D warnings
for f in examples/*.rs; do
  name=$(basename "$f" .rs)
  cargo build --example "$name" >/dev/null || echo "BROKEN: $name"
done
```

Assumptions:
- legacy metaphor types such as `PrimeUniverse` remain in the API for now
- archived materials may still contain older wording as historical artifacts

### Track 7: Subtree Documentation Audit

Status: `complete`

Why this matters:
- The root docs are now much stronger than several subtree READMEs and narrative
  files, which means contributors can still pick up stale framing by entering
  the repo from the wrong path.
- This is the largest remaining documentation drift source.

Todo:
- [x] audit [`agda/README.md`](agda/README.md) and
      [`agda-proofs/README.md`](agda-proofs/README.md) for claim strength and
      status accuracy
- [x] audit [`wasm-demo/README.md`](wasm-demo/README.md) and
      [`pkg/WEB_TUI_README.md`](pkg/WEB_TUI_README.md) for current usability
- [x] audit the most user-facing tool docs under `tools/`
- [x] archive or rewrite any subtree doc that fails the current scrutiny test

Acceptance criteria:
- each audited subtree doc is either current, clearly marked historical, or
  moved under [`archive/`](archive/)
- no audited subtree doc contradicts [`CLAIMS.md`](CLAIMS.md) or
  [`EVIDENCE.md`](EVIDENCE.md)
- all relative links in audited docs resolve

Verification:
```bash
rg -n "Prime Physics Engine|universal|complete theoretical closure|special membrane physics" \
  agda agda-proofs wasm-demo pkg tools
```

Assumptions:
- the goal is to harden user-facing subtree docs first, not every internal note

Completed notes:
- archived the pre-hardening versions of `agda/README.md`,
  `agda-proofs/README.md`, `wasm-demo/README.md`, and
  `pkg/WEB_TUI_README.md` under [`archive/`](archive/)
- replaced those docs with current-facing status notes that point back to
  [`CLAIMS.md`](CLAIMS.md), [`EVIDENCE.md`](EVIDENCE.md), and
  [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md)
- hardened [`tools/README.md`](tools/README.md) and
  [`tools/orthogonality/README.md`](tools/orthogonality/README.md), including
  broken-link cleanup

### Track 8: Structural Control Upgrade

Status: `complete`

Why this matters:
- The palindrome objection is now weaker, but the real unresolved question has
  become more precise: does the centered membrane scaffold outperform matched
  same-budget controls?
- This is the highest-signal research track because it can either sharpen a real
  novel pattern or collapse another attractive but unsupported explanation.

Todo:
- [x] build a control that preserves base, digit budget, and coprimality while
      breaking the centered scaffold
- [x] compare canonical membrane layouts, same-budget broken-symmetry layouts,
      and centered but non-membrane controls
- [x] write the result back into [`EVIDENCE.md`](EVIDENCE.md) and
      [`CLAIMS.md`](CLAIMS.md)

Acceptance criteria:
- a reproducible example or tool exists for the centered-scaffold control
- the resulting comparison supports one of two falsifiable outcomes:
  membrane-specific lift detected, or no lift detected
- active docs reflect the result without ambiguity

Verification:
```bash
cargo run --example membrane_scaffold_probe
```

Assumptions:
- this track is about evidence quality, not new metaphor layers

Completed notes:
- added [`examples/membrane_scaffold_probe.rs`](examples/membrane_scaffold_probe.rs)
  as an exact same-budget structural control
- compared both fixed-anchor membrane templates and broader independent-digit
  spacing families against matched asymmetric controls
- current outcome: no consistent centered-gap lift detected in the tested
  exact families; any remaining structural signal appears narrower than
  "symmetric zero-padding alone"

### Track 9: Drift Guards for Status Surfaces

Status: `complete`

Why this matters:
- The repo now has a clearer source-of-truth stack, but counts and wording can
  drift again unless they are checked automatically.
- This reduces future maintenance burden instead of adding more prose.

Todo:
- [x] add a small verification script or test for top-level example count and
      buildability
- [x] add a doc/link drift check for the active root docs
- [x] centralize status values that are duplicated across README, CLAUDE, and
      EVIDENCE

Acceptance criteria:
- at least one automated check fails when the example count or link set drifts
- duplicated status numbers no longer require manual updates in multiple places

Verification:
```bash
./tools/check_active_doc_drift.sh --build-examples
```

Assumptions:
- generated summaries are preferred over additional handwritten status prose

Completed notes:
- added [`STATUS.md`](STATUS.md) as the canonical mutable status surface for
  repo-level counts
- rewired [`README.md`](README.md), [`CLAUDE.md`](CLAUDE.md), and
  [`EVIDENCE.md`](EVIDENCE.md) to rely on [`STATUS.md`](STATUS.md) instead of
  repeating mutable counts
- added [`tools/check_active_doc_drift.sh`](tools/check_active_doc_drift.sh) to
  fail on active-doc link drift or example-count drift, with optional
  top-level example builds

### Track 10: Agda Narrative Archive Pass

Status: `complete`

Why this matters:
- `agda-proofs/` still had the largest concentration of historical narrative
  docs whose language and timelines exceeded the current audited proof state.
- Without a focused archive pass, contributors could still enter through that
  subtree and recover the old overclaim stack.

Todo:
- [x] classify `agda-proofs/` markdown into active references, rewrite targets,
      and archive candidates
- [x] archive stale sprint/session/theory narratives under
      [`archive/agda-proofs/`](archive/agda-proofs/)
- [x] replace legacy Lagrange subtree READMEs with current status notes
- [x] rewrite surviving active Agda notes to point back to
      [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md)
- [x] extend the active-doc drift script to include the surviving Agda docs

Acceptance criteria:
- stale `agda-proofs/` narratives are either archived or clearly marked
  historical
- active `agda-proofs/` entry docs no longer present universal/publication-ready
  framing that exceeds current support
- relative links in the surviving active Agda docs resolve

Verification:
```bash
./tools/check_active_doc_drift.sh
agda --safe agda-proofs/Specs/Tests.agda
agda --safe agda-proofs/Tests/DevProofs.agda
agda agda-proofs/LagrangePoints/ZeroPaddedPrimes/Asymmetry.agda
```

Assumptions:
- the goal is to harden narrative surfaces and entry docs, not to repair the
  failing Agda modules themselves

Completed notes:
- archived the stale sprint/session/theory notes under
  [`archive/agda-proofs/`](archive/agda-proofs/)
- replaced the legacy `LagrangePoints` READMEs with status-first notes and kept
  the original narratives under
  [`archive/agda-proofs/LagrangePoints/`](archive/agda-proofs/LagrangePoints/)
- rewrote the surviving active Agda notes so they point to
  [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) instead of making their own
  broad progress claims
- extended [`tools/check_active_doc_drift.sh`](tools/check_active_doc_drift.sh)
  to include the active Agda doc set

### Track 11: Agda Signal Map and Test Surface

Status: `complete`

Why this matters:
- the Agda tree still contains far more files than trustworthy proof surface,
  so contributors need a sharper map of where the real signal currently lives
- test improvements are highest leverage when they strengthen the already-clean
  spine instead of pretending the whole tree is stable

Todo:
- [x] identify the strongest clean Agda spine and the most promising repair
      targets
- [x] repair at least one tractable test/spec module that was failing due to
      API drift
- [x] add a one-command verifier for the clean Agda spine
- [x] document the signal map so later proof work targets the right modules

Acceptance criteria:
- at least one previously failing Agda test/spec module is repaired and
  verified
- the strongest clean Agda modules can be rechecked via a maintained helper
  script
- the subtree has a status-first note identifying high-signal areas and the
  next best repair targets

Verification:
```bash
cd agda-proofs
agda --safe Tests/Spec/ResidueCollapseSpec.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- this track strengthens the proof/test surface around current signal; it does
  not attempt to repair every failing theorem module

Completed notes:
- repaired [`agda-proofs/Tests/Spec/ResidueCollapseSpec.agda`](agda-proofs/Tests/Spec/ResidueCollapseSpec.agda)
  against the current stdlib remainder API
- added [`agda-proofs/scripts/verify-clean-spine.sh`](agda-proofs/scripts/verify-clean-spine.sh)
  as the maintained clean-spine verifier
- added [`agda-proofs/SIGNAL_MAP.md`](agda-proofs/SIGNAL_MAP.md) to separate the
  strongest verified areas from the assumption-heavy and exploratory zones
- updated [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) and
  [`agda-proofs/Tests/TESTING_STRATEGY.md`](agda-proofs/Tests/TESTING_STRATEGY.md)
  to reflect the repaired spec module

### Track 12: Residue Framework Surface Repair

Status: `complete`

Why this matters:
- the residue framework is one of the clearest formal spines in the repo, so
  leaving its top-level files broken made the whole Agda layer harder to trust
- a stable postulated surface is materially better than a failing proof sketch:
  it gives downstream modules something honest and rerunnable to build on

Todo:
- [x] repair [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda)
      from its old parse failure through current interface drift
- [x] restore the immediate dependencies
      [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda) and
      [`agda-proofs/Core/ResidueCollapse.agda`](agda-proofs/Core/ResidueCollapse.agda)
      as current-syntax postulated interfaces
- [x] preserve superseded proof sketches under `archive/` where replacement was
      cleaner than incremental patching
- [x] update the Agda status surfaces to classify these files honestly

Acceptance criteria:
- `agda Core/ResidueClasses.agda` passes
- `agda Core/Radical.agda` and `agda Core/ResidueCollapse.agda` pass
- the repaired residue framework is reflected in
  [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) and
  [`agda-proofs/SIGNAL_MAP.md`](agda-proofs/SIGNAL_MAP.md)

Verification:
```bash
cd agda-proofs
agda Core/ResidueClassesComplete.agda
agda Core/Radical.agda
agda Core/ResidueCollapse.agda
agda Core/ResidueClasses.agda
agda Tests/Spec/ResidueClassesRingSpec.agda
```

Assumptions:
- this track intentionally stabilizes interfaces before recovering every
  constructive proof inside them
- the live `Core/ResidueCollapse.agda` surface now keeps the executable
  `distinct-residues` core and a smaller set of explicit postulates, with the
  pre-repair sketch archived for later proof recovery

Completed notes:
- repaired [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda)
  against the current residue-ring interface and current stdlib filter/mod API
- hardened [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda)
  into a current-syntax postulated interface instead of a broken hole-filled
  script
- archived the previous collapse sketch at
  [`archive/agda-proofs/Core/ResidueCollapse_pre_interface_repair.agda`](archive/agda-proofs/Core/ResidueCollapse_pre_interface_repair.agda)
  and replaced the live
  [`agda-proofs/Core/ResidueCollapse.agda`](agda-proofs/Core/ResidueCollapse.agda)
  surface with a smaller stable interface
- updated [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) and
  [`agda-proofs/SIGNAL_MAP.md`](agda-proofs/SIGNAL_MAP.md) to reflect the new
  residue framework status

### Track 13: Residue Wheel-Bridge Recovery

Status: `complete`

Why this matters:
- the repaired residue framework still had a few small but important places
  where the code was better than the proofs: `valid-prime-residues` already
  filtered by coprimality, but the framework still postulated that fact
- recovering one bridge constructively is the right next step because it proves
  the residue repair can now support theorem-sized progress, not just parsing

Todo:
- [x] replace one local residue-framework postulate with a constructive proof
- [x] verify the repaired theorem against the current residue stack
- [x] queue the next two follow-up tracks behind it

Acceptance criteria:
- [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda)
  loses at least one local postulate without regressing compilation
- the repaired bridge is reflected in the active Agda status notes
- the next two queued follow-ups are explicit in the roadmap

Verification:
```bash
cd agda-proofs
agda Core/ResidueClasses.agda
agda Tests/Spec/ResidueClassesRingSpec.agda
cd ..
./agda-proofs/scripts/verify-clean-spine.sh
./tools/check_active_doc_drift.sh
```

Assumptions:
- this track targets a real bridge with immediate framework value, not the full
  unit/coprime constructive proof effort

Completed notes:
- replaced the local `wheel-coprime-lemma` postulate in
  [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda)
  with a constructive proof derived from filtered-list membership
- reduced the local postulate count of `Core/ResidueClasses.agda` from 6 to 5
- updated [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) and
  [`agda-proofs/SIGNAL_MAP.md`](agda-proofs/SIGNAL_MAP.md) to record the new
  bridge and reprioritize the next residue-facing work

### Track 14: Base-10 Residue Filter Activation

Status: `complete`

Why this matters:
- [`agda-proofs/Tests/Spec/Base10ResidueFilterSpec.agda`](agda-proofs/Tests/Spec/Base10ResidueFilterSpec.agda)
  is one of the best theorem-on-concrete-examples regression patterns in the
  tree, but it is still blocked by theorem drift instead of mathematical value
- activating it would turn the repaired residue stack into a more public,
  example-driven test surface

Todo:
- [x] repair the underlying Base10 residue theorem module and its imports
- [x] bring `Base10ResidueFilterSpec.agda` online with current paths and proofs
- [x] classify the theorem surface honestly around the maintained coprimality filter

Acceptance criteria:
- `agda Tests/Spec/Base10ResidueFilterSpec.agda` passes
- the active Agda docs describe the spec as live rather than aspirational

Verification:
```bash
cd agda-proofs
agda Examples/Base10ResidueFilter.agda
agda Tests/Spec/Base10ResidueFilterSpec.agda
```

Assumptions:
- the strongest clean version of this track may formalize the coprimality filter
  first and leave the explicit `{1,3,7,9}` equivalence mostly at the example level

Completed notes:
- replaced the old hole-filled theorem sketch in
  [`agda-proofs/Examples/Base10ResidueFilter.agda`](agda-proofs/Examples/Base10ResidueFilter.agda)
  with a smaller safe theorem module built on `prime⇒coprime`
- made the executable filter check `gcd n 10 ≟ 1`, which matches the theorem
  path directly while preserving the classical last-digit story through
  concrete examples
- rewired
  [`agda-proofs/Tests/Spec/Base10ResidueFilterSpec.agda`](agda-proofs/Tests/Spec/Base10ResidueFilterSpec.agda)
  to use real prime witnesses from `isPrime?` and stdlib boolean-order reflection
- promoted [`agda-proofs/Core/Primality.agda`](agda-proofs/Core/Primality.agda)
  to `--safe`, which keeps the new example/spec pair in the clean safe spine

### Track 15: Agda Boundary Tightening

Status: `complete`

Why this matters:
- the clean Agda spine is now strong enough that the remaining ambiguity is
  mostly documentary: some clean modules depend on postulated layers, and that
  should stay visible
- sharper boundaries make the novel signal more trustworthy, not smaller

Todo:
- [x] audit the active Agda docs for places where “clean” and “clean-local atop
      postulated foundations” are still blurred
- [x] align `agda-proofs/STATUS.md`, `agda-proofs/SIGNAL_MAP.md`, and active
      README references on that distinction
- [x] extend the verifier/docs with a lightweight guard against count drift

Acceptance criteria:
- active Agda status docs use consistent language for clean-local vs
  postulated-foundation modules
- no active doc implies a stronger constructive guarantee than the current tree provides

Verification:
```bash
./tools/check_active_doc_drift.sh
```

Assumptions:
- this track is about terminology and visibility, not about changing theorem code

Completed notes:
- normalized the active Agda wording around `clean-local` versus `with local
  postulates` in [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md)
- removed hand-synced Agda counts from high-level docs in
  [`README.md`](README.md), [`CLAUDE.md`](CLAUDE.md), and
  [`agda-proofs/README.md`](agda-proofs/README.md), leaving numbers only in the
  canonical status surfaces
- updated [`STATUS.md`](STATUS.md) to summarize the current Agda counts using
  the same terminology as [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md)
- extended [`tools/check_active_doc_drift.sh`](tools/check_active_doc_drift.sh)
  to compare root and Agda status counts and to fail if high-level docs start
  hand-syncing Agda counts again

### Track 16: Residue Constructive Bridge Continuation

Status: `complete`

Why this matters:
- the residue framework now has a stable interface and one recovered
  constructive bridge, so the next gain is to turn another central postulated
  residue theorem into a real proof
- this is the highest-signal next step for strengthening the mathematical core
  rather than only its presentation

Todo:
- [x] choose one remaining residue-layer bridge with direct framework value
- [x] recover it constructively without regressing the repaired residue stack
- [x] record the result in the Agda status and signal surfaces

Acceptance criteria:
- at least one residue-layer local postulate is replaced by a constructive proof
- the repaired module and its dependent checks still compile

Verification:
```bash
cd agda-proofs
agda Core/ResidueClasses.agda
agda Core/ResidueClassesComplete.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prioritize framework-value bridges over ornamental theorem recovery

Completed notes:
- recovered the forward unit/coprime bridge in
  [`agda-proofs/Core/ResidueClassesComplete.agda`](agda-proofs/Core/ResidueClassesComplete.agda)
  constructively from the `IsUnit` witness, the division algorithm, and gcd/divisibility
  lemmas
- removed the false `coprime-1⇒m>1` helper and made `units-are-coprime` take
  an explicit `m > 1` witness instead of hiding that assumption
- updated
  [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda)
  to pass the modulus witness honestly into the residue framework
- kept the remaining open part narrow: the converse `coprime -> unit` direction
  and the algebraic ring-law block are still postulated

### Track 17: Residue Unit Converse Recovery

Status: `complete`

Why this matters:
- the residue foundation now proves `unit -> coprime` constructively, so the
  most valuable remaining gap is the converse direction
- closing that gap would turn the `units ↔ coprime` interface from a mixed
  theorem/postulate surface into a genuinely complete theorem for `m > 1`

Todo:
- [x] derive the converse from the stdlib Bézout identity without reintroducing
      the old hidden-assumption bug
- [x] keep the `m > 1` requirement explicit in the theorem surface
- [x] add or update a small spec surface around the completed equivalence

Acceptance criteria:
- `coprime-→-unit` is no longer postulated in
  [`agda-proofs/Core/ResidueClassesComplete.agda`](agda-proofs/Core/ResidueClassesComplete.agda)
- the residue framework and downstream checks still compile

Verification:
```bash
cd agda-proofs
agda Core/ResidueClassesComplete.agda
agda Core/ResidueClasses.agda
agda Tests/Spec/ResidueClassesRingSpec.agda
agda Tests/Spec/ResidueClassesUnitsSpec.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer a narrower truthful theorem over recovering a converse by reintroducing
  hidden or false assumptions

Completed notes:
- reconstructed `coprime-→-unit` constructively from the stdlib Bézout identity
  in [`agda-proofs/Core/ResidueClassesComplete.agda`](agda-proofs/Core/ResidueClassesComplete.agda)
- kept the `m > 1` requirement explicit and handled the negative Bézout branch
  through the canonical `(m - 1)` witness instead of hidden assumptions
- reduced `Core/ResidueClassesComplete.agda` from 2 local postulate blocks to 1;
  the remaining open surface is now the algebraic ring-law block
- added
  [`agda-proofs/Tests/Spec/ResidueClassesUnitsSpec.agda`](agda-proofs/Tests/Spec/ResidueClassesUnitsSpec.agda)
  as a concrete regression surface for the repaired equivalence

### Track 18: Residue Ring-Law Reduction

Status: `complete`

Why this matters:
- the residue unit bridge is now fully constructive, so the main remaining
  postulated core inside the maintained residue API is the ring-law block
- reducing that block would turn more of the current residue shell into a
  genuinely constructive algebraic foundation

Todo:
- [x] recover at least one currently postulated ring law constructively in
      [`agda-proofs/Core/ResidueClassesComplete.agda`](agda-proofs/Core/ResidueClassesComplete.agda)
- [x] keep `Core/ResidueClasses.agda` and the residue specs compiling against the
      tightened ring interface
- [x] update the Agda signal map to reflect which algebraic laws remain open

Acceptance criteria:
- the file-local postulate surface in
  [`agda-proofs/Core/ResidueClassesComplete.agda`](agda-proofs/Core/ResidueClassesComplete.agda)
  is smaller than it is after Track 17
- residue framework modules and residue specs still compile

Verification:
```bash
cd agda-proofs
agda Core/ResidueClassesComplete.agda
agda Core/ResidueClasses.agda
agda Tests/Spec/ResidueClassesRingSpec.agda
agda Tests/Spec/ResidueClassesUnitsSpec.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer recovering a narrow honest algebraic law over rebuilding the whole ring
  package in one pass

Completed notes:
- recovered addition/multiplication associativity and commutativity
  constructively in
  [`agda-proofs/Core/ResidueClassesComplete.agda`](agda-proofs/Core/ResidueClassesComplete.agda)
- recovered additive and multiplicative identities, additive inverses, and left
  distributivity constructively against the maintained canonical residue
  interface
- introduced an explicit `modulo` helper so the repaired proofs carry their
  `NonZero` witness directly instead of relying on fragile instance search
- kept the file-local `postulate` block count at 1, but narrowed that final
  postulate to the bundled `residue-ring` witness instead of the individual
  algebraic laws
- verified that
  [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda),
  [`agda-proofs/Tests/Spec/ResidueClassesRingSpec.agda`](agda-proofs/Tests/Spec/ResidueClassesRingSpec.agda),
  and
  [`agda-proofs/Tests/Spec/ResidueClassesUnitsSpec.agda`](agda-proofs/Tests/Spec/ResidueClassesUnitsSpec.agda)
  still compile

### Track 19: Residue Ring Witness Packaging

Status: `complete`

Why this matters:
- the basic residue-ring laws are now constructive, so the remaining gap is no
  longer the algebra itself but the packaged `IsCommutativeRing` witness
- closing that packaging gap would make the residue foundation more reusable and
  more honestly complete for downstream theorem code

Todo:
- [x] recover the remaining equivalence/congruence helpers needed by
      `IsCommutativeRing`
- [x] replace the postulated `residue-ring` witness with a constructive record,
      or narrow the final postulate further if one helper still resists
- [x] keep residue framework modules and regression specs compiling

Acceptance criteria:
- the final local postulate in
  [`agda-proofs/Core/ResidueClassesComplete.agda`](agda-proofs/Core/ResidueClassesComplete.agda)
  is either eliminated or reduced to something smaller than the full
  `residue-ring` bundle
- residue framework modules and residue specs still compile

Verification:
```bash
cd agda-proofs
agda Core/ResidueClassesComplete.agda
agda Core/ResidueClasses.agda
agda Tests/Spec/ResidueClassesRingSpec.agda
agda Tests/Spec/ResidueClassesUnitsSpec.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer finishing the maintained residue API before expanding into broader
  theorem families that depend on it

Completed notes:
- rebuilt the exported `residue-ring` witness constructively in
  [`agda-proofs/Core/ResidueClassesComplete.agda`](agda-proofs/Core/ResidueClassesComplete.agda)
- recovered the remaining residue equality / congruence helpers needed by
  `IsCommutativeRing`
- derived right distributivity from the constructive left-distributive law plus
  multiplication commutativity
- promoted
  [`agda-proofs/Core/ResidueClassesComplete.agda`](agda-proofs/Core/ResidueClassesComplete.agda),
  [`agda-proofs/Tests/Spec/ResidueClassesRingSpec.agda`](agda-proofs/Tests/Spec/ResidueClassesRingSpec.agda),
  and
  [`agda-proofs/Tests/Spec/ResidueClassesUnitsSpec.agda`](agda-proofs/Tests/Spec/ResidueClassesUnitsSpec.agda)
  to `--safe`
- expanded the maintained clean-safe verifier from 23 modules to 26 modules

### Track 20: Residue Collapse Bridge Recovery

Status: `complete`

Why this matters:
- the residue ring foundation is now constructive and safe, so the next real
  bottleneck in the residue framework is the postulated collapse/radical side
- recovering that bridge would let `Core/ResidueClasses.agda` lean on the new
  constructive ring base instead of stopping at an interface shell

Todo:
- [x] recover at least one currently postulated collapse/radical bridge
      constructively in `Core/ResidueCollapse.agda` or `Core/Radical.agda`
- [x] keep `Core/ResidueClasses.agda` compiling against the tightened residue
      framework
- [x] update the Agda signal map/status docs to reflect the narrower remaining
      residue postulates

Acceptance criteria:
- at least one local postulate is removed from
  [`agda-proofs/Core/ResidueCollapse.agda`](agda-proofs/Core/ResidueCollapse.agda)
  or [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda)
- `Core/ResidueClasses.agda` and the maintained residue specs still compile

Verification:
```bash
cd agda-proofs
agda Core/Radical.agda
agda Core/ResidueCollapse.agda
agda Core/ResidueClasses.agda
agda Tests/Spec/ResidueCollapseSpec.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer the smallest high-leverage bridge over trying to eliminate all
  radical/collapse postulates in one pass

Completed notes:
- recovered the weak collapse/filtering bridge constructively in
  [`agda-proofs/Core/ResidueCollapse.agda`](agda-proofs/Core/ResidueCollapse.agda)
- recovered the canonical `base6` and `base10` collapse-count examples by
  normalization in the same file
- corrected the residue-framework framing in
  [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda):
  the active signal is now described as frequency regularity rather than
  “fewer residue classes appear”
- replaced the framework-level collapse comparison in
  [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda)
  with a constructive inhabited witness
- kept the file-local `postulate` count for
  [`agda-proofs/Core/ResidueCollapse.agda`](agda-proofs/Core/ResidueCollapse.agda)
  at 1, but narrowed that remaining postulated surface to the general
  `all-residues-appear` theorem

### Track 21: Radical Counterexample Recovery

Status: `complete`

Why this matters:
- the residue ring and easy collapse bridges are now in good shape, so the next
  cheap high-signal gain is the radical layer’s standard-language theorem surface
- recovering explicit radical-vs-totient counterexamples would strengthen the
  repo’s use of standard number-theory terminology without requiring a full
  constructive factorization development

Todo:
- [x] recover at least one theorem-level radical counterexample constructively in
      `Core/Radical.agda`
- [x] keep `Core/Radical.agda` and `Core/ResidueClasses.agda` compiling
- [x] update the Agda signal/status docs to reflect the narrowed radical surface

Acceptance criteria:
- at least one currently postulated theorem in
  [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda) is replaced
  by a constructive term
- `Core/Radical.agda` and `Core/ResidueClasses.agda` still compile

Verification:
```bash
cd agda-proofs
agda Core/Radical.agda
agda Core/ResidueClasses.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prioritize narrow theorem recovery built on existing concrete examples over a
  full factorization rewrite

Completed notes:
- recovered `radical-not-totient` constructively in
  [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda) from the
  existing `rad-of-12` and `totient-of-12` witnesses
- recovered the exported
  [`rad-vs-totient-example`](agda-proofs/Core/Radical.agda) from the same
  `n = 12` counterexample instead of leaving it postulated separately
- reduced the file-local `postulate` line count in
  [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda) from 13 to
  11 without pretending the broader factorization layer is finished
- updated [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) and
  [`agda-proofs/SIGNAL_MAP.md`](agda-proofs/SIGNAL_MAP.md) to reflect the
  narrower radical theorem surface

### Track 22: Clean vs Postulated Boundary Guards

Status: `complete`

Why this matters:
- the maintained Agda spine is now strong enough that the docs need a sharper
  explanation of which clean-local modules sit atop postulated layers
- without that boundary guard, later readers can overread the cleaned-up residue
  stack as if every adjacent theorem layer were already fully machine-checked

Todo:
- [x] audit the live Agda-facing docs for places where `clean-local` and
      “fully machine-checked” are still blurred
- [x] tighten the notes for clean-local modules that depend on postulated
      foundations, especially around the radical/collapse/framework layer
- [x] extend the drift guard if needed so the high-level docs do not silently
      flatten that boundary again

Acceptance criteria:
- the active Agda-facing docs distinguish clean-local modules from modules that
  are only clean on top of postulated foundations
- the drift guard fails if those boundary notes disappear from the maintained
  surfaces

Verification:
```bash
./tools/check_active_doc_drift.sh
cd agda-proofs
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer boundary clarification and drift-guarding over another immediate proof
  push unless the audit exposes a very small constructive follow-up

Completed notes:
- identified the current clean-local boundary cases precisely:
  [`agda-proofs/Theorems/ElbowsFromCSV.agda`](agda-proofs/Theorems/ElbowsFromCSV.agda)
  and
  [`agda-proofs/Theorems/GlobalElbowFacts.agda`](agda-proofs/Theorems/GlobalElbowFacts.agda)
  both import postulated
  [`agda-proofs/Theorems/ElbowEvents.agda`](agda-proofs/Theorems/ElbowEvents.agda)
- tightened [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md),
  [`agda-proofs/README.md`](agda-proofs/README.md), and
  [`agda-proofs/SIGNAL_MAP.md`](agda-proofs/SIGNAL_MAP.md) so the clean-local
  boundary is concrete rather than generic
- repaired stale wording in
  [`agda-proofs/Tests/TESTING_STRATEGY.md`](agda-proofs/Tests/TESTING_STRATEGY.md)
  so the residue-ring test surface matches the now-safe
  [`agda-proofs/Core/ResidueClassesComplete.agda`](agda-proofs/Core/ResidueClassesComplete.agda)
- extended [`tools/check_active_doc_drift.sh`](tools/check_active_doc_drift.sh)
  to fail if the current elbow boundary notes disappear

### Track 23: Radical Theorem-Surface Continuation

Status: `complete`

Why this matters:
- the clean/postulated boundary is now explicit, so the next highest-signal
  gain is another honest constructive win in the radical layer itself
- `Core/Radical.agda` now has a real foothold with `rad ≠ φ`; continuing there
  strengthens the repo’s standard number-theory spine without needing a full
  factorization rewrite

Todo:
- [x] recover at least one additional nontrivial radical theorem or example
      constructively in `Core/Radical.agda`
- [x] keep `Core/Radical.agda` and `Core/ResidueClasses.agda` compiling
- [x] update the Agda status/signal docs to reflect the narrower radical gap

Acceptance criteria:
- at least one additional theorem or example currently postulated in
  [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda) is replaced
  by a constructive term
- `Core/Radical.agda` and `Core/ResidueClasses.agda` still compile

Verification:
```bash
cd agda-proofs
agda Core/Radical.agda
agda Core/ResidueClasses.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer narrow theorem/example recovery built on existing concrete witnesses
  over trying to mechanize full factorization in one pass

Completed notes:
- recovered `rad-6`, `rad-10`, `rad-18`, `rad-20`, and `rad-60`
  constructively in [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda)
  from the existing multiplicativity theorem plus base/example witnesses
- kept [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda) and
  [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda)
  compiling
- updated [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) and
  [`agda-proofs/SIGNAL_MAP.md`](agda-proofs/SIGNAL_MAP.md) to reflect the
  narrower radical example layer
- left the file-local `postulate` count at 11 because the remaining base
  witnesses still share the same coarse postulate block

### Track 24: Collapse Coverage Refinement

Status: `complete`

Why this matters:
- the collapse layer is now down to one honest open theorem, so it is the next
  sharpest place to either recover a narrower constructive claim or clarify the
  exact remaining postulated surface
- this is a good fit after Track 23 because the docs are aligned and the
  radical layer just received another honest reduction

Todo:
- [x] inspect `Core/ResidueCollapse.agda` for a narrower constructive coverage
      theorem or a clean split of the remaining postulate
- [x] keep `Core/ResidueCollapse.agda` and `Core/ResidueClasses.agda` compiling
- [x] update the Agda status/signal docs to reflect the refined collapse gap

Acceptance criteria:
- either one additional collapse claim is recovered constructively, or the
  remaining postulated surface in
  [`agda-proofs/Core/ResidueCollapse.agda`](agda-proofs/Core/ResidueCollapse.agda)
  is split into smaller honest pieces with matching docs
- `Core/ResidueCollapse.agda` and
  [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda)
  still compile

Verification:
```bash
cd agda-proofs
agda Core/ResidueCollapse.agda
agda Core/ResidueClasses.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer a narrow honest refinement over a heroic attempt at a fully general
  collapse theorem in one pass

Completed notes:
- split the remaining collapse coverage gap in
  [`agda-proofs/Core/ResidueCollapse.agda`](agda-proofs/Core/ResidueCollapse.agda)
  into `threshold-covers-all-residues` and
  `coverage-stabilizes-above-threshold`
- rebuilt the public `all-residues-appear` theorem from those two narrower open
  claims instead of leaving one broad umbrella postulate
- kept [`agda-proofs/Core/ResidueCollapse.agda`](agda-proofs/Core/ResidueCollapse.agda)
  and [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda)
  compiling
- updated [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) and
  [`agda-proofs/SIGNAL_MAP.md`](agda-proofs/SIGNAL_MAP.md) to reflect the
  sharper collapse gap

### Track 25: Radical Proof-Core Continuation

Status: `complete`

Why this matters:
- after the example-surface gains in Tracks 21 and 23, the next highest-signal
  move is to keep narrowing the still-postulated radical proof core
- this continues to strengthen the standard number-theory backbone of the
  residue framework

Todo:
- [x] recover one additional radical theorem or bridge constructively, or split
      one broad radical postulate into narrower honest pieces
- [x] keep `Core/Radical.agda` and `Core/ResidueClasses.agda` compiling
- [x] update the Agda status/signal docs to reflect the narrower radical core

Acceptance criteria:
- at least one additional theorem-level claim in
  [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda) is either
  recovered constructively or refactored into a sharper honest interface
- `Core/Radical.agda` and
  [`agda-proofs/Core/ResidueClasses.agda`](agda-proofs/Core/ResidueClasses.agda)
  still compile

Verification:
```bash
cd agda-proofs
agda Core/Radical.agda
agda Core/ResidueClasses.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer narrow theorem recovery or interface sharpening over trying to finish
  the whole factorization theory in one pass

Completed notes:
- moved `rad(12)` and `rad(30)` out of the remaining radical base-example
  postulate surface in
  [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda)
- recovered `rad-of-12` from `radical-multiplicative 4 3` and `rad-of-30` from
  `radical-multiplicative 6 5`
- reduced the file-local `postulate` count in
  [`agda-proofs/Core/Radical.agda`](agda-proofs/Core/Radical.agda) from 11 to
  10
- updated [`agda-proofs/STATUS.md`](agda-proofs/STATUS.md) and
  [`agda-proofs/SIGNAL_MAP.md`](agda-proofs/SIGNAL_MAP.md) to reflect the
  narrower radical proof core

### Track 26: Elbow Event Source Recovery

Status: `complete`

Why this matters:
- the current clean-local boundary cases
  [`agda-proofs/Theorems/ElbowsFromCSV.agda`](agda-proofs/Theorems/ElbowsFromCSV.agda)
  and
  [`agda-proofs/Theorems/GlobalElbowFacts.agda`](agda-proofs/Theorems/GlobalElbowFacts.agda)
  both depend on postulated [`agda-proofs/Theorems/ElbowEvents.agda`](agda-proofs/Theorems/ElbowEvents.agda)
- recovering even one event property there would convert part of the empirical
  data spine from “clean-local over assumptions” to a stronger verified layer

Todo:
- [x] inspect `Theorems/ElbowEvents.agda` for one small recoverable event
      property or a cleaner split of the remaining postulates
- [x] keep `Theorems/ElbowEvents.agda`,
      `Theorems/ElbowsFromCSV.agda`, and `Theorems/GlobalElbowFacts.agda`
      compiling
- [x] update the boundary notes if the elbow dependency surface changes

Acceptance criteria:
- at least one event-level claim in
  [`agda-proofs/Theorems/ElbowEvents.agda`](agda-proofs/Theorems/ElbowEvents.agda)
  is recovered constructively or the remaining event assumptions are split more
  honestly
- the elbow ingestion/derived-facts modules still compile

Verification:
```bash
cd agda-proofs
agda Theorems/ElbowEvents.agda
agda Theorems/ElbowsFromCSV.agda
agda Theorems/GlobalElbowFacts.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer strengthening the event source over adding more derived-theorem prose
  on top of the current postulated layer

Completed notes:
- verified that [`agda-proofs/Theorems/ElbowEvents.agda`](agda-proofs/Theorems/ElbowEvents.agda),
  [`agda-proofs/Theorems/ElbowsFromCSV.agda`](agda-proofs/Theorems/ElbowsFromCSV.agda),
  and [`agda-proofs/Theorems/GlobalElbowFacts.agda`](agda-proofs/Theorems/GlobalElbowFacts.agda)
  all type-check cleanly from the `agda-proofs/` workspace
- confirmed that [`agda-proofs/Theorems/ElbowEvents.agda`](agda-proofs/Theorems/ElbowEvents.agda)
  has no live `postulate` declarations; the old status row was stale
- moved `Theorems/ElbowEvents.agda` from the local-postulates category into the
  clean-local set and added it to the maintained clean-spine verifier
- updated the active Agda docs and drift guard to reflect the new truth:
  there are currently no maintained clean-local boundary cases

### Track 27: Totient-Density Interface Narrowing

Status: `complete`

Why this matters:
- with the elbow event source now clean end-to-end, one of the largest
  remaining standard-term overhangs is
  [`agda-proofs/Theorems/TotientDensity.agda`](agda-proofs/Theorems/TotientDensity.agda)
- narrowing that interface would reduce a large assumption-heavy surface
  without requiring a full analytic-number-theory mechanization in one pass

Todo:
- [x] inspect `Theorems/TotientDensity.agda` for one smaller recoverable claim
      or a sharper split of its remaining postulates
- [x] keep `Theorems/TotientDensity.agda` and any immediate consumers compiling
- [x] update the Agda status/signal docs to reflect the narrowed totient-density gap

Acceptance criteria:
- at least one claim in
  [`agda-proofs/Theorems/TotientDensity.agda`](agda-proofs/Theorems/TotientDensity.agda)
  is recovered constructively or its remaining assumptions are split more
  honestly
- the touched totient-density surface still compiles

Verification:
```bash
cd agda-proofs
agda Theorems/TotientDensity.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer interface sharpening and narrow theorem recovery over trying to finish
  the entire totient-density formalization in one pass

Completed notes:
- aligned [`agda-proofs/Theorems/RationalStatistics.agda`](agda-proofs/Theorems/RationalStatistics.agda)
  with `--without-K` so
  [`agda-proofs/Theorems/TotientDensity.agda`](agda-proofs/Theorems/TotientDensity.agda)
  can reuse the shared rational layer directly
- replaced duplicated local postulates in
  [`agda-proofs/Theorems/TotientDensity.agda`](agda-proofs/Theorems/TotientDensity.agda)
  for rationals, `Prime`, `gcd`, and decidable coprimality with live imports
  plus small definitional wrappers
- recovered `totient-1` constructively and reduced the file-local `postulate`
  line count from 39 to 22
- verified that the narrowed totient-density surface compiles and that the
  maintained 27-module clean spine still passes unchanged

### Track 28: Analytic Shell Parse Triage

Status: `complete`

Why this matters:
- the strongest remaining blockers above the repaired totient-density layer are
  parse-era failures in
  [`agda-proofs/Theorems/HardyLittlewoodSingularSeries.agda`](agda-proofs/Theorems/HardyLittlewoodSingularSeries.agda),
  [`agda-proofs/Theorems/ConstellationCriticalLine.agda`](agda-proofs/Theorems/ConstellationCriticalLine.agda),
  and
  [`agda-proofs/Theorems/CoordinateConstellationScaling.agda`](agda-proofs/Theorems/CoordinateConstellationScaling.agda)
- until those files at least parse, the repo cannot cleanly separate “interesting
  steelman shell” from “stale syntax drift” in the analytic theorem layer

Todo:
- [x] inspect the first parse failure in each analytic-shell file and classify
      whether it is notation drift, import drift, or deeper proof debt
- [x] recover at least one file from parse failure to compilable scaffold status
- [x] update the Agda status/signal docs to distinguish parse-era failures from
      theorem-level open surfaces

Acceptance criteria:
- at least one of the three analytic-shell files above compiles again, or their
  failure mode is split into narrower honest blockers with matching docs
- the totient-density surface and clean-spine verifier still pass afterward

Verification:
```bash
cd agda-proofs
agda Theorems/ConstellationCriticalLine.agda
agda Theorems/CoordinateConstellationScaling.agda
agda Theorems/HardyLittlewoodSingularSeries.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer syntax/import recovery and honest interface narrowing over trying to
  finish the entire analytic-number-theory shell in one pass

Completed notes:
- inspected the first parse failures in
  [`agda-proofs/Theorems/ConstellationCriticalLine.agda`](agda-proofs/Theorems/ConstellationCriticalLine.agda),
  [`agda-proofs/Theorems/CoordinateConstellationScaling.agda`](agda-proofs/Theorems/CoordinateConstellationScaling.agda),
  and
  [`agda-proofs/Theorems/HardyLittlewoodSingularSeries.agda`](agda-proofs/Theorems/HardyLittlewoodSingularSeries.agda)
  and confirmed that the failures are not all the same: the coordinate-scaling
  file was primarily notation drift, while the other two still mix parse-era
  syntax with deeper shell debt
- recovered
  [`agda-proofs/Theorems/CoordinateConstellationScaling.agda`](agda-proofs/Theorems/CoordinateConstellationScaling.agda)
  into a current-syntax empirical scaffold that preserves the key observed
  ratios, base-14 outer-constraint shell, and modified-scaling vocabulary
- verified that the maintained 27-module clean spine still passes unchanged

### Track 29: Critical-Line Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Theorems/ConstellationCriticalLine.agda`](agda-proofs/Theorems/ConstellationCriticalLine.agda)
  now looks like the next dependency bottleneck above the repaired totient layer
  and recovered coordinate-scaling shell
- recovering it into a compilable scaffold would let us classify how much of the
  `HardyLittlewoodSingularSeries` failure is genuinely new math versus simple
  downstream drift

Todo:
- [x] replace the stale existential/numeric shorthand in
      `Theorems/ConstellationCriticalLine.agda` with current-syntax scaffold
      declarations
- [x] keep the file honest about what is empirical, what is classical, and what
      remains speculative
- [x] update the Agda status/signal docs if the file moves out of the failing
      set

Acceptance criteria:
- `agda Theorems/ConstellationCriticalLine.agda` passes
- the file no longer relies on parse-era syntax as its primary blocker
- `Theorems/CoordinateConstellationScaling.agda` and the clean-spine verifier
  still pass afterward

Verification:
```bash
cd agda-proofs
agda Theorems/ConstellationCriticalLine.agda
agda Theorems/CoordinateConstellationScaling.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer recovering a narrower current-syntax shell over trying to formalize the
  critical-line heuristic in one step

Completed notes:
- replaced the stale critical-line script with a smaller current-syntax shell
  rooted in the repaired coordinate-scaling and totient-density layers
- made the signed `-1/2` story explicit without pretending the active rational
  helper layer already supports a full signed arithmetic development
- recovered
  [`agda-proofs/Theorems/ConstellationCriticalLine.agda`](agda-proofs/Theorems/ConstellationCriticalLine.agda)
  from the failing set into the local-postulates category

### Track 30: Hardy-Littlewood Shell Recovery

Status: `complete`

Why this matters:
- once the critical-line shell compiles, the next dependency-correct blocker is
  [`agda-proofs/Theorems/HardyLittlewoodSingularSeries.agda`](agda-proofs/Theorems/HardyLittlewoodSingularSeries.agda)
- recovering that file gives the repo a compilable singular-series vocabulary
  layer instead of leaving one of its most visible analytic shells trapped in
  parse drift

Todo:
- [x] replace the stale local-factor script with a current-syntax scaffold that
      preserves the constellation / local obstruction / Euler-product language
- [x] keep the membrane-prediction and pair-correlation bridge explicitly open
      rather than overstated
- [x] update the Agda status/signal docs if the file moves out of the failing
      set

Acceptance criteria:
- `agda Theorems/HardyLittlewoodSingularSeries.agda` passes
- the file no longer relies on parse-era syntax as its primary blocker
- `Theorems/ConstellationCriticalLine.agda` and the clean-spine verifier still
  pass afterward

Verification:
```bash
cd agda-proofs
agda Theorems/HardyLittlewoodSingularSeries.agda
agda Theorems/ConstellationCriticalLine.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer a current-syntax singular-series shell over pretending the full HL +
  pair-correlation bridge is already formalized

Completed notes:
- replaced the stale HL script with a smaller compilable shell that keeps the
  prime-constellation vocabulary, singular-series framing, and totient-style
  Euler-product intuition visible
- tied the membrane-prediction shell to the recovered critical-line layer
  instead of the broken legacy power-law dependency
- recovered
  [`agda-proofs/Theorems/HardyLittlewoodSingularSeries.agda`](agda-proofs/Theorems/HardyLittlewoodSingularSeries.agda)
  from the failing set into the local-postulates category

### Track 31: Gap-Divisibility / Hexagonal Unification Recovery

Status: `complete`

Why this matters:
- the analytic shells now compile again, so the next dependency bottleneck is
  the `GapDivisibility -> HexagonalUnification` chain on the empirical-statistics
  side
- recovering that chain would strengthen the theorem layer that sits directly on
  top of the clean `RationalStatistics` spine

Todo:
- [x] inspect the `GapDivisibility.agda` failure around `base18-enhanced` and
      decide whether it is a missing export, stale rename, or a broader example
      drift problem
- [x] recover `GapDivisibility.agda` if the fix is narrow, then immediately
      retry `HexagonalUnification.agda`
- [x] update the Agda status/signal docs if either file moves out of the
      failing set

Acceptance criteria:
- `GapDivisibility.agda` passes, or its remaining blocker is split into a
  narrower honest interface
- if `GapDivisibility.agda` passes, `HexagonalUnification.agda` is retried in
  the same tranche
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Theorems/GapDivisibility.agda
agda Theorems/HexagonalUnification.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer repairing the dependency chain in order rather than touching
  `HexagonalUnification.agda` first

Completed notes:
- repaired the missing `base18-enhanced` witness and stale corrected-percentage
  drift in [`agda-proofs/Theorems/GapDivisibility.agda`](agda-proofs/Theorems/GapDivisibility.agda)
- followed the chain one layer deeper and repaired
  [`agda-proofs/Theorems/CoordinateEigenspace.agda`](agda-proofs/Theorems/CoordinateEigenspace.agda),
  whose local scope/import/termination drift had become the next blocker for
  the hexagonal theorem surface
- replaced
  [`agda-proofs/Theorems/HexagonalUnification.agda`](agda-proofs/Theorems/HexagonalUnification.agda)
  with a smaller current-syntax synthesis shell rooted in the live base 7/14/18
  witnesses
- moved `GapDivisibility` and `CoordinateEigenspace` into the clean-local set,
  and `HexagonalUnification` into the local-postulates set

### Track 32: Symmetry Shell Import-Boundary Recovery

Status: `complete`

Why this matters:
- with the hexagonal chain recovered, the next visible symmetry-facing blockers
  are [`agda-proofs/Theorems/PhaseLockSymmetry.agda`](agda-proofs/Theorems/PhaseLockSymmetry.agda)
  and [`agda-proofs/Theorems/ResidueSymmetry.agda`](agda-proofs/Theorems/ResidueSymmetry.agda)
- both currently fail on the same import-boundary drift against the clean
  abstract symmetry layer, which makes them a good bounded next tranche

Todo:
- [x] inspect the option/header mismatch in `PhaseLockSymmetry.agda` and
      `ResidueSymmetry.agda`
- [x] recover one or both files into compilable shells without overstating what
      is proved there
- [x] update the Agda status/signal docs if either file moves out of the
      failing set

Acceptance criteria:
- `agda Theorems/PhaseLockSymmetry.agda` or
  `agda Theorems/ResidueSymmetry.agda` passes
- the repaired file no longer fails primarily because of safe/without-K import
  drift
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Theorems/PhaseLockSymmetry.agda
agda Theorems/ResidueSymmetry.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer honest shell recovery over trying to reconstruct the full symmetry
  theorem layer in one pass

Completed notes:
- confirmed that both
  [`agda-proofs/Theorems/PhaseLockSymmetry.agda`](agda-proofs/Theorems/PhaseLockSymmetry.agda)
  and
  [`agda-proofs/Theorems/ResidueSymmetry.agda`](agda-proofs/Theorems/ResidueSymmetry.agda)
  were blocked by the same import-boundary problem and older hole-filled
  instantiation scripts
- replaced both files with smaller current-syntax shells that preserve the
  phase-lock and symmetric-window vocabulary while leaving the abstract
  theorem-instantiation bridges explicit
- moved both modules from the failing set into the local-postulates category

### Track 33: Coprimality / Radical Filter Parse Recovery

Status: `complete`

Why this matters:
- the next clearly paired blockers are
  [`agda-proofs/Theorems/CoprimalityRequirement.agda`](agda-proofs/Theorems/CoprimalityRequirement.agda)
  and
  [`agda-proofs/Theorems/RadicalDivisibilityFilter.agda`](agda-proofs/Theorems/RadicalDivisibilityFilter.agda)
- both currently fail on the same old "type signature cannot have a where
  clause" pattern, suggesting a bounded parse-era recovery tranche

Todo:
- [x] inspect the broken type-signature/`where` pattern in both files
- [x] recover one or both files into compilable shells or narrower honest
      interfaces
- [x] update the Agda status/signal docs if either file moves out of the
      failing set

Acceptance criteria:
- `agda Theorems/CoprimalityRequirement.agda` or
  `agda Theorems/RadicalDivisibilityFilter.agda` passes
- the repaired file no longer fails primarily because of the broken type
  signature / `where` pattern
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Theorems/CoprimalityRequirement.agda
agda Theorems/RadicalDivisibilityFilter.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded shell/interface recovery over forcing the full divisibility
  theorem layer through in one step

Completed notes:
- replaced
  [`agda-proofs/Theorems/CoprimalityRequirement.agda`](agda-proofs/Theorems/CoprimalityRequirement.agda)
  with a smaller current-syntax shell that keeps the core base/config examples
  and the intended divisibility/density claims explicit
- replaced
  [`agda-proofs/Theorems/RadicalDivisibilityFilter.agda`](agda-proofs/Theorems/RadicalDivisibilityFilter.agda)
  with a smaller current-syntax shell that keeps the radical-filter idea and
  its base 10 / 12 / 100 witnesses live
- corrected both module names to match their `Theorems/...` file paths and
  moved both modules from the failing set into the local-postulates category

### Track 34: Non-Abstract Symmetry Wrapper Recovery

Status: `complete`

Why this matters:
- the next visible pair is
  [`agda-proofs/Theorems/SymmetryImpliesRepulsion.agda`](agda-proofs/Theorems/SymmetryImpliesRepulsion.agda)
  and
  [`agda-proofs/Theorems/ConstrainedOrbitals.agda`](agda-proofs/Theorems/ConstrainedOrbitals.agda)
- both sit beside already-clean abstract counterparts, so recovering them would
  sharpen the repo's narrative wrapper layer around the symmetry core

Todo:
- [x] inspect the current blockers in the non-abstract symmetry wrappers
- [x] recover one or both files into compilable shells or narrower honest
      interfaces
- [x] update the Agda status/signal docs if either file moves out of the
      failing set

Acceptance criteria:
- `agda Theorems/SymmetryImpliesRepulsion.agda` or
  `agda Theorems/ConstrainedOrbitals.agda` passes
- the repaired file no longer fails primarily because of wrapper-layer drift
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Theorems/SymmetryImpliesRepulsion.agda
agda Theorems/ConstrainedOrbitals.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer recovering the non-abstract narrative wrappers as honest shells rather
  than rebuilding the full alternate theorem layer in one step

Completed notes:
- repaired
  [`agda-proofs/Theorems/ConstrainedOrbitals.agda`](agda-proofs/Theorems/ConstrainedOrbitals.agda)
  into a current-syntax theorem module that preserves its constructive orbital
  constraints without local postulates
- replaced
  [`agda-proofs/Theorems/SymmetryImpliesRepulsion.agda`](agda-proofs/Theorems/SymmetryImpliesRepulsion.agda)
  with a smaller wrapper shell that keeps midpoint residue witnesses live while
  leaving the abstract-instantiation bridge explicit
- moved `Theorems/ConstrainedOrbitals.agda` from the failing set into the
  clean-local category and `Theorems/SymmetryImpliesRepulsion.agda` into the
  local-postulates category
- updated the active Agda source-of-truth docs to reflect the new counts:
  30 clean-local / 23 with local postulates / 28 failing

### Track 35: Affine Wrapper Namespace Recovery

Status: `complete`

Why this matters:
- the next visible pair is
  [`agda-proofs/Theorems/AffineTransform.agda`](agda-proofs/Theorems/AffineTransform.agda)
  and
  [`agda-proofs/Theorems/AffineTransformComputation.agda`](agda-proofs/Theorems/AffineTransformComputation.agda)
- both currently fail at the module-name boundary, which means the repo still
  has parser-era wrapper drift hiding whatever real signal is left in that
  affine layer

Todo:
- [x] inspect the namespace/import blockers in both affine wrapper files
- [x] recover one or both files into compilable shells or narrower honest
      interfaces
- [x] update the Agda status/signal docs if either file moves out of the
      failing set

Acceptance criteria:
- `agda Theorems/AffineTransform.agda` or
  `agda Theorems/AffineTransformComputation.agda` passes
- the repaired file no longer fails primarily because of module-name drift
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Theorems/AffineTransform.agda
agda Theorems/AffineTransformComputation.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded namespace/interface recovery over trying to finish the whole
  affine theorem layer in one pass

Completed notes:
- replaced
  [`agda-proofs/Theorems/AffineTransform.agda`](agda-proofs/Theorems/AffineTransform.agda)
  with a smaller current-syntax theorem shell that keeps the membrane formula,
  affine shift/gradient vocabulary, and base-6/base-10 example families live
  while leaving the general affine residue proof explicit
- replaced
  [`agda-proofs/Theorems/AffineTransformComputation.agda`](agda-proofs/Theorems/AffineTransformComputation.agda)
  with a clean computation shell carrying maintained base-6 residue checks and
  reported base-10 observations
- repaired both files to use the stdlib 2.3 remainder API through a `toℕ`-based
  helper with explicit `NonZero` handling
- moved `Theorems/AffineTransform.agda` from the failing set into the
  local-postulates category and `Theorems/AffineTransformComputation.agda` into
  the clean-local category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 24 with local postulates / 26 failing

### Track 36: Universal Symmetry Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Theorems/UniversalSymmetryRepulsion.agda`](agda-proofs/Theorems/UniversalSymmetryRepulsion.agda)
  is now the last failing theorem-layer shell in that symmetry family
- its current first blocker is a missing `Data.Nat._>_` import, followed by an
  unfinished `example-symmetry` shell, which makes it a good bounded recovery
  target rather than an amorphous deep-proof project

Todo:
- [x] inspect the namespace/import and unfinished-shell blockers in
      `Theorems/UniversalSymmetryRepulsion.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Theorems/UniversalSymmetryRepulsion.agda` passes
- the repaired file no longer fails primarily because of missing imports or
  placeholder shell drift
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Theorems/UniversalSymmetryRepulsion.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer a bounded universal-symmetry shell recovery over broadening the claim
  surface or rebuilding every example in one step

Completed notes:
- imported the missing `Data.Nat._>_` operator and moved the unfinished
  `example-symmetry` layer into the module's explicit postulate surface
- repaired the universe levels of the indexed multiset/pairing layer in
  [`agda-proofs/Theorems/UniversalSymmetryRepulsion.agda`](agda-proofs/Theorems/UniversalSymmetryRepulsion.agda)
- fixed the contradiction step in the core
  `PerfectBucketsImplyHonoraryZero` proof so the midpoint witness is composed
  back to the original residue before applying `residue-distinct`
- moved `Theorems/UniversalSymmetryRepulsion.agda` from the failing set into the
  local-postulates category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 25 with local postulates / 25 failing

### Track 37: Constellation Power-Law Arithmetic Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Core/ConstellationPowerLaw.agda`](agda-proofs/Core/ConstellationPowerLaw.agda)
  is now the next visible analytical blocker
- its first failure is concrete and bounded: an ambiguous `_+_` between
  `Data.Nat` and `Data.Rational`, followed by a small stale hole layer in the
  analytical shell

Todo:
- [x] inspect the arithmetic-scope and hole blockers in
      `Core/ConstellationPowerLaw.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Core/ConstellationPowerLaw.agda` passes
- the repaired file no longer fails primarily because of arithmetic namespace
  drift
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Core/ConstellationPowerLaw.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded arithmetic/interface recovery over trying to fully mechanize
  the power-law theory in one pass

Completed notes:
- replaced
  [`agda-proofs/Core/ConstellationPowerLaw.agda`](agda-proofs/Core/ConstellationPowerLaw.agda)
  with a smaller current-syntax analytical shell built on the maintained
  rational layer from
  [`agda-proofs/Theorems/RationalStatistics.agda`](agda-proofs/Theorems/RationalStatistics.agda)
- kept the constellation vocabulary, the twin/cousin/sexy examples, and the
  reported near-`-1/2` power-law fit live as explicit shell values
- moved the universal-law, inverse-square-root, and monotonic-ordering claims
  into an honest postulate surface instead of leaving them mixed with stale
  arithmetic imports and holes
- moved `Core/ConstellationPowerLaw.agda` from the failing set into the
  local-postulates category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 26 with local postulates / 24 failing

### Track 38: Arithmetic Helper Parse Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Core/ArithmeticHelpers.agda`](agda-proofs/Core/ArithmeticHelpers.agda)
  is now the next visible core blocker
- its first failure is concrete and bounded: a parse-era illegal identifier
  pattern like `(5*q)` in type signatures

Todo:
- [x] inspect the parse-era identifier and signature blockers in
      `Core/ArithmeticHelpers.agda`
- [x] recover the file into a compilable helper module or narrower honest
      interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Core/ArithmeticHelpers.agda` passes
- the repaired file no longer fails primarily because of illegal identifier
  syntax
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Core/ArithmeticHelpers.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded helper-surface recovery over expanding the divisibility
  theorem layer in one pass

Completed notes:
- replaced
  [`agda-proofs/Core/ArithmeticHelpers.agda`](agda-proofs/Core/ArithmeticHelpers.agda)
  with a smaller current-syntax helper shell that keeps the constructive
  regrouping lemmas and factorization records live
- converted the old division-algorithm/example-template layer into an explicit
  postulate surface instead of leaving it in broken parse-era notation
- repaired the remaining binder and syntax issues so the file now compiles as a
  truthful mixed constructive/postulated helper surface
- moved `Core/ArithmeticHelpers.agda` from the failing set into the
  local-postulates category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 27 with local postulates / 23 failing

### Track 39: Discriminant Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Core/Discriminant.agda`](agda-proofs/Core/Discriminant.agda)
  is now the next visible analytical blocker
- its first failure is concrete and bounded: a missing `Data.Nat._>_` import,
  after which the remaining perfect-square and Legendre-symbol layers can be
  narrowed honestly

Todo:
- [x] inspect the import and hole-driven shell blockers in
      `Core/Discriminant.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Core/Discriminant.agda` passes
- the repaired file no longer fails primarily because of missing imports or
  hole-driven shell drift
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Core/Discriminant.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded discriminant-shell recovery over trying to finish the full
  algebraic lock and Legendre machinery in one pass

Completed notes:
- replaced
  [`agda-proofs/Core/Discriminant.agda`](agda-proofs/Core/Discriminant.agda)
  with a smaller current-syntax analytical shell
- kept the constructive discriminant helpers, perfect-square record, and
  recorded base-6 / base-12 observations live
- moved the perfect-square decision procedure, Legendre-symbol analysis,
  quality analysis, and HL/algebraic-lock bridge into an explicit postulate
  surface instead of leaving them mixed with parser-era drift
- moved `Core/Discriminant.agda` from the failing set into the
  local-postulates category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 28 with local postulates / 22 failing

### Track 40: Goldbach Phase-Lock Bridge Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Core/GoldbachPhaseLocks.agda`](agda-proofs/Core/GoldbachPhaseLocks.agda)
  was still a dead parser-era bridge between the phase-lock vocabulary and the
  concrete Goldbach examples
- recovering it keeps a live formal shell around one of the repo’s more novel
  connective ideas without overstating proof completion

Todo:
- [x] inspect the bridge-shell drift in `Core/GoldbachPhaseLocks.agda`
- [x] recover the file into a compilable current-syntax shell
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Core/GoldbachPhaseLocks.agda` passes
- the repaired file no longer fails primarily because of stale parser drift
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Core/GoldbachPhaseLocks.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded bridge-shell recovery over trying to mechanize the full
  phase-lock / Goldbach equivalence in one pass

Completed notes:
- replaced
  [`agda-proofs/Core/GoldbachPhaseLocks.agda`](agda-proofs/Core/GoldbachPhaseLocks.agda)
  with a self-contained current-syntax bridge shell
- kept concrete base-22 and base-26 phase-lock / Goldbach witnesses live
- moved the equivalence, spectral, and residue bridge into an explicit
  postulate surface instead of leaving it in broken imported drift
- moved `Core/GoldbachPhaseLocks.agda` from the failing set into the
  local-postulates category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 29 with local postulates / 21 failing

### Track 41: Golden-Ratio Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Core/GoldenRatio.agda`](agda-proofs/Core/GoldenRatio.agda)
  is now the next visible analytical blocker
- its first failure is concrete and bounded: a parse-era `where` attached to a
  type signature, after which the continued-fraction and real-analysis shell
  can be narrowed honestly

Todo:
- [x] inspect the parse-era `where` and shell blockers in `Core/GoldenRatio.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Core/GoldenRatio.agda` passes
- the repaired file no longer fails primarily because of parse-era signature
  drift
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Core/GoldenRatio.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded shell recovery over trying to mechanize full real-analysis or
  continued-fraction theory in one pass

Completed notes:
- replaced
  [`agda-proofs/Core/GoldenRatio.agda`](agda-proofs/Core/GoldenRatio.agda)
  with a smaller current-syntax golden-ratio shell
- kept the base-14 crossover story, Fibonacci ratio observations, and
  multi-shell scaling vocabulary live
- moved the irrationality, convergence, periodicity, and universality claims
  into an explicit postulate surface
- moved `Core/GoldenRatio.agda` from the failing set into the
  local-postulates category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 30 with local postulates / 20 failing

### Track 42: Lagrange-Point Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Core/LagrangePoints.agda`](agda-proofs/Core/LagrangePoints.agda)
  still contained a real canonical-pair signal, but it was trapped inside
  hole-driven insertion code and stale proof plumbing
- recovering it preserves the connector-side signal without inflating the
  general theory

Todo:
- [x] inspect the parser and hole-driven shell blockers in `Core/LagrangePoints.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Core/LagrangePoints.agda` passes
- the repaired file no longer fails primarily because of parser drift or holes
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Core/LagrangePoints.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded canonical-pair recovery over trying to mechanize full
  insertion-search theory in one pass

Completed notes:
- replaced
  [`agda-proofs/Core/LagrangePoints.agda`](agda-proofs/Core/LagrangePoints.agda)
  with a smaller current-syntax shell
- kept the canonical concatenation pair, the two reported Lagrange points, and
  small constructive counting helpers live
- moved the general existence, clustering, divisibility-balance, and membrane
  enhancement claims into an explicit postulate surface
- moved `Core/LagrangePoints.agda` from the failing set into the
  local-postulates category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 31 with local postulates / 19 failing

### Track 43: Orthogonality Framework Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Core/OrthogonalityFramework.agda`](agda-proofs/Core/OrthogonalityFramework.agda)
  carried an important repo claim: raw and HL-normalized orthogonality are not
  the same thing, and full decorrelation remains open
- recovering it keeps that steelman signal visible while making the membrane
  singular-series debt explicit

Todo:
- [x] inspect the stale arithmetic and parser blockers in
      `Core/OrthogonalityFramework.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Core/OrthogonalityFramework.agda` passes
- the repaired file keeps the empirical correlation story live without broken
  proof scaffolding
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Core/OrthogonalityFramework.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded shell recovery over trying to derive the membrane singular
  series in one pass

Completed notes:
- replaced
  [`agda-proofs/Core/OrthogonalityFramework.agda`](agda-proofs/Core/OrthogonalityFramework.agda)
  with a smaller current-syntax shell
- kept the signed correlation observations, status classifier, and dual-score
  framing live
- moved the membrane singular series, full normalization, and theorem-level
  orthogonality bridge into an explicit postulate surface
- moved `Core/OrthogonalityFramework.agda` from the failing set into the
  local-postulates category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 32 with local postulates / 18 failing

### Track 44: Spectral Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Core/Spectral.agda`](agda-proofs/Core/Spectral.agda) sits under
  the phase-lock layer and kept failing inside low-level Legendre machinery
- recovering it restores the QR/NQR and `p mod 4` vocabulary used across the
  repo without pretending the entire number-theory bridge is already rebuilt

Todo:
- [x] inspect the parser and low-level proof blockers in `Core/Spectral.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Core/Spectral.agda` passes
- the repaired file keeps the spectral vocabulary live without parser drift
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Core/Spectral.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded shell recovery over trying to finish a full constructive
  Legendre-symbol development in one pass

Completed notes:
- replaced
  [`agda-proofs/Core/Spectral.agda`](agda-proofs/Core/Spectral.agda)
  with a smaller current-syntax spectral shell
- kept the `±1` group, `p mod 4` split, supplement shell values, and primitive
  root examples live
- moved the full Legendre, Euler, and QR/NQR proof bridge into an explicit
  postulate surface
- moved `Core/Spectral.agda` from the failing set into the local-postulates
  category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 33 with local postulates / 17 failing

### Track 45: `2p` Base Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Core/TwoPBase.agda`](agda-proofs/Core/TwoPBase.agda) provides
  the base family underneath the phase-lock layer
- recovering it keeps the concrete `2p` examples and residue sets live while
  leaving the heavier radical/totient/framework bridge explicit

Todo:
- [x] inspect the parse-era `where` and dependency blockers in
      `Core/TwoPBase.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Core/TwoPBase.agda` passes
- the repaired file keeps the concrete `2p` residue story live without parser
  drift
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Core/TwoPBase.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded shell recovery over trying to pull the full residue/radical
  framework into this file in one pass

Completed notes:
- replaced
  [`agda-proofs/Core/TwoPBase.agda`](agda-proofs/Core/TwoPBase.agda)
  with a smaller current-syntax shell
- kept concrete `2p` bases and residue sets for 6, 10, and 14 live
- moved the radical, totient, and framework bridge into an explicit postulate
  surface
- moved `Core/TwoPBase.agda` from the failing set into the local-postulates
  category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 34 with local postulates / 16 failing

### Track 46: Phase-Lock Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Core/PhaseLocks.agda`](agda-proofs/Core/PhaseLocks.agda) is one
  of the repo’s central conceptual files
- once the spectral and `2p` base shells were live again, this became the
  highest-leverage place to restore a truthful midpoint/distance interface

Todo:
- [x] inspect the legacy proof bulk and safe-import blockers in
      `Core/PhaseLocks.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Core/PhaseLocks.agda` passes
- the repaired file keeps the midpoint/distance and Goldbach-bridge vocabulary
  live without pretending the restricted-Goldbach theorem is complete
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Core/PhaseLocks.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded shell recovery over trying to mechanize the full restricted
  Goldbach bridge in one pass

Completed notes:
- replaced
  [`agda-proofs/Core/PhaseLocks.agda`](agda-proofs/Core/PhaseLocks.agda)
  with a smaller current-syntax shell
- kept the midpoint structure, Goldbach/phase-lock conversion vocabulary, and
  concrete base-10/base-14/base-22 examples live
- moved the restricted-Goldbach, spectral, residue-framework, and density
  bridge into an explicit postulate surface
- moved `Core/PhaseLocks.agda` from the failing set into the local-postulates
  category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 35 with local postulates / 15 failing

### Track 47: Advanced Orthogonality Parse Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Advanced/Orthogonality.agda`](agda-proofs/Advanced/Orthogonality.agda)
  is now the next visible blocker after the core analytical shell recovery
  tranche
- its first failure is concrete and bounded: a parse-era clause pattern
  `... | 0`, and the newly live `Core/OrthogonalityFramework.agda` gives it a
  better foundation than before

Todo:
- [x] inspect the parse-era clause blockers in `Advanced/Orthogonality.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Advanced/Orthogonality.agda` passes
- the repaired file no longer fails primarily because of parse-era clause drift
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Advanced/Orthogonality.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded shell recovery over trying to complete the full advanced
  orthogonality pipeline in one pass

Completed notes:
- replaced
  [`agda-proofs/Advanced/Orthogonality.agda`](agda-proofs/Advanced/Orthogonality.agda)
  with a smaller current-syntax advanced shell
- kept the prime-pair experiment framing, the raw-vs-HL interpretation, and
  the membrane comparison link to
  [`agda-proofs/Core/OrthogonalityFramework.agda`](agda-proofs/Core/OrthogonalityFramework.agda)
  live
- moved the Babylonian/prime-pair computational backend and float-alignment
  bridge into an explicit postulate surface
- moved `Advanced/Orthogonality.agda` from the failing set into the
  local-postulates category
- updated the active Agda source-of-truth docs to reflect the new counts:
  31 clean-local / 36 with local postulates / 14 failing

### Track 48: Complete Orthogonality-Float Parse Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Complete/OrthogonalityFloat.agda`](agda-proofs/Complete/OrthogonalityFloat.agda)
  is now the next visible blocker in the orthogonality lane
- the recovered advanced/core orthogonality shells give it a cleaner target:
  keep the executable backend honest without carrying parser-era notation drift

Todo:
- [x] inspect the malformed operator and parser blockers in
      `Complete/OrthogonalityFloat.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Complete/OrthogonalityFloat.agda` passes
- the repaired file no longer fails primarily because of malformed operator or
  clause syntax
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Complete/OrthogonalityFloat.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded backend recovery over trying to finish every foreign/backend
  integration detail in one pass

Completed notes:
- repaired the malformed operator declarations and old `let`/helper syntax in
  [`agda-proofs/Complete/OrthogonalityFloat.agda`](agda-proofs/Complete/OrthogonalityFloat.agda)
- rewired the file onto current builtin/stdlib Nat, Float, String, product,
  and IO surfaces
- kept the executable backend alive instead of narrowing this file into a shell
- added explicit termination pragmas where Agda 2.8.0 no longer inferred the
  recursion automatically
- moved `Complete/OrthogonalityFloat.agda` from the failing set into the
  clean-local category
- updated the active Agda source-of-truth docs to reflect the new counts:
  32 clean-local / 36 with local postulates / 13 failing

### Track 49: Computational Bridge Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Integration/ComputationalBridge.agda`](agda-proofs/Integration/ComputationalBridge.agda)
  is now the next visible integration blocker
- it sits at the interface between the repaired Agda core and external tooling,
  so a truthful current-syntax shell would improve both rigor and usability

Todo:
- [x] inspect the parser and stale-import blockers in
      `Integration/ComputationalBridge.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Integration/ComputationalBridge.agda` passes
- the repaired file no longer fails primarily because of parser drift or stale
  imports from recovered core modules
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Integration/ComputationalBridge.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded integration-shell recovery over trying to reconstruct every
  old external-tool bridge in one pass

Completed:
- replaced the deleted parser-era file with a current-syntax integration shell
  that preserves the live CRT/residue export path
- kept phase-lock, canonical Lagrange, and discriminant summary exports live
  against the recovered core shells
- moved the external Rust/WASM/unified-CLI bridge into one explicit postulate
  block instead of leaving broken helper code in place
- updated the active Agda source-of-truth docs to reflect the new counts:
  32 clean-local / 37 with local postulates / 12 failing

### Track 50: Prime Density Framework Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Integration/PrimeDensityFramework.agda`](agda-proofs/Integration/PrimeDensityFramework.agda)
  is now the next visible integration blocker
- it still tries to present the repo's unified residue / phase-lock /
  discriminant / symmetry analysis surface, so recovering it honestly would
  improve both readability and cross-layer rigor

Todo:
- [x] inspect the parser and stale-import blockers in
      `Integration/PrimeDensityFramework.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Integration/PrimeDensityFramework.agda` passes
- the repaired file no longer fails primarily because of parser drift or stale
  imports from recovered core modules
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Integration/PrimeDensityFramework.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded integration-shell recovery over trying to reconstruct the full
  old unified theorem surface in one pass

Completed:
- replaced the parser-era unified framework with a current-syntax shell that
  integrates residue admissibility, phase-lock context, orthogonality status,
  and optional Lagrange/discriminant slices
- kept concrete framework views live for base 10, base 14, a base-6
  discriminant lane, and the canonical connector lane
- moved the full predictor bridge into one explicit postulate block instead of
  leaving stale theorem holes in place
- updated the active Agda source-of-truth docs to reflect the new counts:
  32 clean-local / 38 with local postulates / 11 failing

### Track 51: Lagrange Examples Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/LagrangePoints/Examples.agda`](agda-proofs/LagrangePoints/Examples.agda)
  is now the next visible blocker
- the canonical connector / insertion story remains one of the repo's most
  distinctive formalized novelty lanes, and the example surface is the next
  place where parser drift still obscures the real signal

Todo:
- [x] inspect the parser and stale-import blockers in
      `LagrangePoints/Examples.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda LagrangePoints/Examples.agda` passes
- the repaired file no longer fails primarily because of parser drift or stale
  local helper declarations
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda LagrangePoints/Examples.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded example-shell recovery over reconstructing the full historical
  Lagrange proof narrative in one pass

Completed:
- replaced the parser-era notebook surface with a current-syntax canonical
  case-study shell
- kept the two reported hits, the reflected open positions, the center-void
  question, and the membrane connection live
- moved the full scan and broader residue/theory bridge into one explicit
  postulate block
- updated the active Agda source-of-truth docs to reflect the new counts:
  32 clean-local / 39 with local postulates / 10 failing

### Track 52: Lagrange Residue-Field Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/LagrangePoints/ResidueField.agda`](agda-proofs/LagrangePoints/ResidueField.agda)
  is now the next visible blocker
- after the canonical example shell recovered, the residue-side connector
  mechanism is the next best place to keep sharpening the canonical Lagrange
  lane without overextending its generality

Todo:
- [x] inspect the parser and stale-import blockers in
      `LagrangePoints/ResidueField.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda LagrangePoints/ResidueField.agda` passes
- the repaired file no longer fails primarily because of parser drift or stale
  local helper declarations
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda LagrangePoints/ResidueField.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded residue-shell recovery over reconstructing the full connector
  mechanism in one pass

Completed:
- replaced the half-implemented arithmetic/search file with a current-syntax
  residue-screen shell
- kept the canonical small-prime screen, the two reported compatible positions,
  and the open non-hit positions live
- moved the CRT/search/primality bridge into one explicit postulate block
- updated the active Agda source-of-truth docs to reflect the new counts:
  32 clean-local / 40 with local postulates / 9 failing

### Track 53: Lagrange Template-Extension Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/LagrangePoints/TemplateExtension.agda`](agda-proofs/LagrangePoints/TemplateExtension.agda)
  is now the next visible blocker
- after the example and residue-side shells recovered, this is the remaining
  local wrapper that still tries to connect the abstract symmetry machinery to
  the canonical connector case study

Todo:
- [x] inspect the flag/import blockers in
      `LagrangePoints/TemplateExtension.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda LagrangePoints/TemplateExtension.agda` passes
- the repaired file no longer fails primarily because of flag mismatch or stale
  wrapper imports
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda LagrangePoints/TemplateExtension.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded wrapper recovery over reconstructing the full symmetry bridge
  in one pass

Completed:
- replaced the flag-mismatched wrapper with a current-syntax
  asymmetric-template shell over the canonical connector case study
- kept buffer reflection, center-position structure, and the abstract
  honorary-zero bridge live
- opportunistically recovered `Test/TestRecord.agda`,
  `Test/TestRecordSimple.agda`, `Tests/InvariantTests.agda`,
  `Verification/ExclusiveConfigurations.agda`,
  `Verification/GCDParadoxComputation.agda`,
  `Verification/ResonanceComputation.agda`, and
  `Examples/UniMathIntegration.agda`
- updated the active Agda source-of-truth docs to reflect the new counts:
  35 clean-local / 45 with local postulates / 1 failing

### Track 54: Certified Resonance Shell Recovery

Status: `complete`

Why this matters:
- [`agda-proofs/Examples/CertifiedResonance.agda`](agda-proofs/Examples/CertifiedResonance.agda)
  is now the sole remaining blocker in the Agda tree
- the certification stack is already the repo's strongest formal spine, so
  recovering the last example wrapper would complete the "all modules compile"
  hardening arc

Todo:
- [x] inspect the parser and stale-helper blockers in
      `Examples/CertifiedResonance.agda`
- [x] recover the file into a compilable shell or narrower honest interface
- [x] update the Agda status/signal docs if it moves out of the failing set

Acceptance criteria:
- `agda Examples/CertifiedResonance.agda` passes
- the repaired file no longer fails primarily because of parser drift or stale
  local helper declarations
- the clean-spine verifier still passes afterward

Verification:
```bash
cd agda-proofs
agda Examples/CertifiedResonance.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer bounded example-shell recovery over reconstructing every old certified
  resonance narrative layer in one pass

Completed:
- replaced the stale generated-proof sketch with a current-syntax wrapper over
  `Examples/CertifiedResonanceComplete.agda`
- kept the concrete base-6 symmetry data, perfect-bucket witness, and
  honorary-zero result live through re-exported values
- moved the runtime residue export / code-generation bridge into one explicit
  postulate block
- updated the active Agda source-of-truth docs to reflect the new counts:
  35 clean-local / 46 with local postulates / 0 failing

### Track 55: Certification Auto-Witness Reduction

Status: `complete`

Why this matters:
- the Agda tree now compiles end-to-end, so the next highest-signal work is no
  longer parser recovery but postulate reduction
- the certification stack is the repo's strongest formal spine, and its
  remaining postulates are now concentrated in a small number of productive
  bridge points

Todo:
- [x] inspect `Examples/CertifiedResonanceParam.agda`,
      `Examples/CertifiedResonanceParamDyn.agda`, and
      `Theorems/Abstract/BucketsAutoMatch.agda`
- [x] recover or narrow one certification postulate surface, starting with
      `autoPerfectBuckets` or one of its wrapper obligations
- [x] update the Agda status/signal docs to reflect the narrower postulated
      surface

Acceptance criteria:
- at least one certification-stack postulate surface becomes smaller or more
  constructive
- the maintained clean spine still passes afterward
- all 81 Agda modules still compile

Verification:
```bash
cd agda-proofs
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
agda Theorems/Abstract/BucketsAutoMatch.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer reducing one sharp certification bridge at a time over attempting a
  full constructive reconstruction in one pass

Completed:
- removed the duplicated local `autoPerfectBuckets` postulate from
  `Examples/CertifiedResonanceParam.agda` and replaced it with a constructive
  wrapper over `BucketsAutoMatch.perfectFromBalanced`
- used the same `BalancedBuckets` wrapper in
  `Examples/CertifiedResonanceParamDyn.agda`, so the dynamic wrapper now keeps
  only the runtime `proof-stable` witness locally
- updated the active Agda source-of-truth docs to reflect the new counts:
  36 clean-local / 45 with local postulates / 0 failing

### Track 56: BucketsAutoMatch Helper Reduction

Status: `complete`

Why this matters:
- `CertifiedResonanceParam.agda` is now a clean-local boundary wrapper, so the
  next real bottleneck is the actual helper layer it rests on
- reducing `BucketsAutoMatch.agda` is higher-signal than adding more wrappers,
  because it would improve the shared certification bridge itself

Todo:
- [x] inspect `indices-with-residue`, `zip-pair`, and one `auto-mate-*` helper
      in `Theorems/Abstract/BucketsAutoMatch.agda`
- [x] recover one helper constructively or split one broad assumption into
      narrower explicit pieces
- [x] update the Agda status/signal docs if the helper surface narrows

Acceptance criteria:
- at least one `BucketsAutoMatch.agda` helper assumption becomes smaller or more
  constructive
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer narrowing one shared auto-matching helper at a time over attempting a
  full constructive implementation of the entire pairing search in one tranche

Completed:
- replaced the local `indices-with-residue` assumption with a constructive
  `fzero` / `fsuc` recursion over `Fin n`
- replaced the local `zip-pair` assumption with a constructive element-wise
  list pairing helper that leaves unmatched indices untouched
- updated the Agda source-of-truth docs to record that the remaining
  `BucketsAutoMatch` bottleneck is the `auto-mate-*` law block itself

### Track 57: Auto-Mate Law Block Narrowing

Status: `complete`

Why this matters:
- the computational helpers inside `BucketsAutoMatch.agda` are now real code,
  so the next remaining bottleneck is the theorem block that certifies the
  resulting mate function
- reducing that block would strengthen both the shared certification bridge and
  the clean-local boundary status of `CertifiedResonanceParam.agda`

Todo:
- [x] inspect `auto-mate-involutive`, `auto-mate-no-fixed`,
      `auto-mate-equivariant`, and `auto-mate-residue-distinct`
- [x] either recover one law constructively or split the block into narrower
      assumptions that match the actual proof bottleneck more precisely
- [x] update the Agda status/signal docs if the law surface narrows

Acceptance criteria:
- at least one `auto-mate-*` law becomes constructive or the remaining law
  block becomes narrower and more explicit
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer narrowing one auto-mate law at a time over attempting a full
  constructive proof of the entire block in one tranche

Completed:
- replaced the local `auto-mate-no-fixed` assumption with a constructive proof
  derived from `auto-mate-residue-distinct` via `cong f`
- updated the Agda source-of-truth docs so the remaining `BucketsAutoMatch`
  bottleneck is named precisely: `auto-mate-involutive / equivariant /
  residue-distinct`
- category counts stay at 36 clean-local / 45 with local postulates / 0
  failing

### Track 58: Fixed-Residue Assumption Surfacing

Status: `complete`

Why this matters:
- the remaining `auto-mate-residue-distinct` burden is where the real
  structural assumption now lives
- surfacing that assumption explicitly would harden the certification stack
  more than adding more wrapper-level conveniences

Todo:
- [x] inspect whether `auto-mate-residue-distinct` should be parameterized by an
      explicit non-fixed-residue or midpoint-exclusion assumption
- [x] thread any required structural assumption through
      `BucketsAutoMatch.agda` and the certification wrappers honestly
- [x] update the Agda status/signal docs if that law surface becomes more
      explicit

Acceptance criteria:
- the remaining `auto-mate-residue-distinct` burden is either smaller or more
  explicit about the structural assumption it needs
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer surfacing the real fixed-residue assumption explicitly over pretending
  balanced counts alone settle the entire residue-distinct question

Completed:
- introduced `ObservedResiduesMove` in `BucketsAutoMatch.agda` as the explicit
  structural witness `∀ i → inv (f i) ≢ f i`
- replaced the local `auto-mate-residue-distinct` postulate with a
  constructive proof from `ObservedResiduesMove` plus
  `auto-mate-equivariant`
- threaded the explicit witness through `perfectFromBalanced`,
  `honoraryZeroFromBalanced`, `Examples/CertifiedResonanceParam.agda`, and
  `Examples/CertifiedResonanceParamDyn.agda`
- category counts stay at 36 clean-local / 45 with local postulates / 0
  failing

### Track 59: Reflection Fixed-Point Exclusion Bridge

Status: `complete`

Why this matters:
- the shared certification bridge is now honest about needing
  `ObservedResiduesMove`
- the next high-signal step is to derive that witness in the concrete modular
  reflection setting from simpler domain assumptions instead of passing it in
  manually everywhere

Todo:
- [x] inspect the fixed points of `mkSymReflect` in the modular settings this
      repo actually uses
- [x] test whether `ObservedResiduesMove` can be derived from a narrower pair of
      assumptions such as midpoint exclusion plus residue-support exclusion of
      other fixed points
- [x] update the certification wrappers or reflection layer if one such bridge
      is defensible

Acceptance criteria:
- either one concrete bridge from simpler reflection assumptions to
  `ObservedResiduesMove` is recovered, or the remaining fixed-point exclusion
  burden is narrowed more precisely in the docs/code
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/SymmetryFiniteReflect.agda
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer recovering a concrete fixed-point exclusion bridge for the modular
  reflection cases this repo actually uses over a fully abstract theorem for
  arbitrary involutions

Completed:
- inspected the concrete fixed-point shape of `mkSymReflect` and narrowed the
  repo's real half-turn burden to the fixed residues that matter in practice
- added `ReflectFixedPointExclusion` to
  `Theorems/Abstract/SymmetryFiniteReflect.agda`, packaging
  `zero-fixed / zeroVoid / midVoid`
- added `observedResiduesMoveFromFixedPointExclusion`, so
  `ObservedResiduesMove` is now derived in the modular reflection layer rather
  than passed manually through the certification wrappers
- rewired `Examples/CertifiedResonanceParam.agda` and
  `Examples/CertifiedResonanceParamDyn.agda` to consume the narrower reflection
  witness
- category counts stay at 36 clean-local / 45 with local postulates / 0
  failing

### Track 60: Auto-Mate Equivariance Narrowing

Status: `complete`

Why this matters:
- Track 59 moved the fixed-point burden into the concrete reflection layer,
  which makes the remaining `BucketsAutoMatch` bottleneck sharper
- the next high-signal certification gain is to reduce one of the remaining
  `auto-mate` law assumptions instead of adding more wrapper structure

Todo:
- [x] inspect whether `auto-mate-equivariant` can be reduced to a narrower list
      alignment or support-matching statement for `indices-with-residue` and
      `zip-pair`
- [x] if full recovery is too large, split `auto-mate-equivariant` into smaller
      explicit helper obligations that match the current constructive code
- [x] update the certification-stack docs to reflect whichever narrower law
      surface survives

Acceptance criteria:
- either `BucketsAutoMatch.agda` recovers one nontrivial piece of
  `auto-mate-equivariant` constructively, or the remaining law block is split
  into smaller honest assumptions
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer reducing the remaining theorem surface in `BucketsAutoMatch.agda`
  directly over pushing more proof burden into wrappers

Completed:
- replaced the top-level `auto-mate-equivariant` postulate in
  `Theorems/Abstract/BucketsAutoMatch.agda` with the smaller bridge pair
  `support-counts-agree / zip-pair-preserves-target-residue`
- recovered `auto-mate-support-lengths` constructively from
  `BalancedBuckets.balanced` plus `support-counts-agree`
- recovered `auto-mate-equivariant` constructively from the new support-length
  bridge plus `zip-pair-preserves-target-residue`
- updated the active Agda truth surfaces so the remaining `BucketsAutoMatch`
  bottleneck is now the smaller bridge trio
  `auto-mate-involutive / support-counts-agree /
  zip-pair-preserves-target-residue`
- category counts stay at 36 clean-local / 45 with local postulates / 0
  failing

### Track 61: Support-Count Agreement Recovery

Status: `complete`

Why this matters:
- Track 60 exposed the real next gap cleanly: the certification stack still
  assumes bucket counts agree with the concrete support lists built by
  `indices-with-residue`
- the repo already has explicit `countResid` machinery in the parameterized
  certification wrappers, so this is a promising place to turn one more bridge
  into real code

Todo:
- [x] inspect whether `support-counts-agree` can be proved constructively for
      the `countResid`-style counts the repo actually uses
- [x] if a fully generic proof is too large, recover a narrower theorem or
      helper surface that matches the `countResid` path honestly
- [x] update the certification-stack docs to reflect whichever support-count
      bridge survives

Acceptance criteria:
- either `support-counts-agree` is recovered constructively in one concrete
  path, or it is split into smaller honest assumptions
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer recovering the count-to-support bridge for the concrete `countResid`
  path before attempting a fully generic list-theoretic normalization proof

Completed:
- added `SupportCountsAgree`, `perfectFromBalancedWithSupport`, and
  `honoraryZeroFromBalancedWithSupport` to
  `Theorems/Abstract/BucketsAutoMatch.agda`
- proved `supportCountsAgreeCountResid` constructively in
  `Examples/CertifiedResonanceParam.agda` and
  `Examples/CertifiedResonanceParamDyn.agda`
- rewired the parameterized `autoPerfectBuckets` helpers to use the explicit
  support-count path, so the concrete `countResid` certification lane no longer
  depends on the generic `support-counts-agree` postulate
- updated the active Agda truth surfaces so the concrete wrapper path now has a
  narrower remaining bridge than the generic `BucketsAutoMatch` layer
- category counts stay at 36 clean-local / 45 with local postulates / 0
  failing

### Track 62: Zip-Pair Target Residue Recovery

Status: `complete`

Why this matters:
- after Track 61, the concrete `countResid` certification lane no longer
  depends on the generic support-count bridge
- the sharp remaining imported theorem in that lane is now
  `zip-pair-preserves-target-residue`, which is closer to the actual support
  lists and list-alignment mechanics than the old monolithic equivariance claim

Todo:
- [x] inspect whether `zip-pair-preserves-target-residue` can be reduced to
      smaller support-membership or list-alignment lemmas
- [x] recover at least one concrete piece of the `zip-pair` target-residue path
      constructively if feasible
- [x] update the certification-stack docs to reflect the narrower remaining
      `zip-pair` bridge

Acceptance criteria:
- either `zip-pair-preserves-target-residue` is reduced constructively in one
  concrete path, or it is split into smaller honest assumptions
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer recovering the `zip-pair` target-residue bridge through concrete
  support-list lemmas before attacking `auto-mate-involutive`

Completed:
- added constructive support lemmas in `Theorems/Abstract/BucketsAutoMatch.agda`:
  `indices-with-residue-complete`, `indices-with-residue-sound`, and
  `support-lists-disjoint`
- replaced the residue-level remaining assumption
  `zip-pair-preserves-target-residue` with the smaller pure list theorem
  `zip-pair-sends-source-to-target-support`
- recovered `zip-pair-preserves-target-residue` constructively from the new
  support lemmas plus the remaining pure list-support theorem
- updated the active Agda truth surfaces so the concrete `countResid`
  certification lane now depends on a pure support-alignment bridge rather than
  a residue-level `zip-pair` claim
- category counts stay at 36 clean-local / 45 with local postulates / 0
  failing

### Track 63: Auto-Mate Involutive Narrowing

Status: `complete`

Why this matters:
- after Track 62, the concrete certification lane no longer carries residue
  semantics in the remaining `zip-pair` bridge
- the deepest remaining imported theorem in that lane is now
  `auto-mate-involutive`, which is the natural next place to either recover a
  concrete piece or split the law into smaller list-alignment facts

Todo:
- [x] inspect whether `auto-mate-involutive` can be reduced to smaller
      `zip-pair` composition or support-alignment lemmas
- [x] recover at least one nontrivial involutive helper constructively if
      feasible
- [x] update the certification-stack docs to reflect the narrower involutive
      bridge that survives

Acceptance criteria:
- either `auto-mate-involutive` is reduced constructively in one concrete path,
  or it is split into smaller honest assumptions
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer reducing `auto-mate-involutive` through concrete support and `zip-pair`
  composition lemmas before attempting a full generic permutation proof

Completed:
- added `auto-mate-second-step-shape` and `auto-mate-involutive-from` in
  `Theorems/Abstract/BucketsAutoMatch.agda`
- replaced the direct involutive assumption with the smaller pure support
  theorem `zip-pair-roundtrips-on-disjoint-support`
- rebuilt `auto-mate-involutive` constructively from support disjointness,
  support lengths, equivariance, and the new roundtrip lemma
- updated the active Agda truth surfaces so the concrete `countResid`
  certification lane no longer depends on a direct imported involutive theorem
- category counts stay at 36 clean-local / 45 with local postulates / 0
  failing

### Track 64: Zip-Pair Roundtrip Narrowing

Status: `complete`

Why this matters:
- after Track 63, the concrete certification lane depends only on the smaller
  pure `zip-pair` support-alignment pair
- the newest imported theorem in that lane is now
  `zip-pair-roundtrips-on-disjoint-support`, which is a cleaner and more local
  next target than the older end-to-end auto-mate names

Todo:
- [x] inspect whether `zip-pair-roundtrips-on-disjoint-support` can be reduced
      constructively or split into smaller list-shape assumptions
- [x] recover at least one nontrivial roundtrip helper constructively if
      feasible
- [x] update the certification-stack docs to reflect the narrower pure
      `zip-pair` bridge that survives

Acceptance criteria:
- either `zip-pair-roundtrips-on-disjoint-support` is recovered constructively
  in one concrete path, or it is split into smaller honest assumptions
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer narrowing `zip-pair-roundtrips-on-disjoint-support` through direct
  support-list structure before attempting a full generic permutation proof of
  `zip-pair`

Completed:
- introduced `ListUnique`, `lift-fin-list-unique`, and
  `indices-with-residue-unique` in `Theorems/Abstract/BucketsAutoMatch.agda`
- replaced the overbroad generic roundtrip assumption with the constructive
  theorem `zip-pair-roundtrips-on-unique-disjoint-support`
- rebuilt the roundtrip step for the certification lane from support-list
  uniqueness plus the remaining transport theorem
  `zip-pair-sends-source-to-target-support`
- updated the active Agda truth surfaces so the concrete `countResid`
  certification lane no longer depends on a generic roundtrip theorem
- category counts stay at 36 clean-local / 45 with local postulates / 0
  failing

### Track 65: Zip-Pair Transport Narrowing

Status: `complete`

Why this matters:
- after Track 64, the concrete certification lane inherits only one imported
  theorem from the auto-pairing layer
- that remaining theorem, `zip-pair-sends-source-to-target-support`, is now the
  cleanest next bottleneck in the certification stack

Todo:
- [x] inspect whether `zip-pair-sends-source-to-target-support` can be proved
      constructively for unique disjoint support lists or split into smaller
      list-shape assumptions
- [x] recover at least one nontrivial support-transport helper constructively
      if feasible
- [x] update the certification-stack docs to reflect the narrower transport
      bridge that survives

Acceptance criteria:
- either `zip-pair-sends-source-to-target-support` is recovered constructively
  in one concrete path, or it is split into smaller honest assumptions
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer narrowing `zip-pair-sends-source-to-target-support` through direct
  support-list structure before attempting a full generic permutation proof of
  `zip-pair`

Completed:
- recovered `zip-pair-sends-source-to-target-support` constructively in
  `Theorems/Abstract/BucketsAutoMatch.agda` by direct induction on the paired
  support lists
- removed the last imported `zip-pair` theorem from the concrete `countResid`
  certification lane
- updated the active Agda truth surfaces so the remaining generic
  `BucketsAutoMatch` burden is now just `support-counts-agree`
- category counts stay at 36 clean-local / 45 with local postulates / 0
  failing

### Track 66: Support-Count Agreement Narrowing

Status: `complete`

Why this matters:
- after Track 65, the certification bottleneck in `BucketsAutoMatch` is now
  singular and explicit
- `support-counts-agree` is the last generic open theorem in that lane, so the
  next tranche can focus on whether it is actually derivable or should be
  narrowed into a more honest contract

Todo:
- [x] inspect whether `support-counts-agree` can be recovered from a smaller
      structural count contract or from a more specific class of counting
      functions
- [x] recover at least one constructive path or split the remaining theorem
      into smaller honest assumptions
- [x] update the certification-stack docs to reflect the narrower count bridge
      that survives

Acceptance criteria:
- either `support-counts-agree` is recovered constructively in one useful
  path, or it is split into smaller honest assumptions
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer narrowing `support-counts-agree` toward an explicit count contract
  before attempting any unnecessary generic counting framework

Completed:
- removed the local `support-counts-agree` postulate from
  `Theorems/Abstract/BucketsAutoMatch.agda` and kept `SupportCountsAgree` as an
  explicit API contract instead
- rewired the generic convenience APIs to take that contract directly, while the
  concrete `countResid` certification wrappers continued to discharge it
  constructively via their explicit-support path
- promoted `BucketsAutoMatch.agda` into the clean-local category and added it
  to the maintained clean spine
- updated the active Agda truth surfaces to reflect the new boundary:
  37 clean-local / 44 with local postulates / 0 failing

### Track 67: WindowCertificate Contract Alignment

Status: `complete`

Why this matters:
- after Track 66, the remaining certification bottleneck has moved outward from
  `BucketsAutoMatch` to the helper contracts consumed by
  `Theorems/Abstract/WindowCertificate.agda`
- the clean auto-pairing lane is now stable enough that `WindowCertificate`
  should name its remaining support-count and fixed-point burdens explicitly
  instead of reading as if the older implicit bridge still exists

Todo:
- [x] inspect `Theorems/Abstract/WindowCertificate.agda` for stale implicit use
      of the old `BucketsAutoMatch` theorem surface
- [x] narrow or rename one remaining helper contract so the file states its
      dependency boundary honestly
- [x] update the certification-stack docs so the next bottleneck is clear

Acceptance criteria:
- `Theorems/Abstract/WindowCertificate.agda` reflects the explicit
  count/support/fixed-point contracts it actually needs
- `agda Theorems/Abstract/WindowCertificate.agda` still passes
- `agda Theorems/Abstract/BucketsAutoMatch.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/WindowCertificate.agda
agda Theorems/Abstract/BucketsAutoMatch.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer tightening the explicit certification contracts before attempting a
  broader re-proof of the window-certificate helper layer

Completed:
- introduced `StaticContracts` in
  `Theorems/Abstract/WindowCertificate.agda` so the builder now names its real
  static boundary explicitly: support-count agreement plus non-fixed residues
- removed the top-level `deriveHonoraryZero` and `deriveInviolability`
  postulates by deriving the static certificate through
  `honoraryZeroFromBalanced` and the dynamic certificate through
  `Inviolability`
- narrowed the remaining local postulate surface in `WindowCertificate.agda`
  to the hypothetical example shell only
- updated the active Agda truth surfaces to reflect that the builder is now
  constructive even though the file remains in the local-postulates category

### Track 68: SymmetryFiniteReflect Fixed-Point Contract Narrowing

Status: `complete`

Why this matters:
- after Track 67, the remaining static certification boundary is even clearer:
  `WindowCertificate` now asks for explicit non-fixed-residue evidence, and the
  next unresolved bridge sits inside `SymmetryFiniteReflect.agda`
- the half-turn fixed-point classification is now the cleanest place to keep
  narrowing the certification stack without backsliding into wrapper-level
  assumptions

Todo:
- [x] inspect whether `ReflectFixedPointExclusion` or
      `reflect-fixed-classification-half-turn` can be split into a smaller,
      more honest fixed-point contract
- [x] recover one constructive helper or sharpen the remaining postulate names
      so the file states its real arithmetic burden explicitly
- [x] update the certification-stack docs so the next boundary after
      `WindowCertificate` is obvious

Acceptance criteria:
- `Theorems/Abstract/SymmetryFiniteReflect.agda` reflects a narrower or clearer
  fixed-point contract than it does now
- `agda Theorems/Abstract/SymmetryFiniteReflect.agda` still passes
- `agda Theorems/Abstract/WindowCertificate.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/SymmetryFiniteReflect.agda
agda Theorems/Abstract/WindowCertificate.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer narrowing the fixed-point classification contract before attempting a
  full constructive modular-arithmetic proof of the reflection layer

Completed:
- split the old mixed `ReflectFixedPointExclusion` witness in
  `Theorems/Abstract/SymmetryFiniteReflect.agda` into
  `HalfTurnFixedPointClassification` for the arithmetic side and
  `ObservedFixedPointExclusion` for the observed-support side
- renamed the remaining arithmetic postulates to
  `half-turn-zero-fixed` and `half-turn-fixed-points-are-zero-or-mid`
- added `canonicalHalfTurnFixedPointClassification` and rewired the
  parameterized certification wrappers to use the split fixed-point contracts
  explicitly
- updated the active Agda truth surfaces so the next certification boundary is
  the half-turn arithmetic shell itself, not a mixed residue/arithmetic witness

### Track 69: Half-Turn Arithmetic Shell Clarification

Status: `complete`

Why this matters:
- after Track 68, the remaining boundary in `SymmetryFiniteReflect.agda` is
  explicit and narrow: two arithmetic postulates about the half-turn
  reflection
- the next highest-signal move is to sharpen those arithmetic contracts
  further, or recover one small helper around them, without re-blurring the
  certification lane

Todo:
- [x] inspect whether `half-turn-zero-fixed` can be derived from a smaller
      arithmetic lemma or whether it should remain a standalone shell theorem
- [x] inspect whether `half-turn-fixed-points-are-zero-or-mid` can be split
      into a more local fixed-point exclusion helper
- [x] update the certification-stack docs so the remaining arithmetic shell is
      named precisely

Acceptance criteria:
- `Theorems/Abstract/SymmetryFiniteReflect.agda` presents a narrower or clearer
  arithmetic shell than it does after Track 68
- `agda Theorems/Abstract/SymmetryFiniteReflect.agda` still passes
- `agda Theorems/Abstract/WindowCertificate.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/SymmetryFiniteReflect.agda
agda Theorems/Abstract/WindowCertificate.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer narrower arithmetic shell theorems before attempting a full
  constructive proof of half-turn modular fixed-point classification

Completed:
- narrowed the exported arithmetic contract in
  `Theorems/Abstract/SymmetryFiniteReflect.agda` from the broader split record
  down to the simpler `HalfTurnFixedPointClassifier`
- kept `half-turn-zero-fixed` as an internal prerequisite used only by the
  canonical classifier helper, which makes the external arithmetic shell
  smaller and more honest
- added `observedFixedPointExclusion` as a constructive helper from classifier
  plus observed support exclusion to per-residue exclusion
- rewired the parameterized certification wrappers and active docs to use the
  classifier-focused interface consistently

### Track 70: Half-Turn Zero-Fixed Shell Positioning

Status: `complete`

Why this matters:
- after Track 69, the remaining arithmetic shell is clearer: the exported
  boundary is the fixed-point classifier, while `half-turn-zero-fixed` remains
  an internal arithmetic prerequisite
- the next clean move is to decide whether `half-turn-zero-fixed` should stay as
  a standalone shell theorem or be folded behind a more local internal helper

Todo:
- [x] inspect whether `half-turn-zero-fixed` should remain an independently
      named shell theorem or be moved behind a smaller canonical-constructor
      helper
- [x] if it remains public in the file, tighten the prose so its internal-only
      role is obvious
- [x] update the certification-stack docs so the remaining arithmetic shell is
      described in one place and without duplicate terminology

Acceptance criteria:
- `Theorems/Abstract/SymmetryFiniteReflect.agda` presents a clearer internal vs
  external role for `half-turn-zero-fixed`
- `agda Theorems/Abstract/SymmetryFiniteReflect.agda` still passes
- `agda Theorems/Abstract/WindowCertificate.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/SymmetryFiniteReflect.agda
agda Theorems/Abstract/WindowCertificate.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer clearer shell positioning before attempting a full constructive proof
  of the half-turn arithmetic

Completed:
- moved `half-turn-zero-fixed` into a private postulate block inside
  `Theorems/Abstract/SymmetryFiniteReflect.agda`
- tightened the file comments so the exported arithmetic shell is clearly just
  `HalfTurnFixedPointClassifier`, with `half-turn-zero-fixed` used only by the
  canonical helper path internally
- updated the active Agda truth surfaces to describe the external arithmetic
  boundary in one place and without duplicate terminology

### Track 71: Half-Turn Classifier Shell Reduction

Status: `complete`

Why this matters:
- after Track 70, the remaining public arithmetic shell in
  `SymmetryFiniteReflect.agda` is a single focused theorem surface:
  `HalfTurnFixedPointClassifier`
- the next clean move is to see whether that classifier can be split into a
  narrower local helper or reduced by one constructive arithmetic step without
  reopening the certification boundary

Todo:
- [x] inspect whether `half-turn-fixed-point-classifier` can be decomposed into
      a smaller helper that better matches the observed-support exclusion use
      case
- [x] recover one constructive sub-lemma if a small arithmetic step is already
      latent in the current reflection setup
- [x] update the certification-stack docs so the remaining public arithmetic
      shell is described precisely

Acceptance criteria:
- `Theorems/Abstract/SymmetryFiniteReflect.agda` presents a narrower or more
  informative public classifier shell than it does after Track 70
- `agda Theorems/Abstract/SymmetryFiniteReflect.agda` still passes
- `agda Theorems/Abstract/WindowCertificate.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/SymmetryFiniteReflect.agda
agda Theorems/Abstract/WindowCertificate.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer one smaller classifier helper or one honest arithmetic sub-lemma over a
  broad attempt at fully constructive modular fixed-point classification

Completed:
- added `ObservedFixedPointClassifier` plus the helper path
  `observedFixedPointClassifierFromClassifier` /
  `canonicalObservedFixedPointClassifier` inside
  `Theorems/Abstract/SymmetryFiniteReflect.agda`
- changed `observedFixedPointExclusion` and
  `observedResiduesMoveFromFixedPointContracts` to consume the observed-only
  classifier rather than the full arithmetic classifier
- added `observedResiduesMoveFromObservedSupportExclusion`, then rewired
  `Examples/CertifiedResonanceParam.agda` and
  `Examples/CertifiedResonanceParamDyn.agda` to drop their explicit classifier
  argument and consume only observed support exclusion
- updated the active Agda truth surfaces so the certification lane now states
  the narrower boundary honestly

### Track 72: WindowCertificate Example Shell Extraction

Status: `complete`

Why this matters:
- after Track 71, the sharpest remaining local postulate in the certification
  stack is no longer the builder path; it is the hypothetical `Example-Base14`
  shell bundled inside `WindowCertificate.agda`
- extracting or isolating that example shell would let the actual dual-certificate
  builder surface stand on its own and may promote `WindowCertificate.agda`
  from `with local postulates` to `clean-local`

Todo:
- [x] inspect whether `Example-Base14` should move into a separate example
      wrapper file or into an archive/example note so the core builder module
      is not carrying example-shell postulates
- [x] if the example stays nearby, narrow its assumptions and make its
      hypothetical status explicit in code and docs
- [x] update the certification-stack docs to reflect the resulting builder vs
      example boundary precisely

Acceptance criteria:
- `Theorems/Abstract/WindowCertificate.agda` carries a narrower or cleaner local
  postulate boundary than it does after Track 71
- `agda Theorems/Abstract/WindowCertificate.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/WindowCertificate.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer extracting or isolating the hypothetical example shell before trying
  to prove more of the half-turn arithmetic constructively

Completed:
- extracted the hypothetical `Example-Base14` shell out of
  `Theorems/Abstract/WindowCertificate.agda`
- preserved that shell as
  `Examples/WINDOW_CERTIFICATE_BASE14_SKETCH.md` so the usage shape remains
  visible without keeping local postulates inside the builder module
- promoted `Theorems/Abstract/WindowCertificate.agda` to a clean-local builder
  surface and added it to the maintained clean spine
- updated the active Agda truth surfaces so the certification stack now states
  that split honestly

### Track 73: CertifiedResonanceParamDyn Runtime Shell Extraction

Status: `complete`

Why this matters:
- after Track 72, the sharpest remaining local postulate inside the active
  certification lane is the runtime `proof-stable` shell in
  `Examples/CertifiedResonanceParamDyn.agda`
- extracting or isolating that runtime example shell would let the static dual
  wrapper surface stand on its own, just as Track 72 did for
  `WindowCertificate.agda`

Todo:
- [x] inspect whether the local runtime `proof-stable` witness should move into
      a separate sketch/example note or a narrower dynamic example wrapper
- [x] if the example stays nearby, make the runtime-only assumption boundary
      explicit in code and docs without implying the wrapper itself is still
      assumption-heavy
- [x] update the certification-stack docs and maintained boundary notes to
      reflect the resulting static-vs-runtime split precisely

Acceptance criteria:
- `Examples/CertifiedResonanceParamDyn.agda` carries a narrower or cleaner local
  postulate boundary than it does after Track 72
- `agda Examples/CertifiedResonanceParamDyn.agda` still passes
- `agda Theorems/Abstract/WindowCertificate.agda` still passes
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Examples/CertifiedResonanceParamDyn.agda
agda Theorems/Abstract/WindowCertificate.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer extracting the local runtime example shell before attempting a deeper
  constructive proof of orbital stability generation

Completed:
- extracted the old `Example-Base6-Dual` runtime shell out of
  `Examples/CertifiedResonanceParamDyn.agda`
- preserved that shell as
  `Examples/CERTIFIED_RESONANCE_PARAM_DYN_BASE6_SKETCH.md` so the dynamic usage
  shape stays visible without keeping `proof-stable` inside the wrapper module
- promoted `Examples/CertifiedResonanceParamDyn.agda` to a clean-local boundary
  wrapper over `SymmetryFiniteReflect.agda`
- updated the active Agda truth surfaces so the next real certification
  bottleneck is back in the half-turn arithmetic shell

### Track 74: SymmetryFiniteReflect Arithmetic Shell Reduction

Status: `complete`

Why this matters:
- after Track 73, the sharpest remaining local postulate in the active
  certification lane is no longer a wrapper/example shell; it is the
  half-turn arithmetic classifier inside `SymmetryFiniteReflect.agda`
- that means the next highest-signal work is finally back on theorem surface
  reduction, not shell extraction

Todo:
- [x] inspect whether `half-turn-fixed-point-classifier` can be split again
      into a smaller arithmetic lemma plus a thinner exported classifier shell
- [x] recover one constructive foothold if there is a low-cost arithmetic step
      already latent in the reflection setup
- [x] update the certification-stack docs so the remaining postulated boundary
      is described as precisely as possible

Acceptance criteria:
- `Theorems/Abstract/SymmetryFiniteReflect.agda` carries a narrower or more
  informative arithmetic shell than it does after Track 73
- `agda Theorems/Abstract/SymmetryFiniteReflect.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/SymmetryFiniteReflect.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer one narrower arithmetic bridge or helper over a broad attempt at full
  constructive half-turn classification in one step

Completed:
- made the broad half-turn classifier internal to
  `Theorems/Abstract/SymmetryFiniteReflect.agda`
- reduced the remaining internal postulate from a function-level classifier to
  the pointwise theorem `half-turn-fixed-point-case`
- corrected the certification API so the wrappers now require an explicit
  `HalfTurnMidpoint mid` witness instead of relying on a hidden universal
  zero-fixed assumption
- updated the active Agda truth surfaces to reflect that narrower and more
  honest arithmetic boundary

### Track 75: Canonical Half-Turn Witness Recovery

Status: `complete`

Why this matters:
- after Track 74, the active wrappers have the right contract, but they still
  need callers to provide `HalfTurnMidpoint mid` explicitly
- the next high-signal step is to recover one constructive helper for the
  canonical midpoint choices the repo actually cares about, instead of leaving
  all midpoint witnessing manual

Todo:
- [x] recover a narrow constructive helper proving `HalfTurnMidpoint mid` for
      the canonical even-base midpoint choices used by the certification
      pipeline
- [x] expose canonical-even convenience entry points so standard callers no
      longer pass the midpoint witness manually
- [x] update the certification-stack docs so the next arithmetic bottleneck is
      recorded precisely

Acceptance criteria:
- `Theorems/Abstract/SymmetryFiniteReflect.agda` carries a narrower or more
  informative midpoint-witness surface than it does after Track 74
- `agda Theorems/Abstract/SymmetryFiniteReflect.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/SymmetryFiniteReflect.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer recovering one concrete midpoint witness helper over attempting the
  full remaining arithmetic theory in one pass

Completed:
- added `canonicalEvenMidpoint` and constructive
  `canonicalEvenHalfTurnMidpoint` in
  `Theorems/Abstract/SymmetryFiniteReflect.agda`
- added canonical-even convenience entry points in
  `Examples/CertifiedResonanceParam.agda` and
  `Examples/CertifiedResonanceParamDyn.agda`
- narrowed the next arithmetic bottleneck to the remaining internal pointwise
  fixed-residue case theorem in `SymmetryFiniteReflect.agda`

### Track 76: Half-Turn Fixed-Residue Case Split

Status: `complete`

Why this matters:
- after Track 75, the standard even-base midpoint witness is constructive, so
  the only real arithmetic shell left in this lane is the pointwise fixed-case
  theorem inside `SymmetryFiniteReflect.agda`
- splitting or narrowing that theorem is the cleanest way to make the
  remaining open burden more legible

Todo:
- [x] split `half-turn-fixed-point-case` into a smaller internal arithmetic
      shell
- [x] recover any cheap constructive fixed-point helper exposed by that split
- [x] update the certification-stack docs to reflect the narrower arithmetic
      boundary

Acceptance criteria:
- `Theorems/Abstract/SymmetryFiniteReflect.agda` carries a narrower or more
  informative internal fixed-point shell than it does after Track 75
- `agda Theorems/Abstract/SymmetryFiniteReflect.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/SymmetryFiniteReflect.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer shrinking the remaining arithmetic shell over adding more wrapper-side
  convenience APIs

Completed:
- proved `reflect mid mid` constructively in
  `Theorems/Abstract/SymmetryFiniteReflect.agda`
- replaced the broad internal shell
  `half-turn-fixed-point-case` with the narrower theorem
  `half-turn-fixed-point-nonzero-is-mid`
- rebuilt the exported fixed-point classifier by a constructive `r ≟ finZero`
  split, so the zero branch is no longer part of the imported arithmetic burden

### Track 77: Reflection Involution Narrowing

Status: `complete`

Why this matters:
- after Track 76, the remaining fixed-point shell is smaller and more honest,
  but the certification lane still relies on the general `reflect-involutive`
  theorem through `mkSymReflect`
- that makes `reflect-involutive` the next clean arithmetic bottleneck in the
  active symmetry stack

Todo:
- [x] inspect whether `reflect-involutive` can be proved constructively for the
      current `reflect` definition
- [x] recover the theorem constructively in the symmetry core
- [x] update the Agda status surfaces so the remaining symmetry-law shell is
      stated precisely

Acceptance criteria:
- `Theorems/Abstract/SymmetryFiniteReflect.agda` carries a narrower or more
  informative involution shell than it does after Track 76
- `agda Theorems/Abstract/SymmetryFiniteReflect.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/SymmetryFiniteReflect.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer recovering or shrinking the general involution law before adding more
  certification-layer convenience wrappers

Completed:
- recovered `reflect-involutive` constructively in
  `Theorems/Abstract/SymmetryFiniteReflect.agda`
- added the maintained helper `reflect-value` so the involution proof is
  expressed through a stable `toℕ` view of reflection
- removed the symmetry-law burden from `mkSymReflect`, leaving the nonzero
  fixed-point classification theorem as the final arithmetic shell

### Track 78: Nonzero Fixed-Point Classification Recovery

Status: `complete`

Why this matters:
- after Track 77, the only remaining local postulate in
  `SymmetryFiniteReflect.agda` is the theorem that any nonzero fixed residue
  must equal `mid`
- proving or narrowing that theorem would collapse the last arithmetic shell in
  the active certification lane

Todo:
- [x] inspect whether `half-turn-fixed-point-nonzero-is-mid` can be recovered
      constructively from the current reflection arithmetic
- [x] replace the false generic theorem with an explicit
      `ObservedFixedPointClassifier` contract for noncanonical callers
- [x] recover the standard even-base classifier constructively
- [x] update the Agda status surfaces to reflect the collapsed symmetry shell

Acceptance criteria:
- `Theorems/Abstract/SymmetryFiniteReflect.agda` carries a narrower or more
  informative nonzero fixed-point shell than it does after Track 77
- `agda Theorems/Abstract/SymmetryFiniteReflect.agda` still passes
- `agda Examples/CertifiedResonanceParam.agda` and
  `agda Examples/CertifiedResonanceParamDyn.agda` still pass
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/SymmetryFiniteReflect.agda
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer collapsing the remaining fixed-point shell over shifting effort back
  to wrapper convenience or narrative work

Completed:
- removed the last local postulate block from
  `Theorems/Abstract/SymmetryFiniteReflect.agda`
- recovered the standard even-base nonzero fixed-point classifier
  constructively and exposed `canonicalEvenObservedFixedPointClassifier`
- changed the generic certification wrappers to consume an explicit
  `ObservedFixedPointClassifier` contract instead of the old hidden half-turn
  theorem path
- promoted `SymmetryFiniteReflect.agda` from `with local postulates` to
  `clean-local`

### Track 79: Dynamic Witness Contract Reduction

Status: `complete`

Why this matters:
- the symmetry side of the active certification lane is now locally clean
- the sharpest remaining boundary in the dual certificate path is the external
  `StableOrbital` witness consumed by `CertifiedResonanceParamDyn.agda` and
  `WindowCertificate.agda`

Todo:
- [x] inspect whether one common finite-window `StableOrbital` helper can be
      recovered constructively
- [x] split the dynamic contract into the smaller named obligation
      `PointwiseSafe`, then derive `StableOrbital` internally
- [x] update the active wrappers/docs to match the sharper dynamic boundary

Acceptance criteria:
- the active dynamic certification lane carries a narrower or more informative
  witness boundary than it does after Track 78
- `agda Theorems/Abstract/WindowCertificate.agda` still passes
- `agda Examples/CertifiedResonanceParamDyn.agda` still passes
- all 81 Agda modules still compile afterward

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/WindowCertificate.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- keep the new symmetry-side cleanliness intact while reassessing the dynamic
  witness shell

Completed:
- added `PointwiseSafe` plus conversions in
  `Theorems/Abstract/ConstrainedOrbitals.agda`
- rewired `WindowCertificate.agda` to consume `PointwiseSafe` and derive
  `StableOrbital` internally
- rewired `CertifiedResonanceParamDyn.agda` so its dynamic side now consumes
  `PointwiseSafe` instead of raw `StableOrbital`
- kept the active certification lane clean-local while making the dynamic
  contract smaller and more informative

### Track 80: PointwiseSafe Construction Helpers

Status: `complete`

Why this matters:
- Track 79 made the dynamic boundary sharper, but external generators still
  have to build `PointwiseSafe` manually
- the next useful gain is ergonomic: recover one or two maintained helper paths
  for common finite-window evidence shapes

Todo:
- [x] inspect whether a list-level or finite-list helper can build
      `PointwiseSafe` from simpler per-position evidence
- [x] expose a maintained helper in
      `Theorems/Abstract/ConstrainedOrbitals.agda`
- [x] update the usage notes in the dual certification lane

Acceptance criteria:
- the dynamic certification lane has a more ergonomic constructive helper than
  it does after Track 79, or the exact reason it does not is documented
- `agda Theorems/Abstract/ConstrainedOrbitals.agda` still passes
- `agda Theorems/Abstract/WindowCertificate.agda` still passes
- `agda Examples/CertifiedResonanceParamDyn.agda` still passes

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/ConstrainedOrbitals.agda
agda Theorems/Abstract/WindowCertificate.agda
agda Examples/CertifiedResonanceParamDyn.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer a small maintained helper over introducing a larger parallel API layer

Completed:
- added `pointwiseSafeNil`, `pointwiseSafeCons`, `pointwiseSafeSingleton`, and
  `pointwiseSafeFromAll` in `Theorems/Abstract/ConstrainedOrbitals.agda`
- rewired the internal conversions to use that maintained helper surface
- updated the active Agda docs so the dynamic boundary is described as
  `PointwiseSafe` plus a small maintained constructor path

### Track 81: PointwiseSafe Regression Alignment

Status: `complete`

Why this matters:
- the dynamic certification lane now has a cleaner input contract and helper
  surface
- the next small win is to make the executable regression shell reflect that
  contract directly instead of staying only at the boolean-check level

Todo:
- [x] inspect whether `Tests/InvariantTests.agda` should gain one small
      `PointwiseSafe`-oriented regression witness
- [x] add the narrowest useful examples without turning the test shell into a
      second theorem module
- [x] update status notes to reflect the sharper dynamic regression shell

Acceptance criteria:
- either the invariant test shell exposes a useful `PointwiseSafe`-aligned
  witness, or the reason not to add one is made explicit
- `agda Tests/InvariantTests.agda` still passes
- `agda Theorems/Abstract/ConstrainedOrbitals.agda` still passes
- `agda Theorems/Abstract/WindowCertificate.agda` still passes

Verification:
```bash
cd agda-proofs
agda Tests/InvariantTests.agda
agda Theorems/Abstract/ConstrainedOrbitals.agda
agda Theorems/Abstract/WindowCertificate.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- keep the helper surface small and avoid turning the regression shell into a
  parallel proof API

Completed:
- added one direct positive `PointwiseSafe` witness for the base-7 pair case
- added one direct negative `InZone` witness for the midpoint-violating case
- kept `Tests/InvariantTests.agda` as a lightweight regression shell rather
  than expanding it into a second theorem layer

### Track 82: Dynamic Helper Consumption Notes

Status: `complete`

Why this matters:
- the dynamic contract and helper surface are now sharper
- the next small hardening step is to make the generated-usage notes reflect
  the new `PointwiseSafe` helper path more concretely

Todo:
- [x] inspect the example scaffolds and generated-usage notes for stale
      "prove it by hand" wording around `PointwiseSafe`
- [x] show one short helper-driven construction snippet
- [x] keep the change documentation-only unless a code helper is genuinely
      missing

Acceptance criteria:
- the active dynamic usage notes match the current helper surface
- `./tools/check_active_doc_drift.sh` still passes
- no Agda module regresses

Verification:
```bash
cd agda-proofs
agda Tests/InvariantTests.agda
agda Theorems/Abstract/ConstrainedOrbitals.agda
agda Theorems/Abstract/WindowCertificate.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- keep the next step documentation-sized unless it reveals a genuinely missing
  helper

Completed:
- updated the extracted Base-6 dual sketch to show a helper-driven
  `PointwiseSafe` witness shape instead of a raw `StableOrbital` witness
- updated the extracted Base-14 window sketch so the generated usage path now
  targets `PointwiseSafe` directly
- aligned the active Agda notes so the dynamic usage story matches the current
  helper surface

### Track 83: Dynamic Usage Snippet Anchoring

Status: `complete`

Why this matters:
- the helper-driven dynamic story is now correct in the extracted notes
- the next small hardening step is to anchor one active usage surface to the
  exact helper names so future note drift is easier to catch

Todo:
- [x] inspect whether one short helper-oriented note belongs in
      `Theorems/Abstract/ConstrainedOrbitals.agda` or
      `Theorems/Abstract/WindowCertificate.agda`
- [x] keep it brief and avoid turning commentary into a second tutorial layer
- [x] update the status surfaces only if the active boundary description
      changes

Acceptance criteria:
- at least one active dynamic usage surface names the maintained helper path
  directly in a way that would catch future drift
- `agda Theorems/Abstract/ConstrainedOrbitals.agda` still passes
- `agda Theorems/Abstract/WindowCertificate.agda` still passes
- `./tools/check_active_doc_drift.sh` still passes

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/ConstrainedOrbitals.agda
agda Theorems/Abstract/WindowCertificate.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- prefer one short anchor note over adding more extracted sketches

Completed:
- anchored the maintained helper names directly in
  `Theorems/Abstract/ConstrainedOrbitals.agda`
- added a matching short note beside `buildDualCertificate` in
  `Theorems/Abstract/WindowCertificate.agda`
- aligned `agda-proofs/README.md` so the active dynamic usage story names the
  helper path concretely

### Track 84: Dynamic Helper Regression Note Guard

Status: `complete`

Why this matters:
- the active helper path is now named concretely
- the next small hardening step is to make sure the regression shell and notes
  stay aligned with that exact helper vocabulary

Todo:
- [x] inspect whether `Tests/InvariantTests.agda` should carry one short
      comment tying its witness shape to the maintained helper path
- [x] keep it comment-sized unless it reveals a real regression gap
- [x] update docs only if the active dynamic vocabulary changes

Acceptance criteria:
- one active regression-oriented surface names the maintained helper path
  explicitly
- `agda Tests/InvariantTests.agda` still passes
- `./tools/check_active_doc_drift.sh` still passes

Verification:
```bash
cd agda-proofs
agda Tests/InvariantTests.agda
agda Theorems/Abstract/ConstrainedOrbitals.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- keep this as a note-level guard unless a real missing regression shows up

Completed:
- added a short regression note in `Tests/InvariantTests.agda` tying the
  positive witness shape directly to `pointwiseSafeCons` / `pointwiseSafeNil`
- kept the change comment-sized; no theorem surface or dynamic API changed
- left the status surfaces otherwise stable because the dynamic boundary did not
  change

### Track 85: Dynamic Counterexample Note Alignment

Status: `complete`

Why this matters:
- the positive regression witness now names the maintained helper path
- the next small guard is to make the negative `InZone` example read just as
  intentionally, so the regression shell shows both sides of the dynamic API

Todo:
- [x] inspect whether the midpoint-violating branch in
      `Tests/InvariantTests.agda` should carry one short note about why it
      stays helper-agnostic
- [x] keep it comment-sized unless it exposes a missing regression helper
- [x] update docs only if the active negative-path vocabulary changes

Acceptance criteria:
- one active regression-oriented surface explains the negative dynamic witness
  path just as explicitly as the positive helper path
- `agda Tests/InvariantTests.agda` still passes
- `./tools/check_active_doc_drift.sh` still passes

Verification:
```bash
cd agda-proofs
agda Tests/InvariantTests.agda
./scripts/verify-clean-spine.sh
```

Assumptions:
- keep this as a note-level guard unless it uncovers a real gap in the dynamic
  regression shell

Completed:
- added a short negative-path note in `Tests/InvariantTests.agda` explaining
  why the midpoint-violating branch stays helper-agnostic
- kept the change comment-sized; no theorem surface or dynamic API changed
- left the formal boundary untouched while making the regression intent more
  explicit

### Track 86: Dynamic Regression Status Surface Alignment

Status: `complete`

Why this matters:
- once both positive and negative regression branches are intentional, the
  status surfaces should say so plainly
- this keeps the dynamic regression shell from drifting back toward vague
  “has an example” wording

Todo:
- [x] align the status and signal surfaces with the positive-helper /
      negative-helper-agnostic split
- [x] keep the change wording-only
- [x] avoid changing counts or theorem classifications

Acceptance criteria:
- active status notes describe the regression shell in the same two-sided
  vocabulary used in `Tests/InvariantTests.agda`
- `./tools/check_active_doc_drift.sh` still passes
- no Agda module regresses

Verification:
```bash
cd agda-proofs
agda Tests/InvariantTests.agda
./scripts/verify-clean-spine.sh
cd ..
./tools/check_active_doc_drift.sh
```

Assumptions:
- keep this as wording alignment only

Completed:
- updated `agda-proofs/STATUS.md` to describe the positive helper-path witness
  and the helper-agnostic negative counterexample explicitly
- updated `agda-proofs/SIGNAL_MAP.md` to match that same two-sided regression
  vocabulary

### Track 87: Dynamic Helper/Counterexample README Anchor

Status: `complete`

Why this matters:
- the regression shell and status notes now agree on the two-sided dynamic
  story
- the next small guard is to decide whether the workspace README should name
  that split explicitly or continue to stay one level higher

Todo:
- [x] inspect whether one short README sentence should mention the positive
      helper-path / negative helper-agnostic regression split
- [x] keep it brief and avoid turning the README into a changelog
- [x] only update the README if it adds real drift resistance

Acceptance criteria:
- either the README gains a useful one-line anchor for the dynamic regression
  split, or the reason to leave it higher-level is made clear in the track
  notes
- `./tools/check_active_doc_drift.sh` still passes

Verification:
```bash
./tools/check_active_doc_drift.sh
```

Assumptions:
- prefer leaving the README higher-level if the status surfaces already carry
  enough detail

Completed:
- added one short README anchor naming the dynamic regression shell as a
  positive helper-path `PointwiseSafe` witness plus a helper-agnostic negative
  `InZone` counterexample
- kept the README at the same level otherwise; the detailed wording still lives
  in `STATUS.md` and `SIGNAL_MAP.md`

### Track 88: Testing Strategy Dynamic Regression Anchor

Status: `complete`

Why this matters:
- the README now names the two-sided dynamic regression split
- the next small guard is to decide whether `Tests/TESTING_STRATEGY.md` should
  echo that split for collaborators who go there first

Todo:
- [x] inspect whether one short testing-strategy note should mention the
      positive helper-path / negative helper-agnostic dynamic regression shape
- [x] keep it brief and avoid duplicating the full status wording
- [x] only update the file if it adds real drift resistance

Acceptance criteria:
- either `Tests/TESTING_STRATEGY.md` gains a useful one-line anchor for the
  dynamic regression split, or the reason to leave it unchanged is made clear
  in the track notes
- `./tools/check_active_doc_drift.sh` still passes

Verification:
```bash
./tools/check_active_doc_drift.sh
```

Assumptions:
- prefer one short anchor over restating the full regression narrative

Completed:
- added `Tests/InvariantTests.agda` back to the audited clean test list in
  `Tests/TESTING_STRATEGY.md`
- added one short testing-strategy note describing the positive helper-path /
  negative helper-agnostic dynamic regression split
- aligned the local verification block so it includes `agda Tests/InvariantTests.agda`

### Track 89: Testing Strategy Spine Boundary Note

Status: `complete`

Why this matters:
- the testing strategy file now names the dynamic regression shell correctly
- the next small guard is to make sure the file stays clear about the
  difference between the safe test surfaces and the broader clean test surface

Todo:
- [x] inspect whether one short note should clarify why
      `Tests/InvariantTests.agda` is audited and clean-local but not part of
      the `--safe` subgroup
- [x] keep it brief and avoid turning the file into a second status table
- [x] only update the file if the distinction is likely to confuse collaborators

Acceptance criteria:
- either `Tests/TESTING_STRATEGY.md` gains a useful one-line note about the
  safe-vs-clean-local test boundary, or the reason to leave it as-is is made
  clear in the track notes
- `./tools/check_active_doc_drift.sh` still passes

Verification:
```bash
./tools/check_active_doc_drift.sh
```

Assumptions:
- prefer one clarifying note over repeating the full clean-spine methodology

Completed:
- added one short note in `Tests/TESTING_STRATEGY.md` clarifying that the file
  tracks the audited clean-local test surface, not only the `--safe` subgroup
- explained why `Tests/InvariantTests.agda` is listed and verified with plain
  `agda`
- kept the change note-level instead of expanding the file into another status table

### Track 90: Testing Strategy Safe Subgroup Label

Status: `complete`

Why this matters:
- the file now explains the clean-local vs `--safe` boundary in prose
- the next small guard is to decide whether the local verification block should
  visually separate the `--safe` subgroup from the broader clean-local checks

Todo:
- [x] inspect whether a short comment in the verification block would reduce
      ambiguity without making the file busier
- [x] keep it formatting-level unless a stronger wording issue appears
- [x] only update the file if it clearly improves scanability

Acceptance criteria:
- either the verification block gains a useful visual cue for the `--safe`
  subgroup boundary, or the reason to leave it as-is is made clear in the
  track notes
- `./tools/check_active_doc_drift.sh` still passes

Verification:
```bash
./tools/check_active_doc_drift.sh
```

Assumptions:
- prefer a tiny scanability cue over more prose if the current block still feels mixed

Completed:
- added a visual `# Safe subgroup` label in the verification block
- moved `agda Tests/InvariantTests.agda` under its own audited clean-local label
- kept the change formatting-level; no counts or classifications changed
