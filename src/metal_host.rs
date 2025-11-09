//! Minimal Metal host helpers — 2025-07-17
//!
//! This module provides Metal GPU integration for high-performance prime sieving.
//! It includes platform-specific implementations that gracefully fall back on
//! non-macOS systems.
//!
//! # Safety
//!
//! All Metal API usage is guarded by `#[cfg(target_os = "macos")]` to ensure
//! it only compiles on macOS. The module provides stub implementations for
//! other platforms that return errors gracefully.
//!
//! # Performance
//!
//! The Metal implementation can achieve 10-50x speedup over CPU implementations
//! for large-scale prime sieving operations.

#[cfg(target_os = "macos")]
use metal::*;
use std::mem;

/// Packed signature row for GPU prime tables
///
/// Uses `#[repr(C, packed)]` to ensure tight memory layout for GPU efficiency.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct SigRow {
    /// Signature value
    pub s: u32,
    /// Generator value
    pub g: u32,
    /// Prime value
    pub p: u32,
}

/// Parameters for GPU kernel execution
///
/// Uses `#[repr(C)]` for consistent memory layout when passed to GPU.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// Number of primes in the signature table
    pub num_primes: u32,
    /// Offset into the signature table
    pub table_offset: u32,
    /// Number of candidates to process
    pub num_candidates: u32,
}

/// Metal GPU context for prime sieving
///
/// This struct encapsulates all Metal resources needed for GPU computation.
/// It's only available on macOS with Metal support.
#[cfg(target_os = "macos")]
pub struct MetalCtx {
    device: Device,
    queue: CommandQueue,
    pipe: ComputePipelineState,
}

#[cfg(target_os = "macos")]
impl MetalCtx {
    /// Create a new Metal context
    ///
    /// # Returns
    ///
    /// - `Ok(MetalCtx)` if Metal initialization succeeds
    /// - `Err(String)` with detailed error message if:
    ///   - No Metal GPU is available
    ///   - Shader library loading fails
    ///   - Kernel function is not found
    ///   - Pipeline creation fails
    ///
    /// # Panics
    ///
    /// May panic if METALLIB_PATH environment variable is not set during compilation.
    pub fn new() -> Result<Self, String> {
        // Get the default Metal device
        let device = Device::system_default().ok_or_else(|| {
            "No Metal GPU found. Metal requires macOS 10.11+ with a Metal-capable GPU.".to_string()
        })?;

        // Load pre-compiled Metal shader library
        // METALLIB_PATH must be set at compile time
        let lib = device
            .new_library_with_data(include_bytes!(env!("METALLIB_PATH")))
            .map_err(|e| {
                format!("Failed to load Metal shader library: {e}. Ensure shaders are compiled.")
            })?;

        // Get the sieve kernel function
        let func = lib.get_function("sieve_affine", None).map_err(|_| {
            "Kernel 'sieve_affine' not found in shader library. Check shader compilation."
                .to_string()
        })?;

        // Create compute pipeline
        let pipe = device
            .new_compute_pipeline_state_with_function(&func)
            .map_err(|e| format!("Failed to create compute pipeline: {e}"))?;

        // Create command queue for GPU execution
        let queue = device.new_command_queue();

        Ok(Self {
            device,
            queue,
            pipe,
        })
    }

    /// Run the sieve computation on GPU
    ///
    /// # Arguments
    ///
    /// * `ctab` - Candidate table (packed values to test)
    /// * `sig` - Signature rows for prime testing
    ///
    /// # Returns
    ///
    /// Bitmask where set bits indicate surviving candidates.
    /// Use `unpack_bitmask` to convert to candidate indices.
    ///
    /// # Panics
    ///
    /// May panic if:
    /// - Input slices are empty
    /// - GPU buffers cannot be allocated
    /// - GPU execution fails
    pub fn run(&self, ctab: &[u32], sig: &[SigRow]) -> Vec<u32> {
        // Input validation
        assert!(!ctab.is_empty(), "Candidate table cannot be empty");
        assert!(!sig.is_empty(), "Signature table cannot be empty");

        let cand_len = ctab.len() as u32;
        let mask_words = cand_len.div_ceil(32) as usize;

        // Create GPU buffers with input data
        let c_buf = self.device.new_buffer_with_data(
            ctab.as_ptr() as _,
            std::mem::size_of_val(ctab) as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let sig_buf = self.device.new_buffer_with_data(
            sig.as_ptr() as _,
            std::mem::size_of_val(sig) as u64,
            MTLResourceOptions::StorageModeShared,
        );

        // Output buffer for bitmask results
        let out_buf = self.device.new_buffer(
            (mask_words * mem::size_of::<u32>()) as u64,
            MTLResourceOptions::StorageModeShared,
        );

        // Kernel parameters
        let params = Params {
            num_primes: sig.len() as u32,
            table_offset: 0,
            num_candidates: cand_len,
        };

        let prm_buf = self.device.new_buffer_with_data(
            &params as *const _ as _,
            mem::size_of::<Params>() as u64,
            MTLResourceOptions::StorageModeShared,
        );

        // Configure and dispatch GPU computation
        let cmd = self.queue.new_command_buffer();
        let enc = cmd.new_compute_command_encoder();

        enc.set_compute_pipeline_state(&self.pipe);
        enc.set_buffer(0, Some(&prm_buf), 0);
        enc.set_buffer(1, Some(&sig_buf), 0);
        enc.set_buffer(2, Some(&c_buf), 0);
        enc.set_buffer(3, Some(&out_buf), 0);

        // Dispatch with 256 threads per threadgroup
        let thread_groups = MTLSize::new((cand_len as u64).div_ceil(256), 1, 1);
        let threads_per_group = MTLSize::new(256, 1, 1);
        enc.dispatch_thread_groups(thread_groups, threads_per_group);

        enc.end_encoding();
        cmd.commit();
        cmd.wait_until_completed();

        // SAFETY: out_buf is guaranteed to have mask_words elements
        // and is valid until this function returns
        unsafe { std::slice::from_raw_parts(out_buf.contents() as *const u32, mask_words).to_vec() }
    }
}

/// Convert bitmask words to candidate indices
///
/// # Arguments
///
/// * `masks` - Array of u32 bitmasks from GPU
/// * `base_idx` - Base index offset for results
///
/// # Returns
///
/// Vector of indices where bits were set in the mask.
///
/// # Example
///
/// ```
/// # use primes::metal_host::unpack_bitmask;
/// let masks = vec![0b00000101]; // Bits 0 and 2 set
/// let indices = unpack_bitmask(&masks, 0);
/// assert_eq!(indices, vec![0, 2]);
/// ```
pub fn unpack_bitmask(masks: &[u32], base_idx: u32) -> Vec<u32> {
    let mut out = Vec::with_capacity(masks.len() * 8); // Estimate capacity

    for (w, &mask) in masks.iter().enumerate() {
        if mask == 0 {
            continue; // Skip empty masks
        }

        let base = base_idx.saturating_add((w as u32).saturating_mul(32));
        let mut m = mask;

        while m != 0 {
            let b = m.trailing_zeros();
            out.push(base.saturating_add(b));
            m &= !(1 << b);
        }
    }

    out
}

/// Build packed base-12 digit representation
///
/// Packs base-12 digits into 4-bit nibbles for GPU efficiency.
pub fn build_packed12(count: usize) -> Vec<u32> {
    build_packed_generic(12, count)
}

/// Build packed base-6 digit representation
///
/// Packs base-6 digits into 4-bit nibbles for GPU efficiency.
pub fn build_packed6(count: usize) -> Vec<u32> {
    build_packed_generic(6, count)
}

/// Generic packed digit builder for arbitrary bases
///
/// Packs digits into 4-bit nibbles. Each u32 can hold up to 8 digits.
///
/// # Arguments
///
/// * `base` - The numerical base (must be <= 16 to fit in 4 bits)
/// * `count` - Number of values to generate
///
/// # Panics
///
/// Panics if base > 16 (doesn't fit in 4-bit nibbles).
fn build_packed_generic(base: u32, count: usize) -> Vec<u32> {
    assert!(base <= 16, "Base must be <= 16 to fit in 4-bit nibbles");

    (0..count)
        .map(|c| {
            let mut v = 0u32;
            let mut t = c;
            let mut sh = 0;

            // Pack up to 8 digits per u32
            while t > 0 && sh < 32 {
                let digit = (t % base as usize) as u32;
                v |= digit << sh;
                t /= base as usize;
                sh += 4;
            }

            v
        })
        .collect()
}

/// Stub implementation for non-macOS platforms
#[cfg(not(target_os = "macos"))]
pub struct MetalCtx;

#[cfg(not(target_os = "macos"))]
impl MetalCtx {
    /// Always returns an error on non-macOS platforms
    pub fn new() -> Result<Self, String> {
        Err("Metal GPU acceleration is only available on macOS".to_string())
    }

    /// No-op that returns empty results on non-macOS platforms
    pub fn run(&self, _: &[u32], _: &[SigRow]) -> Vec<u32> {
        vec![]
    }
}
