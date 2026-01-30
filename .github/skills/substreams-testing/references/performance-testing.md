# Performance Testing Guide

Comprehensive guide for performance testing Substreams applications, ensuring scalability and production readiness.

## Performance Testing Philosophy

Performance testing for Substreams must address:
- **High throughput** - Processing millions of blocks efficiently
- **Parallel execution** - Production mode performance characteristics
- **Memory efficiency** - Avoiding memory leaks and excessive allocation
- **Network resilience** - Performance under network constraints
- **Caching impact** - Understanding cursor and state caching effects

## Types of Performance Tests

### 1. Micro-benchmarks (Function Level)

Test individual functions for optimization opportunities.

```rust
// benches/micro_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use your_substreams::*;
use num_bigint::BigInt;
use std::str::FromStr;

fn benchmark_parsing_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");
    
    // Test with different log sizes
    let test_cases = vec![
        ("small", create_transfer_log_with_topics(1)),
        ("medium", create_transfer_log_with_topics(4)),
        ("large", create_transfer_log_with_topics(8)),
    ];
    
    for (name, log) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("parse_erc20_transfer", name),
            &log,
            |b, log| {
                b.iter(|| parse_erc20_transfer(black_box(log)))
            }
        );
    }
    
    group.finish();
}

fn benchmark_data_structures(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_structures");
    
    // Compare different approaches for storing transfers
    let transfers = create_test_transfers(1000);
    
    group.bench_function("vec_linear_search", |b| {
        b.iter(|| {
            let mut found = 0;
            for transfer in &transfers {
                if transfer.amount > BigInt::from(1000000000u64) {
                    found += 1;
                }
            }
            found
        })
    });
    
    group.bench_function("hashmap_lookup", |b| {
        let transfer_map = build_transfer_hashmap(&transfers);
        b.iter(|| {
            let mut found = 0;
            for key in transfer_map.keys() {
                if let Some(transfer) = transfer_map.get(key) {
                    if transfer.amount > BigInt::from(1000000000u64) {
                        found += 1;
                    }
                }
            }
            found
        })
    });
    
    group.finish();
}

fn benchmark_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    group.throughput(Throughput::Elements(1000));
    
    let transfers = create_test_transfers(1000);
    let transfer_events = TransferEvents { transfers };
    
    group.bench_function("protobuf_encode", |b| {
        b.iter(|| {
            transfer_events.encode_to_vec()
        })
    });
    
    let encoded = transfer_events.encode_to_vec();
    group.bench_function("protobuf_decode", |b| {
        b.iter(|| {
            TransferEvents::decode(black_box(encoded.as_slice()))
        })
    });
    
    group.finish();
}

fn benchmark_address_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("address_ops");
    
    let addresses = generate_test_addresses(1000);
    
    group.bench_function("validate_addresses", |b| {
        b.iter(|| {
            let mut valid_count = 0;
            for addr in &addresses {
                if is_valid_ethereum_address(black_box(addr)) {
                    valid_count += 1;
                }
            }
            valid_count
        })
    });
    
    group.bench_function("normalize_addresses", |b| {
        b.iter(|| {
            let mut normalized = Vec::new();
            for addr in &addresses {
                normalized.push(normalize_address(black_box(addr)));
            }
            normalized
        })
    });
    
    group.finish();
}

fn benchmark_bigint_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bigint_ops");
    
    let amounts = generate_test_amounts(1000);
    
    group.bench_function("bigint_addition", |b| {
        b.iter(|| {
            let mut sum = BigInt::from(0);
            for amount in &amounts {
                sum += amount;
            }
            sum
        })
    });
    
    group.bench_function("bigint_comparison", |b| {
        b.iter(|| {
            let mut large_count = 0;
            let threshold = BigInt::from(1000000000000000000u64); // 1 ETH
            for amount in &amounts {
                if black_box(amount) > &threshold {
                    large_count += 1;
                }
            }
            large_count
        })
    });
    
    group.bench_function("bigint_string_conversion", |b| {
        b.iter(|| {
            let mut strings = Vec::new();
            for amount in &amounts {
                strings.push(amount.to_string());
            }
            strings
        })
    });
    
    group.finish();
}

// Helper functions
fn create_transfer_log_with_topics(topic_count: usize) -> Log {
    let mut topics = vec![
        hex::decode("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").unwrap()
    ];
    
    for i in 1..topic_count {
        topics.push(vec![i as u8; 32]);
    }
    
    Log {
        address: vec![0u8; 20],
        topics,
        data: vec![0u8; 32],
        ..Default::default()
    }
}

fn create_test_transfers(count: usize) -> Vec<Transfer> {
    (0..count).map(|i| Transfer {
        contract_address: format!("0x{:040x}", i),
        from_address: format!("0x{:040x}", i * 2),
        to_address: format!("0x{:040x}", i * 2 + 1),
        amount: BigInt::from(1000000000u64 * i as u64),
        block_number: 17000000 + i as u64,
        log_index: i as u32,
        transaction_hash: format!("0x{:064x}", i),
    }).collect()
}

fn generate_test_addresses(count: usize) -> Vec<String> {
    (0..count).map(|i| format!("0x{:040x}", i)).collect()
}

fn generate_test_amounts(count: usize) -> Vec<BigInt> {
    (0..count).map(|i| BigInt::from(1000000000u64 * i as u64)).collect()
}

criterion_group!(
    micro_benches,
    benchmark_parsing_performance,
    benchmark_data_structures,
    benchmark_serialization,
    benchmark_address_operations,
    benchmark_bigint_operations
);
criterion_main!(micro_benches);
```

### 2. Module Performance Tests

Test entire Substreams modules with realistic data loads.

```rust
// benches/module_benchmarks.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use your_substreams::*;

fn benchmark_map_transfers_by_block_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_transfers_by_block");
    
    let test_blocks = vec![
        ("empty", create_empty_block(17000000)),
        ("light", load_block_fixture("light_activity.json")), // 10-50 transfers
        ("normal", load_block_fixture("normal_activity.json")), // 100-500 transfers  
        ("heavy", load_block_fixture("heavy_activity.json")), // 1000+ transfers
        ("mega", load_block_fixture("mega_block.json")), // 5000+ transfers
    ];
    
    for (block_type, block) in test_blocks {
        let transfer_count = count_potential_transfers(&block);
        
        group.throughput(Throughput::Elements(transfer_count as u64));
        group.bench_with_input(
            BenchmarkId::new("block_processing", block_type),
            &block,
            |b, block| {
                b.iter(|| map_transfers(block.clone()))
            }
        );
    }
    
    group.finish();
}

fn benchmark_store_operations_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("store_operations");
    
    for transfer_count in [100, 500, 1000, 5000, 10000].iter() {
        let transfers = create_test_transfer_events(*transfer_count);
        
        group.throughput(Throughput::Elements(*transfer_count as u64));
        group.bench_with_input(
            BenchmarkId::new("store_balances", transfer_count),
            &transfers,
            |b, transfers| {
                b.iter_batched(
                    || create_test_store(), // Setup fresh store for each iteration
                    |store| store_balances(transfers.clone(), &store),
                    criterion::BatchSize::SmallInput
                )
            }
        );
    }
    
    group.finish();
}

fn benchmark_memory_allocation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    
    // Test different allocation strategies
    let block = load_block_fixture("heavy_activity.json");
    
    group.bench_function("with_preallocation", |b| {
        b.iter(|| {
            // Pre-allocate Vec with estimated capacity
            let estimated_capacity = estimate_transfer_count(&block);
            map_transfers_with_preallocation(block.clone(), estimated_capacity)
        })
    });
    
    group.bench_function("without_preallocation", |b| {
        b.iter(|| {
            // Default Vec allocation strategy
            map_transfers(block.clone())
        })
    });
    
    group.bench_function("with_object_pooling", |b| {
        b.iter_batched(
            || create_transfer_pool(), // Create object pool
            |pool| map_transfers_with_pooling(block.clone(), pool),
            criterion::BatchSize::SmallInput
        )
    });
    
    group.finish();
}

// Helper functions
fn count_potential_transfers(block: &Block) -> usize {
    block.transaction_traces
        .iter()
        .map(|tx| tx.receipt.as_ref().map_or(0, |r| r.logs.len()))
        .sum()
}

fn load_block_fixture(filename: &str) -> Block {
    let path = format!("benches/fixtures/{}", filename);
    let data = std::fs::read_to_string(path).expect("Failed to load fixture");
    serde_json::from_str(&data).expect("Failed to parse block fixture")
}

fn create_empty_block(number: u64) -> Block {
    Block {
        number,
        hash: vec![0u8; 32],
        parent_hash: vec![0u8; 32],
        timestamp_seconds: 1680000000,
        transaction_traces: vec![],
        ..Default::default()
    }
}

criterion_group!(
    module_benches,
    benchmark_map_transfers_by_block_type,
    benchmark_store_operations_scaling,
    benchmark_memory_allocation_patterns
);
criterion_main!(module_benches);
```

### 3. End-to-End Performance Tests

Test complete Substreams execution with real-world scenarios.

```bash
#!/bin/bash
# scripts/performance_tests.sh

set -e

echo "🚀 Starting Substreams Performance Tests"

# Test configurations
NETWORKS=("mainnet")
START_BLOCK=17000000
BLOCK_RANGES=(100 1000 10000)
MODULES=("map_transfers" "store_balances" "map_events")

# Results directory
RESULTS_DIR="performance_results/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

# System info
echo "📊 System Information" | tee "$RESULTS_DIR/system_info.txt"
echo "CPU: $(lscpu | grep 'Model name' | cut -d: -f2 | xargs)" | tee -a "$RESULTS_DIR/system_info.txt"
echo "Memory: $(free -h | grep Mem | awk '{print $2}')" | tee -a "$RESULTS_DIR/system_info.txt"
echo "Disk: $(df -h . | tail -1 | awk '{print $4}' )" | tee -a "$RESULTS_DIR/system_info.txt"

# Function to run performance test
run_perf_test() {
    local network=$1
    local module=$2
    local blocks=$3
    local mode=$4
    
    local test_name="${network}_${module}_${blocks}blocks_${mode}"
    local result_file="$RESULTS_DIR/${test_name}.json"
    
    echo "⚡ Testing: $test_name"
    
    local start_time=$(date +%s.%N)
    
    # Run substreams with timing
    timeout 600s /usr/bin/time -v substreams run \
        -s $START_BLOCK \
        -t +$blocks \
        $module \
        --network $network \
        ${mode:+--production-mode} \
        --output-format json \
        > "$result_file" 2> "${result_file}.timing"
    
    local exit_code=$?
    local end_time=$(date +%s.%N)
    local duration=$(echo "$end_time - $start_time" | bc)
    
    if [ $exit_code -eq 0 ]; then
        # Parse results
        local output_lines=$(wc -l < "$result_file")
        local avg_time_per_block=$(echo "scale=4; $duration / $blocks" | bc)
        local blocks_per_second=$(echo "scale=2; $blocks / $duration" | bc)
        
        # Extract memory usage
        local max_memory=$(grep "Maximum resident set size" "${result_file}.timing" | awk '{print $6}')
        
        echo "✅ $test_name completed successfully"
        echo "   Duration: ${duration}s"
        echo "   Blocks processed: $blocks"  
        echo "   Output lines: $output_lines"
        echo "   Avg time/block: ${avg_time_per_block}s"
        echo "   Blocks/second: $blocks_per_second"
        echo "   Max memory: ${max_memory}KB"
        
        # Create summary
        cat > "${result_file}.summary" << EOF
{
  "test_name": "$test_name",
  "network": "$network",
  "module": "$module", 
  "block_count": $blocks,
  "mode": "$mode",
  "duration_seconds": $duration,
  "blocks_per_second": $blocks_per_second,
  "avg_seconds_per_block": $avg_time_per_block,
  "output_lines": $output_lines,
  "max_memory_kb": $max_memory,
  "success": true
}
EOF
    else
        echo "❌ $test_name failed (exit code: $exit_code)"
        echo "{\"test_name\": \"$test_name\", \"success\": false, \"exit_code\": $exit_code}" > "${result_file}.summary"
    fi
}

# Run performance tests
for network in "${NETWORKS[@]}"; do
    for module in "${MODULES[@]}"; do
        for blocks in "${BLOCK_RANGES[@]}"; do
            # Test development mode
            run_perf_test "$network" "$module" "$blocks" ""
            
            # Test production mode  
            run_perf_test "$network" "$module" "$blocks" "production"
            
            # Cool down between tests
            sleep 5
        done
    done
done

# Generate performance report
echo "📈 Generating Performance Report"
python3 scripts/generate_perf_report.py "$RESULTS_DIR"

echo "🎉 Performance testing completed. Results in: $RESULTS_DIR"
```

### 4. Memory and Resource Profiling

```rust
// tests/memory_profiling_tests.rs
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use your_substreams::*;

// Custom allocator for memory tracking
struct MemoryTracker;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for MemoryTracker {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ptr
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        DEALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: MemoryTracker = MemoryTracker;

#[test]
fn test_memory_usage_single_block() {
    // Reset counters
    ALLOCATED.store(0, Ordering::Relaxed);
    DEALLOCATED.store(0, Ordering::Relaxed);
    
    let initial_allocated = ALLOCATED.load(Ordering::Relaxed);
    let initial_deallocated = DEALLOCATED.load(Ordering::Relaxed);
    
    // Process a single block
    let block = load_test_block(17000000);
    let _result = map_transfers(block).unwrap();
    
    let final_allocated = ALLOCATED.load(Ordering::Relaxed);
    let final_deallocated = DEALLOCATED.load(Ordering::Relaxed);
    
    let net_allocated = (final_allocated - initial_allocated) as i64 - (final_deallocated - initial_deallocated) as i64;
    
    println!("Memory usage for single block:");
    println!("  Allocated: {} bytes", final_allocated - initial_allocated);
    println!("  Deallocated: {} bytes", final_deallocated - initial_deallocated);
    println!("  Net allocated: {} bytes", net_allocated);
    
    // Should not leak significant memory
    assert!(net_allocated.abs() < 1024 * 1024, "Memory leak detected: {} bytes", net_allocated);
}

#[test]
fn test_memory_scaling() {
    let block_counts = vec![1, 10, 100];
    let mut memory_per_block = Vec::new();
    
    for block_count in block_counts {
        ALLOCATED.store(0, Ordering::Relaxed);
        DEALLOCATED.store(0, Ordering::Relaxed);
        
        let initial_allocated = ALLOCATED.load(Ordering::Relaxed);
        
        // Process multiple blocks
        for i in 0..block_count {
            let block = load_test_block(17000000 + i);
            let _result = map_transfers(block).unwrap();
        }
        
        let final_allocated = ALLOCATED.load(Ordering::Relaxed);
        let allocated_per_block = (final_allocated - initial_allocated) / block_count as usize;
        
        memory_per_block.push(allocated_per_block);
        
        println!("Processed {} blocks: {} bytes/block", block_count, allocated_per_block);
    }
    
    // Memory usage should scale roughly linearly
    let first_rate = memory_per_block[0] as f64;
    let last_rate = memory_per_block.last().unwrap().clone() as f64;
    let scaling_factor = last_rate / first_rate;
    
    assert!(scaling_factor < 2.0, "Memory usage scaling too quickly: {}x", scaling_factor);
}

#[cfg(feature = "jemalloc")]
mod jemalloc_profiling {
    use super::*;
    use jemallocator::Jemalloc;
    
    #[global_allocator]
    static ALLOC: Jemalloc = Jemalloc;
    
    #[test]
    fn profile_with_jemalloc() {
        use jemalloc_ctl::{stats, epoch};
        
        // Advance epoch to get fresh stats
        epoch::advance().unwrap();
        let initial_allocated = stats::allocated::read().unwrap();
        
        // Run workload
        let blocks = (0..100).map(|i| load_test_block(17000000 + i)).collect::<Vec<_>>();
        for block in blocks {
            let _result = map_transfers(block).unwrap();
        }
        
        // Get final stats
        epoch::advance().unwrap();
        let final_allocated = stats::allocated::read().unwrap();
        
        println!("Jemalloc stats:");
        println!("  Initial allocated: {} bytes", initial_allocated);
        println!("  Final allocated: {} bytes", final_allocated);
        println!("  Net increase: {} bytes", final_allocated - initial_allocated);
        
        // Force garbage collection and check for leaks
        std::hint::black_box(());
        epoch::advance().unwrap();
        let after_gc = stats::allocated::read().unwrap();
        
        println!("  After GC: {} bytes", after_gc);
        
        if after_gc > initial_allocated + 1024 * 1024 { // 1MB tolerance
            panic!("Potential memory leak: {} bytes not freed", after_gc - initial_allocated);
        }
    }
}
```

### 5. Production Mode vs Development Mode

```rust
// tests/production_mode_tests.rs
use std::process::Command;
use std::time::{Duration, Instant};
use serde_json::Value;

#[test]
fn compare_dev_vs_production_mode() {
    let test_cases = vec![
        (1000, "small_range"),
        (5000, "medium_range"), 
        (10000, "large_range"),
    ];
    
    for (block_count, test_name) in test_cases {
        println!("🔬 Testing {} blocks ({})", block_count, test_name);
        
        // Test development mode
        let (dev_duration, dev_output) = run_substreams_timed(block_count, false);
        
        // Test production mode
        let (prod_duration, prod_output) = run_substreams_timed(block_count, true);
        
        // Compare outputs for correctness
        compare_outputs(&dev_output, &prod_output, test_name);
        
        // Analyze performance
        analyze_performance_difference(
            dev_duration, prod_duration, block_count, test_name
        );
    }
}

fn run_substreams_timed(block_count: u64, production_mode: bool) -> (Duration, String) {
    let start = Instant::now();
    
    let mut args = vec![
        "run",
        "-s", "17000000",
        "-t", &format!("+{}", block_count),
        "map_transfers",
        "--network", "mainnet",
        "--output-format", "json"
    ];
    
    if production_mode {
        args.push("--production-mode");
    }
    
    let output = Command::new("substreams")
        .args(&args)
        .output()
        .expect("Failed to run substreams");
    
    let duration = start.elapsed();
    
    assert!(output.status.success(), 
           "Substreams failed: {}", 
           String::from_utf8_lossy(&output.stderr));
    
    (duration, String::from_utf8_lossy(&output.stdout).to_string())
}

fn compare_outputs(dev_output: &str, prod_output: &str, test_name: &str) {
    let dev_lines: Vec<&str> = dev_output.lines().collect();
    let prod_lines: Vec<&str> = prod_output.lines().collect();
    
    // Parse JSON outputs and compare
    let mut dev_results = Vec::new();
    let mut prod_results = Vec::new();
    
    for line in dev_lines {
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            if let Some(block_num) = json.get("block_number") {
                dev_results.push((block_num.as_u64().unwrap(), json));
            }
        }
    }
    
    for line in prod_lines {
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            if let Some(block_num) = json.get("block_number") {
                prod_results.push((block_num.as_u64().unwrap(), json));
            }
        }
    }
    
    // Sort by block number for comparison
    dev_results.sort_by_key(|(block_num, _)| *block_num);
    prod_results.sort_by_key(|(block_num, _)| *block_num);
    
    assert_eq!(dev_results.len(), prod_results.len(),
              "{}: Different number of blocks processed", test_name);
    
    // Compare block by block
    for ((dev_block, dev_json), (prod_block, prod_json)) in dev_results.iter().zip(prod_results.iter()) {
        assert_eq!(dev_block, prod_block, 
                  "{}: Block number mismatch", test_name);
        
        // Compare transfer counts
        let dev_transfers = dev_json.get("transfers").and_then(|t| t.as_array()).map(|a| a.len()).unwrap_or(0);
        let prod_transfers = prod_json.get("transfers").and_then(|t| t.as_array()).map(|a| a.len()).unwrap_or(0);
        
        assert_eq!(dev_transfers, prod_transfers,
                  "{}: Transfer count mismatch for block {}", test_name, dev_block);
    }
    
    println!("✅ {}: Outputs match between dev and production mode", test_name);
}

fn analyze_performance_difference(
    dev_duration: Duration,
    prod_duration: Duration, 
    block_count: u64,
    test_name: &str
) {
    let dev_ms = dev_duration.as_millis();
    let prod_ms = prod_duration.as_millis();
    
    let speedup = dev_ms as f64 / prod_ms as f64;
    let dev_blocks_per_sec = block_count as f64 / dev_duration.as_secs_f64();
    let prod_blocks_per_sec = block_count as f64 / prod_duration.as_secs_f64();
    
    println!("📊 {} Performance Analysis:", test_name);
    println!("  Development mode: {}ms ({:.2} blocks/sec)", dev_ms, dev_blocks_per_sec);
    println!("  Production mode:  {}ms ({:.2} blocks/sec)", prod_ms, prod_blocks_per_sec);
    println!("  Speedup: {:.2}x", speedup);
    
    // Production mode should be faster for larger block ranges
    if block_count > 1000 {
        assert!(speedup > 1.1, 
               "{}: Production mode should be at least 10% faster, got {:.2}x", 
               test_name, speedup);
    }
    
    // Performance should be reasonable
    assert!(prod_blocks_per_sec > 10.0,
           "{}: Production mode too slow: {:.2} blocks/sec", 
           test_name, prod_blocks_per_sec);
}

#[test]
fn test_parallel_execution_scaling() {
    // Test how performance scales with different parallelism levels
    let block_count = 5000u64;
    
    // Test different configurations (this would require custom builds)
    let configs = vec![
        ("single_thread", vec!["--production-mode", "--parallelism=1"]),
        ("dual_thread", vec!["--production-mode", "--parallelism=2"]),
        ("quad_thread", vec!["--production-mode", "--parallelism=4"]),
        ("auto_thread", vec!["--production-mode"]),
    ];
    
    let mut results = Vec::new();
    
    for (name, extra_args) in configs {
        let start = Instant::now();
        
        let mut args = vec![
            "run", "-s", "17000000", "-t", &format!("+{}", block_count),
            "map_transfers", "--network", "mainnet"
        ];
        args.extend(extra_args);
        
        let output = Command::new("substreams")
            .args(&args)
            .output()
            .expect("Failed to run substreams");
        
        let duration = start.elapsed();
        
        assert!(output.status.success());
        
        let blocks_per_sec = block_count as f64 / duration.as_secs_f64();
        results.push((name, blocks_per_sec, duration));
        
        println!("{}: {:.2} blocks/sec ({:?})", name, blocks_per_sec, duration);
    }
    
    // Analyze scaling characteristics
    let single_perf = results.iter().find(|(name, _, _)| *name == "single_thread").unwrap().1;
    
    for (name, perf, _) in &results {
        if *name != "single_thread" {
            let scaling = perf / single_perf;
            println!("{} scaling vs single thread: {:.2}x", name, scaling);
        }
    }
}
```

### 6. Load Testing and Stress Testing

```bash
#!/bin/bash
# scripts/stress_test.sh

set -e

echo "🔥 Starting Substreams Stress Tests"

# Stress test configuration
STRESS_DURATION_MINS=30
MAX_CONCURRENT_JOBS=4
BLOCK_RANGES=(1000 2000 5000)
NETWORKS=("mainnet" "polygon")

# Create results directory
RESULTS_DIR="stress_results/$(date +%Y%m%d_%H%M%S)" 
mkdir -p "$RESULTS_DIR"

# Function to run stress job
run_stress_job() {
    local job_id=$1
    local network=$2
    local blocks=$3
    local start_block=$((17000000 + job_id * 10000))
    
    local log_file="$RESULTS_DIR/stress_job_${job_id}.log"
    
    echo "🚀 Starting stress job $job_id: $network, $blocks blocks from $start_block"
    
    timeout $((STRESS_DURATION_MINS * 60)) substreams run \
        -s $start_block \
        -t +$blocks \
        map_transfers \
        --network $network \
        --production-mode \
        > "$log_file" 2>&1 &
    
    echo $! > "$RESULTS_DIR/job_${job_id}.pid"
}

# Start concurrent stress jobs
job_counter=0
for network in "${NETWORKS[@]}"; do
    for blocks in "${BLOCK_RANGES[@]}"; do
        for ((i=0; i<MAX_CONCURRENT_JOBS; i++)); do
            run_stress_job $job_counter $network $blocks
            job_counter=$((job_counter + 1))
            sleep 5  # Stagger job starts
        done
    done
done

echo "👀 Started $job_counter stress jobs, running for ${STRESS_DURATION_MINS} minutes"

# Monitor jobs
end_time=$(($(date +%s) + STRESS_DURATION_MINS * 60))
while [ $(date +%s) -lt $end_time ]; do
    active_jobs=0
    
    for pid_file in "$RESULTS_DIR"/job_*.pid; do
        if [ -f "$pid_file" ]; then
            pid=$(cat "$pid_file")
            if kill -0 $pid 2>/dev/null; then
                active_jobs=$((active_jobs + 1))
            fi
        fi
    done
    
    echo "$(date): $active_jobs jobs still running"
    sleep 30
done

# Kill remaining jobs and collect results
echo "⏰ Time limit reached, stopping jobs"
for pid_file in "$RESULTS_DIR"/job_*.pid; do
    if [ -f "$pid_file" ]; then
        pid=$(cat "$pid_file")
        kill $pid 2>/dev/null || true
        rm "$pid_file"
    fi
done

# Analyze stress test results
echo "📊 Analyzing stress test results"

total_success=0
total_failure=0
total_blocks_processed=0

for log_file in "$RESULTS_DIR"/stress_job_*.log; do
    job_id=$(basename "$log_file" .log | sed 's/stress_job_//')
    
    if grep -q "error\|Error\|ERROR\|panic" "$log_file"; then
        echo "❌ Job $job_id failed"
        total_failure=$((total_failure + 1))
    else
        echo "✅ Job $job_id succeeded"
        total_success=$((total_success + 1))
        
        # Count blocks processed
        blocks=$(grep -c '"block_number"' "$log_file" 2>/dev/null || echo 0)
        total_blocks_processed=$((total_blocks_processed + blocks))
    fi
done

# Generate stress test report
cat > "$RESULTS_DIR/stress_report.json" << EOF
{
  "stress_test_summary": {
    "duration_minutes": $STRESS_DURATION_MINS,
    "total_jobs": $job_counter,
    "successful_jobs": $total_success,
    "failed_jobs": $total_failure,
    "success_rate": $(echo "scale=2; $total_success * 100 / $job_counter" | bc)"%",
    "total_blocks_processed": $total_blocks_processed,
    "avg_blocks_per_job": $(echo "scale=0; $total_blocks_processed / $total_success" | bc),
    "timestamp": "$(date -Iseconds)"
  }
}
EOF

echo "🎯 Stress Test Results:"
echo "  Total jobs: $job_counter"
echo "  Successful: $total_success"
echo "  Failed: $total_failure"
echo "  Success rate: $(echo "scale=1; $total_success * 100 / $job_counter" | bc)%"
echo "  Total blocks processed: $total_blocks_processed"

if [ $total_failure -gt $((job_counter / 4)) ]; then
    echo "⚠️  High failure rate detected (>25%)"
    exit 1
fi

echo "✅ Stress test completed successfully"
```

## Performance Analysis and Reporting

### Performance Metrics Collection

```python
#!/usr/bin/env python3
# scripts/performance_analyzer.py

import json
import sys
import os
import numpy as np
import matplotlib.pyplot as plt
from datetime import datetime
from pathlib import Path

class PerformanceAnalyzer:
    def __init__(self, results_dir):
        self.results_dir = Path(results_dir)
        self.metrics = []
        
    def load_results(self):
        """Load all performance test results"""
        for summary_file in self.results_dir.glob("*.summary"):
            with open(summary_file, 'r') as f:
                try:
                    data = json.load(f)
                    self.metrics.append(data)
                except json.JSONDecodeError:
                    print(f"Warning: Could not parse {summary_file}")
                    
    def analyze_scaling(self):
        """Analyze how performance scales with block count"""
        dev_mode = [m for m in self.metrics if m.get('mode') == '']
        prod_mode = [m for m in self.metrics if m.get('mode') == 'production']
        
        # Group by module and block count
        scaling_data = {}
        
        for metric in dev_mode + prod_mode:
            module = metric['module']
            mode = 'dev' if metric['mode'] == '' else 'prod'
            
            if module not in scaling_data:
                scaling_data[module] = {'dev': [], 'prod': []}
                
            scaling_data[module][mode].append({
                'block_count': metric['block_count'],
                'blocks_per_second': metric['blocks_per_second'],
                'duration': metric['duration_seconds'],
                'memory': metric.get('max_memory_kb', 0)
            })
        
        return scaling_data
        
    def generate_scaling_report(self, scaling_data):
        """Generate scaling analysis report"""
        report = {
            'scaling_analysis': {},
            'recommendations': []
        }
        
        for module, data in scaling_data.items():
            module_analysis = {}
            
            # Analyze dev mode scaling
            if data['dev']:
                dev_data = sorted(data['dev'], key=lambda x: x['block_count'])
                block_counts = [d['block_count'] for d in dev_data]
                throughputs = [d['blocks_per_second'] for d in dev_data]
                
                # Calculate scaling efficiency
                if len(throughputs) > 1:
                    scaling_efficiency = throughputs[-1] / throughputs[0]
                    module_analysis['dev_scaling_efficiency'] = scaling_efficiency
                    
                    if scaling_efficiency < 0.5:  # 50% efficiency threshold
                        report['recommendations'].append(
                            f"{module}: Development mode scaling poorly ({scaling_efficiency:.2f})"
                        )
            
            # Analyze production mode scaling
            if data['prod']:
                prod_data = sorted(data['prod'], key=lambda x: x['block_count'])
                prod_throughputs = [d['blocks_per_second'] for d in prod_data]
                
                if len(prod_throughputs) > 1:
                    prod_efficiency = prod_throughputs[-1] / prod_throughputs[0]
                    module_analysis['prod_scaling_efficiency'] = prod_efficiency
                    
                    # Compare dev vs prod
                    if data['dev'] and len(throughputs) > 1:
                        speedup = prod_throughputs[-1] / throughputs[-1]
                        module_analysis['prod_speedup'] = speedup
                        
                        if speedup < 1.5:  # Expected at least 1.5x speedup
                            report['recommendations'].append(
                                f"{module}: Production mode speedup suboptimal ({speedup:.2f}x)"
                            )
            
            report['scaling_analysis'][module] = module_analysis
            
        return report
        
    def detect_performance_regressions(self, baseline_file=None):
        """Compare current results against baseline"""
        if baseline_file and os.path.exists(baseline_file):
            with open(baseline_file, 'r') as f:
                baseline = json.load(f)
                
            regressions = []
            improvements = []
            
            for metric in self.metrics:
                key = f"{metric['module']}_{metric['block_count']}_{metric['mode']}"
                
                if key in baseline:
                    baseline_perf = baseline[key]['blocks_per_second']
                    current_perf = metric['blocks_per_second']
                    
                    change = (current_perf - baseline_perf) / baseline_perf
                    
                    if change < -0.1:  # 10% regression threshold
                        regressions.append({
                            'test': key,
                            'baseline': baseline_perf,
                            'current': current_perf,
                            'change_percent': change * 100
                        })
                    elif change > 0.1:  # 10% improvement threshold
                        improvements.append({
                            'test': key,
                            'baseline': baseline_perf, 
                            'current': current_perf,
                            'change_percent': change * 100
                        })
            
            return {'regressions': regressions, 'improvements': improvements}
        
        return None
        
    def generate_charts(self, output_dir):
        """Generate performance visualization charts"""
        os.makedirs(output_dir, exist_ok=True)
        
        # Scaling chart
        scaling_data = self.analyze_scaling()
        
        for module, data in scaling_data.items():
            plt.figure(figsize=(12, 8))
            
            if data['dev']:
                dev_sorted = sorted(data['dev'], key=lambda x: x['block_count'])
                dev_blocks = [d['block_count'] for d in dev_sorted]
                dev_throughput = [d['blocks_per_second'] for d in dev_sorted]
                plt.plot(dev_blocks, dev_throughput, 'o-', label='Development Mode', linewidth=2)
                
            if data['prod']:
                prod_sorted = sorted(data['prod'], key=lambda x: x['block_count'])
                prod_blocks = [d['block_count'] for d in prod_sorted]
                prod_throughput = [d['blocks_per_second'] for d in prod_sorted]
                plt.plot(prod_blocks, prod_throughput, 's-', label='Production Mode', linewidth=2)
            
            plt.xlabel('Block Count')
            plt.ylabel('Blocks per Second')
            plt.title(f'Performance Scaling - {module}')
            plt.legend()
            plt.grid(True, alpha=0.3)
            plt.savefig(f'{output_dir}/scaling_{module}.png', dpi=300, bbox_inches='tight')
            plt.close()
            
    def generate_report(self):
        """Generate comprehensive performance report"""
        scaling_data = self.analyze_scaling()
        scaling_report = self.generate_scaling_report(scaling_data)
        regression_report = self.detect_performance_regressions()
        
        report = {
            'timestamp': datetime.now().isoformat(),
            'summary': {
                'total_tests': len(self.metrics),
                'successful_tests': len([m for m in self.metrics if m.get('success', True)]),
                'modules_tested': list(set(m['module'] for m in self.metrics)),
                'networks_tested': list(set(m['network'] for m in self.metrics))
            },
            'scaling_analysis': scaling_report,
            'regression_analysis': regression_report,
            'raw_metrics': self.metrics
        }
        
        return report

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 performance_analyzer.py <results_directory>")
        sys.exit(1)
        
    results_dir = sys.argv[1]
    analyzer = PerformanceAnalyzer(results_dir)
    analyzer.load_results()
    
    # Generate report
    report = analyzer.generate_report()
    
    # Save report
    report_file = os.path.join(results_dir, 'performance_report.json')
    with open(report_file, 'w') as f:
        json.dump(report, f, indent=2)
        
    # Generate charts
    analyzer.generate_charts(os.path.join(results_dir, 'charts'))
    
    # Print summary
    print("📊 Performance Analysis Complete")
    print(f"   Report saved to: {report_file}")
    print(f"   Tests analyzed: {report['summary']['total_tests']}")
    
    if report['regression_analysis'] and report['regression_analysis']['regressions']:
        print(f"   ⚠️  Performance regressions detected: {len(report['regression_analysis']['regressions'])}")
        for reg in report['regression_analysis']['regressions']:
            print(f"     - {reg['test']}: {reg['change_percent']:.1f}% slower")
    
    if report['scaling_analysis']['recommendations']:
        print("   💡 Recommendations:")
        for rec in report['scaling_analysis']['recommendations']:
            print(f"     - {rec}")

if __name__ == "__main__":
    main()
```

## Best Practices for Performance Testing

### DO

✅ **Test at multiple scales** - Single blocks to large ranges  
✅ **Use realistic data** - Real blockchain blocks with typical activity  
✅ **Benchmark critical paths** - Focus on most expensive operations  
✅ **Test both modes** - Development and production mode performance  
✅ **Monitor memory usage** - Detect leaks and excessive allocation  
✅ **Establish baselines** - Track performance over time  
✅ **Automate testing** - Include performance tests in CI/CD  
✅ **Profile regularly** - Use profiling tools to identify bottlenecks  
✅ **Test resource limits** - Memory, CPU, and disk constraints  
✅ **Document expectations** - Set clear performance requirements  

### DON'T

❌ **Only test happy path** - Include error scenarios in performance tests  
❌ **Ignore memory patterns** - Memory usage is as important as speed  
❌ **Test only single-threaded** - Production uses parallel execution  
❌ **Skip baseline comparisons** - Performance can regress silently  
❌ **Use only synthetic data** - Real blockchain data has different characteristics  
❌ **Test only small ranges** - Large-scale performance matters  
❌ **Ignore warm-up time** - JIT compilation affects initial performance  
❌ **Test only latest blocks** - Historical blocks may have different patterns  

### Performance Requirements

Define clear performance targets:

1. **Throughput**: Target blocks per second for different scenarios
2. **Latency**: Maximum acceptable processing time per block
3. **Memory**: Maximum memory usage and leak tolerance  
4. **Scalability**: Performance degradation limits as load increases
5. **Reliability**: Maximum acceptable failure rate under load

This comprehensive performance testing approach ensures your Substreams can handle production workloads efficiently and reliably.