#!/usr/bin/env python3
"""
Prime Outer Digit Correlation Analysis

Tests hypothesis: Do M=2 anomalies preferentially have prime outer digits?

Statistical test: Fisher's exact test
H0: No association between outer primality and anomaly status
H1: Prime outer digits increase anomaly probability

Usage:
    python3 prime_outer_analysis.py solution_space_complete.csv
"""

import csv
import sys
from scipy.stats import fisher_exact

def is_prime(n):
    """Simple primality test for small numbers"""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    for i in range(3, int(n**0.5) + 1, 2):
        if n % i == 0:
            return False
    return True

def analyze_prime_outer_correlation(csv_file):
    """Analyze correlation between prime outer digits and M=2 anomalies"""
    
    # Load all M=2 configurations and find optimal k
    configs = {}
    
    with open(csv_file, 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            if int(row['M']) != 2:
                continue
            
            base = int(row['base'])
            outer = int(row['outer'])
            inner = int(row['inner'])
            k = int(row['k'])
            density = float(row['density'])
            
            key = (base, outer, inner)
            
            if key not in configs:
                configs[key] = {'outer': outer, 'densities': {}}
            
            configs[key]['densities'][k] = density
    
    # Find optimal k for each config
    prime_outer_anomalies = 0
    prime_outer_normal = 0
    other_outer_anomalies = 0
    other_outer_normal = 0
    
    anomaly_examples = []
    
    for key, data in configs.items():
        base, outer, inner = key
        densities = data['densities']
        
        # Find k*
        k_star = max(densities.keys(), key=lambda k: densities[k])
        
        outer_is_prime = is_prime(outer)
        is_anomaly = (k_star > 0)
        
        if outer_is_prime and is_anomaly:
            prime_outer_anomalies += 1
            anomaly_examples.append((base, outer, inner, k_star, densities))
        elif outer_is_prime and not is_anomaly:
            prime_outer_normal += 1
        elif not outer_is_prime and is_anomaly:
            other_outer_anomalies += 1
            anomaly_examples.append((base, outer, inner, k_star, densities))
        else:
            other_outer_normal += 1
    
    # Fisher's exact test
    table = [[prime_outer_anomalies, prime_outer_normal],
             [other_outer_anomalies, other_outer_normal]]
    
    odds_ratio, p_value = fisher_exact(table, alternative='greater')
    
    # Print results
    print("\n" + "="*70)
    print("PRIME OUTER DIGIT CORRELATION ANALYSIS")
    print("="*70 + "\n")
    
    print("Contingency Table (M=2 configurations):")
    print(f"                 Anomaly  Normal   Total")
    print(f"Prime outer      {prime_outer_anomalies:4d}     {prime_outer_normal:4d}    {prime_outer_anomalies + prime_outer_normal:4d}")
    print(f"Non-prime outer  {other_outer_anomalies:4d}     {other_outer_normal:4d}    {other_outer_anomalies + other_outer_normal:4d}")
    print(f"Total            {prime_outer_anomalies + other_outer_anomalies:4d}     {prime_outer_normal + other_outer_normal:4d}    {len(configs):4d}")
    
    print(f"\nFisher's Exact Test (one-tailed):")
    print(f"  Odds ratio:  {odds_ratio:.3f}")
    print(f"  P-value:     {p_value:.4f}")
    
    if p_value < 0.001:
        sig = "HIGHLY SIGNIFICANT ***"
    elif p_value < 0.01:
        sig = "SIGNIFICANT **"
    elif p_value < 0.05:
        sig = "SIGNIFICANT *"
    elif p_value < 0.10:
        sig = "MARGINALLY SIGNIFICANT"
    else:
        sig = "NOT SIGNIFICANT"
    
    print(f"  Significance: {sig}")
    
    print(f"\n{'='*70}")
    print("INTERPRETATION")
    print(f"{'='*70}\n")
    
    if p_value < 0.01:
        print("✓ Prime outer digit correlation is STATISTICALLY SIGNIFICANT")
        print("  Prime outer digits increase M=2 anomaly probability")
        print("  This suggests a genuine mechanistic effect")
        print("\n  Recommendation: Investigate cyclic residue group mechanism")
    elif p_value < 0.10:
        print("⚠ Prime outer digit correlation is SUGGESTIVE but not definitive")
        print("  Pattern is consistent with hypothesis but not statistically robust")
        print(f"  With only {prime_outer_anomalies + other_outer_anomalies} total anomalies, larger sample needed")
        print("\n  Recommendation: Collect more M=2 data to clarify")
    else:
        print("✗ Prime outer digit correlation is NOT SIGNIFICANT")
        print("  The 4/4 prime outer pattern in known anomalies is likely COINCIDENCE")
        print(f"  P={p_value:.3f} suggests no genuine association")
        print("\n  Recommendation: Report as statistical artifact, not mechanism")
    
    print(f"\n{'='*70}")
    print("ANOMALY DETAILS")
    print(f"{'='*70}\n")
    
    print(f"Total M=2 anomalies found: {prime_outer_anomalies + other_outer_anomalies}")
    print(f"  With prime outer: {prime_outer_anomalies}")
    print(f"  With non-prime outer: {other_outer_anomalies}\n")
    
    if anomaly_examples:
        print("All anomalies:")
        for base, outer, inner, k_star, densities in sorted(anomaly_examples, key=lambda x: -densities[k_star]):
            outer_type = "PRIME" if is_prime(outer) else "composite"
            print(f"  Base {base:2d} ({outer:2d},{inner:2d}) → k*={k_star} (outer={outer_type})")
            print(f"    Densities: k=0: {densities.get(0, 0):.4f}, k=1: {densities.get(1, 0):.4f}")
    
    print("\n" + "="*70)
    print("Analysis complete.")
    print("="*70 + "\n")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 prime_outer_analysis.py solution_space_complete.csv")
        sys.exit(1)
    
    csv_file = sys.argv[1]
    analyze_prime_outer_correlation(csv_file)
