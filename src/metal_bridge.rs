//! FFI bridge to Metal GPU acceleration
//! 
//! This module provides a safe Rust interface to Metal GPU acceleration for prime number
//! sieving. It handles platform-specific compilation and provides graceful fallbacks
//! for non-macOS platforms.
//!
//! # Safety
//! 
//! This module uses FFI to communicate with Metal APIs which are only available on macOS.
//! All unsafe operations are contained within this module and exposed through safe APIs.
//! 
//! # Platform Support
//! 
//! - **macOS**: Full Metal GPU acceleration support
//! - **Other platforms**: Graceful fallback with None/error returns
//!
//! # Example
//! 
//! ```no_run
//! use prime_physics_engine::metal_bridge::{MetalSieve, MetalConfig};
//! 
//! // Attempt to create a Metal sieve (returns None on non-macOS)
//! if let Some(sieve) = MetalSieve::new() {
//!     let config = MetalConfig {
//!         base: 6,
//!         width: 3,
//!         l_digit: 1,
//!         r_digit: 5,
//!         r1: 0,
//!         r2: 0,
//!     };
//!     
//!     let candidates = vec![1, 2, 3, 4, 5];
//!     if let Some((survivors, metrics)) = sieve.sieve(&candidates, config, false) {
//!         println!("Found {} survivors", survivors.len());
//!         println!("Throughput: {:.2} candidates/sec", metrics.throughput);
//!     }
//! } else {
//!     println!("Metal GPU acceleration not available on this platform");
//! }
//! ```

use std::os::raw::{c_uint, c_void};

/// Configuration for Metal GPU membrane sieving
/// 
/// This struct is marked `#[repr(C)]` to ensure consistent memory layout
/// when passed through FFI to the Metal implementation.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MetalConfig {
    /// Base of the number system (e.g., 6, 12, 30)
    pub base: u32,
    /// Width of the membrane structure
    pub width: u32, 
    /// Left boundary digit (must be coprime to base for optimal performance)
    pub l_digit: u32,
    /// Right boundary digit (must be coprime to base for optimal performance)
    pub r_digit: u32,
    /// Left padding (number of zeros)
    pub r1: u32,
    /// Right padding (number of zeros)
    pub r2: u32,
}

/// Performance metrics from Metal GPU execution
/// 
/// This struct is marked `#[repr(C)]` to ensure consistent memory layout
/// when populated by the Metal implementation through FFI.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MetalMetrics {
    /// Total number of candidates tested
    pub candidates_tested: u32,
    /// Number of candidates that survived the sieve
    pub survivors_found: u32,
    /// Elapsed time in milliseconds
    pub elapsed_ms: u32,
    /// Throughput in candidates per second
    pub throughput: f64,
    /// Survival rate as a percentage (0.0-100.0)
    pub survival_rate: f64,
    /// Number of cache misses (-1 if not measured)
    pub cache_misses: i32,
    /// Number of coalesced memory loads (-1 if not measured)
    pub coalesced_loads: i32,
}

#[cfg(target_os = "macos")]
extern "C" {
    fn metal_sieve_create() -> *mut c_void;
    fn metal_sieve_destroy(sieve: *mut c_void);
    fn metal_sieve_run(
        sieve: *mut c_void,
        candidates: *const u32,
        count: u32,
        config: *const MetalConfig,
        instrumented: bool,
        survivors: *mut u32,
        max_survivors: u32,
        metrics: *mut MetalMetrics,
    ) -> u32;  // Returns actual survivor count
}

/// Safe wrapper for Metal GPU sieve operations
/// 
/// This struct provides a safe Rust interface to the Metal GPU implementation.
/// It automatically handles platform detection and provides graceful fallbacks
/// for non-macOS platforms.
/// 
/// # Thread Safety
/// 
/// The Metal implementation is thread-safe at the GPU level, but the Rust
/// wrapper should not be shared between threads without proper synchronization.
pub struct MetalSieve {
    #[cfg(target_os = "macos")]
    handle: *mut c_void,
}

// Mark as Send but not Sync - can be moved between threads but not shared
unsafe impl Send for MetalSieve {}

impl MetalSieve {
    /// Create a new Metal GPU sieve instance
    /// 
    /// Returns `Some(MetalSieve)` on macOS with Metal support, `None` otherwise.
    /// 
    /// # Errors
    /// 
    /// Returns `None` if:
    /// - Running on a non-macOS platform
    /// - Metal is not available on the system
    /// - GPU initialization fails
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use prime_physics_engine::metal_bridge::MetalSieve;
    /// 
    /// match MetalSieve::new() {
    ///     Some(sieve) => println!("Metal GPU acceleration available"),
    ///     None => println!("Metal GPU not available, using CPU fallback"),
    /// }
    /// ```
    #[cfg(target_os = "macos")]
    pub fn new() -> Option<Self> {
        // SAFETY: metal_sieve_create is an external C function that either:
        // 1. Returns a valid pointer to a Metal sieve instance
        // 2. Returns null if initialization fails
        // We check for null and convert to Option appropriately
        unsafe {
            let handle = metal_sieve_create();
            if handle.is_null() {
                None
            } else {
                Some(MetalSieve { handle })
            }
        }
    }
    
    /// Stub implementation for non-macOS platforms
    /// 
    /// Always returns `None` to indicate Metal is not available.
    #[cfg(not(target_os = "macos"))]
    pub fn new() -> Option<Self> {
        None  // Metal only available on macOS
    }
    
    /// Run membrane sieve on GPU with the given candidates
    /// 
    /// # Arguments
    /// 
    /// * `candidates` - Array of candidate values to test
    /// * `config` - Membrane configuration parameters
    /// * `instrumented` - Enable performance instrumentation
    /// 
    /// # Returns
    /// 
    /// Returns `Some((survivors, metrics))` on success, where:
    /// - `survivors` - Indices of candidates that passed the sieve
    /// - `metrics` - Performance metrics from the GPU execution
    /// 
    /// Returns `None` if:
    /// - The input slice is empty
    /// - Memory allocation fails
    /// - GPU execution fails
    /// 
    /// # Safety
    /// 
    /// This function is safe to call. All unsafe operations are contained within
    /// and properly validated:
    /// - Input validation ensures non-empty candidates
    /// - Buffer sizes are calculated to prevent overflows
    /// - GPU results are bounds-checked before copying
    #[cfg(target_os = "macos")]
    pub fn sieve(&self, candidates: &[u32], config: MetalConfig, instrumented: bool) 
        -> Option<(Vec<u32>, MetalMetrics)> {
        
        // Input validation
        if candidates.is_empty() {
            return None;
        }
        
        // Ensure we don't overflow u32
        let len = candidates.len();
        if len > u32::MAX as usize {
            return None;
        }
        
        let max_survivors = len as u32;
        let mut survivors = vec![0u32; len];
        let mut metrics = MetalMetrics {
            candidates_tested: 0,
            survivors_found: 0,
            elapsed_ms: 0,
            throughput: 0.0,
            survival_rate: 0.0,
            cache_misses: -1,
            coalesced_loads: -1,
        };
        
        // SAFETY: 
        // 1. self.handle is guaranteed valid by constructor
        // 2. candidates.as_ptr() is valid for len elements
        // 3. survivors.as_mut_ptr() is valid for max_survivors elements
        // 4. config and metrics are stack-allocated and valid
        // 5. metal_sieve_run returns count <= max_survivors
        unsafe {
            let survivor_count = metal_sieve_run(
                self.handle,
                candidates.as_ptr(),
                len as u32,
                &config,
                instrumented,
                survivors.as_mut_ptr(),
                max_survivors,
                &mut metrics,
            );
            
            // Validate survivor count
            if survivor_count > max_survivors {
                // GPU returned invalid count, fail safely
                return None;
            }
            
            survivors.truncate(survivor_count as usize);
            Some((survivors, metrics))
        }
    }
    
    /// Stub implementation for non-macOS platforms
    /// 
    /// Always returns `None` to indicate Metal is not available.
    #[cfg(not(target_os = "macos"))]
    pub fn sieve(&self, _candidates: &[u32], _config: MetalConfig, _instrumented: bool) 
        -> Option<(Vec<u32>, MetalMetrics)> {
        None
    }
}

/// Clean up Metal resources when the sieve is dropped
/// 
/// This ensures proper cleanup of GPU resources.
#[cfg(target_os = "macos")]
impl Drop for MetalSieve {
    fn drop(&mut self) {
        // SAFETY: self.handle is guaranteed valid by constructor,
        // and this is the only place it's destroyed
        unsafe {
            metal_sieve_destroy(self.handle);
        }
    }
}

/// No-op drop implementation for non-macOS platforms
#[cfg(not(target_os = "macos"))]
impl Drop for MetalSieve {
    fn drop(&mut self) {
        // Nothing to clean up on non-macOS platforms
    }
}