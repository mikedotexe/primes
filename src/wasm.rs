//! WebAssembly bindings for the membrane prime engine
//!
//! This module provides JavaScript-friendly interfaces to the prime generation engine

#![cfg(target_arch = "wasm32")]

use crate::{
    is_prime as check_prime,
    membrane::{MembraneBuilder, MembraneConfig},
    tui::LagrangeUIState,
};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WasmMembraneEngine {
    base: u32,
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
}

#[wasm_bindgen]
impl WasmMembraneEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(base: u32, outer: u32, inner: u32, k_outer: u32, k_inner: u32) -> Self {
        Self {
            base,
            outer,
            inner,
            k_outer,
            k_inner,
        }
    }

    /// Generate a prime candidate from a seed
    #[wasm_bindgen]
    pub fn generate(&self, seed: u8) -> WasmGenerationResult {
        let config = MembraneConfig::new(
            self.base,
            self.outer,
            self.inner,
            self.k_outer,
            self.k_inner,
        );

        match MembraneBuilder::new(config).with_seed(seed).build() {
            Ok(particle) => {
                let number_str = particle.value.to_string();
                let is_prime = check_prime(&particle.value);

                WasmGenerationResult {
                    success: true,
                    number: number_str,
                    is_prime,
                    error: String::new(),
                }
            }
            Err(e) => WasmGenerationResult {
                success: false,
                number: String::new(),
                is_prime: false,
                error: e.to_string(),
            },
        }
    }

    /// Test multiple seeds and return statistics
    #[wasm_bindgen]
    pub fn test_seeds(&self, start: u8, count: u8) -> String {
        let mut results = TestResults {
            total: 0,
            primes: 0,
            success_rate: 0.0,
            numbers: Vec::new(),
        };

        for i in 0..count {
            let seed = start + i;
            let result = self.generate(seed);

            if result.success {
                results.total += 1;
                if result.is_prime {
                    results.primes += 1;
                }
                results.numbers.push((seed, result.number, result.is_prime));
            }
        }

        if results.total > 0 {
            results.success_rate = (results.primes as f64 / results.total as f64) * 100.0;
        }

        serde_json::to_string(&results).unwrap_or_else(|_| "{}".to_string())
    }

    /// Check if boundary digits are coprime to base
    #[wasm_bindgen]
    pub fn check_coprimality(&self) -> String {
        let gcd_outer = gcd(self.outer, self.base);
        let gcd_inner = gcd(self.inner, self.base);

        let result = CoprimalityCheck {
            outer_coprime: gcd_outer == 1,
            inner_coprime: gcd_inner == 1,
            gcd_outer,
            gcd_inner,
            valid: gcd_outer == 1 && gcd_inner == 1,
        };

        serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WasmGenerationResult {
    #[wasm_bindgen(getter_with_clone)]
    pub success: bool,
    #[wasm_bindgen(getter_with_clone)]
    pub number: String,
    #[wasm_bindgen(getter_with_clone)]
    pub is_prime: bool,
    #[wasm_bindgen(getter_with_clone)]
    pub error: String,
}

#[derive(Serialize, Deserialize)]
struct TestResults {
    total: usize,
    primes: usize,
    success_rate: f64,
    numbers: Vec<(u8, String, bool)>,
}

#[derive(Serialize, Deserialize)]
struct CoprimalityCheck {
    outer_coprime: bool,
    inner_coprime: bool,
    gcd_outer: u32,
    gcd_inner: u32,
    valid: bool,
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Get best known configurations for a base
#[wasm_bindgen]
pub fn get_best_configs(base: u32) -> String {
    let configs = match base {
        6 => vec![
            BestConfig {
                outer: 1,
                inner: 5,
                k_outer: 0,
                k_inner: 0,
                success_rate: 33.0,
            },
            BestConfig {
                outer: 5,
                inner: 1,
                k_outer: 0,
                k_inner: 0,
                success_rate: 31.0,
            },
        ],
        10 => vec![
            BestConfig {
                outer: 3,
                inner: 7,
                k_outer: 0,
                k_inner: 0,
                success_rate: 20.0,
            },
            BestConfig {
                outer: 7,
                inner: 3,
                k_outer: 0,
                k_inner: 0,
                success_rate: 19.5,
            },
        ],
        12 => vec![
            BestConfig {
                outer: 5,
                inner: 7,
                k_outer: 0,
                k_inner: 0,
                success_rate: 25.0,
            },
            BestConfig {
                outer: 7,
                inner: 5,
                k_outer: 0,
                k_inner: 0,
                success_rate: 24.5,
            },
        ],
        _ => vec![BestConfig {
            outer: 1,
            inner: base - 1,
            k_outer: 0,
            k_inner: 0,
            success_rate: 15.0,
        }],
    };

    serde_json::to_string(&configs).unwrap_or_else(|_| "[]".to_string())
}

#[derive(Serialize, Deserialize)]
struct BestConfig {
    outer: u32,
    inner: u32,
    k_outer: u32,
    k_inner: u32,
    success_rate: f64,
}

/// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Convert a string to JavaScript BigInt
/// This allows proper handling of large prime numbers in JavaScript
#[wasm_bindgen]
pub fn string_to_bigint(s: &str) -> Result<JsValue, JsValue> {
    // Parse the string as BigUint first to validate
    match s.parse::<BigUint>() {
        Ok(_) => {
            // Use JavaScript's BigInt constructor
            let js_bigint = js_sys::Function::new_with_args("n", &format!("return BigInt('{s}'))"));
            js_bigint.call0(&JsValue::null())
        }
        Err(e) => Err(JsValue::from_str(&format!("Invalid number: {e}"))),
    }
}

/// Check if a JavaScript BigInt is prime
#[wasm_bindgen]
pub fn is_bigint_prime(bigint_str: &str) -> Result<bool, JsValue> {
    match bigint_str.parse::<BigUint>() {
        Ok(n) => Ok(check_prime(&n)),
        Err(e) => Err(JsValue::from_str(&format!("Invalid BigInt string: {e}"))),
    }
}

/// Generate multiple primes and return as array of BigInt strings
#[wasm_bindgen]
pub fn generate_primes_batch(
    base: u32,
    outer: u32,
    inner: u32,
    count: u32,
) -> Result<Vec<JsValue>, JsValue> {
    let config = MembraneConfig::new(base, outer, inner, 0, 0);
    let mut primes = Vec::new();

    for seed in 0..count {
        if let Ok(particle) = MembraneBuilder::new(config.clone())
            .with_seed((seed % 256) as u8)
            .build()
        {
            if check_prime(&particle.value) {
                let prime_str = particle.value.to_string();
                if let Ok(bigint) = string_to_bigint(&prime_str) {
                    primes.push(bigint);
                }
            }
        }
    }

    Ok(primes)
}

// ========== TUI WASM Bindings ==========

/// WASM wrapper for the Lagrange TUI state
#[wasm_bindgen]
pub struct WasmLagrangeUI {
    state: LagrangeUIState,
}

#[wasm_bindgen]
impl WasmLagrangeUI {
    /// Create a new TUI instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            state: LagrangeUIState::default(),
        }
    }

    /// Get the current state as a JavaScript object
    #[wasm_bindgen(js_name = getState)]
    pub fn get_state(&self) -> Result<JsValue, JsValue> {
        to_value(&self.state).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Generate a new prime pair
    #[wasm_bindgen(js_name = generatePrimePair)]
    pub fn generate_prime_pair(&mut self) {
        self.state.generate_prime_pair();
    }

    /// Test Lagrange points for primality
    #[wasm_bindgen(js_name = testLagrangePoints)]
    pub fn test_lagrange_points(&mut self) {
        self.state.test_lagrange_points();
    }

    /// Cycle through configurations
    #[wasm_bindgen(js_name = cycleConfiguration)]
    pub fn cycle_configuration(&mut self) {
        self.state.cycle_configuration();
    }

    /// Toggle help display
    #[wasm_bindgen(js_name = toggleHelp)]
    pub fn toggle_help(&mut self) {
        self.state.show_help = !self.state.show_help;
    }

    /// Select a prime (0 or 1)
    #[wasm_bindgen(js_name = selectPrime)]
    pub fn select_prime(&mut self, index: usize) {
        if index <= 1 {
            self.state.selected_prime = index;
        }
    }

    /// Get the status message
    #[wasm_bindgen(js_name = getStatusMessage)]
    pub fn get_status_message(&self) -> String {
        self.state.status_message.clone()
    }

    /// Get the current configuration string
    #[wasm_bindgen(js_name = getConfigString)]
    pub fn get_config_string(&self) -> String {
        format!(
            "({},{}) k=({},{}) b{}",
            self.state.config.outer,
            self.state.config.inner,
            self.state.config.k_outer,
            self.state.config.k_inner,
            self.state.config.base
        )
    }

    /// Render the state to text (for debugging)
    #[wasm_bindgen(js_name = renderToText)]
    pub fn render_to_text(&self) -> String {
        crate::tui::render_to_text(&self.state)
    }
}
