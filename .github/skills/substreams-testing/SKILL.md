---
name: substreams-testing
description: Expert knowledge for testing Substreams applications. Covers unit testing, integration testing, performance testing, and FireCode tools usage.
license: Apache-2.0
compatibility:
  platforms: [claude-code, cursor, vscode, windsurf]
metadata:
  version: 1.0.0
  author: StreamingFast
  documentation: https://substreams.streamingfast.io
---

# Substreams Testing Expert

Expert assistant for testing Substreams applications - ensuring reliability, correctness, and performance of blockchain data processing.

## Core Concepts

### Why Test Substreams?

Substreams testing is critical because:
- **Blockchain data is immutable** - mistakes are permanent and costly
- **High throughput** - small bugs amplify across millions of blocks
- **Complex transformations** - multi-stage processing introduces edge cases
- **Real money** - DeFi applications depend on accuracy
- **Parallel execution** - race conditions and state consistency issues

### Testing Philosophy

1. **Test early and often** - Catch issues in development, not production
2. **Test with real data** - Blockchain edge cases are numerous and unexpected
3. **Test at multiple scales** - Single blocks, small ranges, and large datasets
4. **Test reorg scenarios** - Blockchain reorganizations must be handled correctly
5. **Performance testing** - Ensure production scalability

## Testing Pyramid

### Unit Tests (Foundation)

Test individual functions and modules in isolation.

**What to Test**:
- ✅ Data parsing and validation
- ✅ Business logic calculations  
- ✅ Error handling edge cases
- ✅ Protobuf message construction
- ✅ Helper functions and utilities

**Test Structure**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use substreams_ethereum::pb::eth::v2::{Block, TransactionTrace, Log};
    
    #[test]
    fn test_parse_erc20_transfer() {
        // Arrange
        let log = create_test_transfer_log(
            "0xa0b86a33e6fe17d67c8b086c6c4c0e3c8e3b7ec2", // USDC
            "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1", // from
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed", // to  
            "1000000000000000000" // 1 token (18 decimals)
        );
        
        // Act
        let result = parse_erc20_transfer(&log);
        
        // Assert
        assert!(result.is_ok());
        let transfer = result.unwrap();
        assert_eq!(transfer.contract, "0xa0b86a33e6fe17d67c8b086c6c4c0e3c8e3b7ec2");
        assert_eq!(transfer.from, "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1");
        assert_eq!(transfer.to, "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
        assert_eq!(transfer.amount, BigInt::from_str("1000000000000000000").unwrap());
    }
    
    #[test]
    fn test_invalid_transfer_log() {
        // Test with malformed log
        let invalid_log = create_test_log_with_insufficient_topics();
        
        let result = parse_erc20_transfer(&invalid_log);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("insufficient topics"));
    }
    
    #[test] 
    fn test_zero_amount_transfer() {
        let log = create_test_transfer_log(
            "0xa0b86a33e6fe17d67c8b086c6c4c0e3c8e3b7ec2",
            "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1", 
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
            "0" // Zero amount
        );
        
        let result = parse_erc20_transfer(&log);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().amount, BigInt::zero());
    }
    
    // Test helper functions
    fn create_test_transfer_log(contract: &str, from: &str, to: &str, amount: &str) -> Log {
        Log {
            address: hex::decode(&contract[2..]).unwrap(),
            topics: vec![
                hex::decode("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").unwrap(), // Transfer event signature
                hex::decode(&format!("000000000000000000000000{}", &from[2..])).unwrap(), // from (padded)
                hex::decode(&format!("000000000000000000000000{}", &to[2..])).unwrap(), // to (padded)
            ],
            data: hex::decode(&format!("{:0>64}", &amount)).unwrap(), // amount (32 bytes)
            ..Default::default()
        }
    }
}
```

### Integration Tests (Core)

Test complete modules with real blockchain data.

> **Note:** There is no official `substreams::test_utils` module. Integration tests use standard Rust test infrastructure with manually constructed test data or block fixtures.

**Setup Integration Testing**:
```rust
// tests/integration_tests.rs
use your_substreams::*;

#[test]
fn test_map_transfers_integration() {
    // Use real Ethereum block with known transfers
    let block = load_test_block(17000000);
    
    let result = map_transfers(block).unwrap();
    
    // Verify expected transfers were found
    assert!(result.transfers.len() > 0);
    
    // Check specific known transfer
    let usdc_transfers: Vec<_> = result.transfers
        .iter()
        .filter(|t| t.contract == "0xa0b86a33e6fe17d67c8b086c6c4c0e3c8e3b7ec2")
        .collect();
    
    assert!(usdc_transfers.len() > 0);
    
    // Validate data integrity
    for transfer in &result.transfers {
        assert!(transfer.amount > BigInt::zero());
        assert_ne!(transfer.from, transfer.to);
        assert_eq!(transfer.from.len(), 42); // Valid Ethereum address
        assert_eq!(transfer.to.len(), 42);
    }
}

#[test]
fn test_store_balances_integration() {
    // Test balance calculation with multiple blocks
    let blocks = load_test_blocks(17000000..17000010);
    let mut store = create_test_store();
    
    for block in blocks {
        let transfers = map_transfers(block).unwrap();
        store_balances(transfers, &store);
    }
    
    // Verify balance consistency
    let alice = "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1";
    let usdc = "0xa0b86a33e6fe17d67c8b086c6c4c0e3c8e3b7ec2";
    
    let balance = store.get_last(&format!("{}:{}", usdc, alice));
    assert!(balance.is_some());
    assert!(balance.unwrap() >= 0);
}

// Test data loading helpers
fn load_test_block(block_number: u64) -> Block {
    // Load from fixtures or fetch from RPC
    let block_data = std::fs::read(format!("fixtures/block_{}.bin", block_number))
        .expect("Test block data not found");
    
    Block::decode(block_data.as_slice()).expect("Invalid block data")
}

fn load_test_blocks(range: std::ops::Range<u64>) -> Vec<Block> {
    range.map(|n| load_test_block(n)).collect()
}

fn create_test_store() -> TestStore {
    TestStore::new()
}
```

### End-to-End Tests (Integration)

Test complete Substreams with real execution environment.

**E2E Test Framework**:
```rust
// tests/e2e_tests.rs
use std::process::Command;
use serde_json::Value;

#[test]
fn test_full_substreams_execution() {
    // Run substreams with test configuration
    let output = Command::new("substreams")
        .args(&[
            "run",
            "-s", "17000000",  // Start block
            "-t", "+100",      // 100 blocks
            "map_transfers",   // Module to test
            "--network", "mainnet"
        ])
        .output()
        .expect("Failed to execute substreams");
    
    assert!(output.status.success(), 
           "Substreams execution failed: {}", 
           String::from_utf8_lossy(&output.stderr));
    
    // Parse output and validate
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    
    // Should have output for each block
    assert!(lines.len() >= 100);
    
    // Check for valid JSON output
    for line in &lines {
        if line.starts_with("{") {
            let json: Value = serde_json::from_str(line)
                .expect("Invalid JSON output");
            
            // Validate structure
            assert!(json["transfers"].is_array());
            assert!(json["block_number"].is_u64());
        }
    }
}

#[test] 
fn test_performance_requirements() {
    let start = std::time::Instant::now();
    
    let output = Command::new("substreams")
        .args(&[
            "run", 
            "-s", "17000000",
            "-t", "+1000",  // 1000 blocks
            "map_transfers",
            "--production-mode"  // Enable parallel execution
        ])
        .output()
        .expect("Failed to execute substreams");
    
    let duration = start.elapsed();
    
    assert!(output.status.success());
    
    // Performance requirement: should process 1000 blocks in under 60 seconds
    assert!(duration.as_secs() < 60, 
           "Processing took too long: {}s", duration.as_secs());
    
    println!("Processed 1000 blocks in {:?}", duration);
}
```

## Testing with FireCore Tools

### Firehose Data Access

FireCore CLI (`firecore`) provides blockchain data access for testing. Use it to fetch blocks for test fixtures:

```bash
# Fetch a single block as JSON (useful for creating test fixtures)
firecore tools firehose-client mainnet -o json -- 17000000 +1

# Fetch a range of blocks
firecore tools firehose-client mainnet -o json -- 17000000 +100

# Get the current head block
firecore tools firehose-client mainnet -o text -- -1
```

> **Note:** There is no `firehose` Rust crate for loading blocks. Use `substreams_ethereum::pb::eth::v2::Block` with `prost::Message::decode()` for protobuf fixtures, or construct test blocks manually.

**Using Block Fixtures in Tests**:
```rust
use substreams_ethereum::pb::eth::v2::Block;
use prost::Message;
use std::fs;

fn load_test_block(block_number: u64) -> Block {
    // Load protobuf-encoded block fixture
    let data = fs::read(format!("fixtures/block_{}.bin", block_number))
        .expect("Test block fixture not found");

    Block::decode(data.as_slice())
        .expect("Failed to decode block")
}

#[test]
fn test_with_real_block_data() {
    let block = load_test_block(17000000);

    let result = map_transfers(block).unwrap();

    // Validate against known expected results
    assert!(!result.transfers.is_empty());
    for transfer in &result.transfers {
        assert_eq!(transfer.from.len(), 42);
        assert_eq!(transfer.to.len(), 42);
    }
}
```

### CLI-based Integration Testing

For integration testing, use the `substreams` CLI to run modules against real data and validate the output:

```bash
# Run a module for a specific block range and capture output
substreams run -s 17000000 -t +100 map_transfers --network mainnet -o jsonl > output.jsonl

# Validate output is non-empty
test -s output.jsonl || (echo "No output produced" && exit 1)

# Check for specific expected content
grep -q "transfers" output.jsonl || (echo "Missing transfers field" && exit 1)
```

For more structured integration tests, wrap `substreams run` in Rust using `std::process::Command` (see E2E Tests section above).

## Performance Testing

### Benchmarking Module Performance

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use your_substreams::*;

fn benchmark_map_transfers(c: &mut Criterion) {
    let block = load_test_block(17000000); // Heavy block with many transfers
    
    c.bench_function("map_transfers", |b| {
        b.iter(|| {
            black_box(map_transfers(black_box(block.clone())))
        })
    });
}

fn benchmark_store_operations(c: &mut Criterion) {
    let transfers = load_test_transfers();
    let store = create_test_store();
    
    c.bench_function("store_balances", |b| {
        b.iter(|| {
            store_balances(black_box(transfers.clone()), black_box(&store))
        })
    });
}

criterion_group!(benches, benchmark_map_transfers, benchmark_store_operations);
criterion_main!(benches);
```

**Running Benchmarks**:
```bash
# Install criterion
cargo add --dev criterion

# Run benchmarks
cargo bench

# Compare performance across changes
cargo bench -- --save-baseline before
# Make changes...
cargo bench -- --baseline before
```

### Production Mode Testing

```bash
#!/bin/bash
# Performance test script

echo "Testing development mode..."
time substreams run -s 17000000 -t +1000 map_transfers > /tmp/dev_output.txt

echo "Testing production mode..."  
time substreams run -s 17000000 -t +1000 map_transfers --production-mode > /tmp/prod_output.txt

# Compare outputs for correctness
diff /tmp/dev_output.txt /tmp/prod_output.txt
if [ $? -eq 0 ]; then
    echo "✅ Outputs match between dev and production mode"
else
    echo "❌ Output mismatch between modes"
    exit 1
fi

# Performance analysis
echo "Development mode timing:"
grep "real" /tmp/dev_time.txt

echo "Production mode timing:"  
grep "real" /tmp/prod_time.txt
```

### Resource Testing

```rust
#[test]
fn test_large_dataset_processing() {
    // Test with a larger block range to ensure scalability
    let result = std::process::Command::new("substreams")
        .args(&[
            "run",
            "-s", "17000000",
            "-t", "+10000", // 10K blocks
            "map_transfers",
            "--production-mode",
            "--network", "mainnet",
        ])
        .output()
        .expect("Failed to run large dataset test");

    assert!(result.status.success());

    // Verify no out-of-memory errors
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!stderr.contains("out of memory"));
    assert!(!stderr.contains("killed"));
}
```

## Testing Patterns and Best Practices

### Test Data Management

```rust
// tests/fixtures.rs
pub struct TestDataBuilder {
    block: Block,
}

impl TestDataBuilder {
    pub fn new(block_number: u64) -> Self {
        Self {
            block: Block {
                number: block_number,
                timestamp_seconds: 1680000000,
                ..Default::default()
            }
        }
    }
    
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.block.timestamp_seconds = timestamp;
        self
    }
    
    pub fn add_erc20_transfer(
        mut self, 
        contract: &str,
        from: &str, 
        to: &str, 
        amount: &str
    ) -> Self {
        let tx = TransactionTrace {
            hash: generate_tx_hash(),
            receipt: Some(TransactionReceipt {
                logs: vec![create_transfer_log(contract, from, to, amount)],
                ..Default::default()
            }),
            ..Default::default()
        };
        
        self.block.transaction_traces.push(tx);
        self
    }
    
    pub fn add_uniswap_swap(
        mut self,
        pool: &str,
        user: &str,
        amount0_in: &str,
        amount1_out: &str
    ) -> Self {
        let tx = TransactionTrace {
            hash: generate_tx_hash(),
            receipt: Some(TransactionReceipt {
                logs: vec![create_swap_log(pool, user, amount0_in, amount1_out)],
                ..Default::default()
            }),
            ..Default::default()
        };
        
        self.block.transaction_traces.push(tx);
        self
    }
    
    pub fn build(self) -> Block {
        self.block
    }
}

// Usage in tests
#[test]
fn test_complex_scenario() {
    let block = TestDataBuilder::new(17000000)
        .with_timestamp(1680000000)
        .add_erc20_transfer(
            "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2", // USDC
            "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1", // Alice
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed", // Bob
            "1000000000" // 1000 USDC (6 decimals)
        )
        .add_uniswap_swap(
            "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640", // USDC/ETH pool
            "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1", // Alice
            "1000000000", // 1000 USDC in
            "500000000000000000" // 0.5 ETH out
        )
        .build();
    
    let result = map_transfers(block).unwrap();
    
    // Test the complex interaction
    assert_eq!(result.transfers.len(), 3); // Transfer + 2 swap transfers
}
```

### Property-Based Testing

```rust
use quickcheck::*;

#[derive(Clone, Debug)]
struct ArbitraryTransfer {
    contract: String,
    from: String,
    to: String,
    amount: u64,
}

impl Arbitrary for ArbitraryTransfer {
    fn arbitrary(g: &mut Gen) -> ArbitraryTransfer {
        ArbitraryTransfer {
            contract: format!("0x{:040x}", u64::arbitrary(g) as u128),
            from: format!("0x{:040x}", u64::arbitrary(g) as u128),
            to: format!("0x{:040x}", u64::arbitrary(g) as u128),
            amount: u64::arbitrary(g),
        }
    }
}

quickcheck! {
    fn prop_transfer_parsing_preserves_data(transfer: ArbitraryTransfer) -> bool {
        // Create log from transfer data
        let log = create_transfer_log(
            &transfer.contract,
            &transfer.from,
            &transfer.to,
            &transfer.amount.to_string()
        );
        
        // Parse it back
        match parse_erc20_transfer(&log) {
            Ok(parsed) => {
                parsed.contract == transfer.contract &&
                parsed.from == transfer.from &&
                parsed.to == transfer.to &&
                parsed.amount == BigInt::from(transfer.amount)
            },
            Err(_) => {
                // Some random data might be invalid, that's OK
                transfer.from == transfer.to || // Self-transfer
                transfer.amount == 0 || // Zero amount
                transfer.contract.len() != 42 // Invalid address
            }
        }
    }
    
    fn prop_balance_calculation_is_consistent(transfers: Vec<ArbitraryTransfer>) -> bool {
        let block = create_block_with_transfers(&transfers);
        let result = map_transfers(block).unwrap();
        
        // Sum of all amounts should be conserved
        let total_out: BigInt = result.transfers.iter()
            .map(|t| &t.amount)
            .sum();
            
        let expected_total: BigInt = transfers.iter()
            .filter(|t| t.from != t.to) // Exclude self-transfers
            .map(|t| BigInt::from(t.amount))
            .sum();
            
        total_out == expected_total
    }
}
```

### Reorg Testing

```rust
#[test]
fn test_reorganization_handling() {
    // Simulate a blockchain reorganization
    let original_blocks = vec![
        create_block(17000000, "0xabc123", "0x000000"),
        create_block(17000001, "0xdef456", "0xabc123"),
        create_block(17000002, "0x789xyz", "0xdef456"),
    ];
    
    // Alternative chain (reorg)
    let reorg_blocks = vec![
        create_block(17000000, "0xabc123", "0x000000"), // Same
        create_block(17000001, "0x111aaa", "0xabc123"), // Different
        create_block(17000002, "0x222bbb", "0x111aaa"), // Different
        create_block(17000003, "0x333ccc", "0x222bbb"), // New
    ];
    
    let mut store = create_test_store();
    
    // Process original chain
    for block in &original_blocks {
        let transfers = map_transfers(block.clone()).unwrap();
        store_balances(transfers, &store);
    }
    
    let balance_after_original = store.get_last("USDC:alice").unwrap_or(0);
    
    // Process reorg (Substreams handles the undo/redo automatically)
    // In reality, this would be handled by the Substreams engine
    for block in &reorg_blocks {
        let transfers = map_transfers(block.clone()).unwrap();
        store_balances(transfers, &store);
    }
    
    let balance_after_reorg = store.get_last("USDC:alice").unwrap_or(0);
    
    // Verify balances are correct after reorg
    // This depends on your specific test scenario
    assert_ne!(balance_after_original, balance_after_reorg);
    
    println!("Balance before reorg: {}", balance_after_original);
    println!("Balance after reorg: {}", balance_after_reorg);
}

fn create_block(number: u64, hash: &str, parent_hash: &str) -> Block {
    Block {
        number,
        hash: hex::decode(&hash[2..]).unwrap(),
        parent_hash: hex::decode(&parent_hash[2..]).unwrap(),
        timestamp_seconds: 1680000000 + number * 12, // 12 second blocks
        ..Default::default()
    }
}
```

### Error Scenario Testing

```rust
#[test]
fn test_malformed_data_handling() {
    let test_cases = vec![
        ("empty_topics", create_log_with_empty_topics()),
        ("insufficient_topics", create_log_with_one_topic()),
        ("invalid_address", create_log_with_invalid_address()),
        ("corrupted_data", create_log_with_corrupted_data()),
        ("oversized_amount", create_log_with_oversized_amount()),
    ];
    
    for (case_name, log) in test_cases {
        let result = parse_erc20_transfer(&log);
        
        match result {
            Ok(transfer) => {
                // Some malformed data might still parse
                println!("Case '{}' unexpectedly succeeded: {:?}", case_name, transfer);
            }
            Err(e) => {
                // Expected for malformed data
                println!("Case '{}' correctly failed: {}", case_name, e);
                
                // Verify error contains useful information
                assert!(e.to_string().len() > 0);
                assert!(!e.to_string().contains("panic"));
            }
        }
    }
}

#[test]
fn test_extreme_values() {
    // Test with maximum possible values
    let max_amount = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    
    let log = create_transfer_log(
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2",
        "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed", 
        max_amount
    );
    
    let result = parse_erc20_transfer(&log);
    assert!(result.is_ok());
    
    let transfer = result.unwrap();
    assert!(transfer.amount > BigInt::zero());
}
```

## Continuous Integration Setup

### GitHub Actions Configuration

```yaml
# .github/workflows/test.yml
name: Test Substreams

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
        override: true
        components: rustfmt, clippy
        
    - name: Add WASM target
      run: rustup target add wasm32-unknown-unknown
      
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Run unit tests
      run: cargo test --lib
      
    - name: Run clippy
      run: cargo clippy -- -D warnings
      
    - name: Check formatting
      run: cargo fmt -- --check

  integration-tests:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Substreams CLI
      run: |
        # Install latest substreams CLI
        curl -sSL https://github.com/streamingfast/substreams/releases/latest/download/substreams_linux_x86_64.tar.gz | tar -xz
        sudo mv substreams /usr/local/bin/
        
    - name: Download test fixtures
      run: |
        mkdir -p fixtures
        curl -L https://github.com/your-org/substreams-fixtures/releases/download/v1.0/ethereum-blocks.tar.gz | tar -xz -C fixtures/
        
    - name: Build Substreams
      run: substreams build
      
    - name: Run integration tests
      run: |
        substreams run -s 17000000 -t +100 map_transfers --network mainnet
        
    - name: Run performance tests
      run: |
        time substreams run -s 17000000 -t +1000 map_transfers --production-mode --network mainnet
        
  e2e-tests:
    runs-on: ubuntu-latest
    if: github.event_name == 'pull_request'
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup test environment
      run: |
        docker-compose -f docker-compose.test.yml up -d
        sleep 30  # Wait for services to be ready
        
    - name: Run end-to-end tests
      run: |
        cargo test --test e2e_tests
        
    - name: Cleanup
      run: docker-compose -f docker-compose.test.yml down
```

### Pre-commit Hooks

```bash
#!/bin/bash
# .git/hooks/pre-commit

set -e

echo "Running pre-commit checks..."

# Format code
echo "🔧 Formatting code..."
cargo fmt

# Run clippy
echo "🔍 Running clippy..."
cargo clippy -- -D warnings

# Run unit tests
echo "🧪 Running unit tests..."
cargo test --lib --quiet

# Build WASM
echo "🏗️  Building WASM..."
cargo build --target wasm32-unknown-unknown --release

# Quick smoke test
echo "💨 Smoke test..."
substreams build
substreams run -s 17000000 -t +10 map_transfers --network mainnet > /dev/null

echo "✅ All pre-commit checks passed!"
```

## Testing Best Practices

### DO

✅ **Start with unit tests** - Test individual functions first
✅ **Use real blockchain data** - Edge cases are everywhere
✅ **Test error conditions** - Malformed data, network issues, etc.
✅ **Benchmark performance** - Measure before optimizing  
✅ **Test reorg scenarios** - Blockchain reorganizations happen
✅ **Automate testing** - CI/CD pipeline for every change
✅ **Test at multiple scales** - Single blocks to large ranges
✅ **Use property-based testing** - Find edge cases automatically
✅ **Mock external dependencies** - Control test environment
✅ **Document test cases** - Explain why tests exist

### DON'T

❌ **Skip integration tests** - Unit tests aren't enough
❌ **Test only happy path** - Error cases are critical
❌ **Use only synthetic data** - Real blockchain data has surprises
❌ **Ignore performance** - Scalability matters for production
❌ **Test in production first** - Catch issues early
❌ **Hardcode test data** - Use builders and fixtures
❌ **Skip CI/CD setup** - Manual testing doesn't scale
❌ **Test modules in isolation only** - End-to-end flows matter
❌ **Ignore memory/resource usage** - Performance isn't just speed
❌ **Forget to test edge cases** - Zero values, max values, etc.

### Testing Strategy Summary

1. **Test Pyramid**: Unit (many) → Integration (some) → E2E (few)
2. **Data Strategy**: Real blockchain data with synthetic edge cases
3. **Performance Strategy**: Benchmark early, test at scale
4. **Error Strategy**: Test failures, malformed data, network issues
5. **Automation Strategy**: CI/CD pipeline with comprehensive testing
6. **Reorg Strategy**: Test blockchain reorganization scenarios

## Resources

* [Unit Testing Guide](./references/unit-testing.md)
* [Integration Testing Patterns](./references/integration-testing.md)
* [Performance Testing Guide](./references/performance-testing.md)
* [FireCore Tools Documentation](./references/firecore-tools.md)

## Getting Help

* [Substreams Discord](https://discord.gg/streamingfast)
* [Testing Documentation](https://substreams.streamingfast.io/documentation/develop/test)  
* [GitHub Issues](https://github.com/streamingfast/substreams/issues)