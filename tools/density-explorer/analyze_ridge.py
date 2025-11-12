#!/usr/bin/env python3
"""
Ridge Pattern Analyzer - Deep statistical analysis of optimal inner_zero patterns

Analyzes:
- Periodicity and cycles in iz_best sequences
- Correlation between mid_len and optimal iz
- Cross-base pattern similarities
- Predictive models for optimal iz
"""

import csv
import sys
from pathlib import Path
from collections import Counter, defaultdict
import math

def load_ridge_data(filename):
    """Load ridge CSV data"""
    data = []
    with open(filename, 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            data.append({
                'base': int(row['base']),
                'mid_len': int(row['mid_len']),
                'iz_best': int(row['iz_best']),
                'p_any_exact': float(row['p_any_exact']),
                'expected_local_exact': float(row['expected_local_exact']),
            })
    return data

def analyze_periodicity(iz_sequence):
    """Detect periodicities in iz_best sequence"""
    n = len(iz_sequence)
    if n < 6:
        return None

    # Try periods from 2 to n/2
    best_period = None
    best_score = 0

    for period in range(2, min(n//2, 20)):
        matches = 0
        total = 0
        for i in range(period, n):
            if iz_sequence[i] == iz_sequence[i % period]:
                matches += 1
            total += 1

        score = matches / total if total > 0 else 0
        if score > best_score and score > 0.5:
            best_score = score
            best_period = period

    return (best_period, best_score) if best_period else None

def compute_autocorrelation(sequence, max_lag=10):
    """Compute autocorrelation function"""
    n = len(sequence)
    mean = sum(sequence) / n
    variance = sum((x - mean)**2 for x in sequence) / n

    if variance == 0:
        return [0] * max_lag

    acf = []
    for lag in range(1, min(max_lag + 1, n)):
        covariance = sum((sequence[i] - mean) * (sequence[i - lag] - mean)
                        for i in range(lag, n)) / n
        acf.append(covariance / variance)

    return acf

def find_modular_patterns(data):
    """Find if iz_best follows modular patterns"""
    patterns = {}

    for mod in range(2, 13):
        residues = defaultdict(list)
        for row in data:
            residue = row['mid_len'] % mod
            residues[residue].append(row['iz_best'])

        # Check if certain residues always give certain iz values
        consistent = True
        pattern_strength = 0
        for residue, iz_list in residues.items():
            if len(iz_list) > 1:
                # Check variance
                mean_iz = sum(iz_list) / len(iz_list)
                variance = sum((iz - mean_iz)**2 for iz in iz_list) / len(iz_list)
                if variance < 2.0:  # Low variance = strong pattern
                    pattern_strength += 1

        if pattern_strength >= mod * 0.5:
            patterns[mod] = {
                'residues': {k: sum(v)/len(v) for k, v in residues.items()},
                'strength': pattern_strength / mod
            }

    return patterns

def analyze_density_efficiency(data):
    """Analyze the efficiency: density / obstruction"""
    efficiencies = []
    for row in data:
        if row['p_any_exact'] > 0:
            eff = row['expected_local_exact'] / row['p_any_exact']
            efficiencies.append({
                'mid_len': row['mid_len'],
                'iz_best': row['iz_best'],
                'efficiency': eff,
                'density': row['expected_local_exact']
            })

    # Find sweet spots
    efficiencies.sort(key=lambda x: x['efficiency'], reverse=True)
    return efficiencies

def predict_optimal_iz(mid_len, base_data, method='regression'):
    """Predict optimal iz for a given mid_len"""
    if method == 'nearest':
        # Find nearest mid_len
        closest = min(base_data, key=lambda x: abs(x['mid_len'] - mid_len))
        return closest['iz_best']

    elif method == 'regression':
        # Simple linear regression
        n = len(base_data)
        if n < 2:
            return base_data[0]['iz_best'] if base_data else 0

        x = [row['mid_len'] for row in base_data]
        y = [row['iz_best'] for row in base_data]

        x_mean = sum(x) / n
        y_mean = sum(y) / n

        numerator = sum((x[i] - x_mean) * (y[i] - y_mean) for i in range(n))
        denominator = sum((x[i] - x_mean)**2 for i in range(n))

        if denominator == 0:
            return int(y_mean)

        slope = numerator / denominator
        intercept = y_mean - slope * x_mean

        prediction = slope * mid_len + intercept
        return int(round(prediction))

    return 0

def main():
    if len(sys.argv) < 2:
        print("Usage: python analyze_ridge.py <ridge_csv_file> [ridge_csv_file2 ...]")
        print("\nExample: python analyze_ridge.py out/ridge_base10_comprehensive.csv")
        sys.exit(1)

    all_analyses = {}

    for filename in sys.argv[1:]:
        if not Path(filename).exists():
            print(f"Warning: {filename} not found, skipping")
            continue

        print(f"\n{'='*70}")
        print(f"Analyzing: {filename}")
        print(f"{'='*70}\n")

        data = load_ridge_data(filename)
        base = data[0]['base']
        iz_sequence = [row['iz_best'] for row in data]

        # Basic stats
        print(f"📊 Basic Statistics (Base {base}):")
        print(f"   Data points: {len(data)}")
        print(f"   mid_len range: {min(r['mid_len'] for r in data)} - {max(r['mid_len'] for r in data)}")
        print(f"   iz_best range: {min(iz_sequence)} - {max(iz_sequence)}")
        print(f"   Average iz_best: {sum(iz_sequence)/len(iz_sequence):.2f}")

        iz_counter = Counter(iz_sequence)
        most_common = iz_counter.most_common(3)
        print(f"\n   Most common iz values:")
        for iz, count in most_common:
            print(f"      iz={iz}: {count} times ({count/len(data)*100:.1f}%)")

        # Periodicity
        print(f"\n🔄 Periodicity Analysis:")
        period_result = analyze_periodicity(iz_sequence)
        if period_result:
            period, score = period_result
            print(f"   Detected period: {period} (confidence: {score*100:.1f}%)")
            print(f"   Pattern: {iz_sequence[:period]}")
        else:
            print(f"   No strong periodicity detected")

        # Autocorrelation
        print(f"\n📈 Autocorrelation (lag 1-5):")
        acf = compute_autocorrelation(iz_sequence, max_lag=5)
        for lag, value in enumerate(acf, 1):
            bar = '█' * int(abs(value) * 20) if abs(value) > 0 else ''
            sign = '+' if value >= 0 else '-'
            print(f"   Lag {lag}: {sign}{bar} ({value:+.3f})")

        # Modular patterns
        print(f"\n🎯 Modular Patterns:")
        mod_patterns = find_modular_patterns(data)
        if mod_patterns:
            for mod, info in sorted(mod_patterns.items(), key=lambda x: x[1]['strength'], reverse=True)[:3]:
                print(f"   mod {mod} (strength: {info['strength']*100:.1f}%):")
                for residue, avg_iz in sorted(info['residues'].items()):
                    print(f"      mid_len ≡ {residue} (mod {mod}) → iz ≈ {avg_iz:.1f}")
        else:
            print(f"   No strong modular patterns found")

        # Density efficiency
        print(f"\n⚡ Efficiency Analysis (Top 5 Configurations):")
        efficiencies = analyze_density_efficiency(data)
        print(f"   {'mid_len':<10} {'iz_best':<10} {'efficiency':<15} {'density'}")
        print(f"   {'-'*10} {'-'*10} {'-'*15} {'-'*10}")
        for eff in efficiencies[:5]:
            print(f"   {eff['mid_len']:<10} {eff['iz_best']:<10} {eff['efficiency']:<15.6f} {eff['density']*100:.4f}%")

        # Predictions
        print(f"\n🔮 Predictive Model:")
        test_lens = [60, 75, 100]
        print(f"   Predicted optimal iz for future mid_len:")
        for test_len in test_lens:
            pred = predict_optimal_iz(test_len, data, method='regression')
            print(f"      mid_len={test_len} → iz_best ≈ {pred}")

        all_analyses[base] = {
            'data': data,
            'iz_sequence': iz_sequence,
            'periodicity': period_result,
            'acf': acf,
            'patterns': mod_patterns,
            'efficiencies': efficiencies
        }

    # Cross-base comparison
    if len(all_analyses) > 1:
        print(f"\n{'='*70}")
        print(f"🌍 Cross-Base Comparison")
        print(f"{'='*70}\n")

        print(f"Average iz_best by base:")
        for base, analysis in sorted(all_analyses.items()):
            avg_iz = sum(analysis['iz_sequence']) / len(analysis['iz_sequence'])
            print(f"   Base {base:2d}: {avg_iz:.2f}")

        print(f"\nAverage density by base:")
        for base, analysis in sorted(all_analyses.items()):
            avg_density = sum(r['expected_local_exact'] for r in analysis['data']) / len(analysis['data'])
            print(f"   Base {base:2d}: {avg_density*100:.4f}%")

        print(f"\nPeriodicity comparison:")
        for base, analysis in sorted(all_analyses.items()):
            if analysis['periodicity']:
                period, score = analysis['periodicity']
                print(f"   Base {base:2d}: period={period}, confidence={score*100:.1f}%")
            else:
                print(f"   Base {base:2d}: no clear period")

if __name__ == '__main__':
    main()
