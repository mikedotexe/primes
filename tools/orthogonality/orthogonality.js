#!/usr/bin/env node
/**
 * Babylonian-Prime Divergence Analysis (Node.js)
 *
 * Demonstrates the statistical independence between human-convenient
 * mathematics (Babylonian base-60 legacy) and nature's mathematical
 * patterns (prime pair distributions).
 *
 * Usage:
 *   node orthogonality.js --N 1000000 --G 300 --metric norm
 *   node orthogonality.js --N 1000000 --G 300 --metric raw --baseline tau
 *   node orthogonality.js --N 1000000 --G 300 --metric norm --perm 2000
 */

function parseArgs() {
  const a = process.argv.slice(2);
  let N = 1_000_000, G = 300, metric = "norm", perm = 0, baseline = "base60";
  for (let i = 0; i < a.length; i++) {
    if (a[i] === '--N') N = parseInt(a[++i], 10);
    else if (a[i] === '--G') G = parseInt(a[++i], 10);
    else if (a[i] === '--metric') metric = a[++i];
    else if (a[i] === '--perm') perm = parseInt(a[++i], 10);
    else if (a[i] === '--baseline') baseline = a[++i];
  }
  return { N, G, metric, perm, baseline };
}

// ====================== Prime Sieve ======================

function sieveBool(n) {
  const p = new Array(n + 1).fill(true);
  if (n >= 0) p[0] = false;
  if (n >= 1) p[1] = false;
  for (let i = 2; i * i <= n; i++) {
    if (p[i]) {
      for (let m = i * i; m <= n; m += i) p[m] = false;
    }
  }
  return p;
}

// ====================== Factorization ======================

function primeFactorsSmall(n) {
  const f = [];
  let d = 2;
  while (d * d <= n) {
    if (n % d === 0) {
      let e = 0;
      while (n % d === 0) {
        n = Math.floor(n / d);
        e++;
      }
      f.push([d, e]);
    }
    d = (d === 2 ? 3 : d + 2);
  }
  if (n > 1) f.push([n, 1]);
  return f;
}

function tauFromFactor(f) {
  return f.reduce((t, [, e]) => t * (e + 1), 1);
}

// ====================== Babylonian Scores ======================

function babylonianScore60(g) {
  if (g % 2 === 1) return 0;
  const f = primeFactorsSmall(g);
  let e2 = 0, e3 = 0, e5 = 0, others = 0;
  for (const [p, e] of f) {
    if (p === 2) e2 = e;
    else if (p === 3) e3 = e;
    else if (p === 5) e5 = e;
    else others++;
  }
  let s = 2 * (e2 + e3 + e5) + (g % 60 === 0 ? 10 : 0) - 3 * others;
  s += 0.5 * tauFromFactor(f);
  return s;
}

function babylonianScoreTau(g) {
  return tauFromFactor(primeFactorsSmall(g));
}

// ====================== Hardy-Littlewood ======================

const C2 = 0.6601618158468696;

function singularSeries(g) {
  if (g <= 0 || g % 2 === 1) return 0;
  const k = Math.floor(g / 2);
  const f = primeFactorsSmall(k);
  let s = 2 * C2;
  for (const [p] of f) {
    if (p > 2) s *= (p - 1) / (p - 2);
  }
  return s;
}

// ====================== Prime Pair Indexing ======================

function pairsIndex(isPrime, G) {
  const n = isPrime.length - 1;
  const out = Array(Math.floor(G / 2) + 1).fill(0).map(() => []);
  for (let p = 2; p <= n; p++) {
    if (!isPrime[p]) continue;
    const mg = Math.min(G, n - p);
    for (let g = 2; g <= mg; g += 2) {
      if (isPrime[p + g]) out[g >> 1].push(p);
    }
  }
  return out;
}

function upperBound(v, key) {
  let lo = 0, hi = v.length;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (v[mid] <= key) lo = mid + 1;
    else hi = mid;
  }
  return lo;
}

// ====================== Statistics ======================

function corr(x, y) {
  const n = x.length;
  if (n < 2) return NaN;
  const mx = x.reduce((a, b) => a + b, 0) / n;
  const my = y.reduce((a, b) => a + b, 0) / n;
  let num = 0, dx2 = 0, dy2 = 0;
  for (let i = 0; i < n; i++) {
    const dx = x[i] - mx, dy = y[i] - my;
    num += dx * dy;
    dx2 += dx * dx;
    dy2 += dy * dy;
  }
  return num / Math.sqrt(dx2 * dy2);
}

function tstat(r, n) {
  return r * Math.sqrt((n - 2) / (1 - r * r));
}

// ====================== Main ======================

(function main() {
  const { N, G, metric, perm, baseline } = parseArgs();

  console.log("╔══════════════════════════════════════════════════════════════╗");
  console.log("║      BABYLONIAN-PRIME DIVERGENCE (Node.js)                  ║");
  console.log("╚══════════════════════════════════════════════════════════════╝");
  console.log();
  console.log(`Configuration:`);
  console.log(`  N (prime bound):    ${N.toLocaleString()}`);
  console.log(`  G (max even gap):   ${G}`);
  console.log(`  Baseline:           ${baseline}`);
  console.log(`  Metric:             ${metric}`);
  console.log();

  // Build sieve
  process.stdout.write(`Building prime sieve up to ${N.toLocaleString()}... `);
  const isPrime = sieveBool(N);
  console.log('done');

  // Index pairs
  process.stdout.write('Indexing prime pairs by gap... ');
  const pairs = pairsIndex(isPrime, G);
  console.log('done');
  console.log();

  // Compute scores
  const gaps = Array.from({ length: Math.floor(G / 2) }, (_, i) => 2 * (i + 1));
  const bab = gaps.map(g => baseline === "tau" ? babylonianScoreTau(g) : babylonianScore60(g));
  const ser = gaps.map(g => singularSeries(g));
  const scale = N / Math.log(N) ** 2;

  const y = [];
  for (let i = 0; i < gaps.length; i++) {
    const g = gaps[i];
    const raw = upperBound(pairs[g >> 1], N - g);
    const eg = ser[i] * scale;
    if (metric === "raw") y.push(raw);
    else if (metric === "norm") y.push(eg > 0 ? raw / eg : 0);
    else if (metric === "z") y.push(eg > 0 ? (raw - eg) / Math.sqrt(eg) : 0);
    else y.push(raw);
  }

  // Correlation
  const r = corr(bab, y);
  const n = gaps.length;
  const t = tstat(r, n);

  console.log("╔══════════════════════════════════════════════════════════════╗");
  console.log("║                    CORRELATION RESULTS                       ║");
  console.log("╚══════════════════════════════════════════════════════════════╝");
  console.log();
  console.log(`  Pearson r:          ${r.toFixed(4)}`);
  console.log(`  t-statistic:        ${t.toFixed(2)}`);
  console.log(`  Sample size:        ${n}`);
  console.log();

  if (Math.abs(r) < 0.1) {
    console.log("  ✅ Negligible correlation (orthogonal)!");
  } else if (Math.abs(r) < 0.3) {
    console.log("  ⚠️  Weak correlation detected");
  } else {
    console.log("  ❌ Strong correlation detected");
  }

  if (Math.abs(t) < 2.0) {
    console.log("  ✅ Not statistically significant (p > 0.05)");
  }
  console.log();

  // Permutation test
  if (perm > 0) {
    console.log(`Running permutation test (${perm} permutations)...`);
    const ycopy = y.slice();
    let gt = 0;
    for (let b = 0; b < perm; b++) {
      // Fisher-Yates shuffle
      for (let i = ycopy.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        const tmp = ycopy[i];
        ycopy[i] = ycopy[j];
        ycopy[j] = tmp;
      }
      const rp = corr(bab, ycopy);
      if (Math.abs(rp) >= Math.abs(r)) gt++;
    }
    const pval = (gt + 1) / (perm + 1);
    console.log(`  Permutation p-value: ${pval.toFixed(4)}`);
    if (pval > 0.05) {
      console.log("  ✅ Not significant (consistent with orthogonality)");
    }
    console.log();
  }

  // Champions
  const babIdx = gaps.map((g, i) => [g, bab[i]]).sort((a, b) => b[1] - a[1]);
  const harmIdx = gaps.map((g, i) => [g, y[i]]).sort((a, b) => b[1] - a[1]);

  console.log("╔══════════════════════════════════════════════════════════════╗");
  console.log("║                       CHAMPION GAPS                          ║");
  console.log("╚══════════════════════════════════════════════════════════════╝");
  console.log();
  console.log("Top Babylonian Gaps (human-convenient):");
  for (let i = 0; i < 3; i++) {
    console.log(`  #${i + 1}: gap ${babIdx[i][0].toString().padStart(3)}  score ${babIdx[i][1].toFixed(2).padStart(6)}`);
  }
  console.log();
  console.log("Top Prime Harmony Gaps (nature's patterns):");
  for (let i = 0; i < 3; i++) {
    console.log(`  #${i + 1}: gap ${harmIdx[i][0].toString().padStart(3)}  score ${harmIdx[i][1].toFixed(2).padStart(6)}`);
  }
  console.log();

  const top3bab = babIdx.slice(0, 3).map(x => x[0]);
  const top3harm = harmIdx.slice(0, 3).map(x => x[0]);
  const overlap = top3bab.filter(g => top3harm.includes(g));

  if (overlap.length === 0) {
    console.log("  ✅ No overlap between top-3 champions!");
    console.log("     → Human and nature optimize for DIFFERENT structures");
  }
  console.log();

  console.log("╔══════════════════════════════════════════════════════════════╗");
  console.log("║                          SUMMARY                             ║");
  console.log("╚══════════════════════════════════════════════════════════════╝");
  console.log();
  if (metric === "norm" && Math.abs(r) < 0.1) {
    console.log("  ✅ ORTHOGONALITY CONFIRMED!");
    console.log("     Human convenience and nature's patterns are INDEPENDENT");
  } else if (metric === "raw" && r > 0.3) {
    console.log("  ✅ Raw correlation confirms HL singular series bias");
  }
  console.log();
  console.log("  Mathematics transcends human design. 🖤");
  console.log();
})();
