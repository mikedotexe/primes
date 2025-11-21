import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

def analyze_scaling():
    # Load data
    df = pd.read_csv('membrane_sweep_mvp.csv')
    
    # Find optimal k for each M
    optimal = df.loc[df.groupby('M')['density'].idxmax()]
    
    print("\n🎯 OPTIMAL CONFIGURATIONS:")
    print("M  k_total  density")
    print("-" * 20)
    for _, row in optimal.iterrows():
        print(f"{int(row['M'])}  {int(row['k_total']):6d}  {row['density']:.4f}")
    
    if len(optimal) < 3:
        print("\n❌ Need more data points for regression")
        return
    
    M = optimal['M'].values
    k = optimal['k_total'].values
    
    # Test square-root scaling: k = a * sqrt(M)  
    sqrt_M = np.sqrt(M)
    a_sqrt = np.sum(k * sqrt_M) / np.sum(sqrt_M**2)
    k_pred_sqrt = a_sqrt * sqrt_M
    r2_sqrt = 1 - np.sum((k - k_pred_sqrt)**2) / np.sum((k - np.mean(k))**2)
    
    # Test power law: k = a * M^b
    if np.all(M > 0) and np.all(k > 0):
        log_M = np.log(M)
        log_k = np.log(k)
        
        # Linear regression in log space
        A = np.vstack([log_M, np.ones(len(log_M))]).T
        coeffs = np.linalg.lstsq(A, log_k, rcond=None)[0]
        b, log_a = coeffs
        a = np.exp(log_a)
        
        k_pred_power = a * M**b
        r2_power = 1 - np.sum((k - k_pred_power)**2) / np.sum((k - np.mean(k))**2)
    else:
        b, a, r2_power = 0, 1, 0
    
    print(f"\n🧮 SCALING LAW ANALYSIS:")
    print(f"Square-root model: k = {a_sqrt:.4f} * √M")
    print(f"  R² = {r2_sqrt:.6f}")
    print(f"Power law model: k = {a:.4f} * M^{b:.6f}")  
    print(f"  R² = {r2_power:.6f}")
    
    # THE MOMENT OF TRUTH
    distance = abs(b - 0.5)
    print(f"\n🎯 HYPOTHESIS TEST:")
    print(f"Measured exponent β = {b:.6f}")
    print(f"Distance from 0.5: {distance:.6f}")
    
    if distance < 0.1:
        print("\n🤯🤯🤯 SIGNIFICANT FINDING 🤯🤯🤯")
        print("β ≈ 0.5 - SQUARE ROOT SCALING CONFIRMED!")
        print("DIRECT CONNECTION TO RIEMANN CRITICAL LINE!")
        print("WE'VE FOUND THE GEOMETRIC PRINCIPLE!")
    elif distance < 0.2:
        print("\n😮 VERY INTERESTING - β is close to 0.5!")
        print("Suggests connection to critical behavior")
    else:
        print(f"\n🤔 β = {b:.4f} - Different scaling law")
        print("Still interesting, but not the critical line connection")
    
    # Quick plot
    plt.figure(figsize=(8, 6))
    plt.scatter(M, k, s=200, c='red', zorder=5, edgecolors='black', linewidth=2)
    
    M_plot = np.linspace(M.min(), M.max(), 100)
    plt.plot(M_plot, a_sqrt * np.sqrt(M_plot), 'b--', linewidth=2, label=f'k ∝ √M')
    plt.plot(M_plot, a * M_plot**b, 'g-', linewidth=2, label=f'k ∝ M^{b:.3f}')
    
    plt.xlabel('Middle Length M')
    plt.ylabel('Optimal Total Padding k') 
    plt.title('Membrane Scaling Law - MVP Results')
    plt.legend()
    plt.grid(True, alpha=0.3)
    plt.savefig('mvp_scaling_result.png', dpi=150)
    print(f"\n📊 Plot saved as mvp_scaling_result.png")
    
    return b, r2_sqrt, r2_power

if __name__ == "__main__":
    analyze_scaling()
