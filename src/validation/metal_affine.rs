//! Metal-backed affine residue sieving with zero candidate-value transfer.
//!
//! This module is intentionally narrow and maintained: it consumes the same
//! [`FastAffineLane`] used by
//! the deterministic CPU fast path, sends only small residue rows to Metal, and
//! writes back a compact survivor bitmask. Full candidate values are
//! reconstructed on the CPU only for surviving seeds.
//!
//! The performance-critical affine residue loop lives in the dedicated
//! `shaders/sieve_affine.metal` kernel. Rust's `metal` crate is used here as
//! the host API for pipeline loading, shared buffers, dispatch, and readback.

use crate::validation::{
    bounded_k::DEFAULT_PREFILTER_PRIMES,
    fast_affine::{
        build_fast_affine_lane, FastAffineLane, FastLaneConfig, FastPrimeError, FastPrimeWitness,
    },
};
use serde::Serialize;
use std::{mem, time::Instant};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetalAffineError {
    #[error(transparent)]
    FastPrime(#[from] FastPrimeError),
    #[error("seed offset {seed_offset} is outside finite seed capacity {seed_capacity}")]
    SeedOffsetOutOfRange {
        seed_offset: u64,
        seed_capacity: u64,
    },
    #[error("Metal prototype currently accepts at most u32::MAX seeds per dispatch")]
    SeedCountTooLarge,
    #[error("batch seed count must be at least 1")]
    InvalidBatchSeedCount,
    #[error("expected {expected} residue row batches, got {actual}")]
    ResidueBatchCountMismatch { expected: usize, actual: usize },
    #[error("Metal affine prototype is unavailable; build on macOS with --features metal")]
    MetalUnavailable,
    #[error("Metal initialization failed: {0}")]
    MetalInit(String),
    #[error("Metal execution failed: {0}")]
    MetalExecution(String),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct MetalAffineResidueRow {
    pub a: u32,
    pub g: u32,
    pub p: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct MetalAffineParams {
    num_rows: u32,
    seed_offset: u32,
    num_seeds: u32,
    reserved: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetalAffineTransferMetrics {
    pub storage_mode: String,
    pub zero_candidate_value_transfer: bool,
    pub candidate_value_buffer_transferred: bool,
    pub candidate_count: u64,
    pub residue_row_count: usize,
    pub params_bytes: u64,
    pub residue_row_bytes: u64,
    pub input_metadata_bytes: u64,
    pub output_bitmask_bytes: u64,
    pub total_shared_buffer_bytes: u64,
    pub avoided_candidate_value_bytes_u64: u64,
    pub avoided_candidate_value_bytes_u32: u64,
    pub gpu_sieve_seconds: f64,
    pub cpu_confirm_seconds: f64,
    pub total_seconds: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetalAffineSieveResult {
    pub seed_offset: u64,
    pub seed_count: u64,
    pub survivor_seeds: Vec<u64>,
    pub gpu_sieve_seconds: f64,
    pub output_bitmask_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetalAffineBatchSieveResult {
    pub seed_offset: u64,
    pub seed_count: u64,
    pub batch_seed_count: u64,
    pub batch_count: usize,
    pub survivor_seeds: Vec<u64>,
    pub setup_seconds: f64,
    pub buffer_prepare_seconds: f64,
    pub gpu_sieve_seconds: f64,
    pub unpack_seconds: f64,
    pub total_seconds: f64,
    pub input_metadata_bytes: u64,
    pub output_bitmask_bytes: u64,
    pub total_shared_buffer_bytes: u64,
    pub avoided_candidate_value_bytes_u64: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetalAffinePrimeRun {
    pub config: FastLaneConfig,
    pub pair_label: String,
    pub k_label: String,
    pub shift: u64,
    pub gradient: u64,
    pub seed_offset: u64,
    pub requested_seed_count: u64,
    pub seed_capacity: u64,
    pub scanned_seed_count: u64,
    pub capped_to_seed_capacity: bool,
    pub residue_rows: Vec<MetalAffineResidueRow>,
    pub survivor_seed_count: u64,
    pub primality_tests: u64,
    pub primes_found: u64,
    pub witnesses: Vec<FastPrimeWitness>,
    pub metrics: MetalAffineTransferMetrics,
}

pub fn default_metal_affine_moduli(lane: &FastAffineLane) -> Vec<u32> {
    DEFAULT_PREFILTER_PRIMES
        .iter()
        .copied()
        .filter(|&modulus| gcd_u32(lane.config.base, modulus) == 1)
        .collect()
}

pub fn build_metal_affine_residue_rows(
    lane: &FastAffineLane,
    seed_offset: u64,
    moduli: &[u32],
) -> Result<Vec<MetalAffineResidueRow>, MetalAffineError> {
    validate_seed_window(lane, seed_offset, 0)?;
    let rows = moduli
        .iter()
        .copied()
        .filter(|&modulus| gcd_u32(lane.config.base, modulus) == 1)
        .map(|modulus| {
            let p = modulus as u128;
            let a =
                ((lane.shift as u128 + (lane.gradient as u128 * seed_offset as u128)) % p) as u32;
            let g = (lane.gradient % modulus as u64) as u32;
            MetalAffineResidueRow {
                a,
                g,
                p: modulus,
                pad: 0,
            }
        })
        .collect();
    Ok(rows)
}

pub fn build_metal_affine_residue_row_batches(
    lane: &FastAffineLane,
    seed_offset: u64,
    seed_count: u64,
    batch_seed_count: u64,
    moduli: &[u32],
) -> Result<Vec<Vec<MetalAffineResidueRow>>, MetalAffineError> {
    validate_seed_window(lane, seed_offset, seed_count)?;
    if batch_seed_count == 0 {
        return Err(MetalAffineError::InvalidBatchSeedCount);
    }
    let batch_count = seed_count.div_ceil(batch_seed_count) as usize;
    let mut batches = Vec::with_capacity(batch_count);
    for batch_index in 0..batch_count {
        let batch_offset = seed_offset + batch_index as u64 * batch_seed_count;
        batches.push(build_metal_affine_residue_rows(lane, batch_offset, moduli)?);
    }
    Ok(batches)
}

pub fn residue_rows_allow_local_seed(rows: &[MetalAffineResidueRow], local_seed: u64) -> bool {
    rows.iter().all(|row| {
        let p = row.p as u64;
        let value = (row.a as u64 + ((local_seed % p) * row.g as u64) % p) % p;
        value != 0
    })
}

pub fn cpu_affine_survivor_seeds(
    lane: &FastAffineLane,
    seed_offset: u64,
    seed_count: u64,
    rows: &[MetalAffineResidueRow],
) -> Result<Vec<u64>, MetalAffineError> {
    validate_seed_window(lane, seed_offset, seed_count)?;
    Ok((0..seed_count)
        .filter(|&local_seed| residue_rows_allow_local_seed(rows, local_seed))
        .map(|local_seed| seed_offset + local_seed)
        .collect())
}

pub fn scan_metal_affine_lane(
    config: FastLaneConfig,
    requested_seed_count: u64,
    max_witnesses: usize,
    seed_offset: u64,
    residue_row_limit: Option<usize>,
) -> Result<MetalAffinePrimeRun, MetalAffineError> {
    let lane = build_fast_affine_lane(config)?;
    if seed_offset >= lane.seed_capacity {
        return Err(MetalAffineError::SeedOffsetOutOfRange {
            seed_offset,
            seed_capacity: lane.seed_capacity,
        });
    }
    let remaining_capacity = lane.seed_capacity - seed_offset;
    let scanned_seed_count = requested_seed_count.min(remaining_capacity);
    let capped_to_seed_capacity = scanned_seed_count < requested_seed_count;
    let mut moduli = default_metal_affine_moduli(&lane);
    if let Some(limit) = residue_row_limit {
        moduli.truncate(limit);
    }
    let residue_rows = build_metal_affine_residue_rows(&lane, seed_offset, &moduli)?;

    let total_start = Instant::now();
    let sieve =
        sieve_metal_affine_survivor_seeds(&lane, seed_offset, scanned_seed_count, &residue_rows)?;

    let confirm_start = Instant::now();
    let mut primes_found = 0u64;
    let mut witnesses = Vec::new();
    for &seed in &sieve.survivor_seeds {
        if let Some(value) = lane.candidate_value(seed) {
            if primal::is_prime(value) {
                primes_found += 1;
                if witnesses.len() < max_witnesses {
                    witnesses.push(FastPrimeWitness {
                        seed,
                        middle_digits: lane.middle_digits(seed),
                        template_digits: lane.template_digits(seed),
                        value,
                    });
                }
            }
        }
    }
    let cpu_confirm_seconds = confirm_start.elapsed().as_secs_f64();
    let total_seconds = total_start.elapsed().as_secs_f64();
    let metrics = transfer_metrics(
        scanned_seed_count,
        residue_rows.len(),
        sieve.output_bitmask_bytes,
        sieve.gpu_sieve_seconds,
        cpu_confirm_seconds,
        total_seconds,
    );

    Ok(MetalAffinePrimeRun {
        config: lane.config.clone(),
        pair_label: lane.config.pair_label(),
        k_label: lane.config.k_label(),
        shift: lane.shift,
        gradient: lane.gradient,
        seed_offset,
        requested_seed_count,
        seed_capacity: lane.seed_capacity,
        scanned_seed_count,
        capped_to_seed_capacity,
        residue_rows,
        survivor_seed_count: sieve.survivor_seeds.len() as u64,
        primality_tests: sieve.survivor_seeds.len() as u64,
        primes_found,
        witnesses,
        metrics,
    })
}

#[cfg(all(feature = "metal", target_os = "macos"))]
pub fn sieve_metal_affine_survivor_seeds(
    lane: &FastAffineLane,
    seed_offset: u64,
    seed_count: u64,
    rows: &[MetalAffineResidueRow],
) -> Result<MetalAffineSieveResult, MetalAffineError> {
    use metal::{Device, MTLResourceOptions, MTLSize};

    validate_seed_window(lane, seed_offset, seed_count)?;
    if seed_offset > u32::MAX as u64 || seed_count > u32::MAX as u64 {
        return Err(MetalAffineError::SeedCountTooLarge);
    }
    if seed_count == 0 {
        return Ok(MetalAffineSieveResult {
            seed_offset,
            seed_count,
            survivor_seeds: Vec::new(),
            gpu_sieve_seconds: 0.0,
            output_bitmask_bytes: 0,
        });
    }

    let device = Device::system_default()
        .ok_or_else(|| MetalAffineError::MetalInit("no system Metal device".to_string()))?;
    let library = device
        .new_library_with_data(include_bytes!(env!("METALLIB_PATH")))
        .map_err(|err| MetalAffineError::MetalInit(format!("load metallib: {err}")))?;
    let function = library
        .get_function("sieve_affine_lane", None)
        .map_err(|_| {
            MetalAffineError::MetalInit("kernel sieve_affine_lane not found".to_string())
        })?;
    let pipeline = device
        .new_compute_pipeline_state_with_function(&function)
        .map_err(|err| MetalAffineError::MetalInit(format!("pipeline: {err}")))?;
    let queue = device.new_command_queue();

    let params = MetalAffineParams {
        num_rows: rows.len() as u32,
        seed_offset: seed_offset as u32,
        num_seeds: seed_count as u32,
        reserved: 0,
    };
    let row_bytes = std::mem::size_of_val(rows) as u64;
    let mask_words = seed_count.div_ceil(32) as usize;
    let output_bitmask_bytes = (mask_words * mem::size_of::<u32>()) as u64;
    let params_buffer = device.new_buffer_with_data(
        &params as *const _ as _,
        mem::size_of::<MetalAffineParams>() as u64,
        MTLResourceOptions::StorageModeShared,
    );
    let rows_buffer = device.new_buffer_with_data(
        rows.as_ptr() as _,
        row_bytes,
        MTLResourceOptions::StorageModeShared,
    );
    let out_buffer = device.new_buffer(output_bitmask_bytes, MTLResourceOptions::StorageModeShared);
    if out_buffer.contents().is_null() {
        return Err(MetalAffineError::MetalExecution(
            "output buffer allocation returned null".to_string(),
        ));
    }
    unsafe {
        std::ptr::write_bytes(out_buffer.contents() as *mut u32, 0, mask_words);
    }

    let start = Instant::now();
    let command = queue.new_command_buffer();
    let encoder = command.new_compute_command_encoder();
    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(0, Some(&params_buffer), 0);
    encoder.set_buffer(1, Some(&rows_buffer), 0);
    encoder.set_buffer(2, Some(&out_buffer), 0);
    let threads_per_group = MTLSize::new(256, 1, 1);
    let groups = MTLSize::new(seed_count.div_ceil(256), 1, 1);
    encoder.dispatch_thread_groups(groups, threads_per_group);
    encoder.end_encoding();
    command.commit();
    command.wait_until_completed();
    let gpu_sieve_seconds = start.elapsed().as_secs_f64();

    let masks =
        unsafe { std::slice::from_raw_parts(out_buffer.contents() as *const u32, mask_words) };
    let survivor_seeds = unpack_bitmask_to_seeds(masks, seed_offset, seed_count);
    Ok(MetalAffineSieveResult {
        seed_offset,
        seed_count,
        survivor_seeds,
        gpu_sieve_seconds,
        output_bitmask_bytes,
    })
}

#[cfg(not(all(feature = "metal", target_os = "macos")))]
pub fn sieve_metal_affine_survivor_seeds(
    lane: &FastAffineLane,
    seed_offset: u64,
    seed_count: u64,
    _rows: &[MetalAffineResidueRow],
) -> Result<MetalAffineSieveResult, MetalAffineError> {
    validate_seed_window(lane, seed_offset, seed_count)?;
    Err(MetalAffineError::MetalUnavailable)
}

#[cfg(all(feature = "metal", target_os = "macos"))]
pub fn sieve_metal_affine_residue_batches(
    seed_offset: u64,
    seed_count: u64,
    batch_seed_count: u64,
    row_batches: &[Vec<MetalAffineResidueRow>],
) -> Result<MetalAffineBatchSieveResult, MetalAffineError> {
    use metal::{Device, MTLResourceOptions, MTLSize};

    if batch_seed_count == 0 {
        return Err(MetalAffineError::InvalidBatchSeedCount);
    }
    if seed_count == 0 {
        return Ok(MetalAffineBatchSieveResult {
            seed_offset,
            seed_count,
            batch_seed_count,
            batch_count: 0,
            survivor_seeds: Vec::new(),
            setup_seconds: 0.0,
            buffer_prepare_seconds: 0.0,
            gpu_sieve_seconds: 0.0,
            unpack_seconds: 0.0,
            total_seconds: 0.0,
            input_metadata_bytes: 0,
            output_bitmask_bytes: 0,
            total_shared_buffer_bytes: 0,
            avoided_candidate_value_bytes_u64: 0,
        });
    }
    if batch_seed_count > u32::MAX as u64 {
        return Err(MetalAffineError::SeedCountTooLarge);
    }
    let batch_count = seed_count.div_ceil(batch_seed_count) as usize;
    if row_batches.len() != batch_count {
        return Err(MetalAffineError::ResidueBatchCountMismatch {
            expected: batch_count,
            actual: row_batches.len(),
        });
    }

    let total_start = Instant::now();
    let setup_start = Instant::now();
    let device = Device::system_default()
        .ok_or_else(|| MetalAffineError::MetalInit("no system Metal device".to_string()))?;
    let library = device
        .new_library_with_data(include_bytes!(env!("METALLIB_PATH")))
        .map_err(|err| MetalAffineError::MetalInit(format!("load metallib: {err}")))?;
    let function = library
        .get_function("sieve_affine_lane", None)
        .map_err(|_| {
            MetalAffineError::MetalInit("kernel sieve_affine_lane not found".to_string())
        })?;
    let pipeline = device
        .new_compute_pipeline_state_with_function(&function)
        .map_err(|err| MetalAffineError::MetalInit(format!("pipeline: {err}")))?;
    let queue = device.new_command_queue();
    let setup_seconds = setup_start.elapsed().as_secs_f64();

    let mut survivor_seeds = Vec::new();
    let mut buffer_prepare_seconds = 0.0;
    let mut gpu_sieve_seconds = 0.0;
    let mut unpack_seconds = 0.0;
    let mut input_metadata_bytes = 0u64;
    let mut output_bitmask_bytes = 0u64;

    for (batch_index, rows) in row_batches.iter().enumerate() {
        let local_seed_offset = batch_index as u64 * batch_seed_count;
        let absolute_seed_offset = seed_offset + local_seed_offset;
        let current_seed_count = batch_seed_count.min(seed_count - local_seed_offset);
        if current_seed_count > u32::MAX as u64 {
            return Err(MetalAffineError::SeedCountTooLarge);
        }

        let prepare_start = Instant::now();
        let params = MetalAffineParams {
            num_rows: rows.len() as u32,
            seed_offset: 0,
            num_seeds: current_seed_count as u32,
            reserved: 0,
        };
        let row_bytes = std::mem::size_of_val(rows.as_slice()) as u64;
        let mask_words = current_seed_count.div_ceil(32) as usize;
        let batch_output_bytes = (mask_words * mem::size_of::<u32>()) as u64;
        let params_buffer = device.new_buffer_with_data(
            &params as *const _ as _,
            mem::size_of::<MetalAffineParams>() as u64,
            MTLResourceOptions::StorageModeShared,
        );
        let rows_buffer = device.new_buffer_with_data(
            rows.as_ptr() as _,
            row_bytes,
            MTLResourceOptions::StorageModeShared,
        );
        let out_buffer =
            device.new_buffer(batch_output_bytes, MTLResourceOptions::StorageModeShared);
        if out_buffer.contents().is_null() {
            return Err(MetalAffineError::MetalExecution(
                "output buffer allocation returned null".to_string(),
            ));
        }
        unsafe {
            std::ptr::write_bytes(out_buffer.contents() as *mut u32, 0, mask_words);
        }
        buffer_prepare_seconds += prepare_start.elapsed().as_secs_f64();
        input_metadata_bytes += mem::size_of::<MetalAffineParams>() as u64 + row_bytes;
        output_bitmask_bytes += batch_output_bytes;

        let dispatch_start = Instant::now();
        let command = queue.new_command_buffer();
        let encoder = command.new_compute_command_encoder();
        encoder.set_compute_pipeline_state(&pipeline);
        encoder.set_buffer(0, Some(&params_buffer), 0);
        encoder.set_buffer(1, Some(&rows_buffer), 0);
        encoder.set_buffer(2, Some(&out_buffer), 0);
        let threads_per_group = MTLSize::new(256, 1, 1);
        let groups = MTLSize::new(current_seed_count.div_ceil(256), 1, 1);
        encoder.dispatch_thread_groups(groups, threads_per_group);
        encoder.end_encoding();
        command.commit();
        command.wait_until_completed();
        gpu_sieve_seconds += dispatch_start.elapsed().as_secs_f64();

        let unpack_start = Instant::now();
        let masks =
            unsafe { std::slice::from_raw_parts(out_buffer.contents() as *const u32, mask_words) };
        survivor_seeds.extend(unpack_bitmask_to_seeds(
            masks,
            absolute_seed_offset,
            current_seed_count,
        ));
        unpack_seconds += unpack_start.elapsed().as_secs_f64();
    }

    Ok(MetalAffineBatchSieveResult {
        seed_offset,
        seed_count,
        batch_seed_count,
        batch_count,
        survivor_seeds,
        setup_seconds,
        buffer_prepare_seconds,
        gpu_sieve_seconds,
        unpack_seconds,
        total_seconds: total_start.elapsed().as_secs_f64(),
        input_metadata_bytes,
        output_bitmask_bytes,
        total_shared_buffer_bytes: input_metadata_bytes + output_bitmask_bytes,
        avoided_candidate_value_bytes_u64: seed_count * mem::size_of::<u64>() as u64,
    })
}

#[cfg(not(all(feature = "metal", target_os = "macos")))]
pub fn sieve_metal_affine_residue_batches(
    _seed_offset: u64,
    _seed_count: u64,
    batch_seed_count: u64,
    _row_batches: &[Vec<MetalAffineResidueRow>],
) -> Result<MetalAffineBatchSieveResult, MetalAffineError> {
    if batch_seed_count == 0 {
        return Err(MetalAffineError::InvalidBatchSeedCount);
    }
    Err(MetalAffineError::MetalUnavailable)
}

fn transfer_metrics(
    candidate_count: u64,
    residue_row_count: usize,
    output_bitmask_bytes: u64,
    gpu_sieve_seconds: f64,
    cpu_confirm_seconds: f64,
    total_seconds: f64,
) -> MetalAffineTransferMetrics {
    let params_bytes = mem::size_of::<MetalAffineParams>() as u64;
    let residue_row_bytes = (residue_row_count * mem::size_of::<MetalAffineResidueRow>()) as u64;
    let input_metadata_bytes = params_bytes + residue_row_bytes;
    MetalAffineTransferMetrics {
        storage_mode: "Metal StorageModeShared".to_string(),
        zero_candidate_value_transfer: true,
        candidate_value_buffer_transferred: false,
        candidate_count,
        residue_row_count,
        params_bytes,
        residue_row_bytes,
        input_metadata_bytes,
        output_bitmask_bytes,
        total_shared_buffer_bytes: input_metadata_bytes + output_bitmask_bytes,
        avoided_candidate_value_bytes_u64: candidate_count * mem::size_of::<u64>() as u64,
        avoided_candidate_value_bytes_u32: candidate_count * mem::size_of::<u32>() as u64,
        gpu_sieve_seconds,
        cpu_confirm_seconds,
        total_seconds,
    }
}

fn validate_seed_window(
    lane: &FastAffineLane,
    seed_offset: u64,
    seed_count: u64,
) -> Result<(), MetalAffineError> {
    if seed_offset >= lane.seed_capacity {
        return Err(MetalAffineError::SeedOffsetOutOfRange {
            seed_offset,
            seed_capacity: lane.seed_capacity,
        });
    }
    if seed_count > lane.seed_capacity - seed_offset {
        return Err(MetalAffineError::SeedOffsetOutOfRange {
            seed_offset: seed_offset + seed_count,
            seed_capacity: lane.seed_capacity,
        });
    }
    Ok(())
}

#[cfg(all(feature = "metal", target_os = "macos"))]
fn unpack_bitmask_to_seeds(masks: &[u32], seed_offset: u64, seed_count: u64) -> Vec<u64> {
    let mut out = Vec::new();
    for (word_index, &word) in masks.iter().enumerate() {
        let mut word = word;
        while word != 0 {
            let bit = word.trailing_zeros() as u64;
            let local_seed = word_index as u64 * 32 + bit;
            if local_seed < seed_count {
                out.push(seed_offset + local_seed);
            }
            word &= !(1 << bit);
        }
    }
    out
}

fn gcd_u32(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let tmp = left % right;
        left = right;
        right = tmp;
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metal_affine_residue_rows_match_exhaustive_filtering() {
        let lane = build_fast_affine_lane(FastLaneConfig::new(10, 3, 7, 2, (2, 1))).unwrap();
        let moduli = default_metal_affine_moduli(&lane);
        let rows = build_metal_affine_residue_rows(&lane, 0, &moduli).unwrap();

        for seed in 0..lane.seed_capacity {
            let by_rows = residue_rows_allow_local_seed(&rows, seed);
            let by_value = moduli.iter().copied().all(|modulus| {
                let value = lane.candidate_value(seed).unwrap();
                !value.is_multiple_of(modulus as u64)
            });
            assert_eq!(by_rows, by_value, "seed {seed}");
        }
    }

    #[test]
    fn metal_affine_residue_rows_honor_seed_offset() {
        let lane = build_fast_affine_lane(FastLaneConfig::new(22, 17, 19, 2, (2, 2))).unwrap();
        let moduli = default_metal_affine_moduli(&lane);
        let seed_offset = 7;
        let rows = build_metal_affine_residue_rows(&lane, seed_offset, &moduli).unwrap();

        for local_seed in 0..40 {
            let absolute_seed = seed_offset + local_seed;
            let by_rows = residue_rows_allow_local_seed(&rows, local_seed);
            let by_value = moduli.iter().copied().all(|modulus| {
                let value = lane.candidate_value(absolute_seed).unwrap();
                !value.is_multiple_of(modulus as u64)
            });
            assert_eq!(by_rows, by_value, "seed {absolute_seed}");
        }
    }

    #[cfg(all(feature = "metal", target_os = "macos"))]
    #[test]
    fn metal_affine_gpu_survivors_match_cpu_base10() {
        let lane = build_fast_affine_lane(FastLaneConfig::new(10, 3, 7, 2, (2, 1))).unwrap();
        let moduli = default_metal_affine_moduli(&lane);
        let rows = build_metal_affine_residue_rows(&lane, 0, &moduli).unwrap();
        let cpu = cpu_affine_survivor_seeds(&lane, 0, lane.seed_capacity, &rows).unwrap();
        let gpu = sieve_metal_affine_survivor_seeds(&lane, 0, lane.seed_capacity, &rows)
            .unwrap()
            .survivor_seeds;
        assert_eq!(gpu, cpu);
    }

    #[cfg(all(feature = "metal", target_os = "macos"))]
    #[test]
    fn metal_affine_repeated_batches_match_cpu_base10() {
        let lane = build_fast_affine_lane(FastLaneConfig::new(10, 3, 7, 2, (2, 1))).unwrap();
        let moduli = default_metal_affine_moduli(&lane);
        let rows = build_metal_affine_residue_rows(&lane, 0, &moduli).unwrap();
        let row_batches =
            build_metal_affine_residue_row_batches(&lane, 0, lane.seed_capacity, 17, &moduli)
                .unwrap();
        let cpu = cpu_affine_survivor_seeds(&lane, 0, lane.seed_capacity, &rows).unwrap();
        let gpu = sieve_metal_affine_residue_batches(0, lane.seed_capacity, 17, &row_batches)
            .unwrap()
            .survivor_seeds;
        assert_eq!(gpu, cpu);
    }

    #[cfg(all(feature = "metal", target_os = "macos"))]
    #[test]
    fn metal_affine_gpu_survivors_match_cpu_base22_side_pocket() {
        let lane = build_fast_affine_lane(FastLaneConfig::new(22, 17, 19, 2, (2, 2))).unwrap();
        let moduli = default_metal_affine_moduli(&lane);
        let rows = build_metal_affine_residue_rows(&lane, 0, &moduli).unwrap();
        let cpu = cpu_affine_survivor_seeds(&lane, 0, lane.seed_capacity, &rows).unwrap();
        let gpu = sieve_metal_affine_survivor_seeds(&lane, 0, lane.seed_capacity, &rows)
            .unwrap()
            .survivor_seeds;
        assert_eq!(gpu, cpu);
    }
}
