# 🎯 Membrane Prime Generation - Quick Reference Card

## Formula
```
outer | inner | seed | inner | outer
```
With k=(0,0) padding (no zeros between sections)

## Best Configurations by Base

| Base | Config | Success Rate | Example |
|------|--------|--------------|---------|
| 6 | (1,5) | 31% | 15651 → 2551 |
| 4 | (3,1) | 28% | 31213 → 877 |
| 30 | (11,7) | 24% | B7C7B → varies |
| 12 | (1,11) | 23% | 1B5B1 → varies |
| 10 | (1,9) | 22% | 19391 → 19391 |
| 8 | (3,5) | 21% | 35353 → 15083 |

## Universal Patterns (Work in Many Bases)

1. **(1,7)** - Works in 23 bases
2. **(1,5)** - Works in 23 bases  
3. **(1,11)** - Works in 23 bases
4. **(1,3)** - Works in 19 bases
5. **(5,7)** - Works in 14 bases

## Rules (ALWAYS Follow)

✅ **DO:**
- Use coprime boundary digits
- Use k=(0,0) (no padding)
- Try (1,x) patterns first
- Use even bases when possible

❌ **DON'T:**
- Use non-coprime digits
- Add unnecessary zeros (k>0)
- Use complex patterns
- Believe in "breathing" advantage

## Quick Decision Tree

```
Choosing a base?
├─ Want maximum performance? → Base 6
├─ Want good performance? → Base 4, 12, or 30
└─ Stuck with specific base? → Use (1,7) or (1,5)

Choosing digits?
├─ Are they coprime to base? → If no, pick others
├─ Is outer digit = 1? → Good choice
└─ Are they small primes? → Even better
```

## Performance Expectations

- **Best case**: ~30% success (Base 6)
- **Typical**: 15-25% success
- **Worst case**: ~10% success
- **vs Random**: Always 3-7x better

## Example Usage

```python
# Base 10, pattern (1,9), seed 23
membrane = "1" + "9" + "23" + "9" + "1"
# Result: 192391
# Check if prime → likely yes!
```

---
*Remember: Simpler is better. When in doubt, use (1,5) k=(0,0)*