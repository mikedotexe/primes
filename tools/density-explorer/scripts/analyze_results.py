#!/usr/bin/env python3
"""
Simple analysis script for density-explorer results.
Provides clear, factual summaries without excessive formatting.
"""

import csv
import sys
from pathlib import Path

def analyze_csv(filepath):
    """Analyze enrichment data from CSV file."""
    with open(filepath) as f:
        lines = f.readlines()

    # Parse header
    header = lines[0].strip().split(',')

    # Extract data (accounting for array fields that break CSV parsing)
    data = []
    for line in lines[1:]:
        parts = line.strip().split(',')
        try:
            total_len = int(parts[2])
            mid_len = int(parts[3])
            inner_zero = int(parts[4])
            samples = int(parts[6])
            primes = int(parts[7])
            density = float(parts[8])
            ci_lo = float(parts[9])
            ci_hi = float(parts[10])
            expected = float(parts[11])
            enrichment = float(parts[12])

            data.append({
                'total_len': total_len,
                'mid_len': mid_len,
                'inner_zero': inner_zero,
                'samples': samples,
                'primes': primes,
                'density': density,
                'ci_lo': ci_lo,
                'ci_hi': ci_hi,
                'expected': expected,
                'enrichment': enrichment
            })
        except (ValueError, IndexError):
            continue

    if not data:
        print(f"No valid data found in {filepath}")
        return

    # Sort by enrichment
    data.sort(key=lambda x: x['enrichment'], reverse=True)

    # Print analysis
    print("=" * 70)
    print(f"Analysis of {filepath}")
    print("=" * 70)
    print()

    print("Top 5 configurations by enrichment factor:")
    print()
    for i, row in enumerate(data[:5], 1):
        print(f"{i}. Enrichment: {row['enrichment']:.2f}×")
        print(f"   Configuration: mid_len={row['mid_len']}, inner_zero={row['inner_zero']}")
        print(f"   Total length: {row['total_len']} digits")
        print(f"   Prime density: {row['density']*100:.2f}% ({row['primes']:,} / {row['samples']:,})")
        print(f"   PNT baseline:  {row['expected']*100:.2f}%")
        print(f"   95% CI: [{row['ci_lo']*100:.2f}%, {row['ci_hi']*100:.2f}%]")
        print()

    # Summary statistics
    enrichments = [r['enrichment'] for r in data]
    densities = [r['density'] for r in data]

    print("=" * 70)
    print("Summary statistics:")
    print(f"  Configurations tested: {len(data)}")
    print(f"  Enrichment range: {min(enrichments):.2f}× to {max(enrichments):.2f}×")
    print(f"  Mean enrichment: {sum(enrichments)/len(enrichments):.2f}×")
    print(f"  Density range: {min(densities)*100:.2f}% to {max(densities)*100:.2f}%")
    print(f"  Configurations above 2.0× enrichment: {len([e for e in enrichments if e >= 2.0])}")
    print("=" * 70)

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python3 analyze_results.py <csv_file>")
        sys.exit(1)

    analyze_csv(sys.argv[1])
