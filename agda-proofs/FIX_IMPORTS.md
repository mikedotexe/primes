# Agda Import Compatibility Notes

This note captures the import-style drift that shows up when older Agda files
are brought forward to the current local toolchain.

## Current Local Baseline

- Agda `2.8.0`
- standard library `2.3`

See [`STATUS.md`](STATUS.md) for the audited module results under that setup.

## Common Import Repairs

| Older style | Current stdlib style |
|-------------|----------------------|
| `Agda.Builtin.Empty` | `Data.Empty` |
| `Agda.Builtin.Sigma` | `Data.Product` |
| `Agda.Builtin.Equality` | `Relation.Binary.PropositionalEquality` |
| `Agda.Builtin.Nat` | `Data.Nat` |
| `Nat` | `ℕ` |

## Local Helper Script

From the `agda-proofs/` directory:

```bash
./scripts/fix-agda-imports.sh
```

Use it as a bulk starting point, then rerun the specific module you are trying
to repair.

## Scope

- this is troubleshooting guidance, not proof coverage
- successful import repair does not imply the repaired module type-checks
- broader proof status remains centralized in [`STATUS.md`](STATUS.md)
