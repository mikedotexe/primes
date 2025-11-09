#!/usr/bin/env bash
#
# Fix Agda import paths for compatibility with Agda 2.8.0

set -euo pipefail

AGDA_DIR="agda-proofs"

echo "🔧 Fixing Agda imports for Agda 2.8.0 compatibility..."
echo ""

# Count files to process
total=$(find "$AGDA_DIR" -name "*.agda" | wc -l)
echo "📁 Found $total Agda files to process"
echo ""

count=0

# Process each .agda file
find "$AGDA_DIR" -name "*.agda" | while read -r file; do
  count=$((count + 1))
  echo "[$count/$total] Processing: $file"

  # Create backup
  cp "$file" "$file.bak"

  # Apply replacements
  sed -i '' \
    -e 's/Agda\.Builtin\.Empty/Data.Empty/g' \
    -e 's/Agda\.Builtin\.Sigma/Data.Product/g' \
    -e 's/Agda\.Builtin\.Equality/Relation.Binary.PropositionalEquality/g' \
    -e 's/Agda\.Builtin\.Nat/Data.Nat/g' \
    -e 's/Agda\.Builtin\.Bool/Data.Bool/g' \
    -e 's/Agda\.Builtin\.List/Data.List/g' \
    -e 's/Agda\.Builtin\.String/Data.String/g' \
    -e 's/Agda\.Builtin\.Char/Data.Char/g' \
    -e 's/Agda\.Builtin\.Maybe/Data.Maybe/g' \
    -e 's/Agda\.Builtin\.Unit/Data.Unit/g' \
    "$file"
done

echo ""
echo "✅ Import fixes complete!"
echo ""
echo "📝 Backup files created with .bak extension"
echo ""
echo "Next steps:"
echo "  1. Test: cd agda-proofs && agda --safe Theorems/Abstract/SymmetryImpliesRepulsion.agda"
echo "  2. If successful: find agda-proofs -name '*.agda.bak' -delete"
echo "  3. Commit: git add agda-proofs/ && git commit -m 'fix: Update Agda imports for 2.8.0'"
