//! Integration test to verify the validation system works correctly with current structure

use primes::{
    membrane::{MembraneBuilder, MembraneConfig},
    validation::{
        exhaustive_tracker::ExhaustiveTracker,
        failure_analysis::FailureAnalyzer,
        random_baseline::{RandomBaseline, RandomStrategy},
        ValidationContext,
    },
};
use std::time::Instant;

#[test]
fn test_validation_with_single_config() {
    // Create a single membrane configuration - this verifies we're using single values, not vectors
    let config = MembraneConfig::new(10, 3, 7, 2, 2);

    // Verify the config has single values (not vectors)
    assert_eq!(config.outer, 3);
    assert_eq!(config.inner, 7);
    assert_eq!(config.k_outer, 2);
    assert_eq!(config.k_inner, 2);

    // Test with validation context
    let mut context = ValidationContext {
        verbose: false,
        ..Default::default()
    };

    // Create random baseline comparator
    let mut baseline = RandomBaseline::new(&mut context);

    // Test with a few middle digits
    let middle_digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Run comparison
    let result = baseline.compare_with_membrane(
        &config,
        &middle_digits,
        RandomStrategy::StructurePreserving,
    );

    // Verify results make sense
    assert!(
        result.method_success_rate > 0.0,
        "Should find at least some primes"
    );
    assert!(
        result.improvement_factor > 1.0,
        "Should be better than random"
    );
    assert_eq!(result.sample_size, middle_digits.len());
    assert!(result.p_value < 1.0);

    println!("Validation test passed:");
    println!(
        "  Method success rate: {:.2}%",
        result.method_success_rate * 100.0
    );
    println!(
        "  Random success rate: {:.2}%",
        result.random_success_rate * 100.0
    );
    println!("  Improvement factor: {:.1}x", result.improvement_factor);
}

#[test]
fn test_exhaustive_tracker() {
    let mut tracker = ExhaustiveTracker::new();

    // Test multiple configurations - each with single outer/inner values
    let configs = vec![
        MembraneConfig::new(10, 3, 7, 2, 2),
        MembraneConfig::new(10, 3, 3, 2, 2),
        MembraneConfig::new(10, 7, 7, 2, 2),
        MembraneConfig::new(11, 3, 8, 2, 2),
    ];

    // Record tests for each configuration
    for config in configs {
        // Generate some test results
        let mut results = Vec::new();
        for middle in 0..10 {
            if let Ok(num) = config.construct_number(middle) {
                let is_prime = primes::is_prime(&num);
                results.push((num, is_prime));
            }
        }

        let duration = 100; // Mock duration
        let selected = config.outer == 3 && config.inner == 7; // Select the classic config
        let reason = if selected {
            "High performance"
        } else {
            "Below threshold"
        };

        tracker.record_test(config, &results, duration, selected, reason);
    }

    // Verify tracking
    assert_eq!(tracker.all_tests.len(), 4);
    assert!(tracker.parameter_stats.contains_key("outer_3"));
    assert!(tracker.parameter_stats.contains_key("inner_7"));
    assert!(tracker.parameter_stats.contains_key("base_10"));

    // Check that stats are accumulated correctly
    let outer_3_stats = &tracker.parameter_stats["outer_3"];
    assert!(outer_3_stats.configurations_tested >= 3); // At least 3 configs used outer=3

    // Generate report to ensure no crashes
    let report = tracker.generate_report();
    assert!(report.contains("EXHAUSTIVE SEARCH REPORT"));
    assert!(report.contains("Total Configurations Tested: 4"));
}

#[test]
fn test_failure_analyzer() {
    let mut analyzer = FailureAnalyzer::new();

    // Test a configuration that we expect to fail
    let bad_config = MembraneConfig::new(10, 5, 5, 2, 2); // 5 as boundary typically fails

    // Generate results (we don't need to count successes here)

    // Analyze the failure (need to create results first)
    let mut results = Vec::new();
    for middle in 0..10 {
        if let Ok(num) = bad_config.construct_number(middle) {
            let is_prime = primes::is_prime(&num);
            results.push((num, is_prime));
        }
    }

    let analysis = analyzer.analyze_failure(&bad_config, &results);

    // Verify failure was analyzed
    assert_eq!(analyzer.failure_database.len(), 1);
    assert!(!analyzer.failure_patterns.is_empty());

    // Check the analysis
    assert!(analysis.success_rate < 0.1); // Should be low
    assert!(!analysis.lessons.is_empty());
    assert!(!analysis.suggestions.is_empty());
}

#[test]
fn test_membrane_builder_integration() {
    // Test that MembraneBuilder works with single-value configs
    let config = MembraneConfig::new(10, 3, 7, 2, 2);

    let builder = MembraneBuilder::new(config)
        .with_position([0.0, 0.0])
        .with_velocity([0.0, 0.0])
        .with_name("Test Prime".to_string())
        .with_max_attempts(100);

    // Try to build - this may or may not succeed, but shouldn't panic
    match builder.build() {
        Ok(particle) => {
            println!("Successfully generated prime: {}", particle.value);
            assert_eq!(particle.base, 10);
            assert_eq!(particle.name, "Test Prime");
        }
        Err(e) => {
            println!(
                "Failed to generate prime after 100 attempts (expected): {:?}",
                e
            );
        }
    }
}

#[test]
fn test_multiple_configs_separately() {
    // This test verifies we handle multiple configurations correctly
    // by testing them one at a time (not as vectors in a single config)

    let outer_values = vec![1, 3, 7, 9];
    let inner_values = vec![1, 3, 5, 7, 9];

    let mut total_configs = 0;
    let mut total_primes = 0;

    // Test each combination
    for &outer in &outer_values {
        for &inner in &inner_values {
            let config = MembraneConfig::new(10, outer, inner, 2, 2);
            total_configs += 1;

            // Test with middle = 5
            if let Ok(num) = config.construct_number(5) {
                if primes::is_prime(&num) {
                    total_primes += 1;
                    println!("Prime found: ({},{}) -> {}", outer, inner, num);
                }
            }
        }
    }

    println!(
        "Tested {} configurations, found {} primes",
        total_configs, total_primes
    );
    assert_eq!(total_configs, 20);
    assert!(total_primes > 0, "Should find at least some primes");
}

/// Run all validation subsystems together
#[test]
fn test_full_validation_pipeline() {
    let start = Instant::now();

    // 1. Setup
    let mut context = ValidationContext {
        verbose: false,
        bootstrap_iterations: 100, // Faster for tests
        ..Default::default()
    };

    let mut tracker = ExhaustiveTracker::new();
    let mut analyzer = FailureAnalyzer::new();

    // 2. Test a range of configurations
    let test_configs = vec![
        (3, 7, "Classic"),
        (3, 3, "Twin 3s"),
        (7, 7, "Twin 7s"),
        (1, 9, "Extremes"),
        (5, 5, "Center digits"),
    ];

    for (outer, inner, name) in test_configs {
        let config = MembraneConfig::new(10, outer, inner, 2, 2);

        // Generate results
        let mut results = Vec::new();
        for middle in 0..10 {
            if let Ok(num) = config.construct_number(middle) {
                let is_prime = primes::is_prime(&num);
                results.push((num, is_prime));
            }
        }

        let primes_found = results.iter().filter(|(_, p)| *p).count();
        let success_rate = primes_found as f64 / results.len() as f64;

        // Record in tracker
        let selected = success_rate > 0.1; // 10% threshold
        tracker.record_test(
            config.clone(),
            &results,
            10, // mock duration
            selected,
            if selected {
                "High performance"
            } else {
                "Low performance"
            },
        );

        // Analyze failures
        if success_rate < 0.05 {
            analyzer.analyze_failure(&config, &results);
        }

        println!("{}: {:.1}% success rate", name, success_rate * 100.0);
    }

    // 3. Validate against random baseline
    let mut baseline = RandomBaseline::new(&mut context);
    let best_config = MembraneConfig::new(10, 3, 7, 2, 2);
    let validation = baseline.compare_with_membrane(
        &best_config,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        RandomStrategy::StructurePreserving,
    );

    // 4. Generate reports
    let tracker_report = tracker.generate_report();
    let failure_report = analyzer.generate_failure_report();

    // 5. Assertions
    assert!(
        validation.improvement_factor > 2.0,
        "Should be significantly better than random"
    );
    assert!(tracker_report.contains("Total Configurations Tested: 5"));
    assert!(!failure_report.is_empty());

    let elapsed = start.elapsed();
    println!(
        "\nFull validation pipeline completed in {:.2}s",
        elapsed.as_secs_f64()
    );
    println!(
        "Best configuration improvement: {:.1}x over random",
        validation.improvement_factor
    );
}
