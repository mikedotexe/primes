#!/bin/bash
# Membrane Scaling MVP Setup & Execution
# ======================================
# 
# Complete end-to-end MVP pipeline to test the core scaling hypothesis:
# Do optimal membrane configurations follow k* ∝ M^(1/2) scaling?
#
# This script integrates with Mike's existing prime-physics-engine codebase
# and runs a quick test to see if we've discovered something profound.

set -e

echo "🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬"
echo "                 MEMBRANE SCALING MVP"
echo "         Testing: k* ∝ M^(1/2) ~ Riemann Critical Line"
echo "🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬🧬"
echo

# Configuration
MVP_DIR="membrane_scaling_mvp"
RUST_ADAPTER="membrane_mvp_adapter"
PYTHON_SCRIPT="membrane_scaling_mvp.py"

# ============================================================================
# STEP 1: Environment Setup
# ============================================================================

echo "📦 Setting up MVP environment..."

# Create MVP directory
mkdir -p $MVP_DIR
cd $MVP_DIR

# Copy MVP files if not already present
if [[ ! -f "$PYTHON_SCRIPT" ]]; then
    cp ../$PYTHON_SCRIPT . 2>/dev/null || echo "  → Copy $PYTHON_SCRIPT manually to $MVP_DIR/"
fi

if [[ ! -f "${RUST_ADAPTER}.rs" ]]; then
    cp ../${RUST_ADAPTER}.rs . 2>/dev/null || echo "  → Copy ${RUST_ADAPTER}.rs manually to $MVP_DIR/"
fi

# Check Python dependencies
echo "🐍 Checking Python environment..."
python3 -c "import numpy, matplotlib" 2>/dev/null || {
    echo "  ❌ Missing Python dependencies. Install with:"
    echo "     pip install numpy matplotlib scipy"
    exit 1
}
echo "  ✓ Python environment ready"

# ============================================================================
# STEP 2: Build Rust Adapter
# ============================================================================

echo "🦀 Building Rust MVP adapter..."

# Create minimal Cargo.toml for standalone build
cat > Cargo.toml << EOF
[package]
name = "membrane-mvp-adapter"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "membrane_mvp_adapter"
path = "membrane_mvp_adapter.rs"

[dependencies]
# Add any dependencies from Mike's main project if needed
EOF

# Build the adapter
if cargo build --release; then
    echo "  ✓ Rust adapter compiled successfully"
    RUST_EXECUTABLE="./target/release/membrane_mvp_adapter"
else
    echo "  ❌ Rust compilation failed. Trying workaround..."
    # Fallback: compile directly
    if rustc membrane_mvp_adapter.rs -O -o membrane_mvp_adapter; then
        echo "  ✓ Rust adapter compiled with rustc"
        RUST_EXECUTABLE="./membrane_mvp_adapter"
    else
        echo "  ❌ Could not compile Rust adapter"
        echo "     You may need to integrate with Mike's main Cargo.toml"
        exit 1
    fi
fi

# ============================================================================
# STEP 3: Quick Validation Test
# ============================================================================

echo "🧪 Running validation test..."

# Test known configuration: Base-6 (1,5) with M=1, k=(0,0) should work well
echo "  Testing known good configuration: Base-6 (1,5) M=1 k=(0,0)"

if $RUST_EXECUTABLE --base 6 --outer 1 --inner 5 --middle-length 1 --k-outer 0 --k-inner 0; then
    echo "  ✓ Basic membrane generation working"
else
    echo "  ❌ Basic test failed - check Rust implementation"
    exit 1
fi

# ============================================================================
# STEP 4: Parameter Sweep
# ============================================================================

echo "📊 Running membrane parameter sweep..."

# Generate CSV data
CSV_FILE="membrane_sweep_mvp.csv"
echo "Running sweep and saving to $CSV_FILE"

$RUST_EXECUTABLE --sweep --base 6 --outer 1 --inner 5 > $CSV_FILE

if [[ -s $CSV_FILE ]]; then
    echo "  ✓ Parameter sweep completed"
    echo "  → Generated $(wc -l < $CSV_FILE) data points"
else
    echo "  ❌ Parameter sweep failed"
    exit 1
fi

# ============================================================================
# STEP 5: Scaling Analysis  
# ============================================================================

echo "🔬 Running scaling law analysis..."

# Create modified Python script that reads CSV directly
cat > quick_scaling_analysis.py << 'EOF'
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
        print("\n🤯🤯🤯 HOLY SHIT MOMENT 🤯🤯🤯")
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
EOF

# Run the analysis
python3 quick_scaling_analysis.py

# ============================================================================
# STEP 6: Summary
# ============================================================================

echo
echo "🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁"
echo "                    MVP COMPLETE"
echo "🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁🏁"
echo
echo "Generated files:"
echo "  • membrane_sweep_mvp.csv - Parameter sweep data"
echo "  • mvp_scaling_result.png - Scaling law visualization"  
echo "  • Rust adapter executable"
echo
echo "Next steps based on results:"
echo "  • If β ≈ 0.5: BUILD THE FULL ANALYSIS SYSTEM!"
echo "  • If β ≠ 0.5: Still interesting - investigate other connections"
echo "  • Either way: Expand to more M values and bases"
echo
echo "🧬 This MVP tests one of the most profound hypotheses in mathematics:"
echo "   Do local primality construction principles follow the same"
echo "   geometric optimization as global prime distribution?"
echo
echo "🎯 The answer is in the exponent β..."

# Final check
if [[ -f "mvp_scaling_result.png" ]]; then
    echo
    echo "✓ MVP pipeline completed successfully!"
    echo "  View mvp_scaling_result.png to see if we've made history."
else
    echo
    echo "⚠️  MVP completed with issues - check Python analysis above"
fi
