#!/usr/bin/env python3
"""Quick test of atomic prime patterns with 5 in the center"""

def is_prime(n):
    """Simple primality test"""
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

# Test known attractive patterns
patterns = [
    # Single membrane
    ("305", "(3)─(5)"),
    ("30503", "(3)─(5)─(3)"),
    ("70507", "(7)─(5)─(7)"),
    ("90509", "(9)─(5)─(9)"),
    
    # Double membrane - these are likely the attractive ones
    ("30705073", "(3)─(7)─(5)─(7)─(3)"),
    ("307050703", "(3)─(7)──(5)──(7)─(3)"),  # The exclusive configuration!
    ("3070050703", "(3)─(7)──(5)──(7)─(3)"),
    ("30700507003", "(3)─(7)──(5)──(7)──(3)"),
    ("703050307", "(7)─(3)─(5)─(3)─(7)"),
    ("901050109", "(9)─(1)─(5)─(1)─(9)"),
    
    # Triple membrane
    ("3070905090703", "(3)─(7)─(9)─(5)─(9)─(7)─(3)"),
    ("7030905090307", "(7)─(3)─(9)─(5)─(9)─(3)─(7)"),
]

print("⚛️  ATOMIC PRIMES WITH CENTER 5")
print("="*50)

primes_found = []
for pattern, visual in patterns:
    n = int(pattern)
    if is_prime(n):
        print(f"\n✨ PRIME FOUND!")
        print(f"   Pattern: {visual}")
        print(f"   Value: {pattern}")
        print(f"   Verify: https://www.wolframalpha.com/input/?i=isprime({pattern})")
        primes_found.append((pattern, visual))

print(f"\n{'='*50}")
print(f"Summary: Found {len(primes_found)} atomic primes")

if primes_found:
    print("\n🌟 All atomic primes:")
    for pattern, visual in primes_found:
        print(f"  {visual} → {pattern}")

# Write results
with open("atomic_primes_found.txt", "a") as f:
    f.write(f"\n\n{'='*60}\n")
    f.write(f"Search run: {__import__('datetime').datetime.now()}\n")
    f.write(f"{'='*60}\n\n")
    for pattern, visual in primes_found:
        f.write(f"{visual}\n")
        f.write(f"Value: {pattern}\n")
        f.write(f"Prime: YES\n")
        f.write(f"Verify: https://www.wolframalpha.com/input/?i=isprime({pattern})\n\n")
    
print("\n✅ Results appended to: atomic_primes_found.txt")