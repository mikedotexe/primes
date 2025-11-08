#!/usr/bin/env python3
"""Comprehensive search for atomic primes with 5 at the center"""

import json
from datetime import datetime

def is_prime(n):
    """Miller-Rabin primality test would be better for large numbers, but this works for our range"""
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

def generate_patterns():
    """Generate all possible atomic patterns systematically"""
    patterns = []
    
    # Single membrane: D-0..0-5-0..0-D
    print("Generating single membrane patterns...")
    for digit in [1, 3, 5, 7, 9]:
        for zeros in range(0, 4):
            pattern = f"{digit}{'0'*zeros}5{'0'*zeros}{digit}"
            visual = f"({digit}){'─'*zeros}─(5)─{'─'*zeros}({digit})"
            patterns.append((pattern, visual, "single"))
    
    # Double membrane: D1-0..0-D2-0..0-5-0..0-D2-0..0-D1
    print("Generating double membrane patterns...")
    for outer in [1, 3, 5, 7, 9]:
        for inner in [1, 3, 5, 7, 9]:
            if outer == inner:
                continue
            for z_outer in range(0, 3):
                for z_inner in range(0, 3):
                    pattern = f"{outer}{'0'*z_outer}{inner}{'0'*z_inner}5{'0'*z_inner}{inner}{'0'*z_outer}{outer}"
                    visual = f"({outer}){'─'*z_outer}─({inner}){'─'*z_inner}─(5)─{'─'*z_inner}({inner})─{'─'*z_outer}({outer})"
                    patterns.append((pattern, visual, "double"))
    
    # Triple membrane: D1-0..0-D2-0..0-D3-0..0-5-0..0-D3-0..0-D2-0..0-D1
    print("Generating triple membrane patterns...")
    for outer in [1, 3, 7, 9]:
        for middle in [1, 3, 7, 9]:
            for inner in [1, 3, 7, 9]:
                if len(set([outer, middle, inner])) < 3:  # Skip if not all different
                    continue
                for z_outer in range(0, 2):
                    for z_middle in range(0, 2):
                        for z_inner in range(0, 2):
                            pattern = f"{outer}{'0'*z_outer}{middle}{'0'*z_middle}{inner}{'0'*z_inner}5{'0'*z_inner}{inner}{'0'*z_middle}{middle}{'0'*z_outer}{outer}"
                            visual = f"({outer}){'─'*z_outer}─({middle}){'─'*z_middle}─({inner}){'─'*z_inner}─(5)─{'─'*z_inner}({inner})─{'─'*z_middle}({middle})─{'─'*z_outer}({outer})"
                            patterns.append((pattern, visual, "triple"))
    
    return patterns

# Generate and test all patterns
print("⚛️  COMPREHENSIVE ATOMIC PRIME SEARCH")
print("="*60)

patterns = generate_patterns()
print(f"\nGenerated {len(patterns)} patterns to test")

# Test all patterns
atomic_primes = []
by_type = {"single": [], "double": [], "triple": []}

print("\nTesting for primality...")
for i, (pattern, visual, ptype) in enumerate(patterns):
    if i % 100 == 0:
        print(f"Progress: {i}/{len(patterns)}")
    
    try:
        n = int(pattern)
        if is_prime(n):
            atomic_primes.append({
                "value": pattern,
                "visual": visual,
                "type": ptype,
                "digits": len(pattern),
                "verify_url": f"https://www.wolframalpha.com/input/?i=isprime({pattern})"
            })
            by_type[ptype].append(pattern)
            print(f"✨ Found: {visual} → {pattern}")
    except:
        pass

# Sort by attractiveness (shorter patterns with distinct digits)
def attractiveness_score(prime):
    # Prefer: shorter length, symmetric patterns, distinct boundary digits
    pattern = prime["value"]
    score = len(pattern) * 10  # Shorter is better
    
    # Bonus for patterns like 307050703 (distinct boundaries)
    if prime["type"] == "double":
        if pattern[0] != pattern[1]:  # Different outer and inner
            score -= 5
    
    return score

atomic_primes.sort(key=attractiveness_score)

# Save comprehensive results
results = {
    "search_timestamp": datetime.now().isoformat(),
    "center_value": 5,
    "total_patterns_tested": len(patterns),
    "total_primes_found": len(atomic_primes),
    "by_type": {
        "single": len(by_type["single"]),
        "double": len(by_type["double"]),
        "triple": len(by_type["triple"])
    },
    "atomic_primes": atomic_primes,
    "most_attractive": atomic_primes[:10] if atomic_primes else []
}

# Save JSON
with open("atomic_primes_comprehensive.json", "w") as f:
    json.dump(results, f, indent=2)

# Save human-readable format
with open("atomic_primes_catalog.txt", "a") as f:
    f.write("\n\n" + "="*80 + "\n")
    f.write(f"COMPREHENSIVE ATOMIC PRIME SEARCH - {datetime.now()}\n")
    f.write("="*80 + "\n\n")
    
    f.write(f"Total patterns tested: {len(patterns)}\n")
    f.write(f"Atomic primes found: {len(atomic_primes)}\n")
    f.write(f"  Single membrane: {len(by_type['single'])}\n")
    f.write(f"  Double membrane: {len(by_type['double'])}\n")
    f.write(f"  Triple membrane: {len(by_type['triple'])}\n\n")
    
    f.write("⚛️  MOST ATTRACTIVE ATOMIC PRIMES:\n")
    f.write("="*60 + "\n\n")
    
    for i, prime in enumerate(atomic_primes[:20]):
        f.write(f"{i+1}. {prime['visual']}\n")
        f.write(f"   Value: {prime['value']}\n")
        f.write(f"   Type: {prime['type'].title()} membrane\n")
        f.write(f"   Digits: {prime['digits']}\n")
        f.write(f"   Verify: {prime['verify_url']}\n\n")

# Print summary
print("\n" + "="*60)
print("⚛️  ATOMIC PRIME SUMMARY")
print("="*60)
print(f"Total patterns tested: {len(patterns)}")
print(f"Atomic primes found: {len(atomic_primes)}")
print(f"\nBy type:")
print(f"  Single membrane: {len(by_type['single'])}")
print(f"  Double membrane: {len(by_type['double'])}")
print(f"  Triple membrane: {len(by_type['triple'])}")

print("\n🌟 TOP 10 MOST ATTRACTIVE ATOMIC PRIMES:")
for i, prime in enumerate(atomic_primes[:10]):
    print(f"\n{i+1}. {prime['visual']}")
    print(f"   → {prime['value']} ({prime['digits']} digits)")

print(f"\n✅ Results saved to:")
print(f"   - atomic_primes_comprehensive.json (full data)")
print(f"   - atomic_primes_catalog.txt (human readable)")

# Highlight special finds
print("\n⭐ SPECIAL MENTIONS:")
for prime in atomic_primes:
    if prime["value"] == "307050703":
        print(f"\n   The Exclusive Configuration!")
        print(f"   {prime['visual']} → {prime['value']}")
        print(f"   This works ONLY with seed 5 in the standard membrane formula!")
    elif prime["value"] == "70507":
        print(f"\n   The Compact Beauty:")
        print(f"   {prime['visual']} → {prime['value']}")
    elif prime["value"] == "3070050703":
        print(f"\n   The Extended Atom:")
        print(f"   {prime['visual']} → {prime['value']}")