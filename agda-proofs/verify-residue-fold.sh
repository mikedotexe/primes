#!/bin/bash
# Verification script for residue fold modules

set -e

echo "═══════════════════════════════════════════════════════════════"
echo "  Verifying Residue Fold Modules"
echo "═══════════════════════════════════════════════════════════════"
echo ""

echo "✓ Type-checking Core/ResidueFold.agda..."
agda --safe Core/ResidueFold.agda
echo "  Success!"
echo ""

echo "✓ Type-checking Theorems/MirrorObstruction.agda..."
agda --safe Theorems/MirrorObstruction.agda
echo "  Success!"
echo ""

echo "✓ Type-checking Core/CRTVector.agda..."
agda --safe Core/CRTVector.agda
echo "  Success!"
echo ""

echo "✓ Type-checking Tests/DevProofs.agda..."
agda --safe Tests/DevProofs.agda
echo "  Success!"
echo ""

echo "═══════════════════════════════════════════════════════════════"
echo "  All modules verified successfully! ✓"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Key achievements:"
echo "  • Convolution fold with identity + associativity proofs"
echo "  • Mirror obstruction invariant for palindromes"
echo "  • CRT/LCM pushforward certification (projection equals direct DP)"
echo "  • Executable verification suite (all tests pass)"
echo ""
echo "See RESIDUE_FOLD_README.md for details."
echo ""
