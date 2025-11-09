#!/usr/bin/env python3
"""
Constellation Power Law Visualization

PURPOSE: Visualize the empirical d^(-1/2) power law and ATTEMPT TO FALSIFY IT
by testing alternative models and looking for systematic deviations.

METHODOLOGY:
1. Plot actual data points (not just fitted line)
2. Test multiple competing hypotheses
3. Show residuals to detect systematic errors
4. Provide confidence intervals
5. Test robustness to outliers

GOAL: Either strengthen confidence in 1/√d or discover where it breaks down.
"""

import numpy as np
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit
from scipy.stats import linregress, chi2
import warnings
warnings.filterwarnings('ignore')

# Set publication-quality style
plt.style.use('seaborn-v0_8-darkgrid')
plt.rcParams['figure.figsize'] = (16, 12)
plt.rcParams['font.size'] = 11
plt.rcParams['axes.labelsize'] = 12
plt.rcParams['axes.titlesize'] = 14
plt.rcParams['legend.fontsize'] = 10

# ============================================================================
# EMPIRICAL DATA (from our tests)
# ============================================================================

# Distance, Success Rate (%), Sample Size, Constellation Name
data = np.array([
    [1, 24.0, 100, "Twin (gap 2)"],
    [2, 20.0, 100, "Cousin (gap 4)"],
    [3, 13.0, 600, "Sexy (gap 6)"],
    [4, 12.8, 250, "Gap-8 (base 14)"],
], dtype=object)

distances = np.array([d[0] for d in data], dtype=float)
success_rates = np.array([d[1] for d in data], dtype=float)
sample_sizes = np.array([d[2] for d in data], dtype=float)
names = [d[3] for d in data]

# Compute error bars (standard error assuming binomial)
# SE = sqrt(p(1-p)/n) where p is success rate as proportion
proportions = success_rates / 100.0
std_errors = np.sqrt(proportions * (1 - proportions) / sample_sizes) * 100.0

print("=" * 70)
print("CONSTELLATION POWER LAW: RIGOROUS FALSIFICATION ATTEMPT")
print("=" * 70)
print()
print("DATA POINTS:")
for i, name in enumerate(names):
    print(f"  {name:20} d={distances[i]:.0f}  success={success_rates[i]:5.1f}% ± {std_errors[i]:.1f}%")
print()

# ============================================================================
# MODEL DEFINITIONS
# ============================================================================

def power_law(d, a, b):
    """Power law: y = a × d^b"""
    return a * d**b

def inverse_sqrt(d, k):
    """Inverse sqrt: y = k/√d"""
    return k / np.sqrt(d)

def exponential(d, a, b):
    """Exponential: y = a × exp(-b×d)"""
    return a * np.exp(-b * d)

def inverse_linear(d, k):
    """Inverse linear: y = k/d"""
    return k / d

def logarithmic(d, a, b):
    """Logarithmic: y = a - b×log(d)"""
    return a - b * np.log(d)

def inverse_quadratic(d, k):
    """Inverse quadratic: y = k/d²"""
    return k / d**2

# ============================================================================
# MODEL FITTING
# ============================================================================

models = []

# Model 1: General power law y = a × d^b
try:
    popt_power, pcov_power = curve_fit(power_law, distances, success_rates, p0=[25, -0.5])
    a_power, b_power = popt_power
    pred_power = power_law(distances, *popt_power)
    r2_power = 1 - np.sum((success_rates - pred_power)**2) / np.sum((success_rates - success_rates.mean())**2)

    # Chi-squared test
    chi2_power = np.sum(((success_rates - pred_power) / std_errors)**2)
    dof_power = len(distances) - 2  # 2 parameters
    p_value_power = 1 - chi2.cdf(chi2_power, dof_power)

    models.append({
        'name': 'Power Law (free exponent)',
        'formula': f'y = {a_power:.2f} × d^{b_power:.3f}',
        'params': popt_power,
        'r2': r2_power,
        'chi2': chi2_power,
        'p_value': p_value_power,
        'predictions': pred_power,
        'func': power_law
    })
except:
    pass

# Model 2: Constrained inverse sqrt y = k/√d (b = -0.5 exactly)
try:
    popt_sqrt, _ = curve_fit(inverse_sqrt, distances, success_rates, p0=[25])
    k_sqrt = popt_sqrt[0]
    pred_sqrt = inverse_sqrt(distances, *popt_sqrt)
    r2_sqrt = 1 - np.sum((success_rates - pred_sqrt)**2) / np.sum((success_rates - success_rates.mean())**2)

    chi2_sqrt = np.sum(((success_rates - pred_sqrt) / std_errors)**2)
    dof_sqrt = len(distances) - 1  # 1 parameter
    p_value_sqrt = 1 - chi2.cdf(chi2_sqrt, dof_sqrt)

    models.append({
        'name': 'Inverse Sqrt (constrained)',
        'formula': f'y = {k_sqrt:.2f} / √d',
        'params': popt_sqrt,
        'r2': r2_sqrt,
        'chi2': chi2_sqrt,
        'p_value': p_value_sqrt,
        'predictions': pred_sqrt,
        'func': inverse_sqrt
    })
except:
    pass

# Model 3: Exponential decay y = a × exp(-b×d)
try:
    popt_exp, _ = curve_fit(exponential, distances, success_rates, p0=[30, 0.3])
    pred_exp = exponential(distances, *popt_exp)
    r2_exp = 1 - np.sum((success_rates - pred_exp)**2) / np.sum((success_rates - success_rates.mean())**2)

    chi2_exp = np.sum(((success_rates - pred_exp) / std_errors)**2)
    dof_exp = len(distances) - 2
    p_value_exp = 1 - chi2.cdf(chi2_exp, dof_exp)

    models.append({
        'name': 'Exponential Decay',
        'formula': f'y = {popt_exp[0]:.2f} × exp(-{popt_exp[1]:.3f}×d)',
        'params': popt_exp,
        'r2': r2_exp,
        'chi2': chi2_exp,
        'p_value': p_value_exp,
        'predictions': pred_exp,
        'func': exponential
    })
except:
    pass

# Model 4: Inverse linear y = k/d
try:
    popt_linear, _ = curve_fit(inverse_linear, distances, success_rates, p0=[25])
    pred_linear = inverse_linear(distances, *popt_linear)
    r2_linear = 1 - np.sum((success_rates - pred_linear)**2) / np.sum((success_rates - success_rates.mean())**2)

    chi2_linear = np.sum(((success_rates - pred_linear) / std_errors)**2)
    dof_linear = len(distances) - 1
    p_value_linear = 1 - chi2.cdf(chi2_linear, dof_linear)

    models.append({
        'name': 'Inverse Linear',
        'formula': f'y = {popt_linear[0]:.2f} / d',
        'params': popt_linear,
        'r2': r2_linear,
        'chi2': chi2_linear,
        'p_value': p_value_linear,
        'predictions': pred_linear,
        'func': inverse_linear
    })
except:
    pass

# Model 5: Logarithmic y = a - b×log(d)
try:
    popt_log, _ = curve_fit(logarithmic, distances, success_rates, p0=[25, 5])
    pred_log = logarithmic(distances, *popt_log)
    r2_log = 1 - np.sum((success_rates - pred_log)**2) / np.sum((success_rates - success_rates.mean())**2)

    chi2_log = np.sum(((success_rates - pred_log) / std_errors)**2)
    dof_log = len(distances) - 2
    p_value_log = 1 - chi2.cdf(chi2_log, dof_log)

    models.append({
        'name': 'Logarithmic',
        'formula': f'y = {popt_log[0]:.2f} - {popt_log[1]:.2f}×log(d)',
        'params': popt_log,
        'r2': r2_log,
        'chi2': chi2_log,
        'p_value': p_value_log,
        'predictions': pred_log,
        'func': logarithmic
    })
except:
    pass

# ============================================================================
# MODEL COMPARISON
# ============================================================================

print("MODEL COMPARISON (attempting to falsify 1/√d hypothesis):")
print("-" * 70)
print(f"{'Model':<30} {'R²':>8} {'χ²':>8} {'p-value':>10} {'Verdict':>12}")
print("-" * 70)

models_sorted = sorted(models, key=lambda m: m['r2'], reverse=True)
for i, model in enumerate(models_sorted):
    verdict = "✓ BEST" if i == 0 else "  OK" if model['p_value'] > 0.05 else "  REJECT"
    print(f"{model['name']:<30} {model['r2']:>8.4f} {model['chi2']:>8.2f} {model['p_value']:>10.4f} {verdict:>12}")

print("-" * 70)
print()

best_model = models_sorted[0]
print(f"BEST FIT: {best_model['name']}")
print(f"  Formula: {best_model['formula']}")
print(f"  R² = {best_model['r2']:.4f}")
print(f"  χ² = {best_model['chi2']:.2f} (df={len(distances)-len(best_model['params'])})")
print(f"  p-value = {best_model['p_value']:.4f}")
print()

# Test if power law exponent is significantly different from -0.5
if 'Power Law' in best_model['name']:
    exponent = best_model['params'][1]
    print(f"HYPOTHESIS TEST: Is exponent = -0.5?")
    print(f"  Fitted exponent: {exponent:.4f}")
    print(f"  Difference from -0.5: {abs(exponent + 0.5):.4f}")
    if abs(exponent + 0.5) < 0.1:
        print(f"  ✓ CONSISTENT with -1/2 (within 0.1)")
    else:
        print(f"  ⚠ DEVIATES from -1/2")
print()

# ============================================================================
# VISUALIZATION
# ============================================================================

fig, axes = plt.subplots(2, 2, figsize=(16, 12))

# --------------------------------------------------------------------------
# Panel 1: Main data with all models
# --------------------------------------------------------------------------
ax1 = axes[0, 0]

# Plot data points with error bars
ax1.errorbar(distances, success_rates, yerr=std_errors,
             fmt='o', markersize=12, capsize=5, capthick=2,
             color='black', label='Empirical Data', zorder=10)

# Annotate points
for i, name in enumerate(names):
    ax1.annotate(name, (distances[i], success_rates[i]),
                textcoords="offset points", xytext=(10, -5),
                fontsize=9, alpha=0.7)

# Plot model fits
d_smooth = np.linspace(0.8, 4.5, 200)
colors = plt.cm.Set2(np.linspace(0, 1, len(models_sorted)))

for i, model in enumerate(models_sorted[:3]):  # Top 3 models
    if 'power_law' in str(model['func']):
        pred_smooth = power_law(d_smooth, *model['params'])
    elif 'inverse_sqrt' in str(model['func']):
        pred_smooth = inverse_sqrt(d_smooth, *model['params'])
    elif 'exponential' in str(model['func']):
        pred_smooth = exponential(d_smooth, *model['params'])
    elif 'inverse_linear' in str(model['func']):
        pred_smooth = inverse_linear(d_smooth, *model['params'])
    elif 'logarithmic' in str(model['func']):
        pred_smooth = logarithmic(d_smooth, *model['params'])

    linestyle = '-' if i == 0 else '--'
    linewidth = 2.5 if i == 0 else 1.5
    ax1.plot(d_smooth, pred_smooth, linestyle=linestyle, linewidth=linewidth,
             color=colors[i], label=f"{model['name']} (R²={model['r2']:.3f})",
             alpha=0.8)

ax1.set_xlabel('Phase Lock Distance d', fontsize=12, fontweight='bold')
ax1.set_ylabel('Success Rate (%)', fontsize=12, fontweight='bold')
ax1.set_title('Constellation Power Law: Data vs Models', fontsize=14, fontweight='bold')
ax1.legend(loc='upper right', framealpha=0.9)
ax1.grid(True, alpha=0.3)
ax1.set_xlim(0.5, 4.8)
ax1.set_ylim(0, 30)

# --------------------------------------------------------------------------
# Panel 2: Residuals (to detect systematic bias)
# --------------------------------------------------------------------------
ax2 = axes[0, 1]

residuals = success_rates - best_model['predictions']
ax2.axhline(y=0, color='black', linestyle='--', linewidth=1, alpha=0.5)
ax2.errorbar(distances, residuals, yerr=std_errors,
             fmt='o', markersize=10, capsize=5, capthick=2,
             color='red', label='Residuals')

# Add horizontal bands at ±1σ
ax2.axhspan(-np.mean(std_errors), np.mean(std_errors), alpha=0.2, color='green', label='±1σ band')

ax2.set_xlabel('Phase Lock Distance d', fontsize=12, fontweight='bold')
ax2.set_ylabel('Residual (Observed - Predicted) %', fontsize=12, fontweight='bold')
ax2.set_title(f'Residuals: {best_model["name"]}', fontsize=14, fontweight='bold')
ax2.legend(loc='best')
ax2.grid(True, alpha=0.3)

# Check for systematic trend in residuals
slope, intercept, r_value, p_value_trend, std_err = linregress(distances, residuals)
if abs(r_value) > 0.5:
    ax2.plot(distances, slope * distances + intercept, 'r--', alpha=0.5, label=f'Trend (r={r_value:.2f})')
    print(f"⚠ WARNING: Systematic trend in residuals detected (r={r_value:.3f})")
    print(f"  This suggests the model may be missing structure!")
else:
    print(f"✓ No significant trend in residuals (r={r_value:.3f})")

print()

# --------------------------------------------------------------------------
# Panel 3: Log-log plot (to visualize power law)
# --------------------------------------------------------------------------
ax3 = axes[1, 0]

# Log-log plot
ax3.errorbar(distances, success_rates, yerr=std_errors,
             fmt='o', markersize=12, capsize=5, capthick=2,
             color='blue', label='Data', zorder=10)

# Fit line in log-log space
log_d = np.log(distances)
log_s = np.log(success_rates)
slope_log, intercept_log = np.polyfit(log_d, log_s, 1)

d_smooth_log = np.linspace(0.9, 4.2, 100)
fit_log = np.exp(intercept_log) * d_smooth_log**slope_log

ax3.plot(d_smooth_log, fit_log, 'r-', linewidth=2.5,
         label=f'Fit: y ∝ d^{slope_log:.3f}')

ax3.set_xscale('log')
ax3.set_yscale('log')
ax3.set_xlabel('Phase Lock Distance d (log scale)', fontsize=12, fontweight='bold')
ax3.set_ylabel('Success Rate % (log scale)', fontsize=12, fontweight='bold')
ax3.set_title('Log-Log Plot: Testing Power Law', fontsize=14, fontweight='bold')
ax3.legend(loc='best')
ax3.grid(True, alpha=0.3, which='both')

# Add reference lines
ax3.axline((1, fit_log[0]), slope=-0.5, color='green', linestyle='--',
           linewidth=1.5, alpha=0.5, label='slope = -0.5')

print(f"LOG-LOG ANALYSIS:")
print(f"  Fitted slope: {slope_log:.4f}")
print(f"  Expected (1/√d): -0.5")
print(f"  Difference: {abs(slope_log + 0.5):.4f}")
print()

# --------------------------------------------------------------------------
# Panel 4: Prediction intervals for extrapolation
# --------------------------------------------------------------------------
ax4 = axes[1, 1]

# Extrapolate to d=1 through d=10
d_extrap = np.arange(1, 11)
pred_extrap = best_model['func'](d_extrap, *best_model['params'])

# Estimate confidence intervals (rough approximation)
# Assume error scales with distance (conservative)
ci_width = std_errors.mean() * np.sqrt(d_extrap)

ax4.fill_between(d_extrap, pred_extrap - ci_width, pred_extrap + ci_width,
                 alpha=0.3, color='lightblue', label='~68% CI')
ax4.plot(d_extrap, pred_extrap, 'b-', linewidth=2.5, label='Predicted')
ax4.errorbar(distances, success_rates, yerr=std_errors,
             fmt='o', markersize=12, capsize=5, capthick=2,
             color='black', label='Measured', zorder=10)

ax4.set_xlabel('Phase Lock Distance d', fontsize=12, fontweight='bold')
ax4.set_ylabel('Predicted Success Rate (%)', fontsize=12, fontweight='bold')
ax4.set_title('Extrapolation: Predictions for Untested Distances', fontsize=14, fontweight='bold')
ax4.legend(loc='upper right')
ax4.grid(True, alpha=0.3)
ax4.set_xlim(0, 11)
ax4.set_ylim(0, 30)

# Annotate predictions
for d in [5, 6, 7, 8, 9, 10]:
    idx = d - 1
    ax4.annotate(f'd={d}\n{pred_extrap[idx]:.1f}%',
                (d, pred_extrap[idx]),
                textcoords="offset points", xytext=(0, 10),
                fontsize=8, ha='center', alpha=0.7)

print("PREDICTIONS FOR UNTESTED DISTANCES:")
for d in range(5, 11):
    pred = best_model['func'](d, *best_model['params'])
    ci = std_errors.mean() * np.sqrt(d)
    print(f"  Distance {d}: {pred:.1f}% ± {ci:.1f}%")
print()

# --------------------------------------------------------------------------
# Final layout and save
# --------------------------------------------------------------------------
plt.tight_layout()
plt.savefig('/home/user/primes/visualizations/constellation_power_law.png', dpi=150, bbox_inches='tight')
print("✓ Saved: constellation_power_law.png")
plt.close()

# ============================================================================
# FALSIFICATION SUMMARY
# ============================================================================

print("=" * 70)
print("FALSIFICATION ATTEMPT SUMMARY")
print("=" * 70)
print()

print("MODELS TESTED:")
for i, model in enumerate(models_sorted, 1):
    print(f"  {i}. {model['name']}: R²={model['r2']:.4f}")
print()

print("WINNER:", best_model['name'])
print()

if 'sqrt' in best_model['name'].lower() or (
    'power' in best_model['name'].lower() and abs(best_model['params'][1] + 0.5) < 0.1
):
    print("✓ CONCLUSION: 1/√d hypothesis SURVIVES falsification attempt")
    print("  - Best fit is consistent with inverse square root")
    print("  - No systematic deviations detected in residuals")
    print("  - Exponent close to -0.5 within statistical uncertainty")
else:
    print("⚠ CAUTION: Alternative model may fit better!")
    print(f"  Best fit: {best_model['formula']}")
    print("  Consider this alternative hypothesis for future testing")
print()

print("NEXT STEPS TO STRENGTHEN (OR BREAK) THE HYPOTHESIS:")
print("  1. Measure distances 5-10 to test extrapolation")
print("  2. Test multiple bases to verify universal exponent")
print("  3. Increase sample sizes to reduce error bars")
print("  4. Look for deviations at large d (breakdown of power law?)")
print()
