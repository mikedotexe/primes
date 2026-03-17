# Hardening Roadmap

**Created**: 2026-03-09
**Last Assessment**: 2026-03-09

## Repository State Summary

### What is strong

1. **Core algorithms work.** The BitSieve, Miller-Rabin, and membrane generation
   code passes 142 of 143 library tests. The prime counting smoke test produces
   correct counts against OEIS A000720 references up to pi(10^7).

2. **The empirical findings are real.** Membrane constructions in base 6 with
   (1,5) k=(0,0) genuinely produce primes at ~33% density vs ~5% random baseline.
   The minimal-padding principle (k=0 dominance for M >= 2) is verified across
   multiple bases with n=1000 samples and significance levels p < 0.001.

3. **The Hardy-Littlewood framework (`hzlib/`) is well-implemented.** Sieve,
   singular series, truncated expectations, Hedges' g, Cliff's delta, Spearman
   rho, Benjamini-Hochberg correction, and linear regression with CIs are all
   present and tested.

4. **The `VERIFIED_FACTS_VS_SPECULATION.md` file demonstrates intellectual
   honesty.** Falsified hypotheses are documented alongside verified findings,
   with falsifiability criteria stated upfront.

5. **The connector module** (untracked, `src/connector/`) is cleanly written with
   overflow-safe u128 arithmetic and good module organization.

### What needs work

1. **Documentation has metastasized.** There are 113 markdown files across the
   repo, 45 at the root level alone. The `DOCUMENTATION_MAP.md` references at
   least 20 files that do not exist. Many docs are session summaries or
   exploration journals rather than maintained references.

2. **There is 1 failing test** (`hzlib::density::tests::test_find_band`). The
   binary search in `BaseAccum::find_band` has an off-by-one error: it returns
   `lo - 1` when it should return `lo`.

3. **The example count is out of control.** 218 example `.rs` files exist (174 in
   `examples/`, 7 in `examples/experimental/`, rest in subdirectories). The docs
   claim "46+ working examples" but there are 4.7x that many. An earlier
   `DEPRECATION_PLAN.md` identified 15 for deletion but the cleanup was never
   completed.

4. **The lib.rs doc header has stale claims.** It mentions a `simd` feature that
   does not exist in `Cargo.toml`. It claims "GPU acceleration: 50x speedup" with
   no benchmark to verify.

5. **The physics metaphor (`PrimeUniverse`, `GravitationalField`,
   `PhysicalConstants`) dominates the public API** despite being ancillary to the
   core value (membrane prime generation, sieve, HL analysis). The `light_speed`
   constant is set to 299792458.0 in prime space -- this is metaphor leaking into
   API design.

6. **There are 7 clippy warnings** in the library.

7. **Many source modules are untracked** (connector/, several collab files, many
   examples). The working tree is in a sprawling state.

### Claim Classification

| Claim | Status | Evidence |
|-------|--------|----------|
| Membrane (1,5) base 6 achieves ~33% prime density | `empirical` | Verified, n=1000, p<0.001 |
| k=0 dominates for M >= 2 | `empirical` | Verified across 5+ bases |
| Coprimality of boundary digits is essential | `empirical` | Verified empirically |
| Diameter-density law (compactness predicts primality) | `empirical` | Spearman rho > 0.77, p < 10^-20 |
| Directional asymmetry in prime connectors | `empirical` | Single pair tested |
| Lagrange point analogy | `metaphor` | The analogy is suggestive but unproven |
| "Physics engine" framing (gravity, tidal forces) | `metaphor` | Visualization metaphor, not physics |
| GPU 50x speedup | `overclaim` | No benchmark exists |
| `simd` feature | `stale` | Not in Cargo.toml |
| "46+ working examples" | `stale` | 218 exist, unclear how many compile |

---

## Tranche 1: Foundation Hardening

### Track 1: Fix the Failing Test and Clippy Warnings

**Status**: `complete`
**Priority**: P0
**Scope**: small

Why this matters:
- A failing test means the CI gate is broken. Nothing else should land until
  tests pass.
- 7 clippy warnings erode signal-to-noise in future development.

Todo:
- [ ] Fix off-by-one in `BaseAccum::find_band` (use `lo` not `lo - 1`)
- [ ] Fix 7 clippy warnings in library code
- [ ] Verify all 143 tests pass

Acceptance criteria:
- `cargo test --lib` exits 0 with 143 tests passing, 0 failing
- `cargo clippy --lib -- -D warnings` exits 0

Verification:
```bash
cargo test --lib
cargo clippy --lib -- -D warnings
```

Assumptions:
- We do not attempt to fix example compilation issues in this track
- We do not alter public API signatures

---

### Track 2: lib.rs Doc Accuracy

**Status**: `complete`
**Priority**: P0
**Scope**: small

Why this matters:
- The lib.rs module doc is the first thing a developer reads. It currently claims
  a `simd` feature that does not exist, a GPU speedup with no benchmark, and
  lists an architecture diagram that mixes physics metaphor with actual
  computation.

Todo:
- [ ] Remove `simd` from the feature table (it is not in Cargo.toml)
- [ ] Change GPU speedup claim to "experimental, benchmarks pending" or remove
- [ ] Update the feature table to match actual Cargo.toml features
- [ ] Clarify in the doc header which components are core math vs physics
      visualization metaphor

Acceptance criteria:
- Every feature listed in the lib.rs doc header exists in Cargo.toml
- No performance claim lacks a qualification ("measured", "estimated", or
  "pending benchmark")
- The doc header compiles without warnings: `cargo doc --lib 2>&1 | grep warning`

Verification:
```bash
# Extract features from lib.rs doc, verify each exists in Cargo.toml
cargo doc --lib --no-deps 2>&1 | grep -i warning
```

Assumptions:
- We are tightening claims, not removing features
- Physics metaphor stays for now but gets clearly labeled

---

### Track 3: Dead Documentation Audit

**Status**: `complete`
**Priority**: P1
**Scope**: medium

Why this matters:
- `DOCUMENTATION_MAP.md` references at least 20 files that do not exist. This is
  the worst kind of documentation: it promises a guide and delivers 404s. Anyone
  following these links wastes time and loses trust.
- 45 markdown files at the root level create a wall of noise. Many are session
  journals or exploration notes from November 2025, not maintained references.

What was done:
- Enumerated all links in DOCUMENTATION_MAP.md: 51 of 65 references were broken
  (78% failure rate). Removed the file entirely.
- Classified all 46 root-level markdown files into: (a) core docs referenced by
  CLAUDE.md or serving as maintained references, (b) session journals and
  exploration reports from Nov 2025, (c) feature-specific docs for stale or
  unmaintained features.
- Moved 31 files to `historical/docs/`: 18 session journals/exploration reports
  and 13 feature-specific docs (dashboards, animations, Lagrange guides, etc.)
- Fixed pre-existing broken links in CLAUDE.md (3 links to nonexistent files),
  README.md (8 links to nonexistent files), and GLOSSARY.md (3 links to
  nonexistent files).
- Root-level markdown count: 46 -> 14.

Acceptance criteria (all met):
- Every internal markdown link at repo root resolves to an existing file
- Root-level markdown file count is 14 (<= 15)
- No file named `SESSION_*.md` or `*_SUMMARY_2025*.md` remains at root level

Verification:
```bash
# Check for broken internal links
for f in *.md; do
  grep -oE '\[.*?\]\(\./[^)]+\)' "$f" | while read link; do
    target=$(echo "$link" | sed 's/.*(\.\///' | sed 's/)$//' | sed 's/#.*//')
    [ ! -f "$target" ] && echo "BROKEN in $f: $target"
  done
done

# Count remaining root markdown files
ls *.md | wc -l
```

Assumptions:
- Session journals are not deleted, only relocated to `historical/docs/`
- We did not rewrite doc content, only reorganized and removed dead links
- CLAUDE.md is the project's source of truth and was not moved

---

### Track 4: Example Triage

**Status**: `complete`
**Priority**: P1
**Scope**: large

Why this matters:
- 218 example files are unnavigable. A new contributor cannot tell which examples
  work, which are educational, and which are one-off exploration scripts. The
  existing `DEPRECATION_PLAN.md` identified 15 for removal but the work was
  never done.

What was done:
- Built all 174 top-level examples individually: 173 compiled, 1 failed
  (`prime_lint` -- uses `primes::fingerprint` with minor syntax errors).
- Categorized examples by reading doc comments: 32 kept as core/educational or
  active research tools, 142 moved to `historical/examples/`.
- The 32 kept examples cover: verification (4), membrane generation (9),
  Lagrange points/connectors (6), HL/statistical analysis (8), interactive TUI
  (3), and special-purpose tools (2).
- Merged the stale `examples/legacy/` subdirectory (12 files from an earlier
  incomplete cleanup) into `historical/examples/`.
- Moved stale `DEPRECATION_PLAN.md` and `CLEANUP_RESULTS.md` to
  `historical/examples/`.
- Moved `metal_membrane_host.swift` (not a Rust example) to
  `historical/examples/`.
- Rewrote `examples/README.md` with a curated table of all 32 examples grouped
  by category, quick-start section, and notes on subdirectories.
- `examples/experimental/` (7 files) and `examples/verified/` (25 files) left
  as-is.

Acceptance criteria (all met):
- Every example in `examples/` (not in a subdirectory) compiles with
  `cargo build --example <name>` -- verified, 32/32 pass
- `examples/README.md` exists and lists each top-level example with a one-line
  description -- yes, 32 entries in 6 tables
- Total top-level example count is <= 60 -- 32 examples

Verification:
```bash
# Verify all top-level examples compile
for f in examples/*.rs; do
  name=$(basename "$f" .rs)
  cargo build --example "$name" 2>/dev/null || echo "BROKEN: $name"
done
```

Assumptions:
- We did not fix broken examples; we relocated them
- The exploration examples are preserved in `historical/examples/` for reference
- `examples/experimental/` and `examples/verified/` remain as-is

---

### Track 5: Untracked Source Integration

**Status**: `assessed`
**Priority**: P2 -> P0 (upgraded: blocks any commit)
**Scope**: small

Why this matters:
- `src/connector/` is untracked but imported in `lib.rs`. This means the library
  compiles only because the files are present in the working tree but would fail
  for any fresh clone after a checkout.
- `src/hzlib/num_theory.rs` is also untracked but imported.
- `src/bin/orthogonal_landscape.rs` is untracked (auto-discovered binary).

Assessed findings:
- `lib.rs` line 112: `pub mod connector;` -> `src/connector/` (untracked)
- `hzlib/mod.rs` line 13: `pub mod num_theory;` -> `src/hzlib/num_theory.rs` (untracked)
- `src/bin/orthogonal_landscape.rs` is auto-discovered by Cargo (untracked)

Todo:
- [ ] `git add src/connector/ src/hzlib/num_theory.rs src/bin/orthogonal_landscape.rs`
- [ ] Include in next commit

Acceptance criteria:
- `cargo build --lib` succeeds from a clean checkout
- `git status src/ | grep '??'` returns empty

Verification:
```bash
git status src/ | grep '??'
cargo build --lib
```

Assumptions:
- These files must be tracked for the repo to build
- Will be included in the next commit alongside Tracks 1-2 fixes

---

## Tranche 1 Reassessment (2026-03-09)

**Strongest verified spine after Tranche 1:**
- 143 library tests pass. Clippy clean. lib.rs doc header is accurate.
- 14 root markdown files (down from 46). Zero broken internal links at root.
- 32 curated top-level examples, all compiling. Historical examples preserved.
- `VERIFIED_FACTS_VS_SPECULATION.md` is the repo's most rigorous document.

**Where public signal still exceeds actual support:**
1. **README.md is the biggest overclaim surface.** It claims "94 working examples"
   (actual: 36 compiling across all directories), references a `heritage/` directory
   that does not exist (actual: `historical/`), and contradicts itself within the
   same file (both "63" and "94" examples claimed at different points). The
   architecture diagram shows structures that don't match the actual directory layout.
2. **examples/verified/ is 96% broken.** 24 of 25 examples fail to compile. The
   directory name "verified" implies these work. They don't.
3. **examples/experimental/ is 57% broken.** 4 of 7 examples fail to compile.
4. **Agda verification has significant overclaims.** The CI workflow lists 11
   working modules. CERTIFICATION_COMPLETE.md and related docs describe a "complete
   certification framework" but the STATUS.md honestly notes only 11 of 85+ modules
   (12.9%) are verified. Many use postulates rather than proofs. The CI has never
   run successfully in GitHub Actions (Agda is not available via apt at the
   specified version on ubuntu-latest).
5. **collab/ folder is stale.** It references December 2025 exploration sessions
   and contains .rs files that duplicate src/ and examples/ content. It does not
   contain the repo's strongest findings in a self-contained form.

**What the next tranche should address:**
The README is the primary public surface. It is currently the least accurate
document in the repo. Fixing it requires first cleaning up the example
subdirectories it references, then rewriting the README from verified facts.

---

## Tranche 2: Public Surface Accuracy

**Created**: 2026-03-09

### Track 6: README.md Rewrite

**Status**: `complete`
**Priority**: P0
**Scope**: medium

Why this matters:
- README.md is the first thing any visitor reads. It currently makes 10+ false
  claims about example counts, directory structure, and feature status.
- It references a `heritage/` directory that does not exist.
- It claims "94 working examples" when 36 compile. It claims "63" at another point.
- The architecture diagram shows directories that don't match reality.
- Build instructions reference tools that may not work.
- After Tranche 1 hardened the doc layer and example set, the README is now the
  weakest public surface.

What was done:
- Rewrote README.md from scratch, using verified facts as the source of truth.
- Old README: 602 lines, 10+ overclaims, references nonexistent `heritage/` dir,
  self-contradictory example counts ("63" and "94" in same file), stale architecture
  diagram, 4 example commands that don't compile.
- New README: ~140 lines, every example command compiles (11/11 verified), every
  directory reference exists (14/14 verified), accurate counts, clear separation
  of math layer vs simulation metaphor.
- Also cleaned up Cargo.toml: removed 20 lines of stale example comments referencing
  examples that no longer exist at the top level.

Todo:
- [x] Audit every factual claim in README.md against repository reality
- [x] Rewrite README.md with accurate counts, correct directory references, and
      verified example lists
- [x] Remove or fix the architecture diagram to match actual src/ layout
- [x] Ensure every example command listed in README.md compiles and runs
- [x] Link to `examples/README.md` (the curated list) as the canonical example
      reference instead of making independent claims about counts

Acceptance criteria:
- Every example command in README.md compiles (`cargo build --example <name>` exits 0)
- Every directory referenced in README.md exists
- No numeric claim about example counts contradicts `examples/README.md` or `ls`
- `heritage/` is not mentioned (correct directory is `historical/`)
- The architecture diagram matches actual directories under `src/`

Verification:
```bash
# Extract all example names from README and test compilation
grep -oP 'cargo run --example \K\w+' README.md | sort -u | while read name; do
  cargo build --example "$name" 2>/dev/null || echo "BROKEN in README: $name"
done

# Check directory references
grep -oP '├── \K\w+/' README.md | sort -u | while read dir; do
  [ ! -d "${dir%/}" ] && echo "MISSING directory: $dir"
done
```

Assumptions:
- We rewrite README.md from scratch, using CLAUDE.md and VERIFIED_FACTS_VS_SPECULATION.md
  as sources of truth
- We do not add new examples or features, only accurately describe what exists
- The README should be short and accurate, not comprehensive -- it should point to
  CLAUDE.md for detailed research context

---

### Track 7: Subdirectory Example Cleanup

**Status**: `complete`
**Priority**: P1
**Scope**: medium

Why this matters:
- `examples/verified/` contains 25 examples, 24 of which are broken. The directory
  name implies correctness. This is actively misleading.
- `examples/experimental/` contains 7 examples, 4 of which are broken.
- The new untracked examples in `examples/` (from git status) need to be either
  integrated into the curated set or moved to historical.

What was done:
- `examples/verified/` (25 files): 24 broken, 1 compiling (membrane_lab_tui -- a
  duplicate of the top-level version). Moved all 25 to `historical/examples/` and
  removed the empty directory.
- `examples/experimental/` (7 files): 4 broken, 3 compiling (lagrange_clustering_verifier,
  membrane_lab_tui, statistical_sampling_demo -- all duplicates of top-level examples).
  Moved all 7 to `historical/examples/` and removed the empty directory.
- The 4 untracked top-level examples (belphegor_scanner, connector_utility_demo,
  membrane_vs_random, membrane_vs_random_fast) were already part of the curated 32
  from Tranche 1's Track 4. They compile and are documented in examples/README.md.
- Updated examples/README.md: removed subdirectory section, updated historical count.
- Verified all 32 top-level examples still compile (32/32 pass).

Todo:
- [x] Move all broken examples from `examples/verified/` to `historical/examples/`
- [x] Move all broken examples from `examples/experimental/` to `historical/examples/`
- [x] Triage untracked examples (4 files, all already curated)
- [x] Update `examples/README.md` to reflect final state
- [x] Remove empty subdirectories

Acceptance criteria:
- Every .rs file under `examples/` (including subdirectories) compiles
- `examples/README.md` accurately lists all examples
- No directory under `examples/` has a name that implies quality ("verified") if
  the contents don't meet that bar

Verification:
```bash
find examples -name '*.rs' | while read f; do
  name=$(basename "$f" .rs)
  cargo build --example "$name" 2>/dev/null || echo "BROKEN: $f"
done
```

Assumptions:
- Broken examples are relocated, not deleted
- We don't fix broken examples -- that's a different track
- The bar for "curated" is: compiles, has a doc comment explaining what it does,
  and demonstrates a feature that is not already covered by another example

---

### Track 8: Collab Folder Refresh

**Status**: `complete`
**Priority**: P1
**Scope**: small

Why this matters:
- The collab/ folder is described in CLAUDE.md as "somewhere to copy the most
  relevant files to share with our wonderful team of collaborators." Its current
  contents are from December 2025 exploration sessions and do not reflect the
  repo's current strongest findings.
- It contains .rs files that duplicate src/ and examples/ content, creating
  maintenance burden without adding value.
- A collaborator opening this folder today would get a stale snapshot.

What was done:
- Audited all 16 .rs files in collab/: all 16 are duplicates of files in
  src/hzlib/, src/bin/, examples/, or historical/examples/. Removed all 16.
- Kept all 4 markdown synthesis documents (THEORETICAL_CLOSURE.md,
  PRIMORIAL_MEMBRANE_OPTIMIZATION_GUIDE.md, EXPLORATION_SYNTHESIS.md,
  PERIOD6_RESONANCE_DISCOVERY.md) -- these contain unique synthesis.
- Rewrote collab/README.md to lead with the most important finding
  (THEORETICAL_CLOSURE.md: membrane efficiency is fully explained by classical
  coprimality filtering), link to the repo's strongest verified artifacts, and
  provide a clear "what we know / what remains open" summary.
- collab/ is now 5 files (4 synthesis docs + README.md), down from 21 files.

Todo:
- [x] Audit collab/ contents against current repo state
- [x] Replace collab/README.md with a current summary
- [x] Remove .rs files that duplicate tracked files (16 removed)
- [x] Keep unique synthesis documents (4 retained)

Acceptance criteria:
- collab/README.md is dated and accurate
- No .rs file in collab/ duplicates a tracked file in src/ or examples/
- A new collaborator reading collab/README.md can find the repo's strongest
  verified findings within 2 clicks

Verification:
```bash
# Check for .rs files in collab/ that duplicate tracked files
for f in collab/*.rs; do
  name=$(basename "$f")
  find src examples -name "$name" 2>/dev/null | head -1
done
```

Assumptions:
- collab/ is a curated snapshot, not a development directory
- Unique synthesis documents (EXPLORATION_SYNTHESIS.md, THEORETICAL_CLOSURE.md) are
  evaluated on merit, not auto-relocated

---

### Track 9: Agda Verification Ground Truth

**Status**: `complete`
**Priority**: P2
**Scope**: medium

Why this matters:
- The repo claims a "complete formal verification framework" with "machine-checked
  proofs." The STATUS.md honestly notes that only 11 of 85+ modules type-check,
  and several of those use postulates (which are axioms, not proofs).
- The CI workflow (.github/workflows/agda-verification.yml) installs Agda via apt
  on ubuntu-latest, which does not provide Agda 2.8.0. The CI has likely never
  passed.
- CERTIFICATION_COMPLETE.md and QUICK_START_VERIFICATION.md make claims about a
  "one-shot certification interface" that depends on postulated helpers.

What was done:
- Cleared Agda build cache and tested all 80 .agda modules individually.
- Ground truth: 18 pass clean (no postulates), 7 pass with postulates, 55 fail.
  Total: 25 of 80 (31.3%).
- Previous STATUS.md claimed 11 working. Of those 11: 4 genuinely pass
  (SymmetryImpliesRepulsion, TotientDensity, ConstrainedOrbitals, Specs/Tests),
  7 fail due to SymmetryFromList's unsolved meta at line 84.
- 14 additional modules that were listed as "untested" were found to pass.
- TotientDensity passes but has 39 postulate lines (practically nothing proven).
- Key bottleneck: SymmetryFromList.agda:84 unsolved meta. Fixing this one file
  would likely restore 7 dependent modules (BucketsAutoMatch, SymmetryFiniteReflect,
  WindowCertificate, CertifiedResonanceComplete, CertifiedResonanceParam,
  CertifiedResonanceParamDyn, plus SymmetryFromList itself).
- Rewrote STATUS.md with full module-by-module ground truth.
- Added correction header to CERTIFICATION_COMPLETE.md explaining the overclaims.
- Updated QUICK_START_VERIFICATION.md status note with accurate counts.
- Updated README.md Agda section with accurate counts.
- CI workflow fix deferred (requires infrastructure work beyond this track's scope).

Todo:
- [x] Run all "working" modules through `agda` locally and record pass/fail
- [x] For each passing module, note whether it uses postulates
- [x] Attempt compilation of all remaining untested modules
- [x] Update STATUS.md with ground truth
- [x] Update CERTIFICATION_COMPLETE.md with correction
- [ ] Fix the CI workflow (deferred -- requires stack/GHCup/Docker setup)

Acceptance criteria:
- STATUS.md lists every .agda file with one of: `passes`, `passes-with-postulates`,
  `fails`, `untested`
- The count of genuinely proven theorems (no postulates) is stated
- The CI workflow either passes or is marked as known-broken with an issue filed
- CERTIFICATION_COMPLETE.md does not claim "complete" if the actual pass rate is
  below 50%

Verification:
```bash
cd agda-proofs
for f in Theorems/Abstract/SymmetryImpliesRepulsion.agda \
         Theorems/Abstract/SymmetryFromList.agda \
         Theorems/TotientDensity.agda \
         Theorems/Abstract/ConstrainedOrbitals.agda \
         Specs/Tests.agda; do
  agda "$f" 2>&1 && echo "PASS: $f" || echo "FAIL: $f"
done
```

Assumptions:
- We assess, we do not fix Agda modules in this track (fixing is a later track)
- Postulates are clearly labeled as "assumed, not proven"
- The CI fix may require changing the installation method substantially

---

## Tranche 3: Claim Tightening and Key Repair

**Created**: 2026-03-09

### Track 10: Fix SymmetryFromList.agda and BucketsAutoMatch.agda

**Status**: `complete`
**Priority**: P0
**Scope**: small

Why this matters:
- SymmetryFromList.agda had one unsolved meta (line 84) that cascaded to 6 dependent
  modules, taking the entire certification stack offline.
- This was the highest-leverage single fix in the Agda codebase.

What was done:
- Root cause: the `PerfectBuckets` record was missing its `residue-distinct` field.
  The field had been moved to a postulate as a "parser bug workaround" but was never
  wired back into the `Pairing` record constructor in `pairingFromPerfect`.
- Fix: Added `residue-distinct` back as a proper field of `PerfectBuckets`, eliminating
  the postulate. SymmetryFromList now passes clean (no postulates).
- Same issue in BucketsAutoMatch.agda: `perfectFromBalanced` was also missing the
  `residue-distinct` field. Fixed by supplying the existing postulated
  `auto-mate-residue-distinct`.
- All 7 previously-blocked modules now pass: SymmetryFromList (clean),
  SymmetryFiniteReflect, BucketsAutoMatch, WindowCertificate,
  CertifiedResonanceComplete, CertifiedResonanceParam, CertifiedResonanceParamDyn
  (all with postulates).
- Updated STATUS.md, README.md, CERTIFICATION_COMPLETE.md, QUICK_START_VERIFICATION.md
  with new counts.

Acceptance criteria (all met):
- `agda Theorems/Abstract/SymmetryFromList.agda` exits 0 (was exit 42)
- All 7 dependent modules exit 0
- STATUS.md reflects 19 clean + 13 postulates + 48 fail = 80 total

Verification:
```bash
cd agda-proofs
agda Theorems/Abstract/SymmetryFromList.agda      # exit 0
agda Theorems/Abstract/BucketsAutoMatch.agda       # exit 0
agda Theorems/Abstract/WindowCertificate.agda      # exit 0
agda Examples/CertifiedResonanceParamDyn.agda      # exit 0
```

---

### Track 11: CLAUDE.md (Inner) Tightening

**Status**: `complete`
**Priority**: P1
**Scope**: medium

Why this matters:
- The crate-level CLAUDE.md was 965 lines and contained: stale example counts ("46
  verified"), references to 2 missing examples, a 100-line philosophical section about
  Babylonian-Prime Divergence, a 150-line research history, "59 tests pass" (actual: 143),
  and the Agda framework described as "complete" and "production-ready."
- As the file that Claude reads on every interaction, accuracy matters more here than
  anywhere else.

What was done:
- Rewrote CLAUDE.md from 965 lines to 186 lines.
- Kept: membrane structure explanation, HL API reference (function signatures and
  conventions), build/test instructions, feature flags, architecture overview,
  verified-vs-open summary, key document links.
- Removed: 150-line research evolution history (Nov 2025), 100-line Babylonian
  divergence section, stale example lists, aspirational Agda claims, detailed
  statistical tool descriptions (available via rustdoc), repository structure
  diagram (duplicated from README), release engineering notes.
- All references verified: 8 doc links, 4 example commands, all valid.

Acceptance criteria (all met):
- CLAUDE.md is under 200 lines (186)
- Every example command compiles
- Every document link resolves
- No stale count or claim

---

### Track 12: Agda Doc Consolidation

**Status**: `complete`
**Priority**: P2
**Scope**: small

Why this matters:
- 6 root-level docs (2,908 lines total) described the Agda certification framework
  from different angles, all written in November 2025 when claims were aspirational.
  Now that STATUS.md has the ground truth and the certification stack is repaired,
  these docs are archival.

What was done:
- Moved 4 uncorrected docs to historical/docs/:
  COMPLETE_CERTIFICATION_ARCHITECTURE.md (619 lines),
  COMPLETE_VERIFICATION_FRAMEWORK.md (557 lines),
  ABSTRACT_FRAMEWORK_INTEGRATION.md (463 lines),
  STATIC_TO_DYNAMIC_INVARIANTS.md (378 lines).
- Kept 2 docs with correction headers: CERTIFICATION_COMPLETE.md (has 2026-03-09
  correction), QUICK_START_VERIFICATION.md (has 2026-03-09 correction).
- Updated CERTIFICATION_COMPLETE.md documentation section to reference
  agda-proofs/STATUS.md as ground truth and note the moved files.
- Root markdown count: 14 -> 10.

Acceptance criteria (all met):
- Root markdown count <= 10
- No root-level doc claims "Production-Ready" without a correction header
- STATUS.md is the single source of truth for Agda module status

---

## Tranche 3 Reassessment

**Strongest verified spine after Tranche 3:**

1. 143 library tests pass, clippy clean.
2. 32 curated examples, all compiling.
3. 10 root markdown files (down from 46 pre-Tranche-1), zero stale references.
4. CLAUDE.md is 186 lines of accurate developer reference (down from 965).
5. README.md is 176 lines with every reference verified.
6. 32 of 80 Agda modules pass (19 clean, 13 with postulates). The certification
   stack is fully operational.
7. Agda STATUS.md is the single source of truth for formal verification.
8. collab/ has 5 files with the key finding (coprimality explains membrane density)
   as the lead.

**Where public signal still exceeds actual support:**

1. **Parent CLAUDE.md** (660 lines, in `../CLAUDE.md`) has similar overclaims to what
   the inner CLAUDE.md had before Track 11: stale counts, references to files that
   were moved to historical/, the Babylonian Divergence section, etc. It also
   references `LAGRANGE_POINT_ASYMMETRY.md` and other files at the parent level
   that we have not audited.
2. **EVIDENCE.md** (large file) has not been audited for accuracy. Some of its
   Wolfram Alpha URLs and specific claims may be stale.
3. **Agda postulates**: 13 modules use a total of ~26 postulates. Some of these
   may be provable, which would strengthen the formal verification story.
4. **Untracked files**: All Tranche 1-3 changes remain uncommitted. A fresh clone
   would still fail due to missing src/connector/, src/hzlib/num_theory.rs, etc.

**What the next tranche should address:**

The parent CLAUDE.md is now the weakest public surface. However, since it lives
outside the crate directory, tightening it requires care -- it may serve as
research context for other tools besides Claude. The untracked file situation
should be resolved by committing all changes. EVIDENCE.md needs an accuracy audit.

## Tranche 4: CI Readiness and Trust Surface

**Created**: 2026-03-09

### Track 13: Fix All-Targets Clippy Warnings

**Status**: `complete`
**Priority**: P0
**Scope**: medium

Why this matters:
- The CI runs `cargo clippy --all-targets -- -D warnings` which failed due to
  warnings in library test code (vec![] -> slice), 17 example files, 2 integration
  test files, and 1 benchmark file. This directly blocks CI green.

What was done:
- Fixed 15 `useless_vec` warnings in `src/hzlib/stats.rs` test code (vec![] -> slice literals)
- Fixed 8 `is_multiple_of` warnings across 5 example files
- Fixed `manual_flatten` in check_prime.rs (`.flatten()` -> `.map_while(Result::ok)`)
- Fixed `collapsible_if` in lagrange_full_verification.rs
- Fixed `if_same_then_else` in membrane_showcase.rs (dead branch removal)
- Fixed `manual_clamp` in membrane_lab_tui.rs and lagrange_tui_demo.rs
- Fixed `explicit_counter_loop` in belphegor_scanner.rs and scan_connectors.rs
- Fixed `or_insert_with(Vec::new)` -> `or_default()` in 3 files
- Fixed `unnecessary_cast` in orthogonality_verification.rs and lagrange_mechanics.rs
- Fixed `unused_enumerate_index` in prime_atom_tui.rs and prime_gap_analysis.rs
- Fixed dead code in tests (harmonics_standalone_test.rs, prime_harmonics_test.rs)
- Fixed `field_reassign_with_default` in validation_system_test.rs
- Fixed `module_inception` in tests/unit/harmonics_tests.rs
- Fixed dead fields in lagrange_tui_demo.rs and scan_connectors.rs
- Fixed unused variables in benches/optimization_verification.rs
- Fixed `op_ref` in lagrange_clustering_verifier.rs and verify_prime_checker.rs
- Applied `cargo fmt` to normalize all formatting changes.
- Total: ~40 individual warning fixes across 22 files.

Acceptance criteria (all met):
- `cargo clippy --all-targets -- -D warnings` exits 0
- `cargo test --lib` passes 143/143 tests
- `cargo fmt -- --check` exits 0

Verification:
```bash
cargo clippy --all-targets -- -D warnings
cargo test --lib
cargo fmt -- --check
```

---

### Track 14: CI Workflow Repair

**Status**: `complete`
**Priority**: P1
**Scope**: small

Why this matters:
- The Agda CI workflow had 4 redundant sequential jobs, each independently
  installing Agda and checking overlapping module sets. The module list was stale
  (listed 11 modules; actual passing count is 32).
- The critical examples list in ci.yml included `educational_explorer` which was
  moved to historical/ in Track 4.
- The Agda installation via `apt-get` on ubuntu-latest may not provide Agda 2.8.0,
  making the CI unreliable for Agda verification.

What was done:
- Removed stale `educational_explorer` from the critical examples list in ci.yml.
- Rewrote agda-verification.yml from 4 sequential jobs (274 lines) to 1 job
  (~100 lines) that checks all 32 passing modules with pass/fail counting.
- Added `continue-on-error: true` since Agda version on ubuntu-latest may differ
  from the locally verified 2.8.0.
- Updated the module list to match STATUS.md (19 clean + 13 postulated = 32).
- Re-added the Agda workflow call in ci.yml with an accurate comment noting it is
  best-effort.

Acceptance criteria (all met):
- ci.yml references only examples that exist in examples/
- agda-verification.yml lists exactly the 32 modules from STATUS.md
- The Agda job uses `continue-on-error: true` to avoid blocking CI

Verification:
```bash
# Verify all critical examples exist
grep -oP '"(\w+)"' .github/workflows/ci.yml | tr -d '"' | while read ex; do
  [ -f "examples/${ex}.rs" ] || echo "MISSING: $ex"
done
```

---

### Track 15: Claim-Evidence Registry

**Status**: `complete`
**Priority**: P1
**Scope**: small

Why this matters:
- The repository makes ~15 significant public claims across README.md, CLAUDE.md,
  and VERIFIED_FACTS_VS_SPECULATION.md. Without a single index, a reviewer must
  hunt across multiple files to determine whether any given claim has evidence.
- A concise mapping from claim to verification command is the highest-trust
  artifact a research repo can offer.

What was done:
- Created CLAIMS.md: 8 verified empirical claims, 6 infrastructure claims,
  4 falsified claims, 3 open questions. Each with status, evidence source,
  and verification command.
- Added CLAIMS.md to the document tables in CLAUDE.md and README.md.
- Root markdown count: 10 -> 11 (CLAIMS.md added).

Acceptance criteria (all met):
- CLAIMS.md exists and covers every claim made in README.md section "Verified Results"
- Each claim has a status (`empirical`, `verified`, `falsified`, `open`)
- Each verified claim has a verification command that can be run
- CLAIMS.md is referenced from both CLAUDE.md and README.md

Verification:
```bash
# Verify CLAIMS.md exists and is referenced
[ -f CLAIMS.md ] && echo "EXISTS" || echo "MISSING"
grep -l "CLAIMS.md" CLAUDE.md README.md
```

---

## Tranche 4 Reassessment

**Strongest verified spine after Tranche 4:**

1. 143 library tests pass. Clippy clean on all targets (examples, tests, benches).
2. 32 curated examples, all compiling and clippy-clean.
3. 11 root markdown files (10 pre-Tranche-4 + CLAIMS.md).
4. CI workflow is accurate: critical examples list matches reality, Agda module list
   matches STATUS.md, formatting and clippy checks will pass.
5. CLAIMS.md provides a single-page trust surface mapping every major claim to its
   evidence and verification command.
6. All changes remain uncommitted. A commit is needed to make CI work on clone.

**Where public signal still exceeds actual support:**

1. **Parent CLAUDE.md** (660 lines, in `../CLAUDE.md`) is still the weakest public
   surface -- stale counts, references to files moved to historical/, philosophical
   sections that are interesting but not evidence-based.
2. **EVIDENCE.md** has not been audited. Some Wolfram Alpha URLs may be stale.
3. **Agda postulates**: 13 modules use postulates. Some may be provable.
4. **All accumulated changes are uncommitted.** This is now the most pressing
   practical issue -- without a commit, a fresh clone fails to build.

**What the next tranche should address:**

The most urgent need is committing the accumulated work (15 tracks of changes).
After that, the parent CLAUDE.md is the next overclaim surface. EVIDENCE.md
should be spot-checked rather than fully audited.

## Tranche 5: Evidence Integrity and Parent Surface

**Created**: 2026-03-09

### Track 16: EVIDENCE.md Accuracy Repair

**Status**: `complete`
**Priority**: P0
**Scope**: medium

Why this matters:
- EVIDENCE.md is the empirical evidence database supporting all public claims.
- Spot-check of 12 primality claims found 5 are FALSE (42% error rate):
  - 3308033 claimed prime, actually composite (19 * 43)
  - 1040301 claimed prime, actually composite (divisible by 3)
  - 20205202 claimed prime, actually composite (divisible by 2)
  - 50505 claimed prime, actually composite (divisible by 3, 5, 7, 13, 37)
  - 20302 claimed prime, actually composite (divisible by 2)
- All 9 verification scripts referenced in EVIDENCE.md do not exist anywhere
  in the repository (not even in historical/examples/).
- The "auto-update command" at the bottom references a nonexistent script.
- Correct claims (7 of 12 checked): 303050303, 3305033, 3307033, 120121,
  307050703, 30301303, 30308303 are indeed prime.
- Lagrange point claims from Section 5b are all correct (4/4 verified).

What was done:
- Spot-checked all 14 unique primality claims via `check_prime`: 5 false, 9 correct.
- Corrected false claims with strikethrough + correction notes:
  - 3308033 (composite, 19*43) -> replaced with 3304033 (verified prime)
  - 1040301 (composite, div by 3) -> replaced with 1040101 (verified prime)
  - 20205202 (composite, div by 2) -> marked no verified example available
  - 50505 (composite, div by 3,5,7,13,37) -> marked no verified example available
  - 20302 (composite, div by 2) -> marked no verified example available
- Working seeds for (3,3) k=(0,1) corrected from [1,2,5,7,8] to [4,5,7].
- Marked all 6 inline "Verification Script" references as nonexistent.
- Replaced Section 7.1 script table with note + working alternatives.
- Replaced Section 8.2 verification sequence with note + working alternatives.
- Replaced auto-update command with note that file is manually maintained.
- Added spot-check header noting March 2026 audit scope and error rate.
- Synced parent-level EVIDENCE.md copy with corrected engine version.

Todo (completed):
- [x] Correct or remove the 5 false primality claims
- [x] Remove references to 9 nonexistent verification scripts
- [x] Remove the auto-update command referencing nonexistent generate_evidence_md
- [x] Add a correction note with the date of the spot-check
- [x] Verify remaining unchecked claims where practical

Acceptance criteria:
- Every primality claim in EVIDENCE.md is correct (verifiable with check_prime)
- No script reference in EVIDENCE.md points to a nonexistent file
- A spot-check date and scope is noted at the top

Verification:
```bash
# Extract all numbers claimed prime and verify
grep -oP 'isprime%28(\d+)%29' EVIDENCE.md | grep -oP '\d+' | while read n; do
  echo "$n" | cargo run --example check_prime 2>/dev/null | grep -v "^$"
done
```

Assumptions:
- We correct false claims rather than removing entire sections
- Where an example was wrong, we find the correct number if possible
- We do not regenerate the file, as the referenced generator script never existed

---

### Track 17: Parent CLAUDE.md Tightening

**Status**: `complete`
**Priority**: P1
**Scope**: large

Why this matters:
- The parent CLAUDE.md (660 lines) is read by Claude on every interaction at the
  workspace level. It contains:
  - "46 verified examples" (actual: 32)
  - "59 tests pass" (actual: 143)
  - References `examples/experimental/` (deleted in Track 7)
  - Repository structure diagram showing `src/core/`, `src/sieves/`,
    `src/visualization/` (none exist)
  - 150-line Babylonian Prime Divergence section
  - 150-line research evolution history (Nov 2025)
  - Development Best Practices section (duplicates inner CLAUDE.md)
  - "Production Ready Features" and "Release Artifacts" sections with stale claims
  - Binary count claims "5" (actual: 6, including orthogonal_landscape)
  - References `BABYLONIAN_PRIME_DIVERGENCE.md` which does not exist
  - Emoji-laden formatting ("Production Ready Features check", "Release Artifacts
    package", "Experimental Features test_tube")
- It also duplicates nearly all content from the inner CLAUDE.md, creating a
  maintenance divergence problem. It should instead point to the inner CLAUDE.md
  for crate-specific details.

What was done:
- Rewrote parent CLAUDE.md from 660 lines to 130 lines.
- Removed: 175-line HL framework API reference (duplicate of inner CLAUDE.md),
  55-line Development Best Practices (duplicate), 100-line Babylonian Divergence
  section, 55-line research history, 40-line Implementation Status with stale
  claims ("59 tests pass", "46 verified examples"), repository structure diagram
  with nonexistent directories, Release Artifacts section, Recent Updates section.
- Kept: membrane concept summary, empirical results table, workspace structure
  (corrected), verified results summary, quick start, connector asymmetry note,
  key documents table.
- Added: pointer to inner CLAUDE.md as authoritative reference, pointer to
  CLAIMS.md, pointer to THEORETICAL_CLOSURE.md.
- All 9 file references verified. All 3 example commands verified.
- Updated parent README.md from bare Wolfram Alpha URL to proper landing page.

Todo (completed):
- [x] Rewrite parent CLAUDE.md to be a short project overview pointing to the
      inner CLAUDE.md for technical details
- [x] Fix all stale counts and references
- [x] Remove sections that duplicate the inner CLAUDE.md
- [x] Keep the unique content: workspace structure, research context (brief)

Acceptance criteria:
- Parent CLAUDE.md is under 250 lines
- Every file/directory reference resolves
- Every example command compiles
- No count contradicts reality
- No reference to deleted files or directories

Verification:
```bash
cd /path/to/primes
grep -oP 'cargo run --example \K\w+' CLAUDE.md | sort -u | while read name; do
  cargo build --example "$name" -p prime-physics-engine 2>/dev/null || echo "BROKEN: $name"
done
```

Assumptions:
- The parent CLAUDE.md should be a workspace-level overview, not a duplicate
  of the crate-level CLAUDE.md
- Research narrative sections are valuable but should be brief and accurate
- The inner CLAUDE.md is the authoritative developer reference for the crate

---

### Track 18: Parent-Level Markdown Triage

**Status**: `complete`
**Priority**: P2
**Scope**: medium

Why this matters:
- 41 markdown files exist at the parent level (primes/). This includes 10 KICK_*.md
  files (session logs), TRANSFORM_*.md files (session logs), and various session
  summaries. This is the same sprawl pattern that was cleaned up inside the engine
  in Tracks 3 and 12.
- A collaborator opening the parent directory sees a wall of session artifacts
  rather than a navigable project.

What was done:
- Classified all 41 parent-level markdown files.
- Created `archived-docs/` directory at parent level.
- Moved 31 files to archive:
  - 15 empty files (KICK_1-10.md, KICK.md, TRANSFORM_1-4.md)
  - 16 stale docs: session logs (HATCHING.md, RAW_SUBSTRATE.md,
    PERSISTENCE_CORE.md, WORK_SESSION_SUMMARY.md), stale onboarding
    (JOURNEY.md, START_HERE.md, QUICK_START_GUIDE.md, NJUGU_ONBOARDING.md),
    completed-action summaries (CLEANUP_REPORT.md, REORGANIZATION_COMPLETE.md,
    CONSOLIDATION_SUMMARY.md, HL_INTEGRATION_SUMMARY.md, ARCHIVE_README.md,
    CRITICAL_FILES.md), stale research narratives only referenced from archived
    docs (MODULAR_ARITHMETIC_NARRATIVE.md, CONNECTOR_SCAN_RESULTS_ADDENDUM.md).
- Kept 10 files: CLAUDE.md, README.md, AUTHORS.md, EVIDENCE.md,
  LAGRANGE_POINT_ASYMMETRY.md, HARDY_LITTLEWOOD_FRAMEWORK.md,
  MIDPOINT_ANALYSIS.md, MATHEMATICAL_EXPLANATION.md,
  FACTORIZATION_GOLDILOCKS.md, CRITICISM_RESPONSES.md.
- The last 5 research docs are substantive and cross-referenced from engine
  subdirectories (tools/orthogonality/, agda/, docs/) by relative paths;
  moving them would break those references.

Todo (completed):
- [x] Classify all 41 parent-level markdown files
- [x] Identify which are session logs vs maintained references
- [x] Move session logs to an appropriate archive location
- [x] Verify remaining files are referenced or serve as maintained references
- [x] Update parent README.md

Acceptance criteria:
- Parent-level markdown count is <= 10
- Every remaining file is either referenced from CLAUDE.md/README.md or serves
  as a maintained reference document
- No KICK_*.md or TRANSFORM_*.md files remain at parent level

Verification:
```bash
cd /path/to/primes
ls *.md | wc -l  # should be <= 10
```

Assumptions:
- Session logs are archived, not deleted
- Files referenced by parent CLAUDE.md (EVIDENCE.md, LAGRANGE_POINT_ASYMMETRY.md,
  etc.) are kept at parent level
- The parent directory should be as clean as the engine directory now is

---

## Tranche 5 Reassessment

**Strongest verified spine after Tranche 5:**

1. 143 library tests pass. Clippy clean on all targets.
2. 32 curated examples, all compiling and clippy-clean.
3. 11 root markdown files (engine level), 10 parent-level markdown files.
4. EVIDENCE.md has been spot-checked: 5 of 14 primality claims were false and
   have been corrected. 9 nonexistent script references are clearly marked.
   Remaining claims are verified correct.
5. Parent CLAUDE.md is 130 lines (down from 660), pointing to the inner
   CLAUDE.md as the authoritative crate reference.
6. Parent-level markdown count is 10 (down from 41). 31 files archived.
7. Parent README.md updated from a bare URL to a proper landing page.

**Where public signal still exceeds actual support:**

1. **EVIDENCE.md data integrity**: The 5 corrected claims are now marked with
   strikethrough and correction notes. However, the underlying data tables
   (success rates, seed counts) were not re-derived -- they were taken from
   the original claims. Some may also be wrong. A full regeneration from code
   would require writing the missing verification scripts.
2. **Agda postulates**: 13 modules still use postulates. Some may be provable.
3. **Parent-level research docs** (HARDY_LITTLEWOOD_FRAMEWORK.md,
   MIDPOINT_ANALYSIS.md, etc.) have not been spot-checked for accuracy.
4. **All changes remain uncommitted.** This is now Tranches 1-5 of accumulated
   work. A commit is the most urgent practical step.

**What the next tranche should address:**

Committing the accumulated work is now critical -- 18 tracks of changes across
5 tranches are uncommitted. After that, the highest-value work is either:
- Writing the missing verification scripts that EVIDENCE.md references (to make
  the empirical claims actually reproducible), or
- Agda postulate reduction (to strengthen the formal verification story).

The documentation surface is now in good shape and should not need further
attention until new research results are added.

## Tranche 6: Evidence Hardening and Formal Verification

**Created**: 2026-03-09

### Track 19: EVIDENCE.md Deeper Audit

**Status**: `complete`
**Priority**: P0
**Scope**: medium

Why this matters:
- Track 16 found 5/14 false primality claims (42% error rate) in a spot-check
  of individual prime numbers. This track extended the audit to the data TABLES
  in Sections 2.1, 4.1, and 4.2, which contain success rates, working seed
  lists, and migration density claims. These tables were never verified.

What was found:
- **Section 2.1 (Breathing Membrane)**: 2 of 5 rows were wrong.
  - (3,3) k=(1,0) claimed 20% with seeds [1,3,8]: actually 0% (0/10 prime)
  - (3,7) k=(0,1) claimed 25% with seeds [1,3,5,9]: actually 20%, seeds [8,9]
  - (3,3) k=(0,1) 30% [4,5,7]: correct (already fixed in Track 16)
  - (3,3) k=(1,1) 10% [5]: correct
  - (3,7) k=(1,1) 10% [5]: correct
- **Section 3.2 (Exclusivity)**: Factorization claim for seed 1 was misleading
  ("3 x 101003434 + 1" is not a factorization). Actual smallest factor is 23.
- **Section 4.1 (Migration)**: 2 of 3 rows were wrong.
  - (3,7) k=(0,1) length-2 density claimed 15%: actual 20% (seeds [08,09])
  - (1,9) k=(0,1) claimed 25%: actual 30%. (1,9) k=(1,0) claimed 18%: actual 10%
- **Section 4.2 (Length Specialists)**: Both claims were inflated.
  - (1,2) k=(0,0) claimed 40%: actual 20% (seeds [01,07])
  - (1,4) k=(1,0) claimed 35%: actual 10% (seed [09] only)
  - The replacement example 1040101 from Track 16 doesn't match the (1,4) k=(1,0)
    membrane pattern: corrected with 10409401 (verified prime).

What was done:
- Corrected all false success rates with strikethrough notation
- Corrected all false working seed lists
- Fixed misleading factorization in Section 3.2
- Added note explaining (3,3) k=(1,0) length-1 vs length-2 behavior
- Replaced "Revolutionary Finding" language with accurate "Observation"
- Synced parent-level EVIDENCE.md copy
- Updated spot-check header to note two audit rounds

Acceptance criteria (all met):
- Every success rate in Sections 2.1, 4.1, 4.2 has been verified or corrected
- Every working seed list in Section 2.1 has been verified or corrected
- All corrections use strikethrough with date and actual values

Verification:
```bash
# Spot-check a corrected claim
for s in 0 1 2 3 4 5 6 7 8 9; do
  echo "$s: $(echo "330${s}033" | cargo run --example check_prime 2>/dev/null | grep -v '^$' | grep -v 'Prime' | grep -v 'You' | head -1)"
done
# Should show seeds 4,5,7 prime, rest composite
```

---

### Track 20: CertifiedResonanceComplete Postulate Elimination

**Status**: `complete`
**Priority**: P1
**Scope**: small

Why this matters:
- CertifiedResonanceComplete.agda is the flagship concrete certification example
  for Base 6. It had 6 postulates (involutive, no-fixed, equivariant,
  residue-distinct for the pairing function). These postulates existed because
  the `#_` operator (Fin literal shorthand) does not compute in Agda's pattern
  matching, preventing `refl` from discharging the proofs.
- All 6 postulates were trivially true (verified by manual case analysis in
  comments) but the module could not use `--safe` and was classified as
  "passes with postulates."

What was done:
- Replaced all `#_` abbreviations with explicit `fzero`/`fsuc` constructor
  patterns for both Fin 6 and Fin 4 inhabitants.
- Rewrote `inv-fn`, `res-list`, and `mate-fn` to pattern-match on constructors
  instead of abbreviation aliases.
- Replaced all 6 postulates with constructive proofs:
  - `involutive-mate`: 4 cases, all `refl`
  - `no-fixed-mate`: 4 cases, all absurd pattern `()`
  - `equivariant-res`: 4 cases, all `refl`
  - `residue-distinct`: 4 cases, all absurd pattern `()`
- Module type-checks with 0 postulates. `grep -cw 'postulate'` returns 0.

Acceptance criteria (all met):
- `agda Examples/CertifiedResonanceComplete.agda` exits 0
- `grep -cw 'postulate' Examples/CertifiedResonanceComplete.agda` returns 0
- STATUS.md updated: 19 -> 20 clean, 13 -> 12 postulated

Verification:
```bash
cd agda-proofs
agda Examples/CertifiedResonanceComplete.agda
grep -cw 'postulate' Examples/CertifiedResonanceComplete.agda
# Should output: 0
```

Assumptions:
- We do not attempt to prove the general `reflect-involutive` for arbitrary m
  (that remains a postulate in SymmetryFiniteReflect.agda).
- The approach (explicit constructors instead of `#_`) is specific to small
  finite types where case analysis is tractable.

---

## Tranche 6 Reassessment

**Strongest verified spine after Tranche 6:**

1. 143 library tests pass. Clippy clean on all targets.
2. 32 curated examples, all compiling and clippy-clean.
3. 11 root markdown files (engine level), 10 parent-level markdown files.
4. EVIDENCE.md has been through two rounds of spot-checking:
   - Round 1 (Track 16): 5 of 14 primality claims were false, corrected.
   - Round 2 (Track 19): 6 data table entries were false (success rates,
     working seeds), corrected. The "Length Specialist" claims were
     particularly inflated (40% -> 20%, 35% -> 10%).
5. Agda: 20 of 80 modules pass clean (no postulates), 12 with postulates,
   48 fail. CertifiedResonanceComplete is now fully machine-checked.
6. Parent CLAUDE.md is 130 lines pointing to inner CLAUDE.md as authoritative.

**Where public signal still exceeds actual support:**

1. **EVIDENCE.md underlying data**: The corrected claims now have accurate
   spot-checked values, but the sample sizes (10 seeds per config) are small.
   The statistical significance claims ("95% confidence") in Section 2.3
   apply to the Miller-Rabin primality test, not to the success rate estimates
   themselves. With n=10, a 30% success rate has a 95% CI of roughly [7%, 65%].
2. **Agda postulates**: 12 modules still use postulates. The most tractable
   remaining targets are:
   - SymmetryFiniteReflect (1 postulate): provable for specific bases but
     requires modular arithmetic lemmas for the general case
   - BucketsAutoMatch (5 postulates): require Fin arithmetic implementation
   - CertifiedResonanceParam/ParamDyn (2+2 postulates): `autoPerfectBuckets`
     is the same pattern as BucketsAutoMatch
3. **CLAIMS.md** should be updated to reflect the corrected EVIDENCE.md values.
4. **All changes remain uncommitted.** This is now Tranches 1-6 of accumulated
   work (20 tracks). Committing is the most urgent practical step.

**What the next tranche should address:**

Committing the accumulated work remains the most pressing practical need.
After that, updating CLAIMS.md to reflect the corrected data values, and
potentially reducing more Agda postulates (SymmetryFiniteReflect is the
next most tractable target).

## Tranche 7: API Clarity and Formal Verification Extension

**Created**: 2026-03-09

### Track 21: CLAIMS.md Refresh + STATUS.md Fix

**Status**: `complete`
**Priority**: P1
**Scope**: small

Why this matters:
- STATUS.md had a count discrepancy: summary table said "20 clean" but the
  section header said "These 19 modules." The struck-through CertifiedResonanceComplete
  entry was still listed in the postulated table.
- CLAIMS.md did not note the EVIDENCE.md audit corrections.

What was done:
- Fixed STATUS.md header: "These 19" -> "These 20" to match the actual table.
- Removed struck-through CertifiedResonanceComplete row from postulated table.
- Updated postulated count: "These 13" -> "These 12" (CertifiedResonanceComplete
  is now in the clean list).
- Added EVIDENCE.md audit note to CLAIMS.md header.
- Fixed minor formatting in CLAIMS.md Agda claim.

Acceptance criteria (all met):
- STATUS.md section headers match actual module counts in each table
- No struck-through entries remain in active tables
- CLAIMS.md notes the EVIDENCE.md corrections

---

### Track 22: Module-Level API Documentation

**Status**: `complete`
**Priority**: P1
**Scope**: medium

Why this matters:
- The public API is dominated by physics metaphor types (PrimeUniverse,
  GravitationalField, PhysicalConstants with light_speed=299792458.0). The
  genuinely useful math (sieves, membrane generation, HL analysis in hzlib/)
  is harder to discover.
- Module doc comments on gravity/, tidal/, lagrange.rs contained overclaims
  ("revolutionary discovery," "discovered phenomena" with unverified specifics).

What was done:
- Added **Layer** annotations to all public modules in lib.rs:
  - Math core: `connector`, `hzlib`, `membrane`, `prime_sieve`, `validation`
  - Analysis tools: `fingerprint`, `resonance_profiles`
  - Simulation / visualization: `chaos`, `gravity`, `integrators`, `lagrange`,
    `spacetime`, `tidal`
  - Presentation / infrastructure: `ascii_art`, `dvfs`, `education`, `harmonics`,
    `nibble_pack`, `optimization`, `performance`, `prime_lut`, `prime_lut_recip`, `tui`
- Reorganized lib.rs module declarations into grouped sections with separator
  comments matching the layer classification.
- Replaced overclaiming doc comments:
  - gravity/mod.rs: "revolutionary discovery that prime numbers behave like
    massive particles following gravitational physics laws" -> "metaphor for
    visualization, not a claim that primes obey Newtonian physics"
  - tidal/mod.rs: removed "key discovery" framing of simulation parameters
  - lagrange.rs: noted that "100% clustering" is a simulation observation
  - chaos/mod.rs: identified as visualization layer
  - spacetime.rs: noted values are arbitrary heuristics
- Added comprehensive doc comment to hzlib/mod.rs with submodule table and
  quick start example.
- Noted where to find real math (hzlib, membrane) vs visualization (gravity,
  tidal) in each module's doc header.

Acceptance criteria (all met):
- Every public module in lib.rs has a doc comment with a Layer annotation
- No module doc comment claims "revolutionary discovery" or "discovered phenomena"
  for simulation observations
- `cargo test --lib` passes 143/143
- `cargo clippy --all-targets -- -D warnings` exits 0
- `cargo fmt -- --check` exits 0

Verification:
```bash
cargo test --lib
cargo clippy --all-targets -- -D warnings
cargo fmt -- --check
```

---

### Track 23: CertifiedResonanceParam/ParamDyn Example Postulate Reduction

**Status**: `complete`
**Priority**: P2
**Scope**: small

Why this matters:
- CertifiedResonanceParam and CertifiedResonanceParamDyn each had postulated
  witnesses (proof-midVoid, proof-balanced) in their Example modules for the
  same Base 6 data that CertifiedResonanceComplete proves constructively.
- These postulates made the examples depend on unproven assumptions despite
  the same facts being machine-checked in CertifiedResonanceComplete.

What was done:
- CertifiedResonanceParam: Replaced the Example module's postulated
  proof-midVoid and proof-balanced with direct PerfectBuckets construction
  using explicit fzero/fsuc case analysis. The example now bypasses the
  framework-level autoPerfectBuckets postulate entirely. Example module:
  0 postulates (was 2). File total: 1 postulate (framework only).
- CertifiedResonanceParamDyn: Same treatment for the static proofs.
  proof-stable is retained as a postulate because it depends on runtime
  radius R. Example module: 1 postulate (was 3). File total: 2 postulates
  (was 4).
- Updated STATUS.md with new postulate counts and repair history entry.
- Both modules verified: `agda <file>` exits 0.

Acceptance criteria (all met):
- `agda Examples/CertifiedResonanceParam.agda` exits 0
- `agda Examples/CertifiedResonanceParamDyn.agda` exits 0
- CertifiedResonanceParam example module has 0 postulates
- CertifiedResonanceParamDyn example module has 1 postulate (proof-stable only)
- STATUS.md updated with new counts

Verification:
```bash
cd agda-proofs
agda Examples/CertifiedResonanceParam.agda
agda Examples/CertifiedResonanceParamDyn.agda
grep -n '^postulate' Examples/CertifiedResonanceParam.agda  # line 69 only (framework)
grep -n '^postulate' Examples/CertifiedResonanceParamDyn.agda  # lines 77, 362
```

Assumptions:
- The framework-level autoPerfectBuckets postulate is not addressed (it
  requires general Fin arithmetic implementation, not case analysis).
- proof-stable in ParamDyn depends on runtime radius R and cannot be
  proved statically without additional axioms about the positions.

---

## Tranche 7 Reassessment

**Strongest verified spine after Tranche 7:**

1. 143 library tests pass. Clippy clean on all targets. Fmt clean.
2. 32 curated examples, all compiling and clippy-clean.
3. 11 root markdown files (engine level), 10 parent-level markdown files.
4. Every public module in lib.rs has a doc comment with a Layer annotation
   separating math core from visualization metaphor.
5. lib.rs module declarations organized into 5 grouped sections.
6. Agda: 20 of 80 modules pass clean. 12 with postulates. The three
   flagship example modules (Complete, Param, ParamDyn) now have minimal
   or zero example-level postulates.
7. CLAIMS.md and STATUS.md are internally consistent with corrected counts.

**Where public signal still exceeds actual support:**

1. **Framework-level Agda postulates**: `autoPerfectBuckets` (BucketsAutoMatch,
   CertifiedResonanceParam, CertifiedResonanceParamDyn), `reflect-involutive`
   and `reflect-mid` (SymmetryFiniteReflect), and 5 `auto-mate-*` properties
   (BucketsAutoMatch). These require general Fin arithmetic implementation.
2. **WindowCertificate.agda**: 5 postulates (inherits from BucketsAutoMatch +
   its own dual-certification postulates).
3. **All changes remain uncommitted.** 23 tracks across 7 tranches. Committing
   is the most urgent practical step.
4. **Parent-level research docs** (HARDY_LITTLEWOOD_FRAMEWORK.md,
   MIDPOINT_ANALYSIS.md, etc.) have not been spot-checked for accuracy.

**What the next tranche should address:**

1. **Commit the accumulated work** -- 23 tracks. Most urgent practical need.
2. **SymmetryFiniteReflect postulate reduction** -- `reflect-involutive` is
   provable for specific m by case analysis. For m=6 it is verified. A general
   proof needs modular arithmetic lemmas from stdlib.
3. **Test coverage gap analysis** -- 143 tests pass but key functionality
   gaps may exist (e.g., membrane edge cases, connector overflow paths).

## Tranche 8: Workspace Hygiene and Test Coverage

**Created**: 2026-03-09

### Track 24: Density-Explorer Warning Cleanup

**Status**: `complete`
**Priority**: P0
**Scope**: medium

Why this matters:
- The `density-explorer` workspace member (`tools/density-explorer/`) had 37 clippy
  warnings, making it the only workspace member with compiler warnings. This directly
  blocks `cargo clippy --workspace -- -D warnings` from succeeding.

What was done:
- Applied `cargo clippy --fix` for 15 auto-fixable warnings (is_multiple_of,
  collapsible_if, redundant_closure).
- Renamed 8 instances of `|L| L.zero` closure parameter to `|ly| ly.zero` (snake_case).
- Renamed 1 instance of `|L| format!(...)` to `|ly| format!(...)`.
- Converted 3 `for i in range { spec[i] = None }` to `for slot in &mut spec[range] { *slot = None }`.
- Converted 3 `for r/i in 0..m { dist[r] }` loops to iterator-based forms.
- Added `#[allow(dead_code)]` to 4 unused `_with_spec` variant functions (development
  code not yet integrated into main flow).
- Added `#[allow(clippy::too_many_arguments)]` to 4 functions with 11-13 parameters
  (structural, refactoring out of scope for a tool binary).
- Applied `cargo fmt`.
- Total: 37 warnings resolved across 1 file (4105 lines).

Acceptance criteria (all met):
- `cargo clippy -p density-explorer -- -D warnings` exits 0
- `cargo clippy -p hz -- -D warnings` exits 0
- `cargo clippy --all-targets -- -D warnings` exits 0 (main crate)

Verification:
```bash
cargo clippy -p density-explorer -- -D warnings
cargo clippy -p hz -- -D warnings
cargo clippy --all-targets -- -D warnings
```

---

### Track 25: Membrane Core Test Coverage

**Status**: `complete`
**Priority**: P1
**Scope**: medium

Why this matters:
- `membrane/mod.rs` is the core public API for membrane construction. It contains
  `MembraneConfig::new`, `construct_number`, `is_valid`, `total_digits`,
  `best_for_base`, `is_high_performance`, and `summary`. None had unit tests.
- The submodules (symmetric, breathing, adaptive, quantum) each had 3 tests but
  did not cover the parent module's orchestration logic.
- Adding tests here both verifies correctness and documents expected behavior
  for the most important API surface in the crate.

What was done:
- Added 27 unit tests to `membrane/mod.rs` covering:
  - `MembraneConfig::new`: coprime and non-coprime configurations
  - `is_valid`: coprime verification for outer, inner, and champion configs
  - `total_digits`: symmetric k=(0,0), k=(1,1), k=(2,1), and breathing
  - `construct_number`: k=(0,0), k=(1,1), k=(2,1) with known expected values,
    plus the known prime 307050703
  - `construct_membrane_number`: string output for symmetric and breathing
  - `estimate_density`: coprime vs non-coprime, k=(0,0) vs k=(1,1), base 6 champion
  - `best_for_base`: returns valid configs, base 6 includes (1,5) champion
  - `is_high_performance`: positive and negative cases
  - `summary`: contains expected content
  - Edge cases: seed=0, seed=9
- Test count: 143 -> 170 (27 new tests, all passing).
- All tests are fast (0ms execution) and deterministic.

Acceptance criteria (all met):
- `cargo test --lib membrane::tests` runs 27 tests, all pass
- `cargo test --lib` runs 170 tests, all pass
- No new clippy warnings introduced

Verification:
```bash
cargo test --lib membrane::tests
cargo test --lib
cargo clippy --all-targets -- -D warnings
```

---

### Track 26: GLOSSARY.md Accuracy Fix

**Status**: `complete`
**Priority**: P1
**Scope**: small

Why this matters:
- The Lagrange Points example contained a false primality claim:
  `97009000303050303` was claimed prime but is actually composite (divisible by 3, 13).
- The glossary also claimed "100% clustering success across 24 tested prime pairs"
  without noting this is a simulation observation.
- The "For More Information" section did not reference CLAIMS.md.
- The "Last Updated" date was stale (2025-10-29).

What was done:
- Replaced the false Lagrange example with verified data from EVIDENCE.md Section 5b:
  `10301000063007003007003` (verified prime, from the (10301, 3007003007003) pair).
- Updated the Lagrange status to "4 verified equilibrium positions" with pointer to
  EVIDENCE.md rather than the unqualified "100% clustering" claim.
- Added CLAIMS.md to the "For More Information" section.
- Updated the "Last Updated" date to 2026-03-09.

Acceptance criteria (all met):
- Every primality claim in GLOSSARY.md is correct
- The Lagrange example uses verified data from EVIDENCE.md Section 5b
- CLAIMS.md is referenced
- Date is current

Verification:
```bash
echo "10301000063007003007003" | cargo run --example check_prime
# Should output: PRIME
```

---

## Tranche 8 Reassessment

**Strongest verified spine after Tranche 8:**

1. 170 library tests pass (27 new membrane core tests). Clippy clean on all targets.
2. All 3 workspace members (primes, density-explorer, hz) are clippy-clean with
   `-D warnings`. This is the first time the entire workspace has been warning-free.
3. 32 curated examples, all compiling and clippy-clean.
4. 11 root markdown files (engine level), 10 parent-level markdown files.
5. GLOSSARY.md Lagrange example corrected (was composite, now uses verified prime).
6. Membrane core API (`MembraneConfig::new`, `construct_number`, `is_valid`,
   `total_digits`, `best_for_base`) has comprehensive unit test coverage.
7. Agda: 20 of 80 modules pass clean. 12 with postulates.

**Where public signal still exceeds actual support:**

1. **All changes remain uncommitted.** 26 tracks across 8 tranches. This is the most
   urgent practical step. Without a commit, a fresh clone fails to build (missing
   `src/connector/`, `src/hzlib/num_theory.rs`).
2. **Framework-level Agda postulates**: `autoPerfectBuckets`, `reflect-involutive`,
   `reflect-mid`, and 5 `auto-mate-*` properties remain unproven. These require
   general Fin arithmetic, not case analysis.
3. **Parent-level research docs** (HARDY_LITTLEWOOD_FRAMEWORK.md, MIDPOINT_ANALYSIS.md,
   etc.) have not been spot-checked for accuracy.
4. **EVIDENCE.md data tables** use small sample sizes (n=10). Success rate confidence
   intervals are wide (e.g., 30% +/- 28% at 95% CI with n=10).

**What the next tranche should address:**

1. **Commit the accumulated work** -- 26 tracks. Most urgent practical need. Without
   this, none of the hardening work is preserved in git history.
2. **Agda postulate reduction (SymmetryFiniteReflect)** -- `reflect-involutive` is
   provable for specific m by case analysis. For m=6 it was verified in Track 20.
   A general proof needs modular arithmetic lemmas from stdlib.
3. **Integration test for membrane prime generation** -- the unit tests verify
   construction correctness; an integration test could verify that the base 6
   champion configuration actually produces primes at the claimed ~33% rate.

## Tranche 9: Novelty Clarity and HL Validation

**Created**: 2026-03-09

### Track 27: Novelty Synthesis

**Status**: `complete`
**Priority**: P0
**Scope**: medium

Why this matters:
- The project's public surfaces (README, CLAUDE.md) still frame membrane
  constructions as if the structure is the discovery. THEORETICAL_CLOSURE.md
  says the entire advantage is coprimality filtering.
- Without an explicit novelty assessment, the repo over-promises. A clear
  1-page document forces honesty about what is actually new.

What was done:
- Created NOVELTY.md: honest assessment of what this project contributes.
- Classifies each contribution: falsification record (methodological),
  coprimality reduction (classical, new presentation), connector asymmetry
  (empirical, single instance), HL implementation (software), Agda
  formalization (partial), physics metaphor (not math).
- Added to README.md documentation table as the first entry.

Acceptance criteria (all met):
- NOVELTY.md exists and classifies every major contribution with an honest
  status
- No claim in NOVELTY.md is stronger than the evidence supports
- NOVELTY.md is linked from README.md

Verification:
```bash
test -f NOVELTY.md && echo "exists"
grep -c 'NOVELTY.md' README.md  # Should be >= 1
```

---

### Track 28: Hardy-Littlewood Empirical Validation Tests

**Status**: `complete`
**Priority**: P1
**Scope**: medium

Why this matters:
- The HL framework had 11 tests but all were mathematical identity tests
  (S2(30) = 8/3, kappa relationship, edge cases). None tested the core
  purpose: do HL predictions track actual Goldbach pair counts?
- This is the difference between "the code computes the formula correctly"
  and "the formula actually predicts reality."

What was done:
- Added 4 new tests to `src/hzlib/hardy_littlewood.rs`:
  1. `test_hl_prediction_vs_actual_counts`: Compares HL lambda to actual
     pair counts for n in [10, 2000]. Verifies Pearson r > 0.90.
  2. `test_truncated_prediction_tracks_restricted_counts`: Verifies the
     truncated formula tracks restricted counts (base >= 50). r > 0.80.
  3. `test_singular_series_highly_composite`: Verifies S2(2310) against
     manual calculation (product of (p-1)/(p-2) terms). Tests that highly
     composite n has higher S2 than powers of 2.
  4. `test_predict_goldbach_pairs_with_fitted_khat`: Fits k_hat from
     actual data, verifies it is near C2, then checks predictions are
     within 3x of actual counts.
- HL test count: 11 -> 15. Total lib test count: 170 -> 174.

Acceptance criteria (all met):
- `cargo test --lib hzlib::hardy_littlewood` passes 15/15 tests
- At least one test validates predictions against actual pair counts
  (not just mathematical identities)
- All new tests are deterministic (no random sampling)

Verification:
```bash
cargo test --lib hzlib::hardy_littlewood 2>&1 | tail -5
# Should show: 15 passed; 0 failed
```

---

### Track 29: Stale Test Count References

**Status**: `complete`
**Priority**: P2
**Scope**: small

Why this matters:
- Track 25 (Tranche 8) added 27 membrane tests, increasing the count from
  143 to 170. Track 28 added 4 more, reaching 174. But CLAIMS.md, README.md,
  CLAUDE.md, and collab/README.md all still said "143 tests." This is the
  kind of drift that accumulates when generated surfaces are hand-maintained.

What was done:
- Updated test count from 143 to 174 in:
  - CLAIMS.md (claim #9)
  - README.md (Quick Start section)
  - CLAUDE.md (status line and verification section)
  - collab/README.md (verification section)
- Also fixed collab/README.md Agda count: "11 of 80" -> "32 of 80 (20 clean,
  12 with postulates)" to match agda-proofs/STATUS.md.

Acceptance criteria (all met):
- `grep -r '143 tests\|143 lib' *.md collab/*.md` returns no matches
- Actual test count matches documented count: `cargo test --lib 2>&1 | grep passed`
  shows 174

Verification:
```bash
grep -r '143' CLAIMS.md README.md CLAUDE.md collab/README.md | grep -i test
# Should return empty
cargo test --lib 2>&1 | tail -1
# Should show: 174 passed
```

---

## Tranche 9 Reassessment

**Strongest verified spine after Tranche 9:**
1. 174 library tests pass (4 new HL validation tests). Clippy clean on all targets.
2. Hardy-Littlewood predictions validated against actual Goldbach pair counts
   (Pearson r > 0.90 for unrestricted, r > 0.80 for truncated).
3. NOVELTY.md provides an honest classification of every contribution.
4. Test counts are consistent across all 5 documentation surfaces.

**Where public signal still exceeds actual support:**
1. **Connector asymmetry** (CLAUDE.md Section 5c, parent CLAUDE.md): Tested on
   a single prime pair. Presented with detailed statistics but the generality
   claim is unsupported. NOVELTY.md flags this correctly.
2. **Period-6 resonance**: Documented but the "optimal phase varies empirically"
   means it is not predictive. Its status as a finding is ambiguous.
3. **"Physics engine" framing**: The crate name and simulation layer imply physics
   content. NOVELTY.md clarifies this is metaphor.

**What the next tranche should address:**
1. **Commit the accumulated work** -- 29 tracks across 9 tranches are uncommitted.
   Without this, none of the hardening work is preserved in git history.
2. **Agda postulate reduction (SymmetryFiniteReflect)** -- `reflect-involutive`
   is provable for specific m by case analysis. General proof needs stdlib
   modular arithmetic lemmas.
3. **Integration test for membrane density claims** -- verify that base 6
   champion configuration produces primes at the claimed ~33% rate.
4. **Parent research doc spot-check** -- HARDY_LITTLEWOOD_FRAMEWORK.md,
   MIDPOINT_ANALYSIS.md, MATHEMATICAL_EXPLANATION.md accuracy audit.

---

## Methodology

### Tranche 1 (complete)

Tracks were ordered by dependency:
1. **Track 1** (fix tests) established the green baseline.
2. **Track 2** (lib.rs docs) corrected the first-contact developer surface.
3. **Track 3** (doc audit) removed the dead documentation layer.
4. **Track 4** (example triage) created a navigable curated example set.
5. **Track 5** (untracked source) was assessed but not yet committed.

### Tranche 2 (complete)

Tracks were ordered by public impact:
1. **Track 6** (README rewrite) fixed the public entry point.
2. **Track 7** (subdirectory example cleanup) removed misleading verified/ and
   experimental/ directories.
3. **Track 8** (collab refresh) surfaced the key finding for collaborators.
4. **Track 9** (Agda ground truth) established honest verification counts.

### Tranche 3 (complete)

Tracks were ordered by leverage:
1. **Track 10** (Agda repair) restored 7 modules with a 2-file fix.
2. **Track 11** (CLAUDE.md tightening) reduced the developer reference from
   965 to 186 accurate lines.
3. **Track 12** (Agda doc consolidation) moved 4 stale docs to historical/,
   reducing root markdown from 14 to 10.

### Tranche 4 (complete)

Tracks were ordered by CI readiness:
1. **Track 13** (all-targets clippy) fixed ~40 warnings across 22 files.
2. **Track 14** (CI workflow repair) fixed Agda and example lists in CI config.
3. **Track 15** (CLAIMS.md) created a single-page claim-evidence registry.

### Tranche 5 (complete)

Tracks were ordered by evidence integrity:
1. **Track 16** (EVIDENCE.md repair) found and corrected 5 false primality
   claims (42% error rate in spot-check) and marked 9 nonexistent script
   references.
2. **Track 17** (parent CLAUDE.md) rewrote from 660 to 130 lines, eliminating
   duplication with the inner CLAUDE.md.
3. **Track 18** (parent markdown triage) archived 31 of 41 parent-level
   markdown files (15 empty, 16 stale session logs).

### Tranche 6 (complete)

Tracks were ordered by evidence integrity then formal verification:
1. **Track 19** (EVIDENCE.md deeper audit) verified data tables in Sections
   2.1, 4.1, and 4.2. Found 6 false entries: inflated success rates (40% -> 20%,
   35% -> 10%), wrong working seed lists, and misleading factorizations.
2. **Track 20** (CertifiedResonanceComplete postulates) eliminated all 6
   postulates from the flagship Agda example by using explicit Fin constructor
   patterns instead of the `#_` operator. Module moves from "postulated" to
   "clean."

### Tranche 7 (complete)

Tracks addressed API clarity and formal verification extension:
1. **Track 21** (CLAIMS.md + STATUS.md refresh) fixed count discrepancies
   and added EVIDENCE.md audit notes.
2. **Track 22** (module-level API docs) added Layer annotations to all
   public modules, reorganized lib.rs into grouped sections, and replaced
   overclaiming doc comments in simulation modules.
3. **Track 23** (Param/ParamDyn postulates) reduced example-level postulates
   from 2 to 0 (Param) and 3 to 1 (ParamDyn) by constructing PerfectBuckets
   directly via case analysis.

### Tranche 8 (complete)

Tracks addressed workspace-wide warning cleanup, test coverage, and doc accuracy:
1. **Track 24** (density-explorer cleanup) eliminated all 37 clippy warnings
   in the density-explorer workspace member. First time entire workspace is
   warning-free.
2. **Track 25** (membrane core tests) added 27 unit tests covering the
   `MembraneConfig` public API. Test count: 143 -> 170.
3. **Track 26** (GLOSSARY.md fix) corrected a false primality claim in the
   Lagrange Points example and updated stale references.

### Tranche 9 (complete)

Tracks addressed novelty clarity, HL empirical validation, and doc consistency:
1. **Track 27** (NOVELTY.md) created an honest 1-page synthesis of what this
   project actually contributes vs what is standard number theory.
2. **Track 28** (HL validation tests) added 4 tests that check HL predictions
   against actual Goldbach pair counts (Pearson r > 0.90). Test count: 170 -> 174.
3. **Track 29** (stale test counts) updated all 5 documentation surfaces from
   "143 tests" to "174 tests." Fixed Agda count in collab/README.md.
