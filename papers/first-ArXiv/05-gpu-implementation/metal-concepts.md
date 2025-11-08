# Metal GPU Concepts Explained

## Thread Hierarchy Demystified

When you launch a Metal kernel, you're orchestrating thousands of threads in a specific hierarchy:

```
┌─────────────────────────────────────────────┐
│                 GPU Device                  │
│  ┌─────────────────────────────────────┐   │
│  │         Compute Command              │   │
│  │  ┌─────────────┐  ┌─────────────┐   │   │
│  │  │ Threadgroup │  │ Threadgroup │   │   │
│  │  │  ┌──┬──┬──┐│  │  ┌──┬──┬──┐│   │   │
│  │  │  │T0│T1│..││  │  │T0│T1│..││   │   │
│  │  │  ├──┼──┼──┤│  │  ├──┼──┼──┤│   │   │
│  │  │  │T32│T33│.││  │  │T32│T33│.│   │   │
│  │  │  └──┴──┴──┘│  │  └──┴──┴──┘│   │   │
│  │  └─────────────┘  └─────────────┘   │   │
│  └─────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

### The Key Players

**Thread**: The individual worker
- Has its own registers
- Executes independently
- Identified by `thread_position_in_grid`

**SIMD Group**: 32 threads that execute in lockstep
- Share the same instruction pointer
- Can communicate via `simd_ballot`, `simd_sum`, etc.
- Like synchronized swimmers - all do the same move

**Threadgroup**: Up to 1024 threads that can share memory
- Have access to threadgroup memory (32KB on M2)
- Can synchronize with barriers
- Like a classroom working together

**Grid**: All threads for the entire kernel launch
- Can be millions of threads
- Divided into threadgroups

## Metal Attributes Explained

```metal
kernel void my_kernel(
    device uint* data [[buffer(0)]],           // GPU memory buffer
    uint tid [[thread_position_in_grid]],      // Global thread ID (0, 1, 2, ...)
    uint lid [[thread_position_in_threadgroup]], // Local thread ID within group
    uint gid [[threadgroup_position_in_grid]], // Which threadgroup
    uint tpg [[threads_per_threadgroup]],      // Size of threadgroup
    uint simd_lane [[thread_index_in_simdgroup]], // 0-31 within SIMD group
    uint simd_gid [[simdgroup_index_in_threadgroup]] // Which SIMD group
)
```

## Launching the Kernel

From the CPU side (Rust/Swift):

```rust
// Define the work
let num_candidates = 4_000_000;
let threads_per_threadgroup = 1024;  // Max for M2
let threadgroups = (num_candidates + 1023) / 1024;  // Round up

// Create command
let command_encoder = command_buffer.compute_command_encoder();
command_encoder.set_compute_pipeline_state(&pipeline);
command_encoder.set_buffer(&candidates_buffer, 0, 0);
command_encoder.set_buffer(&signatures_buffer, 0, 1);
command_encoder.set_buffer(&output_buffer, 0, 2);

// Launch!
command_encoder.dispatch_threadgroups(
    MTLSize { width: threadgroups, height: 1, depth: 1 },
    MTLSize { width: threads_per_threadgroup, height: 1, depth: 1 }
);
```

## SIMD Group Operations

SIMD groups have special powers because all 32 threads execute together:

```metal
// All threads vote, result appears instantly in all threads
bool any_found_prime = simd_any(is_prime);  
bool all_found_prime = simd_all(is_prime);

// Collect all 32 boolean values into a bitmask
uint32_t ballot = simd_ballot(is_prime);
// If threads 0,2,5 have is_prime=true: ballot = 0b00100101

// Sum across all 32 threads
uint total = simd_sum(my_value);

// Broadcast from one lane to all
uint value_from_lane_0 = simd_broadcast(my_value, 0);
```

## Threadgroup Memory and Barriers

Threadgroup memory is shared but needs synchronization:

```metal
threadgroup float shared_data[256];

// Thread 0 writes
if (lid == 0) {
    shared_data[0] = expensive_calculation();
}

// WRONG: Other threads might read before thread 0 writes!
float value = shared_data[0];  

// RIGHT: Barrier ensures all threads wait
threadgroup_barrier(mem_flags::mem_threadgroup);
float value = shared_data[0];  // Now safe!
```

## Memory Qualifiers

Metal has specific keywords for different memory types:

```metal
device uint* data       // Global GPU memory (read/write)
constant Params& params // Constant memory (read-only, cached)
threadgroup float shared[256] // Shared within threadgroup
thread float local_var  // Private to thread (usually implicit)
```

## Atomic Operations

When multiple threads need to update the same location:

```metal
// WRONG: Race condition!
if (found_prime) {
    output[0] = output[0] + 1;  // Multiple threads clash!
}

// RIGHT: Atomic operation
if (found_prime) {
    atomic_fetch_add(&output[0], 1);  // Hardware handles conflicts
}

// Our optimized version: Minimize atomics
uint ballot = simd_ballot(found_prime);
if (simd_lane == 0) {  // Only one thread per SIMD group
    atomic_fetch_or(&output[word], ballot);
}
```

## Metal Shader Compilation

Metal shaders compile in two stages:

1. **Offline**: `.metal` → `.air` (Apple IR)
   ```bash
   xcrun -sdk macosx metal -c shader.metal -o shader.air
   ```

2. **Runtime**: `.air` → `.metallib` (device-specific)
   ```bash
   xcrun -sdk macosx metallib shader.air -o shader.metallib
   ```

Our `build.rs` automates this:
```rust
// build.rs
fn main() {
    println!("cargo:rerun-if-changed=shaders/");
    // Compile .metal files to .metallib
    compile_metal_shaders();
}
```

## Debugging Tips

1. **Buffer Overruns**: Always check bounds
   ```metal
   if (tid >= params.numCandidates) return;  // Critical!
   ```

2. **Barrier Placement**: Too few = race conditions, too many = slow
   ```metal
   threadgroup_barrier(mem_flags::mem_threadgroup);  // Only when needed
   ```

3. **Divergence**: Minimize branches
   ```metal
   // Bad: Divergent execution
   if (condition) { expensive_work(); }
   
   // Good: All threads do same work
   result = condition ? expensive_work() : 0;
   ```

4. **Occupancy**: Use Xcode's GPU Frame Capture
   - Shows threadgroup occupancy
   - Memory bandwidth utilization  
   - Register pressure

## The Mental Model

Think of the GPU as a massive parallel factory:

- **Threads** = Individual workers
- **SIMD Groups** = Assembly lines (32 workers doing synchronized moves)
- **Threadgroups** = Factory floors (can share tools/whiteboards)
- **Barriers** = Synchronization whistles
- **Atomic operations** = Taking turns at shared resources
- **Threadgroup memory** = Shared workbench
- **Global memory** = Warehouse

The art is organizing the work so all workers stay busy, minimize trips to the warehouse, and maximize use of local workbenches!