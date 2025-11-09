# Agda Scan Tool

Scanner utility for analyzing Agda codebases and detecting structural issues.

## Features

- **Module ↔ Path Mismatches**: Detects when module declarations don't match file paths
- **Missing Internal Modules**: Identifies referenced modules that don't exist
- **Import Histogram**: Shows most frequently imported modules
- **Postulate Analysis**: Counts postulates by file and directory

## Usage

```bash
# Scan the agda-proofs directory
cargo run -- ../../agda-proofs

# From repository root
cargo run --manifest-path tools/agda-scan/Cargo.toml -- agda-proofs
```

## Output

The tool provides four categories of analysis:

1. **Module/path mismatches** - Files where the module declaration doesn't match the file path (e.g., `module tests.Spec.Foo` in `Tests/Spec/Foo.agda`)

2. **Missing internal modules** - Modules imported but not found in the codebase (excludes stdlib modules like `Data.*`, `Relation.*`, etc.)

3. **Top imports** - Most frequently imported modules to understand dependencies

4. **Postulates** - Count of postulated (unproven) definitions, which must be resolved to enable Safe Agda

## Example Output

```
=== Module ↔ path mismatches (3) ===
- Tests/Spec/Foo.agda declares tests.Spec.Foo (expected suffix Tests/Spec/Foo.agda)

=== Missing internal modules (4 kinds) ===
- Core.Primality  (referenced in 5 files)
    - Core/MembraneTheory.agda
    - Examples/Base6Analysis.agda

=== Postulates ===
Total: 373
By directory:
- Theorems: 245
- Core: 45
...
```

## Use Cases

- **Pre-commit validation**: Check for structural issues before committing
- **Refactoring**: Track impact of moving/renaming modules
- **Safe Agda migration**: Monitor progress removing postulates
- **Dependency analysis**: Understand module coupling
