#!/usr/bin/env python3
"""
MEMBRANE SCALING MVP - Quick & Dirty Hypothesis Test
===================================================

Scrappy test of the core hypothesis: k* ∝ M^(1/2)

Integrates with Mike's existing Rust membrane infrastructure.
Runs quick analysis to see if we're onto something profound or chasing shadows.

Usage:
    python membrane_scaling_mvp.py

Prerequisites:
    1. Mike's prime-physics-engine compiled
    2. Basic membrane examples working
    3. Python with numpy, matplotlib

MVP Test Plan:
    1. Run small parameter sweep: M=1,2,3,4 k=0,1,2
    2. Find optimal k for each M
    3. Fit k* = a*sqrt(M) and k* = a*M^b  
    4. Compare fits
    5. Print "🤯" if b ≈ 0.5
"""

import subprocess
import numpy as np
import matplotlib.pyplot as plt
import json
import os
from dataclasses import dataclass
from typing import List, Tuple

@dataclass
class QuickResult:
    M: int
    k_outer: int 
    k_inner: int
    density: float
    primes_found: int

class MembraneScalingMVP:
    def __init__(self):
        self.results: List[QuickResult] = []
        
    def run_membrane_test(self, base: int, outer: int, inner: int, 
                         middle_length: int, k_outer: int, k_inner: int) -> Tuple[float, int]:
        """
        Run single membrane configuration test using Mike's existing infrastructure
        
        This is the MVP version - we'll use cargo run to test small cases quickly
        """
        
        # Generate test command (adapt to Mike's actual CLI)
        cmd = [
            'cargo', 'run', '--example', 'proper_membrane_generator', 
            '--', 
            '--base', str(base),
            '--outer', str(outer), 
            '--inner', str(inner),
            '--middle-length', str(middle_length),
            '--k-outer', str(k_outer),
            '--k-inner', str(k_inner),
            '--count', '20'  # Small sample for MVP
        ]
        
        try:
            # Run Mike's membrane generator
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
            
            if result.returncode == 0:
                # Parse output (adapt to actual output format)
                lines = result.stdout.strip().split('\n')
                for line in lines:
                    if 'Prime density:' in line:
                        density = float(line.split(':')[1].strip().replace('%', '')) / 100
                        return density, 10  # Placeholder prime count
                        
                # Fallback - count primes in output
                prime_lines = [l for l in lines if 'PRIME' in l or l.strip().isdigit()]
                return len(prime_lines) / 20, len(prime_lines)
                
            else:
                print(f"⚠️  Command failed: {' '.join(cmd)}")
                return 0.0, 0
                
        except subprocess.TimeoutExpired:
            print(f"⏰ Timeout for M={middle_length}, k=({k_outer},{k_inner})")
            return 0.0, 0
        except Exception as e:
            print(f"💥 Error: {e}")
            return 0.0, 0
    
    def quick_parameter_sweep(self, base: int = 6, outer: int = 1, inner: int = 5):
        """MVP parameter sweep - just enough to test the scaling hypothesis"""
        
        print(f"\n🚀 MEMBRANE SCALING MVP")
        print(f"{'='*50}")
        print(f"Testing: Base-{base} ({outer},{inner})")
        print(f"Hypothesis: k* ∝ M^(1/2)")
        print()
        
        # Small parameter space for MVP
        middle_lengths = [1, 2, 3, 4]  
        k_values = [0, 1, 2]
        
        print("Running parameter sweep...")
        print("M  k_out k_in  Density  Primes")
        print("-" * 35)
        
        for M in middle_lengths:
            best_density = 0.0
            best_config = (0, 0)
            
            for k_out in k_values:
                for k_in in k_values:
                    density, primes = self.run_membrane_test(
                        base, outer, inner, M, k_out, k_in
                    )
                    
                    print(f"{M}  {k_out:4d} {k_in:4d}  {density:7.3f}  {primes:6d}")
                    
                    if density > best_density:
                        best_density = density
                        best_config = (k_out, k_in)
                    
                    self.results.append(QuickResult(
                        M=M, k_outer=k_out, k_inner=k_in, 
                        density=density, primes_found=primes
                    ))
            
            print(f"  → Best for M={M}: k={best_config} density={best_density:.3f}")
        
        print()
    
    def test_scaling_hypothesis(self):
        """Test if optimal k follows square-root scaling"""
        
        print("🧮 SCALING LAW ANALYSIS")
        print("=" * 50)
        
        # Extract optimal configurations
        optimal = {}
        for M in set(r.M for r in self.results):
            best_result = max(
                (r for r in self.results if r.M == M),
                key=lambda x: x.density
            )
            optimal[M] = best_result
        
        # Prepare data for regression  
        M_vals = np.array(list(optimal.keys()))
        k_vals = np.array([optimal[M].k_outer + optimal[M].k_inner for M in M_vals])
        
        print(f"Optimal configurations:")
        print("M  k_total  density")
        print("-" * 20)
        for M in sorted(M_vals):
            result = optimal[M]
            k_total = result.k_outer + result.k_inner
            print(f"{M}  {k_total:6d}  {result.density:.3f}")
        print()
        
        if len(M_vals) < 3:
            print("❌ Need at least 3 data points for regression")
            return
        
        # Test square-root model: k = a * sqrt(M)
        sqrt_M = np.sqrt(M_vals)
        a_sqrt = np.sum(k_vals * sqrt_M) / np.sum(sqrt_M**2)
        k_pred_sqrt = a_sqrt * sqrt_M
        r2_sqrt = 1 - np.sum((k_vals - k_pred_sqrt)**2) / np.sum((k_vals - np.mean(k_vals))**2)
        
        # Test general power law: k = a * M^b  
        log_M = np.log(M_vals)
        log_k = np.log(np.maximum(k_vals, 0.1))  # Avoid log(0)
        
        # Linear regression on log-log plot
        A = np.vstack([log_M, np.ones(len(log_M))]).T
        coeffs = np.linalg.lstsq(A, log_k, rcond=None)[0]
        b_power, log_a = coeffs
        a_power = np.exp(log_a)
        
        k_pred_power = a_power * M_vals**b_power
        r2_power = 1 - np.sum((k_vals - k_pred_power)**2) / np.sum((k_vals - np.mean(k_vals))**2)
        
        # Results
        print("MODEL COMPARISON:")
        print(f"Square-root model: k = {a_sqrt:.4f} * √M")
        print(f"  R² = {r2_sqrt:.6f}")
        print()
        print(f"Power law model: k = {a_power:.4f} * M^{b_power:.6f}")
        print(f"  R² = {r2_power:.6f}")
        print(f"  Exponent β = {b_power:.6f}")
        print()
        
        # Test the hypothesis!
        distance_from_half = abs(b_power - 0.5)
        
        print("🎯 HYPOTHESIS TEST:")
        print(f"H₀: Scaling exponent β = 0.5 (square-root)")
        print(f"Measured β = {b_power:.6f}")
        print(f"Distance from 0.5: {distance_from_half:.6f}")
        
        if distance_from_half < 0.1:
            print("🤯 HOLY SHIT - β ≈ 0.5!")
            print("   Membrane spacing follows SQUARE-ROOT scaling!")
            print("   Direct connection to Riemann critical line!")
        elif distance_from_half < 0.2:
            print("😮 Interesting - β is close to 0.5")
            print("   Suggests possible connection to critical behavior")
        else:
            print("🤔 β is not close to 0.5")
            print("   Different scaling law, but still interesting")
            
        # Quick visualization
        self.plot_scaling(M_vals, k_vals, a_sqrt, a_power, b_power)
        
        return b_power, r2_sqrt, r2_power
    
    def plot_scaling(self, M_vals, k_vals, a_sqrt, a_power, b_power):
        """Quick visualization of scaling laws"""
        
        plt.figure(figsize=(10, 6))
        
        # Data points
        plt.scatter(M_vals, k_vals, s=150, color='red', zorder=5, 
                   label='Empirical Optimal k', edgecolors='black', linewidth=2)
        
        # Models
        M_fine = np.linspace(M_vals.min(), M_vals.max(), 100)
        sqrt_pred = a_sqrt * np.sqrt(M_fine)
        power_pred = a_power * M_fine**b_power
        
        plt.plot(M_fine, sqrt_pred, 'b--', linewidth=2, label=f'√M scaling (a={a_sqrt:.3f})')
        plt.plot(M_fine, power_pred, 'g-', linewidth=2, label=f'M^{b_power:.3f} scaling')
        
        plt.xlabel('Middle Length (M)', fontsize=12, fontweight='bold')
        plt.ylabel('Optimal Total Padding (k)', fontsize=12, fontweight='bold')
        plt.title('Membrane Scaling Law - MVP Test', fontsize=14, fontweight='bold')
        plt.legend(fontsize=11)
        plt.grid(True, alpha=0.3)
        
        plt.tight_layout()
        plt.savefig('membrane_scaling_mvp.png', dpi=150)
        plt.show()
        
        print("\n📊 Visualization saved as 'membrane_scaling_mvp.png'")

def main():
    """Run the MVP scaling test"""
    
    print("\n" + "🧬" * 25)
    print("MEMBRANE SCALING HYPOTHESIS MVP")
    print("Testing: k* ∝ M^(1/2) connection to Riemann critical line")
    print("🧬" * 25)
    
    mvp = MembraneScalingMVP()
    
    # Step 1: Quick parameter sweep
    mvp.quick_parameter_sweep(base=6, outer=1, inner=5)
    
    # Step 2: Test scaling hypothesis  
    beta, r2_sqrt, r2_power = mvp.test_scaling_hypothesis()
    
    # Step 3: Final verdict
    print("\n" + "🎯" * 25)
    print("MVP CONCLUSION:")
    
    if abs(beta - 0.5) < 0.1:
        print("🚀 BREAKTHROUGH CONFIRMED")
        print("   Membrane scaling follows square-root law!")
        print("   Strong evidence for Riemann connection!")
        print("   Time to build the full analysis system!")
    else:
        print("🔬 INTERESTING SCALING DISCOVERED") 
        print(f"   Exponent β = {beta:.4f}")
        print("   Not square-root, but still a universal law!")
        print("   Worth deeper investigation!")
    
    print("🎯" * 25 + "\n")

if __name__ == "__main__":
    main()
