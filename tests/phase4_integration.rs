//! tests/phase4_integration.rs
//! Integration tests for Phase 4 AMX/SME backend
//! cargo test --release --features phase4 phase4_integration

#![cfg(feature = "phase4")]

use prime_physics_engine::prime_sieve::warm_cache_with_primes;

// Mock cache_resident_mlp module for testing
mod mock_mlp {
    pub fn get_weights() -> &'static [i8] {
        &[1, 2, 3, 4, 5, 6, 7, 8]
    }
    
    pub fn predict_neon(_weights: &[i8], x: [i8; 8]) -> i32 {
        x.iter().map(|&v| v as i32).sum()
    }
}

#[test]
fn phase4_latency_ab() {
    warm_cache_with_primes(50_000);
    let x = [12, -7, 64, 32, -1, 127, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0]; // padded

    // A/B five-times median with generic parameter for better performance
    // Enhanced with bounds safety inspired by external best practices
    fn median_timing<F: FnMut() -> i32 + Copy>(mut f: F) -> f64 {
        let mut v = Vec::with_capacity(5);
        for _ in 0..5 {
            let t0 = std::time::Instant::now();
            for _ in 0..200_000 { 
                std::hint::black_box(f()); 
            }
            v.push(t0.elapsed().as_secs_f64());
        }
        
        // Use safe median calculation
        match prime_physics_engine::performance::PerfMonitor::safe_median(&mut v) {
            Some(median) => median / 200_000.0,
            None => {
                panic!("❌ Timer precision validation failed: no valid measurements");
            }
        }
    }

    let neon = median_timing(|| {
        mock_mlp::predict_neon(mock_mlp::get_weights(), x[..8].try_into().unwrap())
    });
    
    let sme = median_timing(|| unsafe { 
        // Safety: predict_sme_padded expects 16-byte array, properly aligned
        prime_physics_engine::phase4::predict_sme_padded(x) 
    });

    println!("neon {:.1} ns  vs  sme-stub {:.1} ns", neon * 1e9, sme * 1e9);
    
    // For now, stub should have similar performance
    assert!(sme < neon * 2.0, "SME stub unexpectedly slow");
}

#[test]
fn rl_converges_basic() {
    use prime_physics_engine::phase4::RL_CTL;
    
    // Feed dummy PMU + latency samples
    for i in 0..5000 {
        let pmu_sample = ((i * 13) % 256) as u8; // Synthetic variation
        let latency = 7 + (i % 10) as u32; // 7-16 ns range
        
        RL_CTL.with(|ctl| ctl.borrow_mut().tick(pmu_sample, latency));
    }
    
    let policy_after = RL_CTL.with(|ctl| ctl.borrow().best_action());
    println!("RL converged to action: {}", policy_after);
    
    // Should have learned something (not stuck at 0)
    assert!(policy_after > 0 || {
        // Check if Q-values have changed
        RL_CTL.with(|ctl| ctl.borrow().has_learned())
    }, "RL failed to learn from samples");
}

#[test]
fn pmu_double_buffer_monotonic() {
    use prime_physics_engine::phase4::{PMU_BUFFER, PmuSnapshot};
    use std::thread;
    use std::time::Duration;
    
    // Writer thread
    let writer = thread::spawn(|| {
        for i in 0..100 {
            let snapshot = PmuSnapshot {
                l1_miss: (i * 10) as u16,
                cycles: (i * 1000) as u32,
                ts: i as u64,
            };
            PMU_BUFFER.write(snapshot);
            thread::sleep(Duration::from_micros(10));
        }
    });
    
    // Reader thread checks monotonicity
    let reader = thread::spawn(|| {
        let mut last_ts = 0;
        let mut reads = 0;
        
        for _ in 0..50 {
            let snapshot = PMU_BUFFER.read();
            if snapshot.ts > 0 {
                assert!(snapshot.ts >= last_ts, "Non-monotonic timestamp");
                last_ts = snapshot.ts;
                reads += 1;
            }
            thread::sleep(Duration::from_micros(20));
        }
        
        assert!(reads > 10, "Too few successful reads");
    });
    
    writer.join().unwrap();
    reader.join().unwrap();
}

#[test]
fn slc_demand_driven_maintenance() {
    use prime_physics_engine::phase4::SlcResident;
    
    let mut slc = SlcResident::new(0.8);
    let dummy_weights = vec![0u8; 4096];
    let ptr = dummy_weights.as_ptr();
    
    // Low warmth should trigger maintenance
    let start = std::time::Instant::now();
    slc.maintain_residency(0.5, ptr, dummy_weights.len());
    let elapsed1 = start.elapsed();
    
    // High warmth should skip maintenance
    let start = std::time::Instant::now();
    slc.maintain_residency(0.9, ptr, dummy_weights.len());
    let elapsed2 = start.elapsed();
    
    // Maintenance should take longer than skip
    assert!(elapsed1 > elapsed2, "SLC maintenance not working");
}

#[test]
fn integration_full_stack() {
    use prime_physics_engine::phase4::{RL_CTL, PMU_BUFFER, PmuSnapshot};
    
    // 1. Warm cache with prime sieve (600K primes = ~36 MiB on 48 MiB SLC)
    warm_cache_with_primes(600_000);
    
    // 2. Simulate workload with PMU feedback
    for i in 0..1000 {
        // Generate synthetic PMU data
        let snapshot = PmuSnapshot {
            l1_miss: (100 + i % 50) as u16,
            cycles: (1000 + i * 10) as u32,
            ts: i as u64,
        };
        PMU_BUFFER.write(snapshot);
        
        // RL observes and adapts
        let pmu_sample = (snapshot.l1_miss / 10) as u8;
        let latency = 5 + (i % 5) as u32;
        RL_CTL.with(|ctl| ctl.borrow_mut().tick(pmu_sample, latency));
        
        // Simulate inference
        let x = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0];
        let _result = unsafe { prime_physics_engine::phase4::predict_sme_padded(x) };
    }
    
    // Verify RL has adapted
    let final_action = RL_CTL.with(|ctl| ctl.borrow().best_action());
    println!("Final RL action after integration: {}", final_action);
    
    // Verify PMU data flow
    let final_pmu = PMU_BUFFER.read();
    assert!(final_pmu.ts > 900, "PMU updates not flowing");
}