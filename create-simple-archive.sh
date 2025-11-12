#!/bin/bash

# Archive script for prime-physics-engine project
# Creates: primes.zip

set -e

PROJECT_NAME="primes"
ARCHIVE_NAME="primes.zip"

echo "Creating archive of ${PROJECT_NAME}..."

# Remove old archive if it exists
[ -f "${ARCHIVE_NAME}" ] && rm "${ARCHIVE_NAME}"

# Create archive excluding build artifacts, caches, and temporary files
zip -r "${ARCHIVE_NAME}" . \
    -x "target/*" \
    -x "*/target/*" \
    -x "*.zip" \
    -x "*.tar.gz" \
    -x ".DS_Store" \
    -x ".claude/*" \
    -x "__pycache__/*" \
    -x "*.pyc" \
    -x ".git/*" \
    -x "node_modules/*" \
    -x "agda-proofs/_build/*" \
    -x "hz_res/*" \
    -x "hz_out/*" \
    -x "debug.log" \
    -x "ai-output.txt" \
    -x "*.db" \
    -x "*.db-shm" \
    -x "*.db-wal" \
    -x ".vscode/*" \
    -x ".idea/*" \
    -x "*.swp" \
    -x "*~" \
    -x ".env" \
    -x ".env.*"

# Display archive info
if [ -f "${ARCHIVE_NAME}" ]; then
    SIZE=$(du -h "${ARCHIVE_NAME}" | cut -f1)
    FILE_COUNT=$(unzip -l "${ARCHIVE_NAME}" | grep -E "^\s*[0-9]+" | wc -l | tr -d ' ')

    echo ""
    echo "✓ Archive created successfully!"
    echo "  File: ${ARCHIVE_NAME}"
    echo "  Size: ${SIZE}"
    echo "  Files: ${FILE_COUNT}"
    echo ""
    echo "Archive contents:"
    unzip -l "${ARCHIVE_NAME}"
else
    echo "Error: Failed to create archive"
    exit 1
fi
