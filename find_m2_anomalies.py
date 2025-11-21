#!/usr/bin/env python3
"""
Quick script to identify the 4 M=2 anomalies from solution_space_complete.csv
"""

import csv
from collections import defaultdict

# Load CSV and find optimal k for each M=2 configuration
configs = defaultdict(list)

with open('solution_space_complete.csv', 'r') as f:
    reader = csv.DictReader(f)
    for row in reader:
        if row['M'] == '2':  # Only M=2
            base = int(row['base'])
            outer = int(row['outer'])
            inner = int(row['inner'])
            k = int(row['k'])
            density = float(row['density'])

            key = (base, outer, inner)
            configs[key].append((k, density, row))

# Find anomalies (k* > 0)
anomalies = []

for key, k_densities in configs.items():
    # Sort by density descending
    k_densities.sort(key=lambda x: x[1], reverse=True)

    optimal_k = k_densities[0][0]
    optimal_density = k_densities[0][1]

    if optimal_k > 0:
        base, outer, inner = key
        anomalies.append({
            'base': base,
            'outer': outer,
            'inner': inner,
            'k_star': optimal_k,
            'density': optimal_density,
            'all_k': k_densities
        })

# Sort by base
anomalies.sort(key=lambda x: x['base'])

print(f"\n{'='*70}")
print(f"FOUND {len(anomalies)} M=2 ANOMALIES (configurations with k*>0)")
print(f"{'='*70}\n")

for i, anom in enumerate(anomalies, 1):
    print(f"{i}. Base {anom['base']}, ({anom['outer']},{anom['inner']}) → k*={anom['k_star']} (density={anom['density']:.6f})")
    print(f"   Density progression:")
    for k, dens, _ in anom['all_k']:
        marker = "★" if k == anom['k_star'] else " "
        print(f"     k={k}: {dens:.6f} {marker}")
    print()

# Analyze common properties
print(f"\n{'='*70}")
print("COMMON PROPERTIES ANALYSIS")
print(f"{'='*70}\n")

print("k* distribution:")
k_star_counts = defaultdict(int)
for anom in anomalies:
    k_star_counts[anom['k_star']] += 1
for k, count in sorted(k_star_counts.items()):
    print(f"  k*={k}: {count} anomalies")

print("\nBase distribution:")
base_counts = defaultdict(int)
for anom in anomalies:
    base_counts[anom['base']] += 1
for base, count in sorted(base_counts.items()):
    print(f"  Base {base}: {count} anomalies")

print("\nBoundary digit properties:")
for anom in anomalies:
    # Get full row data for first entry
    full_row = anom['all_k'][0][2]
    print(f"  Base {anom['base']} ({anom['outer']},{anom['inner']}): "
          f"outer_prime={full_row['outer_is_prime']}, "
          f"inner_prime={full_row['inner_is_prime']}, "
          f"outer_gcd={full_row['outer_gcd']}, "
          f"inner_gcd={full_row['inner_gcd']}")
