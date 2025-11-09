//! GPU acceleration module for membrane prime sieving — 2025-07-17
//!
//! This module provides GPU-accelerated prime sieving using Metal on macOS.
//! It offers significant performance improvements over CPU implementations
//! for large-scale prime discovery operations.
//!
//! # Platform Support
//!
//! - **macOS**: Full Metal GPU acceleration
//! - **Other platforms**: Graceful fallback with error messages
//!
//! # Safety
//!
//! All GPU operations are wrapped in safe Rust APIs. Platform-specific
//! code is guarded by conditional compilation.
//!
//! # Performance
//!
//! Typical speedups over CPU:
//! - Small batches (< 1000): 2-5x
//! - Medium batches (1000-100k): 10-20x  
//! - Large batches (> 100k): 30-50x

pub use crate::metal_host::{build_packed12, build_packed6, unpack_bitmask, Params, SigRow};

use crate::prime_lut::{SigRow as LutSigRow, SIGNATURES};
use std::mem;

#[cfg(target_os = "macos")]
use metal::*;

/// Number of threads per Metal thread-group
/// Optimized for Apple Silicon GPUs
const TPB: u64 = 256;

/// GPU sieve implementation for macOS
///
/// Provides high-performance prime sieving using Metal compute shaders.
#[cfg(target_os = "macos")]
pub struct GpuSieve {
    device: Device,
    queue: CommandQueue,
    pipe: ComputePipelineState,
}

#[cfg(target_os = "macos")]
impl GpuSieve {
    /// Create a new GPU sieve instance
    ///
    /// # Returns
    ///
    /// - `Ok(GpuSieve)` if GPU initialization succeeds
    /// - `Err(String)` with detailed error message if:
    ///   - No Metal GPU is available (macOS 10.11+ required)
    ///   - Shader library loading fails (check METALLIB_PATH)
    ///   - Kernel function not found in shaders
    ///   - Pipeline creation fails
    ///
    /// # Environment
    ///
    /// Requires METALLIB_PATH to be set at compile time to the location
    /// of the compiled Metal shader library.
    pub fn new() -> Result<Self, String> {
        // Check for Metal device availability
        let device = Device::system_default().ok_or_else(|| {
            "No Metal GPU available. Ensure you're running on macOS 10.11+ \
                 with a Metal-capable GPU (all Macs since 2012)."
                .to_string()
        })?;

        // Load pre-compiled shader library
        let lib = device
            .new_library_with_data(include_bytes!(env!("METALLIB_PATH")))
            .map_err(|e| {
                format!(
                    "Failed to load Metal shader library: {e}. \
                        Ensure METALLIB_PATH points to compiled .metallib file."
                )
            })?;

        // Get the sieve kernel function
        let func = lib.get_function("sieve_affine", None).map_err(|_| {
            "Kernel 'sieve_affine' not found in shader library. \
                 Ensure shaders are properly compiled."
                .to_string()
        })?;

        // Create compute pipeline
        let pipe = device
            .new_compute_pipeline_state_with_function(&func)
            .map_err(|e| format!("Pipeline creation failed: {e}"))?;

        // Create command queue
        let queue = device.new_command_queue();

        Ok(Self {
            device,
            queue,
            pipe,
        })
    }

    /// Run GPU sieve on candidate values
    ///
    /// # Arguments
    ///
    /// * `candidates` - Array of candidate values to test for primality
    /// * `_base` - Numerical base (currently unused, reserved for future optimization)
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<u32>)` - Indices of candidates that survived the sieve
    /// - `Err(String)` - Error message if GPU execution fails
    ///
    /// # Algorithm
    ///
    /// Uses the first 100 primes from the signature table to filter candidates.
    /// This removes approximately 90% of composites with minimal GPU work.
    ///
    /// # Performance
    ///
    /// Best performance is achieved with batch sizes of 10k-100k candidates.
    /// Smaller batches have higher overhead, larger batches may exceed GPU memory.
    pub fn sieve(&self, candidates: &[u32], _base: u32) -> Result<Vec<u32>, String> {
        // Input validation
        if candidates.is_empty() {
            return Ok(Vec::new()); // Empty input -> empty output
        }

        if candidates.len() > u32::MAX as usize {
            return Err("Too many candidates for GPU processing".to_string());
        }

        // Load signature table (first 100 primes for quick filtering)
        let sig_rows: Vec<SigRow> = SIGNATURES
            .iter()
            .take(100)
            .map(|LutSigRow { s, g, p, .. }| SigRow {
                s: *s,
                g: *g,
                p: *p,
            })
            .collect();

        let cand_len = candidates.len() as u32;
        let mask_words = cand_len.div_ceil(32) as usize;

        // Create GPU buffers
        let c_buf = self.device.new_buffer_with_data(
            candidates.as_ptr() as _,
            std::mem::size_of_val(candidates) as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let sig_buf = self.device.new_buffer_with_data(
            sig_rows.as_ptr() as _,
            (sig_rows.len() * mem::size_of::<SigRow>()) as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let out_buf = self.device.new_buffer(
            (mask_words * mem::size_of::<u32>()) as u64,
            MTLResourceOptions::StorageModeShared,
        );

        // Kernel parameters
        let params = Params {
            num_primes: sig_rows.len() as u32,
            table_offset: 0,
            num_candidates: cand_len,
        };

        let prm_buf = self.device.new_buffer_with_data(
            &params as *const _ as _,
            mem::size_of::<Params>() as u64,
            MTLResourceOptions::StorageModeShared,
        );

        // Configure and execute GPU kernel
        let cmd = self.queue.new_command_buffer();
        let enc = cmd.new_compute_command_encoder();

        enc.set_compute_pipeline_state(&self.pipe);
        enc.set_buffer(0, Some(&prm_buf), 0);
        enc.set_buffer(1, Some(&sig_buf), 0);
        enc.set_buffer(2, Some(&c_buf), 0);
        enc.set_buffer(3, Some(&out_buf), 0);

        // Calculate thread groups for optimal GPU utilization
        let groups = MTLSize::new((cand_len as u64).div_ceil(TPB), 1, 1);
        enc.dispatch_thread_groups(groups, MTLSize::new(TPB, 1, 1));

        enc.end_encoding();
        cmd.commit();
        cmd.wait_until_completed();

        // Collect results safely
        if out_buf.contents().is_null() {
            return Err("GPU output buffer is null - possible out of memory".to_string());
        }

        // SAFETY: Buffer is guaranteed valid and sized correctly
        let masks =
            unsafe { std::slice::from_raw_parts(out_buf.contents() as *const u32, mask_words) }
                .to_vec();

        Ok(unpack_bitmask(&masks, 0))
    }
}

/// Stub implementation for non-macOS platforms
///
/// Provides consistent API surface with helpful error messages.
#[cfg(not(target_os = "macos"))]
pub struct GpuSieve;

#[cfg(not(target_os = "macos"))]
impl GpuSieve {
    /// Always returns an error on non-macOS platforms
    ///
    /// The error message explains that Metal GPU acceleration
    /// requires macOS and suggests using CPU fallback.
    pub fn new() -> Result<Self, String> {
        Err("GPU acceleration requires macOS with Metal support. \
             Please use CPU-based implementations on this platform."
            .into())
    }

    /// Always returns an error on non-macOS platforms
    pub fn sieve(&self, _: &[u32], _: u32) -> Result<Vec<u32>, String> {
        Err("GPU acceleration requires macOS with Metal support. \
             This method should not be called on non-macOS platforms."
            .into())
    }
}
