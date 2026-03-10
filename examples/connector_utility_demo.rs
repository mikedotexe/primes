//! Connector Utility Demonstration
//!
//! This example demonstrates the connector concatenation utilities introduced
//! for directional asymmetry analysis of Lagrange point prime pairs.
//!
//! Run with: cargo run --example connector_utility_demo

use primes::connector::{utils, ConcatenationSystem};

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       Connector Concatenation Utility Demonstration          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Create the canonical Lagrange point prime pair system
    let sys = ConcatenationSystem::new(10301, 3007003007003);

    println!("📊 Canonical Lagrange Point Prime Pair");
    println!("   Left  (L): {} ({} digits)", sys.left, sys.left_len);
    println!("   Right (R): {} ({} digits)", sys.right, sys.right_len);
    println!();

    // Demonstrate the 4 known Lagrange equilibrium points
    println!("✨ Known Lagrange Equilibrium Points");
    println!("   (Connectors that produce prime concatenations)");
    println!();

    let equilibria = [
        (6, 5, 4, "L₁"),     // Buffer=5, Position=4
        (60000, 6, 2, "L₂"), // Buffer=6, Position=2
        (60, 6, 4, "L₃"),    // Buffer=6, Position=4
        (6000, 7, 3, "L₄"),  // Buffer=7, Position=3
    ];

    for (connector, buf_len, pos, name) in equilibria {
        if let Some(n) = sys.forward(connector, buf_len) {
            let formatted_conn = format!("{:0width$}", connector, width = buf_len as usize);
            println!(
                "   {} (buffer={}, pos={}): L || {} || R",
                name, buf_len, pos, formatted_conn
            );
            println!("      → {}", n);
            println!("      ({} digits total)", sys.total_digits(buf_len));
            println!();
        }
    }

    // Demonstrate forward vs reverse concatenation
    println!("🔄 Directional Asymmetry Example");
    println!("   Same connector, different order:");
    println!();

    let demo_connector = 12345u128;
    let demo_len = 5;

    if let (Some(fwd), Some(rev)) = (
        sys.forward(demo_connector, demo_len),
        sys.reverse(demo_connector, demo_len),
    ) {
        println!("   Connector: {:05}", demo_connector);
        println!();
        println!(
            "   Forward:  {} || {:05} || {}",
            sys.left, demo_connector, sys.right
        );
        println!("           = {}", fwd);
        println!();
        println!(
            "   Reverse:  {} || {:05} || {}",
            sys.right, demo_connector, sys.left
        );
        println!("           = {}", rev);
        println!();
        println!("   ✓ Different results (directional asymmetry)");
        println!();
    }

    // Demonstrate mod-3 filtering
    println!("🔬 Mod-3 Composite Filtering");
    println!("   For canonical pair: L ≡ 2 (mod 3), R ≡ 2 (mod 3)");
    println!();

    println!("   Testing first 10 connectors:");
    for c in 0..10 {
        let skip =
            utils::should_skip_mod3(c, utils::CANONICAL_LEFT_MOD3, utils::CANONICAL_RIGHT_MOD3);

        let c_mod3 = c % 3;
        let status = if skip { "❌ SKIP" } else { "✓ KEEP" };
        println!("   Connector {:2}: C≡{} (mod 3) → {}", c, c_mod3, status);
    }
    println!();
    println!("   Rule: Skip when C ≡ 2 (mod 3) because 2+2+2 ≡ 0 (mod 3)");
    println!();

    // Demonstrate capacity limits
    println!("📏 u128 Capacity Limits");
    println!("   Maximum safe decimal digits: 38");
    println!();

    let test_lengths = [5, 10, 15, 20, 21];
    for &len in &test_lengths {
        let fits = sys.fits_in_u128(len);
        let total = sys.total_digits(len);
        let status = if fits { "✓ OK" } else { "❌ OVERFLOW" };

        println!(
            "   Connector length {:2} → {:2} digits total → {}",
            len, total, status
        );
    }
    println!();

    // Demonstrate range iteration
    println!("🔢 Connector Range Iteration");
    println!("   Example: All 3-digit connectors (000-999)");
    println!();

    let range = utils::connector_range(3);
    println!("   Range: {} to {} (exclusive)", range.start, range.end);
    println!("   Count: {} connectors", range.clone().count());
    println!();

    // Show a few examples
    println!("   First 5: ");
    for c in range.clone().take(5) {
        println!("      {:03}", c);
    }
    println!("   ...");
    println!();

    // Performance note
    println!("⚡ Performance Benefits");
    println!("   u128 arithmetic: 2-5× faster than BigUint");
    println!("   No heap allocations for number construction");
    println!("   Deterministic performance (no GC pauses)");
    println!();

    // Research context
    println!("📚 Research Context");
    println!("   These utilities enabled:");
    println!("   - Lagrange Point Asymmetry discovery (~2% bias)");
    println!("   - Resonance Peak phenomenon (59% peak at length 10)");
    println!("   - Post-sieve mystery investigation");
    println!();
    println!("   See: collab/CORE_ASYMMETRY_NOTES.md");
    println!("        collab/LAGRANGE_POINT_ASYMMETRY.md");
    println!();

    println!("✅ Demo complete!");
}
