//! Optimized GPU implementation with all performance improvements
//! 
//! Note: This module requires manually compiled Metal shaders and is currently
//! experimental. Use the CPU implementations for production code.

#[cfg(feature = "metal")]
use crate::prime_lut_recip::{SigRowRecip, generate_signatures_with_reciprocals};

#[cfg(feature = "metal")]
use std::mem;

#[cfg(all(feature = "metal", target_os = "macos"))]
use metal::*;

#[cfg(feature = "metal")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OptimizedParams {
    pub num_primes: u32,
    pub num_candidates: u32,
    pub base: u32,
    pub l: u32,      // left boundary
    pub r: u32,      // right boundary  
    pub width: u32,  // membrane width
}

#[cfg(feature = "metal")]
const TPB: u64 = 256; // threads per block

#[cfg(all(feature = "metal", target_os = "macos"))]
pub struct GpuSieveOptimized {
    device: Device,
    queue: CommandQueue,
    pipe: ComputePipelineState,
}

#[cfg(all(feature = "metal", target_os = "macos"))]
impl GpuSieveOptimized {
    pub fn new() -> Result<Self, String> {
        let device = Device::system_default().ok_or("No Metal GPU available")?;
        
        // Load optimized shader
        let lib = device
            .new_library_with_data(include_bytes!(env!("METALLIB_OPTIMIZED_PATH")))
            .map_err(|e| format!("Failed to load optimized metallib: {e}"))?;
            
        let func = lib
            .get_function("sieve_optimized", None)
            .map_err(|_| "Kernel sieve_optimized not found".to_string())?;
            
        let pipe = device
            .new_compute_pipeline_state_with_function(&func)
            .map_err(|e| format!("Pipeline creation failed: {e}"))?;
            
        let queue = device.new_command_queue();
        
        Ok(Self { device, queue, pipe })
    }
    
    pub fn sieve_direct(&self, base: u32, l: u32, r: u32, width: u32, count: u32) -> Result<Vec<u32>, String> {
        // Generate signatures with reciprocals
        let sig_rows = generate_signatures_with_reciprocals(base, l, r, width);
        let num_primes = sig_rows.len().min(100) as u32;
        
        // Parameters
        let params = OptimizedParams {
            num_primes,
            num_candidates: count,
            base,
            l,
            r,
            width,
        };
        
        // Allocate output buffer
        let mask_words = ((count + 31) / 32) as usize;
        
        // Create Metal buffers
        let sig_buf = self.device.new_buffer_with_data(
            sig_rows.as_ptr() as _,
            (sig_rows.len() * mem::size_of::<SigRowRecip>()) as u64,
            MTLResourceOptions::StorageModeShared,
        );
        
        let out_buf = self.device.new_buffer(
            (mask_words * mem::size_of::<u32>()) as u64,
            MTLResourceOptions::StorageModeShared,
        );
        
        let prm_buf = self.device.new_buffer_with_data(
            &params as *const _ as _,
            mem::size_of::<OptimizedParams>() as u64,
            MTLResourceOptions::StorageModeShared,
        );
        
        // Clear output buffer
        if out_buf.contents().is_null() {
            return Err("GPU output buffer is null".to_string());
        }
        
        unsafe {
            // Safe because we checked for null above
            let ptr = out_buf.contents() as *mut u32;
            std::ptr::write_bytes(ptr, 0, mask_words);
        }
        
        // Dispatch kernel
        let cmd = self.queue.new_command_buffer();
        let enc = cmd.new_compute_command_encoder();
        
        enc.set_compute_pipeline_state(&self.pipe);
        enc.set_buffer(0, Some(&prm_buf), 0);
        enc.set_buffer(1, Some(&sig_buf), 0);
        enc.set_buffer(2, Some(&out_buf), 0);
        
        let groups = MTLSize::new((count as u64 + TPB - 1) / TPB, 1, 1);
        enc.dispatch_thread_groups(groups, MTLSize::new(TPB, 1, 1));
        
        enc.end_encoding();
        cmd.commit();
        cmd.wait_until_completed();
        
        // Collect results
        if out_buf.contents().is_null() {
            return Err("GPU output buffer is null after computation".to_string());
        }
        
        let masks = unsafe {
            // Safe because we checked for null above
            std::slice::from_raw_parts(out_buf.contents() as *const u32, mask_words)
        }.to_vec();
        
        // Unpack bitmask to indices
        let mut survivors = Vec::new();
        for (w, &mask) in masks.iter().enumerate() {
            let base_idx = (w * 32) as u32;
            let mut m = mask;
            while m != 0 {
                let b = m.trailing_zeros();
                let idx = base_idx + b;
                if idx < count {
                    survivors.push(idx);
                }
                m &= !(1 << b);
            }
        }
        
        Ok(survivors)
    }
}

// Non-macOS stub
#[cfg(not(target_os = "macos"))]
pub struct GpuSieveOptimized;

#[cfg(not(target_os = "macos"))]
impl GpuSieveOptimized {
    pub fn new() -> Result<Self, String> {
        Err("GPU acceleration is macOS-only".into())
    }
    
    pub fn sieve_direct(&self, _: u32, _: u32, _: u32, _: u32, _: u32) -> Result<Vec<u32>, String> {
        Err("GPU acceleration is macOS-only".into())
    }
}