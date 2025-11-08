# Agda Proof Techniques for Membrane Verification

**A Practical Guide to Common Proof Patterns**

This guide shows you how to actually *write* proofs in Agda for membrane properties. We'll cover techniques you'll use constantly in this project.

---

## The Proof Toolkit

### Level 0: Computational Proofs (`refl`)

**When to use**: When Agda can compute both sides and check they're equal.

```agda
-- Example: GCD computation
_ : gcd 12 18 ≡ 6
_ = refl  -- Agda computes: gcd 12 18 → ... → 6 ✓

-- Example: Coprimality check
_ : 7 ⊥ radical 10
_ = refl  -- Expands to: gcd 7 10 ≡ 1, computes to 1 ≡ 1 ✓
```

**Why it works**: Agda's type checker includes a normalizer that evaluates expressions. If both sides normalize to the same thing, `refl` (reflexivity) suffices.

**Limitations**: Only works for *definitional* equality. Can't prove `n + 0 ≡ n` with `refl` because addition is defined by recursion on the first argument.

---

### Level 1: Rewriting with Equations

**When to use**: When you need to transform one side using known equations.

```agda
-- Example: Prove commutativity affects result
example : ∀ m n → gcd m n ≡ gcd n m
example m n = gcd-comm m n  -- Use the theorem gcd-comm

-- Example: Chain multiple rewrites
example2 : ∀ m → gcd m 0 ≡ m
example2 m = refl  -- Base case, computes directly
```

**The `trans` combinator**: Chain multiple equality steps.

```agda
-- Prove: gcd 12 18 ≡ 6 via intermediate steps
proof : gcd 12 18 ≡ 6
proof = trans step1 step2
  where
    step1 : gcd 12 18 ≡ gcd 18 (12 mod 18)
    step1 = refl

    step2 : gcd 18 12 ≡ 6
    step2 = refl  -- Continue algorithm
```

**The `sym` combinator**: Flip an equation.

```agda
-- If you have: a ≡ b
-- You get: sym : b ≡ a

example : 1 ≡ gcd 3 10
example = sym proof
  where
    proof : gcd 3 10 ≡ 1
    proof = refl
```

---

### Level 2: Case Analysis (`with` clauses)

**When to use**: When the proof depends on examining a value.

```agda
-- Example: GCD base case
gcd-with-zero : ∀ n → gcd n 0 ≡ n
gcd-with-zero n with n
... | zero = refl
... | suc n' = refl
```

**Pattern matching in definitions**:

```agda
is-coprime? : ℕ → ℕ → Bool
is-coprime? m n with gcd m n
... | zero = false
... | suc zero = true  -- gcd = 1
... | suc (suc _) = false  -- gcd > 1
```

**Matching on proofs**:

```agda
example : ∀ d m n → d ∣ m → d ∣ (m + n) → d ∣ n
example d m n (k₁ , eq1) (k₂ , eq2) = ...
  -- eq1 : m ≡ k₁ * d
  -- eq2 : (m + n) ≡ k₂ * d
  -- Derive: n ≡ (k₂ - k₁) * d
```

---

### Level 3: Induction on Natural Numbers

**When to use**: Proving properties that hold for all ℕ.

**Basic pattern**:

```agda
proof : ∀ n → Property n
proof zero = base-case-proof
proof (suc n) = inductive-step n (proof n)
  where
    inductive-step : ∀ n → Property n → Property (suc n)
    inductive-step n IH = ...  -- IH = inductive hypothesis
```

**Example: Addition commutativity** (classic induction)

```agda
+-comm : ∀ m n → m + n ≡ n + m
+-comm zero n = sym (+-zero n)  -- Base: 0 + n ≡ n + 0
  where
    postulate +-zero : ∀ n → n + 0 ≡ n
+-comm (suc m) n = trans step1 step2
  where
    step1 : suc m + n ≡ suc (m + n)
    step1 = refl  -- By definition of +

    step2 : suc (m + n) ≡ n + suc m
    step2 = cong suc (+-comm m n)  -- Use IH!
```

**Membrane application**: Proving properties for all seed values.

```agda
all-seeds-coprime : ∀ config seed →
  BoundariesCoprime config →
  membraneValue config seed ⊥ radical (base config)
all-seeds-coprime config zero prf = base-case
all-seeds-coprime config (suc seed) prf =
  inductive-step seed (all-seeds-coprime config seed prf)
```

---

### Level 4: Induction on Lists

**When to use**: Proving properties about digit sequences (crucial for membranes!).

**Basic pattern**:

```agda
list-property : ∀ (xs : List ℕ) → Property xs
list-property [] = base-case
list-property (x ∷ xs) = inductive-step x xs (list-property xs)
```

**Example: Reverse of reverse is identity**

```agda
reverse-involutive : ∀ (xs : List ℕ) → reverse (reverse xs) ≡ xs
reverse-involutive [] = refl
reverse-involutive (x ∷ xs) = trans step1 step2
  where
    step1 : reverse (reverse (x ∷ xs)) ≡ reverse (reverse xs ++ [ x ])
    step1 = cong reverse refl

    step2 : reverse (reverse xs ++ [ x ]) ≡ x ∷ xs
    step2 = {! ... needs reverse-++ lemma !}
```

**Membrane application**: Proving digit list symmetry.

```agda
membrane-symmetric : ∀ config seed →
  let digits = buildMembraneDigits config seed
  in reverse digits ≡ digits

membrane-symmetric config seed = induct-on-structure digits
  where
    digits = buildMembraneDigits config seed
    -- Proof by induction on how digits are built
```

---

### Level 5: Using Helper Lemmas

**Strategy**: Break complex proofs into smaller lemmas.

**Example: Prove coprimality preserved under concatenation**

```agda
-- Main theorem (complex)
coprime-concat : ∀ d₁ d₂ base →
  d₁ ⊥ base → d₂ ⊥ base →
  (d₁ * base + d₂) ⊥ base

-- Break it down:

-- Lemma 1: If d ⊥ b, then d * b ⊥ b (false! need correction)
-- Actually: d * b shares factor b, so revise...

-- Lemma 1 (corrected): gcd properties under multiplication
lemma1 : ∀ d b → d ⊥ b → gcd (d * b) b ≡ b
lemma1 d b prf = ...

-- Lemma 2: Adding coprime numbers
lemma2 : ∀ n d b → d ⊥ b → gcd (n + d) b ≡ gcd n b
lemma2 n d b prf = ...

-- Now prove main theorem using lemmas
coprime-concat d₁ d₂ base prf1 prf2 =
  trans (lemma2 (d₁ * base) d₂ base prf2)
        (lemma1 d₁ base prf1)
```

**Membrane application**: Symmetry proof uses multiple lemmas.

```agda
-- Main theorem
membrane-symmetric : ∀ config seed → ...

-- Lemmas needed:
lemma-left-half-reverse : reverse left-half ≡ right-half
lemma-seed-symmetric : seed is palindrome or single digit
lemma-concat-symmetric : symmetric parts → symmetric whole
```

---

### Level 6: The `where` Clause (Clean Proofs)

**When to use**: To organize complex proofs with named intermediate results.

```agda
-- Example: GCD divisibility chain
theorem : ∀ a b c → a ∣ b → b ∣ c → a ∣ c
theorem a b c a∣b b∣c = final-step
  where
    -- Name the witnesses
    k₁ : ℕ
    k₁ = proj₁ a∣b  -- Extract k from (k, proof)

    eq1 : b ≡ k₁ * a
    eq1 = proj₂ a∣b

    k₂ : ℕ
    k₂ = proj₁ b∣c

    eq2 : c ≡ k₂ * b
    eq2 = proj₂ b∣c

    -- Build the final proof
    k-final : ℕ
    k-final = k₂ * k₁

    final-step : a ∣ c
    final-step = (k-final , {! c ≡ k-final * a !})
```

**Best practice**: Name your intermediate results with descriptive identifiers.

---

### Level 7: Working with Decidable Properties

**When to use**: When a property is computable (like coprimality).

```agda
-- Coprimality is decidable
coprime? : (m n : ℕ) → Dec (m ⊥ n)
coprime? m n with gcd m n ≟ 1
... | yes prf = yes prf  -- Coprime!
... | no ¬prf = no ¬prf  -- Not coprime

-- Use in proofs
example : ∀ m n → ...
example m n with coprime? m n
... | yes coprime-prf = -- Use the proof
... | no ¬coprime = -- Handle non-coprime case
```

**Membrane application**: Check if config is valid.

```agda
valid-config? : MembraneConfig → Dec (ValidConfig config)
valid-config? config with coprime? outer (radical base)
                         | coprime? inner (radical base)
  where
    open MembraneConfig config
... | yes prf1 | yes prf2 = yes (prf1 , prf2)
... | no ¬prf1 | _ = no ...
... | _ | no ¬prf2 = no ...
```

---

## Common Membrane Proof Patterns

### Pattern 1: Proving Coprimality for Concrete Values

```agda
-- Step 1: Compute the radical
_ : radical 10 ≡ 10
_ = refl

-- Step 2: Prove coprimality by computation
_ : 3 ⊥ radical 10
_ = refl  -- gcd 3 10 → 1 ✓

-- Step 3: Generalize if needed
all-coprime-digits-base-10 : ∀ d → d ∈ [1,3,7,9] → d ⊥ 10
all-coprime-digits-base-10 1 _ = refl
all-coprime-digits-base-10 3 _ = refl
all-coprime-digits-base-10 7 _ = refl
all-coprime-digits-base-10 9 _ = refl
```

### Pattern 2: Proving Symmetry by Construction

```agda
-- Strategy: Show the structure is built symmetrically

step1 : left-half ≡ [outer] ++ k₁-zeros ++ [inner] ++ k₂-zeros
step1 = refl  -- By definition

step2 : right-half ≡ k₂-zeros ++ [inner] ++ k₁-zeros ++ [outer]
step2 = refl  -- By definition

step3 : reverse left-half ≡ right-half
step3 = reverse-concat-lemma ...

step4 : left-half ++ seed-part ++ right-half is symmetric
step4 = concat-symmetric-parts step3 seed-symmetric
```

### Pattern 3: Proving Necessity (Contrapositive)

```agda
-- To prove: Prime → Coprime
-- Prove contrapositive: ¬Coprime → ¬Prime

non-coprime-prevents-prime : ∀ config seed →
  ¬ (outer ⊥ radical base) →
  ¬ (IsPrime (membraneValue config seed))
non-coprime-prevents-prime config seed ¬coprime prime-prf =
  contradiction
  where
    -- Extract the shared factor
    d : ℕ
    d = gcd outer (radical base)

    d>1 : d > 1
    d>1 = ¬coprime-means-gcd>1 ¬coprime

    d∣outer : d ∣ outer
    d∣outer = gcd-divides-left outer (radical base)

    d∣membrane : d ∣ membraneValue config seed
    d∣membrane = divisor-of-digit-divides-number d outer d∣outer

    -- But if membrane is prime and d > 1 divides it...
    contradiction : ⊥
    contradiction = prime-not-divisible prime-prf d d>1 d∣membrane
```

---

## Debugging Proof Failures

### Technique 1: Goal Inspection

```
-- In Emacs: C-c C-,
-- Shows:
Goal: membraneValue config seed ≡ expectedValue
————————————————————————————————————
config : MembraneConfig
seed   : ℕ
```

**What to do**: Look at what you have vs. what you need. If they don't match, you need a lemma to bridge them.

### Technique 2: Normalization

```
-- In Emacs: C-c C-n
-- Prompts: Expression to normalize?
-- Type: gcd 12 18
-- Shows: 6
```

**Use case**: Check if computational proofs should work.

### Technique 3: Type-Directed Search

```
-- In a hole, type C-c C-r (refine)
-- Agda tries to fill it automatically
```

### Technique 4: Case Splitting

```
-- On a variable, type C-c C-c
-- Prompts: Variable to case split?
-- Type: n
-- Agda generates:
example n = {!!}
-- Becomes:
example zero = {!!}
example (suc n) = {!!}
```

---

## Tips from Experience

1. **Start with `postulate`**: State the theorem first, prove it later
2. **Test with examples**: If `refl` works for examples, the theorem is probably true
3. **Name your lemmas**: `lemma1`, `lemma2` is bad; `gcd-divides-left` is good
4. **Draw diagrams**: Agda is precise, but humans think visually
5. **Ask for the goal**: `C-c C-,` constantly to see what you're proving
6. **Normalize expressions**: `C-c C-n` to see computed values
7. **Don't fill all holes**: Focus on interesting proofs, defer trivial ones

---

## Next Steps

- **Practice**: Try the exercises in `GETTING_STARTED.md`
- **Study**: Read proofs in `agda-stdlib`, especially `Data.Nat.Properties`
- **Experiment**: Break things! See what error messages teach you
- **Ask**: Agda's errors are precise—learn to read them

---

**Happy Proving!**

*"Every proof is a conversation with the type checker. Learn to listen to what it's telling you!"*
