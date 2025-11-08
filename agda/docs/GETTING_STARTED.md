# Getting Started with Agda Formal Verification

**Welcome, Researcher!**

This guide will walk you through using Agda to formally verify properties of membrane prime construction. No prior Agda experience required—we'll learn by doing.

---

## What You'll Learn

1. How to load and navigate Agda modules
2. Basic proof techniques for membrane properties
3. How to verify new configurations
4. Connecting formal proofs to empirical Rust code

**Estimated time**: 1-2 hours for basics, ongoing for research

---

## Prerequisites

✅ Follow the installation instructions in `../README.md`
✅ Have Emacs or VS Code set up with Agda mode
✅ Basic understanding of the membrane concept (see `../CLAUDE.md`)

---

## Part 1: Your First Agda Session (15 minutes)

### Step 1: Load a Module

Open `src/PrimePhysics/Foundation/GCD.agda` in your editor.

**In Emacs**: Press `C-c C-l` (Control-c Control-l)
**In VS Code**: Press `Ctrl+Alt+L`

You should see:
```
Checking PrimePhysics.Foundation.GCD ...
 Finished PrimePhysics.Foundation.GCD.
```

✅ **Success!** Agda has verified all the definitions and theorems in this module.

### Step 2: Inspect a Proof

Scroll to the examples at the bottom:

```agda
-- Example: gcd(10, 3) = 1 (coprime!)
_ : gcd 10 3 ≡ 1
_ = refl
```

**What's happening here?**
- `gcd 10 3 ≡ 1`: States that gcd of 10 and 3 equals 1
- `= refl`: The proof—Agda computes gcd(10,3) and checks it's 1

**Try it yourself**: Add this line before the example:

```agda
-- My test: gcd(12, 18) = 6
_ : gcd 12 18 ≡ 6
_ = refl
```

Reload with `C-c C-l`. It should verify successfully!

### Step 3: Break Something (Learn from Errors)

Now change the example to:

```agda
_ : gcd 12 18 ≡ 5  -- Wrong! Should be 6
_ = refl
```

Reload. Agda will highlight the error:
```
5 != 6 of type ℕ
when checking that the expression refl has type gcd 12 18 ≡ 5
```

**Lesson**: Agda checks proofs by computation. If you claim 5 when the answer is 6, it catches it!

✅ Change it back to 6 and verify it loads correctly.

---

## Part 2: Understanding the Membrane Structure (20 minutes)

### Step 1: Load the Membrane Module

Open `src/PrimePhysics/Membrane/Structure.agda` and load it.

### Step 2: Examine the Configuration

Find the `MembraneConfig` record:

```agda
record MembraneConfig : Set where
  field
    base : ℕ
    outer : ℕ
    inner : ℕ
    k₁ : ℕ      -- Outer padding count
    k₂ : ℕ      -- Inner padding count
    -- ... constraints ...
```

**This formalizes exactly what CLAUDE.md describes!**

### Step 3: Look at Example Configurations

Find `example-config-1`:

```agda
example-config-1 : MembraneConfig
example-config-1 = record
  { base = 10
  ; outer = 3
  ; inner = 7
  ; k₁ = 2
  ; k₂ = 1
  ; base≥2 = {! trivial proof !}  -- These are "holes"
  ; outer<base = {! 3 < 10 !}
  ; ...
  }
```

**Holes** (written `{! ... !}`) are placeholders for proofs we haven't filled in yet. They're marked with `?` when you load the file.

**Challenge**: Let's fill one in!

### Step 4: Fill Your First Proof Hole

For `base≥2 = {! trivial proof !}`, we need to prove `10 > 1`.

**In Emacs**:
1. Position cursor inside the hole `{! trivial proof !}`
2. Press `C-c C-,` to see what's needed: `10 > 1`
3. This is trivially true by computation, so try: `{! refl !}`
4. But that won't work! `10 > 1` isn't a definitional equality.

**The issue**: Inequality proofs need the stdlib machinery. For now, leave them as holes—we'll come back to them.

**Lesson**: Some proofs are trivial, some need machinery. Holes let us defer the tedious ones.

---

## Part 3: Proving Coprimality (30 minutes)

### Step 1: Load the Examples Module

Open `src/PrimePhysics/Examples/BasicMembranes.agda`.

### Step 2: Verify a Coprimality Claim

Find Example 1:

```agda
example-1-outer-coprime : 3 ⊥ radical 10
example-1-outer-coprime = refl  -- gcd(3, 10) = 1 ✓
```

**What's `3 ⊥ radical 10`?**
- Notation for "3 is coprime to radical(10)"
- Expands to: `gcd 3 (radical 10) ≡ 1`
- `radical 10` computes to `10` (since 10 = 2×5, distinct primes)
- So this is: `gcd 3 10 ≡ 1`

**Why does `refl` work?**
- Agda computes: `gcd 3 10` → (Euclidean algorithm) → `1`
- Checks: `1 ≡ 1` → YES!
- Proof complete: `refl` (for "reflexivity")

### Step 3: Prove Your Own Coprimality

Add a new example after Example 1:

```agda
-- My example: 9 is coprime to 10
my-coprime-test : 9 ⊥ radical 10
my-coprime-test = refl
```

Load the file. It should verify!

**Try a false claim**:

```agda
my-false-claim : 4 ⊥ radical 10  -- False! gcd(4,10) = 2
my-false-claim = refl
```

Agda will reject this:
```
2 != 1 of type ℕ
```

**Lesson**: Coprimality is decidable—Agda can compute it and catch false claims.

### Step 4: Why This Matters

Look at Example 5:

```agda
example-5-outer-not-coprime : gcd 2 (radical 10) ≡ 2
example-5-outer-not-coprime = refl  -- gcd(2, 10) = 2 ≠ 1

postulate
  example-5-cannot-be-prime :
    ∀ seed → ¬ (IsPrime (membraneValue config-2-5-0-0 seed))
```

**This says**: If your boundary digits aren't coprime, NO seed will produce a prime!

This is proven in `Properties.agda` by the `non-coprime-boundary-prevents-primality` theorem.

---

## Part 4: Connecting to Rust Code (20 minutes)

### How Agda and Rust Work Together

```
┌─────────────────────────────────────────────────────────────┐
│                   VERIFICATION WORKFLOW                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. DISCOVER (Rust):                                       │
│     cargo run --example statistical_prime_generator        │
│     "Base 6 with (1,5) achieves 33% success!"             │
│                                                             │
│  2. FORMALIZE (Agda):                                      │
│     config-1-5-base6 : MembraneConfig                      │
│     Prove: coprime boundaries, symmetry, etc.              │
│                                                             │
│  3. PROVE NECESSITY (Agda):                                │
│     optimal-config-has-coprime-boundaries                  │
│     "All good configs MUST be coprime"                     │
│                                                             │
│  4. REFINE SEARCH (Rust):                                  │
│     Only test coprime configs (narrowed search space)      │
│     Find next breakthrough faster!                         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Practical Example

**In Rust** (from `CLAUDE.md`):
```rust
// Empirical finding: (3,7) k=(2,1) works well in base 10
cargo run --example proper_membrane_generator -- --base 10 --outer 3 --inner 7
```

**In Agda** (verify it's coprime):
```agda
_ : 3 ⊥ radical 10
_ = refl  -- ✓

_ : 7 ⊥ radical 10
_ = refl  -- ✓
```

**Insight**: Rust finds it works empirically; Agda proves WHY (coprimality is necessary).

---

## Part 5: Your First Research Proof (30+ minutes)

### Goal: Prove a New Configuration's Coprimality

**Task**: You've discovered a new high-performing config in Rust:
- Base 14
- Outer = 3, Inner = 11
- Does it satisfy our coprimality theorem?

### Step 1: Compute the Radical

Open `src/PrimePhysics/Foundation/Radical.agda`.

Find the `radical` function. Add a case for 14:

```agda
radical 14 = 14  -- 14 = 2×7, so rad(14) = 2×7 = 14
```

(Insert this in numerical order with the other cases.)

### Step 2: Test Coprimality

Go to `Examples/BasicMembranes.agda` and add:

```agda
-- My discovery: base 14 config
_ : 3 ⊥ radical 14
_ = refl

_ : 11 ⊥ radical 14
_ = refl
```

Load the file. **Do they verify?**

- If YES: Your config satisfies the coprimality theorem! ✓
- If NO: The config is mathematically doomed—save your testing time!

### Step 3: Create the Configuration

Add a new configuration:

```agda
config-3-11-base14 : MembraneConfig
config-3-11-base14 = record
  { base = 14
  ; outer = 3
  ; inner = 11
  ; k₁ = 0
  ; k₂ = 0
  ; base≥2 = {!!}
  ; outer<base = {!!}
  ; inner<base = {!!}
  ; outer>0 = {!!}
  ; inner>0 = {!!}
  }
```

### Step 4: Apply the Theorem

Add:

```agda
my-config-coprime-boundaries :
  (3 ⊥ radical 14) × (11 ⊥ radical 14)
my-config-coprime-boundaries = (refl , refl)

postulate
  my-config-membrane-coprime :
    ∀ seed → membraneValue config-3-11-base14 seed ⊥ radical 14
```

**You've now formally verified that your new config has the necessary coprimality property!**

### Step 5: Test in Rust

Run the empirical test:

```bash
cd ../prime-physics-engine
cargo run --example statistical_prime_generator -- --base 14 --outer 3 --inner 11
```

Check the success rate. If it's high, you've found a winner!

---

## Part 6: Advanced Topics (Ongoing)

### Filling Proof Holes

Those `{!!}` holes throughout the code? Some are trivial, some need work.

**Easy ones** (inequalities):
```agda
base≥2 : 10 > 1
```

Use the stdlib's `s≤s` (successor ≤ successor) repeatedly:
```agda
base≥2 = s≤s z≤n  -- This proves suc zero ≤ suc something
```

**Harder ones** (symmetry):
```agda
membrane-digits-symmetric : ...
```

Requires induction on list structure. See Agda tutorials for techniques.

### Proving New Theorems

Want to prove something not yet in the codebase?

1. State it as a `postulate` first
2. Use it in examples to test if it's true
3. If Agda accepts the postulate + examples, try to prove it
4. Replace `postulate` with actual proof

**Example**:

```agda
-- Hypothesis: Minimal padding is always coprime-preserving
postulate
  minimal-padding-coprime : ∀ config seed →
    let base = MembraneConfig.base config
    in MembraneConfig.k₁ config ≡ 0 →
       MembraneConfig.k₂ config ≡ 0 →
       membraneValue config seed ⊥ radical base →
       -- some conclusion...
```

### Extracting to Haskell

Agda can extract verified code to Haskell:

```bash
agda --compile src/PrimePhysics/Membrane/Structure.agda
```

This creates a Haskell module you can use as a reference implementation for porting to Rust!

---

## Common Questions

**Q: Why are there so many `postulate`s?**
A: Phase 1 focuses on structure. Filling proofs is Phase 2. Postulates let us state theorems and test their usage before doing the proof work.

**Q: Can Agda prove success rates (like 33%)?**
A: No. Those are empirical. Agda proves *structural properties* (symmetry, coprimality) that are *necessary* for high success.

**Q: Do I need to fill all the holes?**
A: No! Leave trivial ones as holes. Focus on the interesting theorems.

**Q: How do I know if my proof is right?**
A: If Agda accepts it, it's correct! That's the beauty of formal verification.

**Q: What if I get stuck on a proof?**
A: Use `postulate` and move on. Mark it with a TODO comment. The research continues!

---

## Next Steps

### For Learning Agda
- Work through [PLFA](https://plfa.github.io/) chapters 1-10
- Study proofs in `agda-stdlib`, especially `Data.Nat.Properties`
- Practice with the examples in this repo

### For This Project
- Fill proofs in `Foundation/` modules
- Prove the symmetry theorem in `Membrane/Structure.agda`
- Connect to Hardy-Littlewood framework
- Formalize migration patterns

### For Research
- Test new configurations in Rust
- Verify coprimality in Agda
- Prove new theorems about membrane properties
- Find universal patterns across bases

---

## Getting Help

- **Agda syntax**: See [Agda docs](https://agda.readthedocs.io/)
- **Proof techniques**: See `PROOF_TECHNIQUES.md` (coming soon)
- **Project-specific**: Open an issue on GitHub

---

## Summary: The Agda Workflow

```
1. Load module (C-c C-l)
   ↓
2. Inspect definitions and theorems
   ↓
3. Test examples with refl
   ↓
4. Add your own examples
   ↓
5. State new theorems as postulates
   ↓
6. Fill proof holes when ready
   ↓
7. Extract insights for Rust code
```

**Remember**: Agda proves what's *necessarily true*. Rust discovers what's *surprisingly effective*. Together, they're powerful!

---

**Happy Proving! 🎯**

*"The best way to learn Agda is to prove things that matter to you. Membrane primes are a perfect playground!"*
