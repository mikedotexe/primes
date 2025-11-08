# Test configurations with non-coprime digits
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

# Find configurations with non-coprime digits
import math
print("Configurations with NON-COPRIME digits:")
print("Base | Config | Success Rate | Notes")
print("-----|--------|--------------|-------")

for base in [6, 10, 12]:
    for b, o, i, r in configs:
        if b == base and (math.gcd(o, base) > 1 or math.gcd(i, base) > 1):
            outer_gcd = math.gcd(o, base)
            inner_gcd = math.gcd(i, base)
            notes = []
            if outer_gcd > 1:
                notes.append(f"outer shares {outer_gcd}")
            if inner_gcd > 1:
                notes.append(f"inner shares {inner_gcd}")
            print(f"{base:4} | ({o},{i}) | {r*100:5.1f}%       | {', '.join(notes)}")
            
# Also let's specifically test (2,4) in base 6, (2,5) in base 10, etc.
print("\n\nWhat if we use digits that share factors with the base?")
print("These configurations should perform poorly...")
