import math

# Read the CSV data
configs = []
with open('base_membrane_data.csv', 'r') as f:
    next(f)  # Skip header
    for line in f:
        parts = line.strip().split(',')
        base = int(parts[0])
        outer = int(parts[2])
        inner = int(parts[3])
        rate = float(parts[8])
        configs.append((base, outer, inner, rate))

# Check coprimality
for base in [6, 8, 10, 12, 14, 16, 18, 20, 24, 30]:
    print(f"\nBase {base}:")
    base_configs = [(o, i, r) for b, o, i, r in configs if b == base][:5]
    
    for outer, inner, rate in base_configs:
        outer_coprime = math.gcd(outer, base) == 1
        inner_coprime = math.gcd(inner, base) == 1
        print(f"  ({outer},{inner}): {rate*100:.0f}% - outer coprime: {outer_coprime}, inner coprime: {inner_coprime}")
